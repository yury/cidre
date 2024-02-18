use crate::{
    arc, cf, cv, define_cf_type, os, vt,
    vt::pixel_rotation_properties::{keys, rotation},
};

define_cf_type!(
    #[doc(alias = "VTPixelRotationSessionRef")]
    Session(vt::Session)
);

impl Session {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { VTPixelRotationSessionGetTypeID() }
    }

    #[inline]
    pub fn create() -> Result<arc::R<Self>, os::Status> {
        Self::create_in(None)
    }

    #[inline]
    pub fn create_in(allocator: Option<&cf::Allocator>) -> Result<arc::R<Self>, os::Status> {
        let mut out = None;
        unsafe { VTPixelRotationSessionCreate(allocator, &mut out).to_result_unchecked(out) }
    }

    #[inline]
    pub fn set_rotate_0(&mut self) -> Result<(), os::Status> {
        self.set_prop(keys::rotation(), Some(rotation::_0()))
    }

    #[inline]
    pub fn set_rotate_cw_90(&mut self) -> Result<(), os::Status> {
        self.set_prop(keys::rotation(), Some(rotation::cw_90()))
    }

    #[inline]
    pub fn set_rotate_ccw_90(&mut self) -> Result<(), os::Status> {
        self.set_prop(keys::rotation(), Some(rotation::ccw_90()))
    }

    #[inline]
    pub fn set_rotate_180(&mut self) -> Result<(), os::Status> {
        self.set_prop(keys::rotation(), Some(rotation::_180()))
    }

    #[inline]
    pub fn set_vertical_flip(&mut self, value: bool) -> Result<(), os::Status> {
        let value: &'static cf::Boolean = value.into();
        self.set_prop(keys::flip_vertical_orientation(), Some(value))
    }

    #[inline]
    pub fn set_horizontal_flip(&mut self, value: bool) -> Result<(), os::Status> {
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
    pub fn rotate(&self, src: &cv::PixelBuf, dst: &mut cv::PixelBuf) -> Result<(), os::Status> {
        unsafe { VTPixelRotationSessionRotateImage(self, src, dst).result() }
    }
}

extern "C" {
    fn VTPixelRotationSessionGetTypeID() -> cf::TypeId;
    fn VTPixelRotationSessionCreate(
        allocator: Option<&cf::Allocator>,
        pixel_rotation_session_out: *mut Option<arc::R<Session>>,
    ) -> os::Status;
    fn VTPixelRotationSessionInvalidate(session: &mut Session);
    fn VTPixelRotationSessionRotateImage(
        session: &Session,
        source_buffer: &cv::PixelBuf,
        destination_buffer: &mut cv::PixelBuf,
    ) -> os::Status;
}
