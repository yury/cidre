use crate::{cf, cg, cv, os};

pub fn create_cg_image_from_cv_pixel_buffer(
    pixel_buffer: &cv::PixelBuffer,
    options: Option<&cf::Dictionary>,
) -> Result<cf::Retained<cg::Image>, os::Status> {
    let mut image_out = None;
    unsafe {
        VTCreateCGImageFromCVPixelBuffer(pixel_buffer, options, &mut image_out)
            .to_result_unchecked(image_out)
    }
}

extern "C" {
    fn VTCreateCGImageFromCVPixelBuffer(
        pixel_buffer: &cv::PixelBuffer,
        options: Option<&cf::Dictionary>,
        image_out: *mut Option<cf::Retained<cg::Image>>,
    ) -> os::Status;
}
