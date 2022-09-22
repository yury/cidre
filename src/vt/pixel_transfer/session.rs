use crate::{
    cf::{self, Retained},
    cv, define_cf_type, os, vt,
};

define_cf_type!(Session(vt::Session));

impl Session {
    pub fn create(allocator: Option<&cf::Allocator>) -> Result<Retained<Self>, os::Status> {
        unsafe {
            let mut pixel_transfer_ssession_out = None;
            VTPixelTransferSessionCreate(allocator, &mut pixel_transfer_ssession_out)
                .to_result_unchecked(pixel_transfer_ssession_out)
        }
    }

    pub fn invalidate(&self) {
        unsafe { VTPixelTransferSessionInvalidate(self) }
    }

    pub fn transfer_image(
        &self,
        source_buffer: &cv::PixelBuffer,
        destination_buffer: &cv::PixelBuffer,
    ) -> Result<(), os::Status> {
        unsafe {
            VTPixelTransferSessionTransferImage(self, source_buffer, destination_buffer).result()
        }
    }
}

extern "C" {
    fn VTPixelTransferSessionCreate(
        allocator: Option<&cf::Allocator>,
        pixel_transfer_ssession_out: &mut Option<Retained<Session>>,
    ) -> os::Status;

    fn VTPixelTransferSessionInvalidate(session: &Session);
    fn VTPixelTransferSessionTransferImage(
        session: &Session,
        source_buffer: &cv::PixelBuffer,
        destination_buffer: &cv::PixelBuffer,
    ) -> os::Status;
}
