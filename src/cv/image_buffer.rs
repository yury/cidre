use crate::{cg, cv};

pub type ImageBuffer = cv::Buffer;

impl ImageBuffer {
    pub fn get_display_size(&self) -> cg::Size {
        unsafe { CVImageBufferGetDisplaySize(self) }
    }

    pub fn get_clean_rect(&self) -> cg::Rect {
      unsafe {
        CVImageBufferGetCleanRect(self)
      }
    }

    pub fn is_flipped(&self) -> bool {
      unsafe {
        CVImageBufferIsFlipped(self)
      }
    }
}

extern "C" {
    fn CVImageBufferGetDisplaySize(image_buffer: &ImageBuffer) -> cg::Size;
    fn CVImageBufferGetCleanRect(image_buffer: &ImageBuffer) -> cg::Rect;
    fn CVImageBufferIsFlipped(image_buffer: &ImageBuffer) -> bool;
}
