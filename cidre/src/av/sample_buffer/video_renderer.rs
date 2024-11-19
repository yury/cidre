use crate::{api, arc, av, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVSampleBufferVideoRenderer")]
    pub VideoRenderer(ns::Id),
    AV_SAMPLE_BUFFER_VIDEO_RENDERER,
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0, visionos = 1.0)]
);

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
