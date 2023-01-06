use cidre::{arc, mps, mps::graph, ns};

fn make_graph(synchonize: bool) -> arc::R<graph::Graph> {
    let mut graph = graph::Graph::new();

    let options = if synchonize {
        graph::Options::SynchronizeResults
    } else {
        graph::Options::None
    };

    graph.set_options(options);
    graph
}

fn load_const(
    graph: &graph::Graph,
    name: &str,
    shape: &[&ns::Number],
    fp32: bool,
) -> arc::R<graph::Tensor> {
    let (prefix, data_type, size) = if fp32 {
        ("_fp32", mps::DataType::F32, 4)
    } else {
        ("", mps::DataType::Float16, 2)
    };

    let data = ns::Data::with_contents_of_file_options(
        &ns::String::with_str(&format!("bins/{name}{prefix}.bin")),
        ns::DataReadingOptions::MAPPED_ALWAYS,
    )
    .unwrap();

    let numels = shape.iter().fold(1, |acc, x| acc * x.as_i64());
    assert_eq!(numels * size, data.length() as i64, "mismatched data sizes");

    let shape = ns::Array::from_slice(shape);
    graph.constant_with_data_shape_data_type(&data, &shape, data_type)
}

fn make_conv(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
    out_channels: &ns::Number,
    khw: &ns::Number,
    stride: usize,
    bias: bool,
) -> arc::R<graph::Tensor> {
    let weights = load_const(
        graph,
        &format!("{name}.weight"),
        &[out_channels, &x_in.shape().unwrap()[3], khw, khw],
        false,
    );

    let p = khw.as_i64() as usize / 2;

    let conv_desc = graph::Convolution2DOpDescriptor::with(
        stride,
        stride,
        1,
        1,
        1,
        p,
        p,
        p,
        p,
        graph::PaddingStyle::Explicit,
        graph::TensorNamedDataLayout::NHWC,
        graph::TensorNamedDataLayout::OIHW,
    )
    .unwrap();

    let conv = graph.convolution_2d(x_in, &weights, &conv_desc, None);

    if bias {
        let one = ns::Number::with_i64(1);
        let b = load_const(
            graph,
            &format!("{name}.bias"),
            &[&one, &one, &one, out_channels],
            false,
        );
        return graph.add(&conv, &b, None);
    }
    conv
}

fn make_upsample_nearest(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    scale_factor: i64,
) -> arc::R<graph::Tensor> {
    let in_shape = x_in.shape().unwrap();
    let shape: &[&ns::Number] = &[
        &ns::Number::with_i64(in_shape[1].as_i64() * scale_factor),
        &ns::Number::with_i64(in_shape[2].as_i64() * scale_factor),
    ];
    graph.resize(
        x_in,
        &mps::Shape::from_slice(shape),
        graph::ResizeMode::Nearest,
        true,
        false,
        graph::TensorNamedDataLayout::NHWC,
        None,
    )
}

fn make_group_norm(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
) -> arc::R<graph::Tensor> {
    let mut x = x_in.retained();
    if x_in.shape().unwrap().len() == 3 {
        x = graph.expand_dims(&x, 1, None);
    }

    let shape = x.shape().unwrap();
    let n_groups = ns::Number::with_f32(32f32);
    let n_grouped = ns::Number::with_f32(shape[3].as_f32() / n_groups.as_f32());

    let one = ns::Number::with_i32(1);
    let sh: &[&ns::Number] = &[&one, &one, &one, &n_groups, &n_grouped];

    let gamma = load_const(graph, &format!("{name}.weight"), sh, false);
    let beta = load_const(graph, &format!("{name}.bias"), sh, false);

    let x = graph.reshape(
        &x,
        &mps::Shape::from_slice(&[&shape[0], &shape[1], &shape[2], &n_groups, &n_grouped]),
        None,
    );

    let axes: &[&ns::Number] = &[&one, &ns::Number::with_i32(2), &ns::Number::with_i32(4)];
    let axes = ns::Array::from_slice(axes);
    let mean = graph.mean(&x, &axes, None);
    let variance = graph.variance(&x, &axes, None);
    let x = graph.normalize(&x, &mean, &variance, Some(&gamma), Some(&beta), 1e-5, None);

    graph.reshape(&x, x_in.shape().unwrap(), None)
}

