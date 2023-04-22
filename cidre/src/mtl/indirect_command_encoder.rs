use crate::{define_obj_type, mtl, ns, objc};

define_obj_type!(IndirectRenderCmd(ns::Id));

impl IndirectRenderCmd {
    /// Sets the render pipeline state object used by the command.
    ///
    /// If you created the indirect command buffer with inheritPipelineState set to true,
    /// do not call this method. The command gets the pipeline state object from the parent
    /// encoder when you execute the command.
    ///
    /// If you created the indirect command buffer with inheritPipelineState set to false,
    /// you must set the pipeline state prior to encoding the drawing command.
    /// TODO: if it throws wrap it in try - catch
    #[objc::msg_send(setRenderPipelineState:)]
    pub fn set_render_ps(&mut self, ps: &mtl::RenderPipelineState);

    /// Sets a vertex buffer argument for the command.
    ///
    /// If you created the indirect command buffer with inheritBuffers set to true,
    /// do not call this method. The command gets the arguments from the parent encoder
    /// when you execute the command.
    ///
    /// If you need to pass other kinds of parameters to your shader, such as textures
    /// and samplers, create an argument buffer and pass it to the shader using this method.
    #[objc::msg_send(setVertexBuffer:offset:atIndex:)]
    pub fn set_vertex_buf_at(&mut self, buf: &mtl::Buf, offset: usize, index: usize);

    /// Sets a vertex buffer argument for the command.
    ///
    /// If you created the indirect command buffer with inheritBuffers set to true,
    /// do not call this method. The command gets the arguments from the parent encoder
    /// when you execute the command.
    ///
    /// If you need to pass other kinds of parameters to your shader, such as textures
    /// and samplers, create an argument buffer and pass it to the shader using this method.
    #[objc::msg_send(setFragmentBuffer:offset:atIndex:)]
    pub fn set_fragment_buf_at(&mut self, buf: &mtl::Buf, offset: usize, index: usize);

    /// Encodes a command to render a number of instances of tessellated patches.
    #[objc::msg_send(drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)]
    pub fn draw_patches(
        &self,
        number_of_patch_ctrl_points: usize,
        patch_start: usize,
        patch_count: usize,
        patch_index_buf: &mtl::Buf,
        patch_index_buf_offset: usize,
        instance_count: usize,
        base_instance: usize,
        tessellation_factor_buf: &mtl::Buf,
        tessellation_factor_buf_offset: usize,
        tessellation_factor_buf_instance_stride: usize,
    );
}
define_obj_type!(IndirectComputeCmd(ns::Id));
