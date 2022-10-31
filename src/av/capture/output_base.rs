use crate::{av, cf, define_obj_type, objc::Id};

define_obj_type!(Output(Id));

impl Output {
    pub fn connections(&self) -> &cf::ArrayOf<av::CaptureConnection> {
        unsafe { rsel_connections(self) }
    }

    pub fn connection_with_media_type(
        &self,
        media_type: av::MediaType,
    ) -> Option<&av::CaptureConnection> {
        unsafe { rsel_connectionWithMediaType(self, media_type) }
    }
}

extern "C" {
    fn rsel_connections(output: &Output) -> &cf::ArrayOf<av::CaptureConnection>;
    fn rsel_connectionWithMediaType(
        output: &Output,
        media_type: av::MediaType,
    ) -> Option<&av::CaptureConnection>;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum DataDroppedReason {
    None = 0,
    LateData = 1,
    OutOfBuffers = 2,
    Discontinuity = 3,
}
