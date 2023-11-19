use crate::{arc, av, define_obj_type, ns, objc};

define_obj_type!(pub VideoRenderer(ns::Id), AV_SAMPLE_BUFFER_VIDEO_RENDERER);

impl av::QueuedSampleBufRendering for VideoRenderer {}

impl VideoRenderer {
    #[objc::msg_send(status)]
    pub fn status(&self) -> av::QueuedSampleBufRenderingStatus;

    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<&ns::Error>;

    #[objc::msg_send(requiresFlushToResumeDecoding)]
    pub fn requires_flush_to_resume_decoding(&self) -> bool;
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static AV_SAMPLE_BUFFER_VIDEO_RENDERER: &'static objc::Class<VideoRenderer>;
}
