use crate::{api, arc, define_obj_type, mtl, mtl4, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4MachineLearningPipelineDescriptor")]
    pub MlPipelineDesc(mtl4::PipelineDesc),
    MTL4_MACHINE_LEARNING_PIPELINE_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26)]
);

impl MlPipelineDesc {
    #[objc::msg_send(machineLearningFunctionDescriptor)]
    pub fn ml_fn_desc(&self) -> Option<arc::R<mtl4::FnDesc>>;

    #[objc::msg_send(setMachineLearningFunctionDescriptor:)]
    pub fn set_ml_fn_desc(&mut self, val: Option<&mtl4::FnDesc>);

    /// Sets the dimension of an input tensor at a buffer index.
    #[objc::msg_send(setInputDimensions:atBufferIndex:)]
    pub fn set_buf_input_dims(&mut self, val: Option<&mtl::TensorExtents>, buffer_index: usize);

    #[objc::msg_send(setInputDimensions:withRange:)]
    pub fn set_bufs_input_dims_with_range(val: &ns::Array<mtl::TensorExtents>, range: ns::Range);

    // value shoulbe mtl::TensorExtent or ns::Null
    #[objc::msg_send(setInputDimensions:withRange:)]
    pub fn set_optional_bufs_input_dims_with_range(val: &ns::Array<ns::Id>, range: ns::Range);

    #[objc::msg_send(inputDimensionsAtBufferIndex:)]
    pub fn buf_input_dims(&self, buffer_index: usize) -> Option<arc::R<mtl::TensorExtents>>;

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}

define_obj_type!(
    #[doc(alias = "MTL4MachineLearningPipelineReflection")]
    pub MlPipelineReflection(ns::Id)
);

impl MlPipelineReflection {
    #[objc::msg_send(bindings)]
    pub fn bindings(&self) -> arc::R<ns::Array<mtl::Binding>>;
}

define_obj_type!(
    #[doc(alias = "MTL4MachineLearningPipelineState")]
    pub MlPipelineState(mtl::Allocation)
);

impl MlPipelineState {
    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(reflection)]
    pub fn reflection(&self) -> Option<arc::R<mtl4::MlPipelineReflection>>;

    #[objc::msg_send(intermediatesHeapSize)]
    pub fn intermediates_heap_size(&self) -> usize;
}

unsafe extern "C" {
    static MTL4_MACHINE_LEARNING_PIPELINE_DESCRIPTOR: &'static objc::Class<MlPipelineDesc>;
}
