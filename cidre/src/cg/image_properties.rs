#[doc(alias = "kCGImagePropertyOrientation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Orientation {
    /// 0th row at top,    0th column on left   - default orientation
    #[doc(alias = "kCGImagePropertyOrientationUp")]
    Up = 1,

    /// 0th row at top,    0th column on right  - horizontal flip
    #[doc(alias = "kCGImagePropertyOrientationUpMirrored")]
    UpMirrored,

    /// 0th row at bottom, 0th column on right  - 180 deg rotation
    #[doc(alias = "kCGImagePropertyOrientationDown")]
    Down,

    /// 0th row at bottom, 0th column on left   - vertical flip
    #[doc(alias = "kCGImagePropertyOrientationDownMirrored")]
    DownMirrored,

    /// 0th row on left,   0th column at top
    #[doc(alias = "kCGImagePropertyOrientationLeftMirrored")]
    LeftMirrored,

    /// 0th row on right,  0th column at top    - 90 deg CW
    #[doc(alias = "kCGImagePropertyOrientationRight")]
    Right,

    /// 0th row on right,  0th column on bottom
    #[doc(alias = "kCGImagePropertyOrientationRightMirrored")]
    RightMirrored,

    /// 0th row on left,   0th column at bottom - 90 deg CCW
    #[doc(alias = "kCGImagePropertyOrientationLeft")]
    Left,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Up
    }
}

/// Properties that, if returned by CGImageSourceCopyProperties or
/// CGImageSourceCopyPropertiesAtIndex, contain a dictionary of file-format
/// or metadata-format specific key-values.
pub mod dictionary {
    use crate::cf;

    #[doc(alias = "kCGImagePropertyTIFFDictionary")]
    #[inline]
    pub fn tiff() -> &'static cf::String {
        unsafe { kCGImagePropertyTIFFDictionary }
    }

    #[doc(alias = "kCGImagePropertyGIFDictionary")]
    #[inline]
    pub fn gif() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFDictionary }
    }

    #[doc(alias = "kCGImagePropertyHEIFDictionary")]
    #[inline]
    pub fn heif() -> &'static cf::String {
        unsafe { kCGImagePropertyHEIFDictionary }
    }

    #[doc(alias = "kCGImagePropertyHEICSDictionary")]
    #[inline]
    pub fn heic() -> &'static cf::String {
        unsafe { kCGImagePropertyHEICSDictionary }
    }

    #[doc(alias = "kCGImagePropertyExifDictionary")]
    #[inline]
    pub fn exif() -> &'static cf::String {
        unsafe { kCGImagePropertyExifDictionary }
    }

    #[doc(alias = "kCGImagePropertyPNGDictionary")]
    #[inline]
    pub fn png() -> &'static cf::String {
        unsafe { kCGImagePropertyPNGDictionary }
    }

    /// A dictionary of key-value pairs for an image that has Global Positioning System (GPS) information.
    #[doc(alias = "kCGImagePropertyGPSDictionary")]
    #[inline]
    pub fn gps() -> &'static cf::String {
        unsafe { kCGImagePropertyGPSDictionary }
    }

    #[doc(alias = "kCGImagePropertyRawDictionary")]
    #[inline]
    pub fn raw() -> &'static cf::String {
        unsafe { kCGImagePropertyRawDictionary }
    }

    #[doc(alias = "kCGImagePropertyCIFFDictionary")]
    #[inline]
    pub fn ciff() -> &'static cf::String {
        unsafe { kCGImagePropertyCIFFDictionary }
    }

    #[doc(alias = "kCGImagePropertyWebPDictionary")]
    pub fn webp() -> &'static cf::String {
        unsafe { kCGImagePropertyWebPDictionary }
    }

    #[link(name = "ImageIO", kind = "framework")]
    extern "C" {
        static kCGImagePropertyTIFFDictionary: &'static cf::String;
        static kCGImagePropertyGIFDictionary: &'static cf::String;
        static kCGImagePropertyHEIFDictionary: &'static cf::String;
        static kCGImagePropertyHEICSDictionary: &'static cf::String;
        static kCGImagePropertyExifDictionary: &'static cf::String;
        static kCGImagePropertyPNGDictionary: &'static cf::String;
        static kCGImagePropertyGPSDictionary: &'static cf::String;
        static kCGImagePropertyRawDictionary: &'static cf::String;
        static kCGImagePropertyCIFFDictionary: &'static cf::String;
        // ...

        static kCGImagePropertyWebPDictionary: &'static cf::String;
    }
}

pub mod webp_keys {
    use crate::cf;

