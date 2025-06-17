use crate::{arc, define_obj_type, define_opts, mtl, mtl4, ns, objc};

define_opts!(
    #[doc(alias = "MTL4VisibilityOptions")]
    pub VisibilityOpts(isize)
);

define_obj_type!(
    #[doc(alias = "MTL4CommandEncoder")]
    pub CmdEncoder(ns::Id)
);

impl CmdEncoder {
    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);

    /// Returns the command buffer that is currently encoding commands.
    ///
    /// This property may return undefined results if you call it after calling `end_encoding`.
    #[objc::msg_send(commandBuffer)]
    pub fn cmd_buf(&self) -> Option<arc::R<mtl4::CmdBuf>>;

    #[objc::msg_send(barrierAfterQueueStages:beforeStages:visibilityOptions:)]
    pub fn barrier_after_queue_stages_before_stages(
        &mut self,
        after_queue_stages: mtl::Stages,
        before_stages: mtl::Stages,
        visibility_opts: VisibilityOpts,
    );

    #[objc::msg_send(barrierAfterStages:beforeQueueStages:visibilityOptions:)]
    pub fn barrier_after_stages_before_queue_stages(
        &mut self,
        after_stages: mtl::Stages,
        before_queue_stages: mtl::Stages,
        visibility_opts: VisibilityOpts,
    );

    #[objc::msg_send(barrierAfterEncoderStages:beforeEncoderStages:visibilityOptions:)]
    pub fn barrier_after_encoder_stages_before_encoder_stages(
        &mut self,
        after_encoder_stages: mtl::Stages,
        before_encoder_stages: mtl::Stages,
        visibility_opts: VisibilityOpts,
    );

    #[objc::msg_send(updateFence:afterEncoderStages:)]
    pub fn update_fence_after_encoder_stages(
        &mut self,
        fence: &mut mtl::Fence,
        stages: mtl::Stages,
    );

    #[objc::msg_send(waitForFence:beforeEncoderStages:)]
    pub fn wait_fence_before_encoder_stages(&mut self, fence: &mtl::Fence, stages: mtl::Stages);

    #[objc::msg_send(insertDebugSignpost:)]
    pub fn insert_debug_signpost(&mut self, val: &ns::String);

    #[objc::msg_send(popDebugGroup:)]
    pub fn push_debug_group(&mut self, val: &ns::String);

    #[objc::msg_send(popDebugGroup)]
    pub fn pop_debug_group(&mut self);

    #[objc::msg_send(endEncoding)]
    pub fn end_encoding(&mut self);
}
