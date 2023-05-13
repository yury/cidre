use cidre::mlc;

fn main() {
    let t_a = mlc::Tensor::with_shape_dt([1, 2, 2], mlc::DataType::F32);
    let t_b = mlc::Tensor::with_shape_dt([1, 2, 2], mlc::DataType::F32);
    let t_c = mlc::Tensor::with_shape_dt([1, 2, 2], mlc::DataType::F32);

    let buf_a = [1f32, 2.0, 3.0, 4.0];
    let buf_b = [1f32, 2.0, 3.0, 4.0];
    let buf_c = [1f32, 2.0, 3.0, 4.0];

    let dat_a = mlc::TensorData::with_bytes_no_copy(
        buf_a.as_ptr() as _,
        buf_a.len() * std::mem::size_of::<f32>(),
    );

    let dat_b = mlc::TensorData::with_bytes_no_copy(
        buf_b.as_ptr() as _,
        buf_b.len() * std::mem::size_of::<f32>(),
    );

    let dat_c = mlc::TensorData::with_bytes_no_copy(
        buf_c.as_ptr() as _,
        buf_c.len() * std::mem::size_of::<f32>(),
    );

    let graph = mlc::Graph::new();

    let t_ab = graph
        .node_with_layer_sources(
            &mlc::MatMulLayer::with_descriptor(&mlc::MatMulDescriptor::new()).unwrap(),
            &[&t_a, &t_b],
        )
        .unwrap();

    println!("{graph:?} {t_ab:?}");
}
