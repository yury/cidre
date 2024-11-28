use crate::{arc, av, define_obj_type, ns, objc};

define_obj_type!(pub Output(ns::Id));

impl Output {
    #[objc::msg_send(connections)]
    pub fn connections(&self) -> arc::R<ns::Array<av::CaptureConnection>>;

    #[objc::msg_send(connectionWithMediaType:)]
    pub fn connection_with_media_type(
        &self,
        media_type: av::MediaType,
    ) -> Option<arc::R<av::CaptureConnection>>;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum DataDroppedReason {
    None = 0,
    LateData = 1,
    OutOfBuffers = 2,
    Discontinuity = 3,
}