fn make_swish(graph: &graph::Graph, x_in: &graph::Tensor) -> arc::R<graph::Tensor> {
    graph.mul(x_in, &graph.sigmoid(x_in, None), None)
}

fn make_group_norm_swish(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
) -> arc::R<graph::Tensor> {
    make_swish(graph, &make_group_norm(graph, x_in, name))
}

fn make_decoder_res_block(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
    out_channels: &ns::Number,
) -> arc::R<graph::Tensor> {
    let x = make_group_norm_swish(graph, x_in, &format!("{name}.norm1"));
    let x = make_conv(
        graph,
        &x,
        &format!("{name}.conv1"),
        out_channels,
        &ns::Number::with_i32(3),
        1,
        false,
    );
    let x = make_group_norm_swish(graph, &x, &format!("{name}.norm2"));
    let x = make_conv(
        graph,
        &x,
        &format!("{name}.conv2"),
        out_channels,
        &ns::Number::with_i32(3),
        1,
        false,
    );
    let skip = ns::String::with_str("skip");
    if &x_in.shape().unwrap()[3] != out_channels {
        let nin_shortcut = make_conv(
            graph,
            &x_in,
            &format!("{name}.nin_shortcut"),
            out_channels,
            &ns::Number::with_i32(3),
            1,
            false,
        );
        return graph.add(&x, &nin_shortcut, Some(&skip));
    }
    graph.add(&x, x_in, Some(&skip))
}

pub fn make_decoder_attention(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
) -> arc::R<graph::Tensor> {
    let x = make_group_norm(graph, x_in, &format!("{name}.norm"));
    let shape = x.shape().unwrap();
    let c = &shape[3];
    let new_share: &[&ns::Number] = &[
        &shape[0],
        &ns::Number::with_i64(shape[1].as_i64() * shape[2].as_i64()),
        &c,
    ];
    let x = graph.reshape(&x, &mps::Shape::from_slice(new_share), None);
    let q = make_linear(graph, &x, &format!("{name}.q"), c, false);
    let k = make_linear(graph, &x, &format!("{name}.k"), c, false);
    let k = graph.mul(
        &k,
        &graph.constant(1f64 / c.as_f64().sqrt(), mps::DataType::Float16),
        None,
    );
    let k = graph.transpose_with_dimension(&k, 1, 2, None);
    let v = make_linear(graph, &x, &format!("{name}.v"), c, false);
    let att = graph.mat_mul(&q, &k, None);
    let att = graph.soft_max(&att, 2, None);
    let att = graph.mat_mul(&att, &v, None);
    let x = make_linear(graph, &att, &format!("{name}.proj_out"), c, true);
    let x = graph.reshape(&x, &x_in.shape().unwrap(), None);

    graph.add(&x, x_in, None)
}

fn make_byte_converter(graph: &graph::Graph, x_in: &graph::Tensor) -> arc::R<graph::Tensor> {
    let one = ns::Number::with_i64(1);
    let one_shape = mps::Shape::from_slice(&[&one]);
    let dt = mps::DataType::Float16;
    let x = graph.clamp(
        x_in,
        &graph.constant_shape(0f64, &one_shape, dt),
        &graph.constant_shape(1f64, &one_shape, dt),
        None,
    );
    let x = graph.mul(&x, &graph.constant_shape(255f64, &one_shape, dt), None);
    let x = graph.round(&x, None);
    let x = graph.cast(
        &x,
        mps::DataType::U8,
        Some(&ns::String::with_str("cast to uint8 rgba")),
    );
    let alpha = graph.constant_shape(
        255f64,
        &mps::Shape::from_slice(&[&one, &x.shape().unwrap()[1], &x.shape().unwrap()[2], &one]),
        mps::DataType::U8,
    );
    let tensors: &[&graph::Tensor] = &[&x, &alpha];
    graph.concat_tensors(&ns::Array::from_slice(tensors), 3, None)
}

