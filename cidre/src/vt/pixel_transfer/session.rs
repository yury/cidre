use crate::{arc, cf, cv, define_cf_type, os, vt};

define_cf_type!(Session(vt::Session));

impl Session {
    #[inline]
    pub fn create() -> Result<arc::R<Self>, os::Status> {
        Self::create_in(None)
    }

    #[inline]
    pub fn create_in(allocator: Option<&cf::Allocator>) -> Result<arc::R<Self>, os::Status> {
        unsafe {
            let mut result = None;
            VTPixelTransferSessionCreate(allocator, &mut result).to_result_unchecked(result)
        }
    }

    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { VTPixelTransferSessionInvalidate(self) }
    }

    #[inline]
    pub fn transfer_image(
        &self,
        src_buf: &cv::PixelBuffer,
        dst_buf: &cv::PixelBuffer,
    ) -> Result<(), os::Status> {
        unsafe { VTPixelTransferSessionTransferImage(self, src_buf, dst_buf).result() }
    }
}

extern "C" {
    fn VTPixelTransferSessionCreate(
        allocator: Option<&cf::Allocator>,
        pixel_transfer_session_out: &mut Option<arc::R<Session>>,
    ) -> os::Status;
    fn VTPixelTransferSessionInvalidate(session: &mut Session);
    fn VTPixelTransferSessionTransferImage(
        session: &Session,
        source_buffer: &cv::PixelBuffer,
        destination_buffer: &cv::PixelBuffer,
    ) -> os::Status;
}
