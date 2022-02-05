use crate::{cg, cv};

pub type ImageBuffer = cv::Buffer;

impl ImageBuffer {
    pub fn get_display_size(&self) -> cg::Size {
        unsafe { CVImageBufferGetDisplaySize(self) }
    }
}

extern "C" {
    fn CVImageBufferGetDisplaySize(image_buffer: &ImageBuffer) -> cg::Size;
}
