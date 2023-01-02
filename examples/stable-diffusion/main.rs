use cidre::{cf, mps, mps::graph, ns};

fn make_graph(synchonize: bool) -> cf::Retained<graph::Graph> {
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
) -> cf::Retained<graph::Tensor> {
    let (prefix, data_type, size) = if fp32 {
        ("_fp32", mps::DataType::F32, 4)
    } else {
        ("", mps::DataType::Float16, 2)
    };

    let data = ns::Data::with_contents_of_file_options(
        &cf::String::from_str(&format!("bins/{name}{prefix}.bin")),
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
) -> cf::Retained<graph::Tensor> {
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
        return graph.addition(&conv, &b, None);
    }
    conv
}

fn make_upsample_nearest(
    graph: graph::Graph,
    x_in: &graph::Tensor,
    scale_factor: i64,
) -> cf::Retained<graph::Tensor> {
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
) -> cf::Retained<graph::Tensor> {
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

fn make_swish(graph: &graph::Graph, x_in: &graph::Tensor) -> cf::Retained<graph::Tensor> {
    graph.multiplication(x_in, &graph.sigmoid(x_in, None), None)
}

fn make_group_norm_swish(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
) -> cf::Retained<graph::Tensor> {
    make_swish(graph, &make_group_norm(graph, x_in, name))
}

fn make_decoder_res_block(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
    out_channels: &ns::Number,
) -> cf::Retained<graph::Tensor> {
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
    todo!();
}

fn main() {}
