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
    shape: &[&cf::Number],
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

    let numels = shape.iter().fold(1, |acc, x| acc * x.to_i64().unwrap());
    assert_eq!(numels * size, data.length() as i64, "mismatched data sizes");

    let shape = cf::ArrayOf::from_slice(shape);
    graph.constant_with_data_shape_data_type(&data, &shape, data_type)
}

fn make_conv(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &str,
    out_channels: &cf::Number,
    khw: &cf::Number,
    stride: usize,
    bias: bool,
) -> graph::Tensor {
    let w = load_const(
        graph,
        &format!("{name}.weight"),
        &[out_channels, &x_in.shape().unwrap()[3], khw, khw],
        false,
    );

    let p = khw.to_i64().unwrap() as usize / 2;

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

    let conv = graph.convolution_2d(x_in, weights, &conv_desc, None);

    if bias {
        let one = cf::Number::from_i64(1);
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
    scale_factor: isize,
) -> cf::Retaned<graph::Tensor> {
    let in_shape = x_in.shape().unwrap();
    let shape: [&cf::Number] = [
        in_shape[1].to_i64().unwrap() * scale_factor,
        in_shape[2].to_i64().unwrap() * scale_factor,
    ];
    graph.resize(
        x_in,
        &mps::Shape::from_slice(&shape),
        graph::ResizeMode::Nearesta,
        true,
        false,
        graph::TensorNamedDataLayout::NHWC,
        None,
    )
}

fn make_group_norm(
    graph: &graph::Graph,
    x_in: &graph::Tensor,
    name: &cf::String,
) -> cf::Retained<graph::Tensor> {
    let mut x = x_in.retained();
    if x_in.shape().unwrap().len() == 3 {
        x = graph.expand_dims(&x, 1, None);
    }

    let shape = x.shape().unwrap();
    let n_groups = 32f64;
    let n_grouped = shape[3].to_f64().unwrap() / n_groups;

    let gamma = load_const(graph, &format("{name}.weight"), &[], false);
    let beta = load_const(graph, &format("{name}.weight"), &[], false);
}

fn main() {}
