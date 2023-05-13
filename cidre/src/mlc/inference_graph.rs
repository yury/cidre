use crate::{arc, blocks::Block, define_cls, define_obj_type, mlc, ns, objc};

define_obj_type!(InferenceGraph(mlc::Graph));
impl InferenceGraph {
    define_cls!(MLC_INFERENCE_GRAPH);

    #[objc::cls_msg_send(graphWithGraphObjects:)]
    pub fn with_graph_objects_ar(graph_objects: &ns::Array<mlc::Graph>) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_graph_objects(graph_objects: &ns::Array<mlc::Graph>) -> arc::R<Self>;

    #[inline]
    pub fn with_graphs_slice(graph_objects: &[&mlc::Graph]) -> arc::R<Self> {
        let graph_objects = ns::Array::from_slice(graph_objects);
        Self::with_graph_objects(&graph_objects)
    }

    // #[objc::add]
    // - (BOOL)addInputs:(NSDictionary<NSString *, MLCTensor *> *)inputs;
    #[objc::msg_send(addInputs:)]
    fn add_inputs_(&mut self, inputs: &ns::Dictionary<ns::String, mlc::Tensor>) -> bool;

    #[inline]
    pub fn add_inputs(
        &mut self,
        inputs: &ns::Dictionary<ns::String, mlc::Tensor>,
    ) -> Result<(), ()> {
        if self.add_inputs_(inputs) {
            Ok(())
        } else {
            Err(())
        }
    }

    // - (BOOL)compileWithOptions:(MLCGraphCompilationOptions)options
    //                 device:(MLCDevice *)device;
    #[objc::msg_send(compileWithOptions:device:)]
    pub fn compile_with_options_(
        &mut self,
        options: mlc::GraphCompilationOptions,
        device: &mlc::Device,
    ) -> bool;

    #[inline]
    pub fn compile(
        &mut self,
        options: mlc::GraphCompilationOptions,
        device: &mlc::Device,
    ) -> Result<(), ()> {
        if self.compile_with_options_(options, device) {
            Ok(())
        } else {
            Err(())
        }
    }

    #[objc::msg_send(executeWithInputsData:batchSize:options:completionHandler:)]
    pub fn execute_with_inputs_data_batch_size_options<'ar, F>(
        &self,
        input_data: &ns::Dictionary<ns::String, mlc::TensorData>,
        batch_size: usize,
        options: mlc::ExecutionOptions,
        completion_handler: Option<&'static Block<F>>,
    ) -> bool
    where
        F: FnOnce(Option<&'ar mlc::Tensor>, Option<&'ar ns::Error>, ns::TimeInterval);
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_INFERENCE_GRAPH: &'static objc::Class<InferenceGraph>;
}
