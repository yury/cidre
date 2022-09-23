use crate::{cf, cv, define_cf_type, os, vt};

define_cf_type!(Session(vt::Session));

impl Session {
    pub fn create() -> Result<cf::Retained<Self>, os::Status> {
        Self::create_in(None)
    }

    pub fn create_in(allocator: Option<&cf::Allocator>) -> Result<cf::Retained<Self>, os::Status> {
        let mut out = None;
        unsafe { VTPixelRotationSessionCreate(allocator, &mut out).to_result_unchecked(out) }
    }

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
    #[inline]
    pub fn rorate_image(
        &self,
        source_buffer: &cv::PixelBuffer,
        destination_buffer: &mut cv::PixelBuffer,
    ) -> Result<(), os::Status> {
        unsafe {
            VTPixelRotationSessionRotateImage(self, source_buffer, destination_buffer).result()
        }
    }
}

extern "C" {
    fn VTPixelRotationSessionCreate(
        allocator: Option<&cf::Allocator>,
        pixel_rotation_session_out: &mut Option<cf::Retained<Session>>,
    ) -> os::Status;

    fn VTPixelRotationSessionInvalidate(session: &mut Session);

    fn VTPixelRotationSessionRotateImage(
        session: &Session,
        source_buffer: &cv::PixelBuffer,
        destination_buffer: &mut cv::PixelBuffer,
    ) -> os::Status;
}