fn make_decoder(graph: &graph::Graph, x_in: &graph::Tensor) -> arc::R<graph::Tensor> {
    let name = "first_stage_model.decoder";
    let x = graph.mul(
        x_in,
        &graph.constant(1f64 / 0.18215f64, mps::DataType::Float16),
        Some(&ns::String::with_str("rescale")),
    );
    let out_channels = ns::Number::with_i64(512);
    let kwh = ns::Number::with_i64(3);
    let x = make_conv(
        graph,
        &x,
        "first_stage_model.post_quant_conv",
        &out_channels,
        &kwh,
        1,
        true,
    );

    let x = make_conv(
        graph,
        &x,
        &format!("{name}.conv_in"),
        &out_channels,
        &kwh,
        1,
        true,
    );

    let x = make_decoder_res_block(graph, &x, &format!("{name}.mid.block_1"), &out_channels);
    let x = make_decoder_attention(graph, &x, &format!("{name}.mid.attn_1"));
    let x = make_decoder_res_block(graph, &x, &format!("{name}.mid.block_2"), &out_channels);

    // block 3
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.3.block.0"), &out_channels);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.3.block.1"), &out_channels);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.3.block.2"), &out_channels);
    let x = make_upsample_nearest(graph, &x, 2);
    let x = make_conv(
        graph,
        &x,
        &format!("{name}.up.3.upsample.conv"),
        &out_channels,
        &kwh,
        1,
        true,
    );
    // block 2
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.2.block.0"), &out_channels);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.2.block.1"), &out_channels);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.2.block.2"), &out_channels);
    let x = make_upsample_nearest(graph, &x, 2);
    let x = make_conv(
        graph,
        &x,
        &format!("{name}.up.2.upsample.conv"),
        &out_channels,
        &kwh,
        1,
        true,
    );

    // block 1
    let out_channels = ns::Number::with_i64(256);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.1.block.0"), &out_channels);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.1.block.1"), &out_channels);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.1.block.2"), &out_channels);
    let x = make_upsample_nearest(graph, &x, 2);
    let x = make_conv(
        graph,
        &x,
        &format!("{name}.up.1.upsample.conv"),
        &out_channels,
        &kwh,
        1,
        true,
    );

    // block 0
    let out_channels = ns::Number::with_i64(128);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.0.block.0"), &out_channels);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.0.block.1"), &out_channels);
    let x = make_decoder_res_block(graph, &x, &format!("{name}.up.0.block.2"), &out_channels);

    let three = ns::Number::with_i64(3);
    let x = make_group_norm_swish(graph, &x, &format!("{name}.norm_out"));
    let x = make_conv(
        graph,
        &x,
        &format!("{name}.conv_out"),
        &three,
        &three,
        1,
        true,
    );
    let x = graph.add(&x, &graph.constant(1f64, mps::DataType::Float16), None);
    let x = graph.mul(&x, &graph.constant(0.5f64, mps::DataType::Float16), None);

    make_byte_converter(graph, &x)
}

fn make_layer_norm(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
) -> arc::R<graph::Tensor> {
    let in_shape = x_in.shape().unwrap();
    assert_eq!(in_shape.len(), 3, "layernorm requires NTC");

    let one = ns::Number::with_i64(1);
    let gamma = load_const(
        graph,
        &format!("{name}.weight"),
        &[&one, &one, &in_shape[2]],
        false,
    );
    let beta = load_const(
        graph,
        &format!("{name}.bias"),
        &[&one, &one, &in_shape[2]],
        false,
    );

    let two = ns::Number::with_i64(2);
    let axes: &[&ns::Number] = &[&two];
    let axes = ns::Array::from_slice(axes);

    let mean = graph.mean(x_in, &axes, None);
    let variance = graph.variance(x_in, &axes, None);
    let x = graph.normalize(
        x_in,
        &mean,
        &variance,
        Some(&gamma),
        Some(&beta),
        1e-5f32,
        None,
    );
    graph.reshape(&x, x_in.shape().unwrap(), None)
}

