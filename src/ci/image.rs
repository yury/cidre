use crate::{cf, define_obj_type, mtl, objc};

define_obj_type!(Image(objc::Id));

impl Image {
    pub fn with_mtl_texture(
        texture: &mtl::Texture,
        options: Option<&cf::Dictionary>,
    ) -> Option<cf::Retained<Self>> {
        unsafe { CIImage_imageWithMTLTexture_options(texture, options) }
    }
}

/// Pixel data formats for image input, output, and processing.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Format(pub i32);

impl Format {
    /// A 32-bit-per-pixel, fixed-point pixel format in which
    /// the alpha value precedes the red, green, and blue color components.
    #[inline]
    pub fn argb8() -> Self {
        unsafe { kCIFormatARGB8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format in which
    /// the blue, green, and red color components precede the alpha value.
    #[inline]
    pub fn bgra8() -> Self {
        unsafe { kCIFormatBGRA8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format in which
    /// the red, green, and blue color components precede the alpha value.
    #[inline]
    pub fn rgba8() -> Self {
        unsafe { kCIFormatRGBA8 }
    }
}
#[link(name = "CoreImage", kind = "framework")]
extern "C" {
    static kCIFormatARGB8: Format;
    static kCIFormatBGRA8: Format;
    static kCIFormatRGBA8: Format;
}

#[link(name = "ci", kind = "static")]
extern "C" {
    fn CIImage_imageWithMTLTexture_options(
        texture: &mtl::Texture,
        options: Option<&cf::Dictionary>,
    ) -> Option<cf::Retained<Image>>;
}
