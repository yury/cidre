use crate::{cf, cg, cv};

pub type ImageBuffer = cv::Buffer;

impl ImageBuffer {
    /// Returns the full encoded dimensions of a cv::ImageBuffer.  For example, for an NTSC DV frame this would be 720x480
    ///
    /// Example:
    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32BGRA, None).unwrap();
    ///
    /// let size = pixel_buffer.get_encoded_size();
    /// assert_eq!(cg::Size { width: 200.0, height: 100.0 }, size);
    /// ```
    #[inline]
    pub fn get_encoded_size(&self) -> cg::Size {
        unsafe { CVImageBufferGetEncodedSize(self) }
    }

    /// Returns the nominal output display size, in square pixels, of a Core Video image buffer.
    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32BGRA, None).unwrap();
    ///
    /// let display_size = pixel_buffer.get_display_size();
    /// assert_eq!(cg::Size { width: 200.0, height: 100.0}, display_size);
    /// ```
    #[inline]
    pub fn get_display_size(&self) -> cg::Size {
        unsafe { CVImageBufferGetDisplaySize(self) }
    }

    /// Returns the source rectangle of a Core Video image buffer that represents the clean aperture of the buffer in encoded pixels.
    /// 
    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32BGRA, None).unwrap();
    ///
    /// let rect = pixel_buffer.get_clean_rect();
    /// assert_eq!(cg::Rect { origin: cg::Point::zero(), size: cg::Size { width: 200.0, height: 100.0 }}, rect);
    /// ```
    #[inline]
    pub fn get_clean_rect(&self) -> cg::Rect {
        unsafe { CVImageBufferGetCleanRect(self) }
    }

    /// Returns a Boolean value indicating whether the image is flipped vertically.
    /// 
    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32BGRA, None).unwrap();
    ///
    /// assert_eq!(true, pixel_buffer.is_flipped());
    /// ```
    #[inline]
    pub fn is_flipped(&self) -> bool {
        unsafe { CVImageBufferIsFlipped(self) }
    }

    #[inline]
    pub fn get_color_space(&self) -> Option<&cg::ColorSpace> {
        unsafe { CVImageBufferGetColorSpace(self) }
    }

    #[inline]
    pub fn create_color_space_form_attachments(
        attachments: &cf::Dictionary,
    ) -> Option<cf::Retained<cg::ColorSpace>> {
        unsafe { CVImageBufferCreateColorSpaceFromAttachments(attachments) }
    }
}

extern "C" {
    fn CVImageBufferGetColorSpace(image_buffer: &ImageBuffer) -> Option<&cg::ColorSpace>;
    fn CVImageBufferGetEncodedSize(image_buffer: &ImageBuffer) -> cg::Size;
    fn CVImageBufferGetDisplaySize(image_buffer: &ImageBuffer) -> cg::Size;
    fn CVImageBufferGetCleanRect(image_buffer: &ImageBuffer) -> cg::Rect;
    fn CVImageBufferIsFlipped(image_buffer: &ImageBuffer) -> bool;
    fn CVImageBufferCreateColorSpaceFromAttachments(
        attachments: &cf::Dictionary,
    ) -> Option<cf::Retained<cg::ColorSpace>>;
}
