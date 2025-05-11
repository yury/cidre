use crate::{api, arc, blocks, cf, define_mtl, define_obj_type, mtl, ns, objc};

/// Reports the current stage in the lifetime of MTLCommandBuffer, as it proceeds to enqueued, committed, scheduled, and completed.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Status {
    /// The command buffer has not been enqueued yet.
    NotEnqueued = 0,

    /// This command buffer is enqueued, but not committed.
    Enqueued = 1,

    /// Commited to its command queue, but not yet scheduled for execution.
    Committed = 2,

    /// All dependencies have been resolved and the command buffer has been scheduled for execution.
    Scheduled = 3,

    /// The command buffer has finished executing successfully: any blocks set with -addCompletedHandler: may now be called.
    Completed = 4,

    /// Execution of the command buffer was aborted due to an error during execution.  Check -error for more information.
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
pub type CmdBufHandler = blocks::SyncBlock<fn(buf: &mut mtl::CmdBuf)>;

define_obj_type!(
    /// A serial list of commands for the device to execute.
    #[doc(alias = "MTLCommandBuffer")]
    pub CmdBuf(ns::Id)
);

impl CmdBuf {
    define_mtl!(set_label, push_debug_group, pop_debug_group);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

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
    pub fn new_blit_cmd_enc(&self) -> Option<arc::R<mtl::BlitCmdEncoder>>;

    pub fn blit<F: FnMut(&mut mtl::BlitCmdEncoder)>(&mut self, mut commands: F) {
        let mut encoder = self
            .new_blit_cmd_enc()
            .expect("Can't create blit command encoder");
        commands(&mut encoder);
        encoder.end();
    }

    #[objc::msg_send(computeCommandEncoder)]
    pub fn new_compute_cmd_enc(&self) -> Option<arc::R<mtl::ComputeCmdEncoder>>;

    pub fn compute<F: FnMut(&mut mtl::ComputeCmdEncoder)>(&mut self, mut commands: F) {
        let mut encoder = self
            .new_compute_cmd_enc()
            .expect("Can't create compute command encoder");
        commands(&mut encoder);
        encoder.end();
    }

    #[objc::msg_send(computeCommandEncoderWithDescriptor:)]
    pub fn new_compute_cmd_enc_desc(
        &self,
        descriptor: &mtl::ComputePassDesc,
    ) -> Option<arc::R<mtl::ComputeCmdEncoder>>;

    /// Encodes a command that pauses execution of this command buffer until the specified event reaches a given value.
    ///
    /// This method may only be called if there is no current command encoder on the receiver.
    #[objc::msg_send(encodeWaitForEvent:value:)]
    pub fn encode_wait_for_event(&mut self, event: &mtl::Event, value: u64);

    /// Encodes a command that signals an event with a given value.
    ///
    /// This method may only be called if there is no current command encoder on the receiver.
    #[objc::msg_send(encodeSignalEvent:value:)]
    pub fn encode_signal_event(&mut self, event: &mtl::Event, value: u64);

    #[objc::msg_send(renderCommandEncoderWithDescriptor:)]
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

    /// Reports the current stage in the lifetime of MTLCommandBuffer, as it proceeds
    /// to enqueued, committed, scheduled, and completed.
    #[objc::msg_send(status)]
    pub fn status(&self) -> Status;

    /// If an error occurred during execution, the ns::Error may contain more details
    /// about the problem.
    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<arc::R<ns::Error>>;

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

    /// Marks the residency set as part of the current command buffer execution.
    /// This ensures that the residency set is resident during execution of the command buffer.
    #[objc::msg_send(useResidencySet:)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn use_residency_set(&self, set: &mtl::ResidencySet);

    #[objc::msg_send(useResidencySets:count:)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub unsafe fn use_residency_sets_count(&self, sets: *const &mtl::ResidencySet, count: usize);

    /// Marks the residency sets as part of the current command buffer execution.
    /// This ensures that the residency sets are resident during execution of the command buffer.
    #[inline]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn use_residency_sets(&self, sets: &[&mtl::ResidencySet]) {
        unsafe { self.use_residency_sets_count(sets.as_ptr(), sets.len()) }
    }
}
