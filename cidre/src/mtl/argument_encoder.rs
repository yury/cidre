use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(pub ArgEncoder(ns::Id));

impl ArgEncoder {
    define_mtl!(set_label);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(encodedLength)]
    pub fn encoded_len(&self) -> usize;

    #[objc::msg_send(alignment)]
    pub fn alignment(&self) -> usize;

    #[objc::msg_send(setArgumentBuffer:offset:)]
    pub fn set_argument_buf(&mut self, buffer: Option<&mtl::Buf>, offset: usize);

    /// Encodes a reference to a buffer into the argument buffer.
    #[objc::msg_send(setBuffer:offset:atIndex:)]
    pub fn set_buf(&mut self, buffer: Option<&mtl::Buf>, offset: usize, at_index: usize);

    /// Set a texture at the given bind point index.
    ///
    /// Encodes a reference to a texture into the argument buffer.
    #[objc::msg_send(setTexture:atIndex:)]
    pub fn set_texture(&mut self, texture: Option<&mtl::Texture>, at_index: usize);

    /// Set a sampler at the given bind point index.
    #[objc::msg_send(setSamplerState:atIndex:)]
    pub fn set_sampler_state(&mut self, sampler: Option<&mtl::SamplerState>, at_index: usize);

    /// Returns a pointer for an inlined constant data argument in the argument buffer.
    ///
    /// Constants declared contiguously in the Metal shading language (in an array or structure)
    /// are contiguous in memory. You can encode contiguous ranges of inlined constant data through
    /// a pointer to the first constant.
    ///
    /// To encode inlined constant data into the argument buffer, perform a memory copy operation from
    /// your data’s source pointer to the returned destination pointer.
    #[objc::msg_send(constantData:)]
    pub fn const_data(&mut self, at_index: usize) -> *mut u8;

    /// Encodes a reference to a render pipeline state into the argument buffer.
    #[objc::msg_send(setRenderPipelineState:atIndex:)]
    pub fn set_render_ps(&mut self, state: Option<&mtl::RenderPipelineState>, at_index: usize);

    /// Sets a compute pipeline state at a given bind point index
    ///    
    /// Encodes a reference to a compute pipeline state into the argument buffer.
    #[objc::msg_send(setComputePipelineState:atIndex:)]
    pub fn set_compute_ps(&mut self, state: Option<&mtl::ComputePipelineState>, at_index: usize);

    /// Sets an indirect command buffer at a given bind point index
    ///
    /// Encodes a reference to an indirect command buffer into the argument buffer.
    #[objc::msg_send(setIndirectCommandBuffer:atIndex:)]
    pub fn set_indirect_cmd_buf(&mut self, buffer: Option<&mtl::IndirectCmdBuf>, at_index: usize);
}
