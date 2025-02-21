use crate::{arc, cf, cg, cv, os};

pub fn cg_image_from_cv_pixel_buf(
    pixel_buffer: &cv::PixelBuf,
    options: Option<&cf::Dictionary>,
) -> os::Result<arc::R<cg::Image>> {
    unsafe {
        os::result_unchecked(|res| VTCreateCGImageFromCVPixelBuffer(pixel_buffer, options, res))
    }
}

unsafe extern "C" {
    fn VTCreateCGImageFromCVPixelBuffer(
        pixel_buffer: &cv::PixelBuf,
        options: Option<&cf::Dictionary>,
        image_out: *mut Option<arc::R<cg::Image>>,
    ) -> os::Status;
}
