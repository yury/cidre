use crate::{cg, cv, cf};

pub type ImageBuffer = cv::Buffer;

impl ImageBuffer {
    #[inline]
    pub fn get_encoded_size(&self) -> cg::Size {
        unsafe { CVImageBufferGetEncodedSize(self) }
    }
    #[inline]
    pub fn get_display_size(&self) -> cg::Size {
        unsafe { CVImageBufferGetDisplaySize(self) }
    }

    #[inline]
    pub fn get_clean_rect(&self) -> cg::Rect {
        unsafe { CVImageBufferGetCleanRect(self) }
    }

    #[inline]
    pub fn is_flipped(&self) -> bool {
        unsafe { CVImageBufferIsFlipped(self) }
    }

    #[inline]
    pub fn get_color_space(&self) -> Option<&cg::ColorSpace> {
      unsafe {
        CVImageBufferGetColorSpace(self)
      }
    }

    #[inline]
    pub fn create_color_space_form_attachments(attachments: &cf::Dictionary) -> Option<cf::Retained<cg::ColorSpace>> {
      unsafe {
        CVImageBufferCreateColorSpaceFromAttachments(attachments)
      }
    } 
}

extern "C" {
    fn CVImageBufferGetColorSpace(image_buffer: &ImageBuffer) -> Option<&cg::ColorSpace>;
    fn CVImageBufferGetEncodedSize(image_buffer: &ImageBuffer) -> cg::Size;
    fn CVImageBufferGetDisplaySize(image_buffer: &ImageBuffer) -> cg::Size;
    fn CVImageBufferGetCleanRect(image_buffer: &ImageBuffer) -> cg::Rect;
    fn CVImageBufferIsFlipped(image_buffer: &ImageBuffer) -> bool;
    fn CVImageBufferCreateColorSpaceFromAttachments(attachments: &cf::Dictionary) -> Option<cf::Retained<cg::ColorSpace>>;
}
