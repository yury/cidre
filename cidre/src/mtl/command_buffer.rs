use crate::{arc, blocks, cf, define_mtl, define_obj_type, mtl, ns, objc};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Status {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Error {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    AccessRevoked = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,
    StackOverflow = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum DispatchType {
    Serial,
    Concurrent,
}

#[doc(alias = "MTLCommandBufferHandler")]
pub type CmdBufHandler = blocks::SyncBlock<fn(buf: &mtl::CmdBuf)>;

define_obj_type!(
    /// A serial list of commands for the device to execute.
    #[doc(alias = "MTLCommandBuffer")]
    pub CmdBuf(ns::Id)
);

impl CmdBuf {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

    #[objc::msg_send(commandQueue)]
    pub fn cmd_queue(&self) -> &mtl::CmdQueue;

    #[objc::msg_send(enqueue)]
    pub fn enque(&self);

    #[objc::msg_send(commit)]
    pub fn commit(&self);

    #[objc::msg_send(waitUntilScheduled)]
    pub fn wait_until_scheduled(&self);

    #[objc::msg_send(waitUntilCompleted)]
    pub fn wait_until_completed(&self);

    #[objc::msg_send(addScheduledHandler:)]
    pub fn add_scheduled_handler(&mut self, block: &mut CmdBufHandler);

    #[objc::msg_send(addCompletedHandler:)]
    pub fn add_completed_handler(&mut self, block: &mut CmdBufHandler);

    #[objc::msg_send(blitCommandEncoder)]
    pub fn new_blit_cmd_enc_ar(&self) -> Option<arc::Rar<mtl::BlitCmdEncoder>>;

    #[objc::rar_retain()]
    pub fn new_blit_cmd_enc(&self) -> Option<arc::R<mtl::BlitCmdEncoder>>;

    pub fn blit<F: FnMut(&mut mtl::BlitCmdEncoder)>(&mut self, mut commands: F) {
        let mut encoder = self
            .new_blit_cmd_enc()
            .expect("Can't create blit command encoder");
        commands(&mut encoder);
        encoder.end();
    }

    #[objc::msg_send(computeCommandEncoder)]
    pub fn new_compute_cmd_enc_ar(&self) -> Option<arc::Rar<mtl::ComputeCmdEncoder>>;

    #[objc::rar_retain()]
    pub fn new_compute_cmd_enc(&self) -> Option<arc::R<mtl::ComputeCmdEncoder>>;

    pub fn compute<F: FnMut(&mut mtl::ComputeCmdEncoder)>(&mut self, mut commands: F) {
        let mut encoder = self
            .new_compute_cmd_enc()
            .expect("Can't create compute command encoder");
        commands(&mut encoder);
        encoder.end();
    }

    #[objc::msg_send(computeCommandEncoderWithDescriptor:)]
    pub fn new_compute_cmd_enc_desc_ar(
        &self,
        descriptor: &mtl::ComputePassDesc,
    ) -> Option<arc::Rar<mtl::ComputeCmdEncoder>>;

    #[objc::rar_retain()]
    pub fn new_compute_cmd_enc_desc(
        &self,
        descriptor: &mtl::ComputePassDesc,
    ) -> Option<arc::R<mtl::ComputeCmdEncoder>>;

    #[objc::msg_send(renderCommandEncoderWithDescriptor:)]
    pub fn new_render_cmd_enc_ar(
        &self,
        descriptor: &mtl::RenderPassDesc,
    ) -> Option<arc::Rar<mtl::RenderCmdEncoder>>;

    #[objc::rar_retain()]
    pub fn new_render_cmd_enc(
        &self,
        descriptor: &mtl::RenderPassDesc,
    ) -> Option<arc::R<mtl::RenderCmdEncoder>>;

    #[inline]
    pub fn render<F: FnMut(&mut mtl::RenderCmdEncoder)>(
        &mut self,
        descriptor: &mtl::RenderPassDesc,
        mut commands: F,
    ) {
        let mut encoder = self
            .new_render_cmd_enc(descriptor)
            .expect("Can't create render command encoder");
        commands(&mut encoder);
        // TODO: think. may be it should be guard?
        encoder.end();
    }

    /// Add a drawable present that will be invoked when this command buffer has
    /// been scheduled for execution.
    ///
    /// The submission thread will be lock stepped with present call been serviced
    /// by window server
    #[objc::msg_send(presentDrawable:)]
    pub fn present_drawable<O: objc::Obj, D: mtl::Drawable<O>>(&self, drawable: &D);

    /// Add a drawable present that will be invoked when this command buffer has been
    /// scheduled for execution.
    ///
    /// The submission thread will be lock stepped with present call been serviced
    /// by window server
    #[objc::msg_send(presentDrawable:atTime:)]
    pub fn present_drawable_at<O: objc::Obj, D: mtl::Drawable<O>>(
        &self,
        drawable: &D,
        at_time: cf::TimeInterval,
    );

    /// If an error occurred during execution, the ns::Error may contain more details
    /// about the problem.
    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<&ns::Error>;

    /// The host time in seconds that GPU starts executing this command buffer.
    /// Returns zero if it has not started. This usually can be called in command buffer completion handler.
    #[objc::msg_send(GPUStartTime)]
    pub fn gpu_start_time(&self) -> cf::TimeInterval;

    /// The host time in seconds that GPU finishes executing this command buffer.
    /// Returns zero if CPU has not received completion notification. This usually can
    /// be called in command buffer completion handler.
    #[objc::msg_send(GPUEndTime)]
    pub fn gpu_end_time(&self) -> cf::TimeInterval;

    /// The host time, in seconds, when the CPU begins to schedule the command buffer.
    #[objc::msg_send(kernelStartTime)]
    pub fn kernel_start_time(&self) -> cf::TimeInterval;

    /// The host time, in seconds, when the CPU finishes scheduling the command buffer.
    #[objc::msg_send(kernelEndTime)]
    pub fn kernel_end_time(&self) -> cf::TimeInterval;
}
