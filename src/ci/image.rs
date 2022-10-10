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

    /// A 32-bit-per-pixel, fixed-point pixel format in which
    /// the red, green, and blue color components precede the alpha value.
    #[inline]
    pub fn abgr8() -> Self {
        unsafe { kCIFormatABGR8 }
    }

    /// A 64-bit-per-pixel, floating-point pixel format.
    pub fn rgbah() -> Self {
        unsafe { kCIFormatRGBAh }
    }

    /// A 64-bit-per-pixel, fixed-point pixel format.
    pub fn rgba16() -> Self {
        unsafe { kCIFormatRGBA16 }
    }

    /// A 128-bit-per-pixel, floating-point pixel format.
    pub fn rgbaf() -> Self {
        unsafe { kCIFormatRGBAf }
    }

    /// An 8-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is alpha.
    pub fn a8() -> Self {
        unsafe { kCIFormatA8 }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is alpha.
    pub fn a16() -> Self {
        unsafe { kCIFormatA16 }
    }

    /// A 16-bit-per-pixel, half-width floating-point pixel format in which
    /// the sole component is alpha.
    pub fn ah() -> Self {
        unsafe { kCIFormatAh }
    }

    /// A 32-bit-per-pixel, full-width floating-point pixel format in which
    /// the sole component is alpha.
    pub fn af() -> Self {
        unsafe { kCIFormatAf }
    }

    /// An 8-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is a red color value.
    pub fn r8() -> Self {
        unsafe { kCIFormatR8 }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is a red color value.
    pub fn r16() -> Self {
        unsafe { kCIFormatR16 }
    }

    /// A 16-bit-per-pixel, floating-point pixel format in which
    /// the sole component is a red color value.
    pub fn rh() -> Self {
        unsafe { kCIFormatRh }
    }

    /// A 32-bit-per-pixel, floating-point pixel format in which
    /// the sole component is a red color value.
    pub fn rf() -> Self {
        unsafe { kCIFormatRf }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format
    /// with only red and green color components.
    pub fn rg8() -> Self {
        unsafe { kCIFormatRG8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format
    /// with only red and green color components.
    pub fn rg16() -> Self {
        unsafe { kCIFormatRG16 }
    }

    /// A 32-bit-per-pixel, floating-point pixel format
    /// with only red and green color components.
    pub fn rgh() -> Self {
        unsafe { kCIFormatRGh }
    }

    /// A 64-bit-per-pixel, floating-point pixel format
    /// with only red and green color components.
    pub fn rgf() -> Self {
        unsafe { kCIFormatRGf }
    }

    /// An 8-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is luminance.
    pub fn l8() -> Self {
        unsafe { kCIFormatL8 }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is luminance.
    pub fn l16() -> Self {
        unsafe { kCIFormatL16 }
    }

    /// A 16-bit-per-pixel, floating-point pixel format in which
    /// the sole component is luminance.
    pub fn lh() -> Self {
        unsafe { kCIFormatLh }
    }

    /// A 32-bit-per-pixel, floating-point pixel format in which
    /// the sole component is luminance.
    pub fn lf() -> Self {
        unsafe { kCIFormatLf }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format
    /// with only 8-bit luminance and alpha components.
    pub fn la8() -> Self {
        unsafe { kCIFormatLA8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format with
    /// only 16-bit luminance and alpha components.
    pub fn la16() -> Self {
        unsafe { kCIFormatLA16 }
    }

    /// A 32-bit-per-pixel, half-width floating-point pixel format
    /// with 16-bit luminance and alpha components.
    pub fn lah() -> Self {
        unsafe { kCIFormatLAh }
    }

    /// A 64-bit-per-pixel, full-width floating-point pixel format
    /// with 32-bit luminance and alpha components.
    pub fn laf() -> Self {
        unsafe { kCIFormatLAf }
    }
}

#[link(name = "CoreImage", kind = "framework")]
extern "C" {
    static kCIFormatARGB8: Format;
    static kCIFormatBGRA8: Format;
    static kCIFormatRGBA8: Format;
    static kCIFormatABGR8: Format;

    static kCIFormatRGBAh: Format;
    static kCIFormatRGBA16: Format;
    static kCIFormatRGBAf: Format;

    static kCIFormatA8: Format;
    static kCIFormatA16: Format;
    static kCIFormatAh: Format;
    static kCIFormatAf: Format;

    static kCIFormatR8: Format;
    static kCIFormatR16: Format;
    static kCIFormatRh: Format;
    static kCIFormatRf: Format;

    static kCIFormatRG8: Format;
    static kCIFormatRG16: Format;
    static kCIFormatRGh: Format;
    static kCIFormatRGf: Format;

    static kCIFormatL8: Format;
    static kCIFormatL16: Format;
    static kCIFormatLh: Format;
    static kCIFormatLf: Format;

    static kCIFormatLA8: Format;
    static kCIFormatLA16: Format;
    static kCIFormatLAh: Format;
    static kCIFormatLAf: Format;
}

#[link(name = "ci", kind = "static")]
extern "C" {
    fn CIImage_imageWithMTLTexture_options(
        texture: &mtl::Texture,
        options: Option<&cf::Dictionary>,
    ) -> Option<cf::Retained<Image>>;
}
