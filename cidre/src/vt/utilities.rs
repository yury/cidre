use crate::{arc, cf, cg, cv, os};

pub fn cg_image_from_cv_pixel_buf(
    pixel_buffer: &cv::PixelBuf,
    options: Option<&cf::Dictionary>,
) -> Result<arc::R<cg::Image>, os::Status> {
    let mut image_out = None;
    unsafe {
        VTCreateCGImageFromCVPixelBuffer(pixel_buffer, options, &mut image_out)
            .to_result_unchecked(image_out)
    }
}

extern "C" {
    fn VTCreateCGImageFromCVPixelBuffer(
        pixel_buffer: &cv::PixelBuf,
        options: Option<&cf::Dictionary>,
        image_out: *mut Option<arc::R<cg::Image>>,
    ) -> os::Status;
}