fn make_linear(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
    out_channels: &ns::Number,
    bias: bool,
) -> arc::R<graph::Tensor> {
    let in_shape = x_in.shape().unwrap();
    let one = ns::Number::with_i64(1);
    if in_shape.len() == 2 {
        let x = graph.reshape(
            x_in,
            &mps::Shape::from_slice(&[&in_shape[0], &one, &one, &in_shape[1]]),
            None,
        );
        let x = make_conv(graph, &x, name, out_channels, &one, 1, bias);
        return graph.reshape(
            &x,
            &mps::Shape::from_slice(&[&in_shape[0], out_channels]),
            None,
        );
    };

    let x = graph.reshape(
        x_in,
        &mps::Shape::from_slice(&[&in_shape[0], &one, &in_shape[2]]),
        None,
    );
    let x = make_conv(graph, &x, name, out_channels, &one, 1, bias);
    graph.reshape(
        &x,
        &mps::Shape::from_slice(&[&in_shape[0], &in_shape[1], out_channels]),
        None,
    )
}

fn make_time_embed(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
) -> arc::R<graph::Tensor> {
    let out_channels = ns::Number::with_i64(1280);
    let x = make_linear(graph, x_in, &format!("{name}.0"), &out_channels, true);
    let x = make_swish(graph, &x);
    make_linear(graph, &x, &format!("{name}.2"), &out_channels, true)
}

fn make_unet_res_block(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    emb_in: &graph::Tensor,
    name: &str,
    in_channels: &ns::Number,
    out_channels: &ns::Number,
) -> arc::R<graph::Tensor> {
    let kwh = ns::Number::with_i32(3);
    let x = make_group_norm_swish(graph, x_in, &format!("{name}.in_layers.0"));
    let x = make_conv(
        graph,
        &x,
        &format!("{name}.in_layers.2"),
        out_channels,
        &kwh,
        1,
        true,
    );
    let emb = make_swish(graph, emb_in);
    let emb = make_linear(
        graph,
        &emb,
        &format!("{name}.emb_layers.1"),
        out_channels,
        true,
    );

    let axes: &[&ns::Number] = &[&ns::Number::with_i32(1), &ns::Number::with_i32(2)];
    let axes = ns::Array::from_slice(axes);
    let emb = graph.expand_dims_axes(&emb, &axes, None);

    let x = graph.add(&x, &emb, None);
    let x = make_group_norm_swish(graph, &x, &format!("{name}.out_layers.0"));
    let x = make_conv(
        graph,
        &x,
        &format!("{name}.out_layers.3"),
        out_channels,
        &kwh,
        1,
        true,
    );

    if in_channels != out_channels {
        let skip = make_conv(
            graph,
            x_in,
            &format!("{name}.skip_connection"),
            out_channels,
            &ns::Number::with_i32(1),
            1,
            true,
        );
        graph.add(&x, &skip, None)
    } else {
        graph.add(&x, x_in, None)
    }
}

fn make_cross_attention(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
    context: Option<&graph::Tensor>,
    save_mem: bool,
) -> arc::R<graph::Tensor> {
    let in_shape = x_in.shape().unwrap();
    let c = &in_shape[2];
    let n_heads = ns::Number::with_i32(8);
    let d_head = ns::Number::with_i64(c.as_i64() / 8);

    let q = make_linear(graph, x_in, &format!("{name}.to_q"), c, false);
    let context = context.unwrap_or(x_in);
    let k = make_linear(graph, context, &format!("{name}.to_k"), c, false);
    let v = make_linear(graph, context, &format!("{name}.to_v"), c, false);
    let n = &in_shape[0];
    let hw = &in_shape[1];
    let t = &context.shape().unwrap()[1];

    let q = graph.reshape(
        &q,
        &mps::Shape::from_slice(&[n, hw, &n_heads, &d_head]),
        None,
    );
    let k = graph.reshape(
        &k,
        &mps::Shape::from_slice(&[n, t, &n_heads, &d_head]),
        None,
    );
    let v = graph.reshape(
        &v,
        &mps::Shape::from_slice(&[n, t, &n_heads, &d_head]),
        None,
    );

    let q = graph.transpose_with_dimension(&q, 1, 2, None);
    let k = graph.transpose_with_dimension(&k, 1, 2, None);
    let k = graph.transpose_with_dimension(&k, 2, 3, None);
    let k = graph.mul(
        &k,
        &graph.constant(1.0f64 / d_head.as_f64().sqrt(), mps::DataType::Float16),
        None,
    );
    let v = graph.transpose_with_dimension(&v, 1, 2, None);
    let att = if save_mem {
        todo!();
    } else {
        let att = graph.mat_mul(&q, &k, None);
        let att = graph.soft_max(&att, 3, None);
        let att = graph.mat_mul(&att, &v, None);
        graph.transpose_with_dimension(&att, 1, 2, None)
    };
    let att = graph.reshape(&att, &x_in.shape().unwrap(), None);
    make_linear(graph, &att, &format!("{name}.to_out.0"), c, true)
}

