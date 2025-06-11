use crate::{arc, blocks, cf, define_obj_type, ns, objc};

pub type CommitFeedbackHandler = blocks::EscBlock<fn(commit_feedback: &CommitFeedback)>;

define_obj_type!(
    #[doc(alias = "MTL4CommitFeedback")]
    pub CommitFeedback(ns::Id)
);

impl CommitFeedback {
    /// A description of an error when the GPU encounters an issue as it runs the committed command buffers.
    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<arc::R<ns::Error>>;

    /// The host time, in seconds, when the GPU starts execution of the committed command buffers.
    #[objc::msg_send(GPUStartTime)]
    pub fn gpu_start_time(&self) -> cf::TimeInterval;

    /// The host time, in seconds, when the GPU finishes execution of the committed command buffers.
    #[objc::msg_send(GPUEndTime)]
    pub fn gpu_end_time(&self) -> cf::TimeInterval;
}
