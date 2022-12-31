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
    stride: isize,
    bias: bool,
) -> graph::Tensor {
    let w = load_const(
        graph,
        &format!("{name}.weight"),
        &[out_channels, &x_in.shape().unwrap()[3], khw, khw],
        false,
    );

    let p = khw.to_i64().unwrap() / 2;

    if bias {
        let one = cf::Number::from_i64(1);
        let b = load_const(
            graph,
            &format!("{name}.bias"),
            &[&one, &one, &one, out_channels],
            false,
        );
    }
    todo!();
}

// func makeConv(graph: MPSGraph, xIn: MPSGraphTensor, name: String, outChannels: NSNumber, khw: NSNumber, stride: Int = 1, bias: Bool = true) -> MPSGraphTensor {
//     let w = loadConstant(graph: graph, name: name + ".weight", shape: [outChannels, xIn.shape![3], khw, khw])
//     let p: Int = khw.intValue / 2;
//     let convDesc = MPSGraphConvolution2DOpDescriptor(strideInX: stride, strideInY: stride, dilationRateInX: 1, dilationRateInY: 1, groups: 1, paddingLeft: p, paddingRight: p, paddingTop: p, paddingBottom: p, paddingStyle: MPSGraphPaddingStyle.explicit, dataLayout: MPSGraphTensorNamedDataLayout.NHWC, weightsLayout: MPSGraphTensorNamedDataLayout.OIHW)!
//     let conv = graph.convolution2D(xIn, weights: w, descriptor: convDesc, name: nil)
//     if (bias) {
//         let b = loadConstant(graph: graph, name: name + ".bias", shape: [1, 1, 1, outChannels])
//         return graph.addition(conv, b, name: nil)
//     }
//     return conv
// }

fn make_upsample_nearest(
    graph: graph::Graph,
    x_in: &graph::Tensor,
    scale_factor: isize,
) -> &graph::Tensor {
    todo!();
}
// func makeUpsampleNearest(graph: MPSGraph, xIn: MPSGraphTensor, scaleFactor: Int=2) -> MPSGraphTensor {
//     return graph.resize(xIn, size: [NSNumber(value:xIn.shape![1].intValue * scaleFactor), NSNumber(value:xIn.shape![2].intValue * scaleFactor)], mode: MPSGraphResizeMode.nearest, centerResult: true, alignCorners: false, layout: MPSGraphTensorNamedDataLayout.NHWC, name: nil)
// }

fn main() {}