//     if (saveMemory) {
//         // MEM-HACK - silly graph seems to use less peak memory
//         var attRes = [MPSGraphTensor]()
//         let sliceSize = 1
//         for i in 0..<nHeads.intValue/sliceSize {
//             let qi = graph.sliceTensor(q, dimension: 1, start: i*sliceSize, length: sliceSize, name: nil)
//             let ki = graph.sliceTensor(k, dimension: 1, start: i*sliceSize, length: sliceSize, name: nil)
//             let vi = graph.sliceTensor(v, dimension: 1, start: i*sliceSize, length: sliceSize, name: nil)
//             var attI = graph.matrixMultiplication(primary: qi, secondary: ki, name: nil)
//             attI = graph.softMax(with: attI, axis: 3, name: nil)
//             attI = graph.matrixMultiplication(primary: attI, secondary: vi, name: nil)
//             attI = graph.transposeTensor(attI, dimension: 1, withDimension: 2, name: nil)
//             attRes.append(attI)
//         }
//         att = graph.concatTensors(attRes, dimension: 2, name: nil)
//     } else {
//     }
//     att = graph.reshape(att, shape: xIn.shape!, name: nil)
//     return makeLinear(graph: graph, xIn: att, name: name + ".to_out.0", outChannels: c)
// }

fn make_gelu(graph: &graph::Graph, x_in: &graph::Tensor) -> arc::R<graph::Tensor> {
    let x = graph.mul(
        x_in,
        &graph.constant(1f64 / 2f64.sqrt(), mps::DataType::Float16),
        None,
    );
    let x = graph.erf(&x, None);
    let x = graph.add(&x, &graph.constant(1f64, mps::DataType::Float16), None);
    let x = graph.mul(&x, &graph.constant(1f64, mps::DataType::Float16), None);
    graph.mul(x_in, &x, None)
}

fn make_feed_forward(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
) -> arc::R<graph::Tensor> {
    let in_shape = x_in.shape().unwrap();
    assert_eq!(in_shape.len(), 3);
    let dim = &in_shape[2];
    let dim_mult = dim.as_isize() * 4;
    let dim_proj = ns::Number::with_isize(dim_mult * 2);
    let proj = make_linear(graph, x_in, &format!("{name}.0.proj"), &dim_proj, true);
    let x = graph.slice_tensor(&proj, 2, 0, dim_mult, None);
    let gate = graph.slice_tensor(&proj, 2, dim_mult, dim_mult, None);
    let gate = make_gelu(graph, &gate);
    let x = graph.mul(&x, &gate, None);
    make_linear(graph, &x, &format!("{name}."), dim, true)
}

fn make_basic_transformer_block(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
    context_in: &graph::Tensor,
    save_mem: bool,
) -> arc::R<graph::Tensor> {
    let attn1 = make_layer_norm(graph, x_in, &format!("{name}.norm1"));
    let attn1 = make_cross_attention(graph, &attn1, &format!("{name}.attn1"), None, save_mem);
    let x = graph.add(&attn1, x_in, None);
    let attn2 = make_layer_norm(graph, &x, &format!("{name}.norm2"));
    let x = graph.add(&attn2, &x, None);
    let ff = make_layer_norm(graph, &x, &format!("{name}.norm3"));
    let ff = make_feed_forward(graph, &ff, &format!("{name}.ff.net"));
    graph.add(&ff, &x, None)
}

fn main() {}
