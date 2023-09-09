use crate::{arc, av, ca, cm, define_obj_type, ns, objc};

define_obj_type!(VideoRenderer(ns::Id), AV_SAMPLE_BUFFER_VIDEO_RENDERER);
impl VideoRenderer {}

#[link(name = "ca", kind = "static")]
extern "C" {
    static AV_SAMPLE_BUFFER_VIDEO_RENDERER: &'static objc::Class<VideoRenderer>;
}
