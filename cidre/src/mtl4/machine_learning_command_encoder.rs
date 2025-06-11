use crate::{define_obj_type, mtl, mtl4, objc};

define_obj_type!(
    #[doc(alias = "MTL4MachineLearningCommandEncoder")]
    pub MlCmdEncoder(mtl4::CmdEncoder)
);

impl MlCmdEncoder {
    #[objc::msg_send(setArgumentTable:)]
    pub fn set_arg_table(&mut self, val: &mtl4::ArgTable);

    #[objc::msg_send(setPipelineState:)]
    pub fn set_ps(&mut self, val: &mtl4::MlPipelineState);

    #[objc::msg_send(dispatchNetworkWithIntermediatesHeap:)]
    pub fn dispatch_network_with_intermediates_heap(&self, heap: &mtl::Heap);
}