    /// The number of times to play the sequence.
    #[doc(alias = "kCGImagePropertyWebPLoopCount")]
    #[inline]
    pub fn loop_count() -> &'static cf::String {
        unsafe { kCGImagePropertyWebPLoopCount }
    }

    /// The number of seconds to wait before displaying the next image in the sequence.
    ///
    /// The value of this key is a cf::Number with a floating-point value. The value of this key is never less than
    /// 100 millseconds, and the system adjusts values less than that amount to 100 milliseconds, as needed.
    /// See kCGImagePropertyGIFUnclampedDelayTime.
    #[doc(alias = "kCGImagePropertyWebPDelayTime")]
    #[inline]
    pub fn deplay_time() -> &'static cf::String {
        unsafe { kCGImagePropertyWebPDelayTime }
    }

    /// The unadjusted number of seconds to wait before displaying the next image in the sequence.
    ///
    /// The value of this key is a cf::Number with a floating-point value.
    #[doc(alias = "kCGImagePropertyWebPUnclampedDelayTime")]
    #[inline]
    pub fn unclamped_delay_time() -> &'static cf::String {
        unsafe { kCGImagePropertyWebPUnclampedDelayTime }
    }

    /// An array of dictionaries that contain timing information for the image sequence.
    ///
    /// The value of this property is a cf::Array. Each cf::Dictionary in the array contains timing information about
    /// an image in the sequence.
    #[doc(alias = "kCGImagePropertyWebPFrameInfoArray")]
    #[inline]
    pub fn frame_info_array() -> &'static cf::String {
        unsafe { kCGImagePropertyWebPFrameInfoArray }
    }

    /// The width of the main image, in pixels.
    #[doc(alias = "kCGImagePropertyWebPCanvasPixelWidth")]
    #[inline]
    pub fn canvas_pixel_width() -> &'static cf::String {
        unsafe { kCGImagePropertyWebPCanvasPixelWidth }
    }

    /// The height of the main image, in pixels.
    #[doc(alias = "kCGImagePropertyWebPCanvasPixelHeight")]
    #[inline]
    pub fn canvas_pixel_height() -> &'static cf::String {
        unsafe { kCGImagePropertyWebPCanvasPixelHeight }
    }

    #[link(name = "ImageIO", kind = "framework")]
    extern "C" {
        static kCGImagePropertyWebPLoopCount: &'static cf::String;
        static kCGImagePropertyWebPDelayTime: &'static cf::String;
        static kCGImagePropertyWebPUnclampedDelayTime: &'static cf::String;
        static kCGImagePropertyWebPFrameInfoArray: &'static cf::String;
        static kCGImagePropertyWebPCanvasPixelWidth: &'static cf::String;
        static kCGImagePropertyWebPCanvasPixelHeight: &'static cf::String;
    }
}

pub mod gif_keys {
    use crate::cf;

    /// The number of times to repeat an animated sequence.
    #[doc(alias = "kCGImagePropertyGIFLoopCount")]
    #[inline]
    pub fn loop_count() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFLoopCount }
    }

    /// The number of seconds to wait before displaying the next image in an animated sequence,
    /// clamped to a minimum of 100 milliseconds.
    #[doc(alias = "kCGImagePropertyGIFDelayTime")]
    #[inline]
    pub fn delay_time() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFDelayTime }
    }

    /// The image color map.
    #[doc(alias = "kCGImagePropertyGIFImageColorMap")]
    #[inline]
    pub fn image_color_map() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFImageColorMap }
    }

    /// A Boolean value that indicates whether the GIF has a global color map.
    ///
    /// The value of this key is a cf::Boolean.
    #[doc(alias = "kCGImagePropertyGIFHasGlobalColorMap")]
    #[inline]
    pub fn has_global_color_map() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFHasGlobalColorMap }
    }

    /// The number of seconds to wait before displaying the next image in an animated sequence.
    #[doc(alias = "kCGImagePropertyGIFUnclampedDelayTime")]
    #[inline]
    pub fn unclamped_delay_time() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFUnclampedDelayTime }
    }

    /// The width of the main image, in pixels.
    #[doc(alias = "kCGImagePropertyGIFCanvasPixelWidth")]
    #[inline]
    pub fn canvas_pixel_width() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFCanvasPixelWidth }
    }

    /// The height of the main image, in pixels.
    #[doc(alias = "kCGImagePropertyGIFCanvasPixelHeight")]
    #[inline]
    pub fn canvas_pixel_height() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFCanvasPixelHeight }
    }

    /// An array of dictionaries that contain timing information for the image sequence.
    ///
    /// The value of this property is a cf::Array. Each cf::Dictionary in the array contains timing information
    /// about an image in the sequence.
    #[doc(alias = "kCGImagePropertyGIFFrameInfoArray")]
    #[inline]
    pub fn frame_info_array() -> &'static cf::String {
        unsafe { kCGImagePropertyGIFFrameInfoArray }
    }

    #[link(name = "ImageIO", kind = "framework")]
    extern "C" {
        static kCGImagePropertyGIFLoopCount: &'static cf::String;
        static kCGImagePropertyGIFDelayTime: &'static cf::String;
        static kCGImagePropertyGIFImageColorMap: &'static cf::String;
        static kCGImagePropertyGIFHasGlobalColorMap: &'static cf::String;
        static kCGImagePropertyGIFUnclampedDelayTime: &'static cf::String;
        static kCGImagePropertyGIFCanvasPixelWidth: &'static cf::String;
        static kCGImagePropertyGIFCanvasPixelHeight: &'static cf::String;
        static kCGImagePropertyGIFFrameInfoArray: &'static cf::String;
    }
}
