/// Very simple example of an ML Compute matrix multiplication.
/// Port of https://gist.github.com/xrq-phys/c9d198dcd97647f73c0092733b77dec5
use cidre::{blocks, mlc, ns};

fn main() {
    let t_a = mlc::Tensor::with_shape_dt([1, 2, 2], mlc::DType::F32);
    let t_b = mlc::Tensor::with_shape_dt([1, 2, 2], mlc::DType::F32);
    let t_c = mlc::Tensor::with_shape_dt([1, 2, 2], mlc::DType::F32);

    let dat_a = mlc::TensorData::with_slice_no_copy(&[1f32, 2.0, 3.0, 4.0]);
    let dat_b = mlc::TensorData::with_slice_no_copy(&[1f32, 2.0, 3.0, 4.0]);
    let dat_c = mlc::TensorData::with_slice_no_copy(&[1f32, 1.0, 1.0, 1.0]);

    let graph = mlc::Graph::new();

    let t_ab = graph
        .node_with_layer_sources(&mlc::MatMulLayer::new(), &[&t_a, &t_b])
        .unwrap();
    graph
        .node_with_layer_sources(
            &mlc::ArithmeticLayer::with_op(mlc::ArithmeticOp::Add),
            &[&t_ab, &t_c],
        )
        .unwrap();

    let mut plan = mlc::InferenceGraph::with_graphs_slice(&[graph.as_ref()]);

    let a = ns::String::with_str("A");
    let b = ns::String::with_str("B");
    let c = ns::String::with_str("C");

    let keys = [a.as_ref(), &b, &c];
    let values = [t_a.as_ref(), &t_b, &t_c];
    let inputs = ns::Dictionary::with_keys_values(&keys, &values);
    plan.add_inputs(&inputs).unwrap();

    plan.compile(Default::default(), &mlc::Device::new())
        .unwrap();

    let values = [dat_a.as_ref(), &dat_b, &dat_c];
    let mut handler = blocks::SyncBlock::new3(|r: Option<&mlc::Tensor>, _e, time| {
        let r = r.unwrap();

        let mut buf_o = vec![0.0f32; 4];
        r.copy_from_device_mem_to_buf(&mut buf_o, true).unwrap();
        println!("output {buf_o:?}, {time}");
        assert_eq!(buf_o, [8.0f32, 11.0f32, 16.0f32, 23.0f32]);
    });

    let input_data = ns::Dictionary::with_keys_values(&keys, &values);
    plan.execute_ch(&input_data, 0, Default::default(), Some(&mut handler));
}
