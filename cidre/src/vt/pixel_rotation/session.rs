use crate::{
    arc, cf, cv, define_cf_type, os, vt,
    vt::pixel_rotation_properties::{keys, rotation},
};

define_cf_type!(
    #[doc(alias = "VTPixelRotationSessionRef")]
    Session(vt::Session)
);

impl Session {
    #[doc(alias = "VTPixelRotationSessionGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { VTPixelRotationSessionGetTypeID() }
    }

    #[doc(alias = "VTPixelRotationSessionCreate")]
    #[inline]
    pub fn new() -> os::Result<arc::R<Self>> {
        Self::new_in(None)
    }

    #[doc(alias = "VTPixelRotationSessionCreate")]
    #[inline]
    pub fn new_in(allocator: Option<&cf::Allocator>) -> os::Result<arc::R<Self>> {
        unsafe { os::result_unchecked(|res| VTPixelRotationSessionCreate(allocator, res)) }
    }

    #[inline]
    pub fn set_rotate_0(&mut self) -> os::Result {
        self.set_prop(keys::rotation(), Some(rotation::_0()))
    }

    #[inline]
    pub fn set_rotate_cw_90(&mut self) -> os::Result {
        self.set_prop(keys::rotation(), Some(rotation::cw_90()))
    }

    #[inline]
    pub fn set_rotate_ccw_90(&mut self) -> os::Result {
        self.set_prop(keys::rotation(), Some(rotation::ccw_90()))
    }

    #[inline]
    pub fn set_rotate_180(&mut self) -> os::Result {
        self.set_prop(keys::rotation(), Some(rotation::_180()))
    }

    #[inline]
    pub fn set_vertical_flip(&mut self, value: bool) -> os::Result {
        let value: &'static cf::Boolean = value.into();
        self.set_prop(keys::flip_vertical_orientation(), Some(value))
    }

    #[inline]
    pub fn set_horizontal_flip(&mut self, value: bool) -> os::Result {
        let value: &'static cf::Boolean = value.into();
        self.set_prop(keys::flip_horizontal_orientation(), Some(value))
    }

    #[doc(alias = "VTPixelRotationSessionInvalidate")]
    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { VTPixelRotationSessionInvalidate(self) }
    }

    /// Rotates sourceBuffer and places the output in destinationBuffer.
    /// For 90 and 270 degree rotations, the width and height of destinationBuffer must be the inverse
    /// of sourceBuffer.
    /// For 180 degree rotations, the width and height of destinationBuffer and sourceBuffer must match.
    /// By default, all existing attachments on destinationBuffer are removed and new attachments
    /// are set describing the transferred image.  Unrecognised attachments on sourceBuffer will
    /// be propagated to destinationBuffer.
    /// Some properties may modify this behaviour; see VTPixelRotationProperties.h for more details.
    #[doc(alias = "VTPixelRotationSessionRotateImage")]
    #[inline]
    pub fn rotate(&self, src_buf: &cv::PixelBuf, dst_buf: &mut cv::PixelBuf) -> os::Result {
        unsafe { VTPixelRotationSessionRotateImage(self, src_buf, dst_buf).result() }
    }
}

extern "C-unwind" {
    fn VTPixelRotationSessionGetTypeID() -> cf::TypeId;
    fn VTPixelRotationSessionCreate(
        allocator: Option<&cf::Allocator>,
        pixel_rotation_session_out: *mut Option<arc::R<Session>>,
    ) -> os::Status;
    fn VTPixelRotationSessionInvalidate(session: &mut Session);
    fn VTPixelRotationSessionRotateImage(
        session: &Session,
        src_buf: &cv::PixelBuf,
        dst_buf: &mut cv::PixelBuf,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::vt;

    #[test]
    fn basics() {
        let mut s = vt::PixelRotationSession::new().unwrap();
        s.set_vertical_flip(true).unwrap();
        s.set_horizontal_flip(true).unwrap();
    }
}
