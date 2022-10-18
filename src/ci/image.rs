use std::mem::transmute;

use crate::{cf, define_cf_type, define_obj_type, mtl, objc};

define_obj_type!(Image(objc::Id));

impl Image {
    pub fn with_mtl_texture(
        texture: &mtl::Texture,
        options: Option<&cf::DictionaryOf<ImageOption, cf::Type>>,
    ) -> Option<cf::Retained<Self>> {
        unsafe { CIImage_imageWithMTLTexture_options(texture, transmute(options)) }
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

define_cf_type!(ImageOption(cf::String));

impl ImageOption {
    /// A &cg::ColorSpace defining the color space of the image. This value
    /// overrides the image's implicit color space.
    /// If cf::Null::null() then dont color manage the image.
    pub fn color_space() -> &'static Self {
        unsafe { kCIImageColorSpace }
    }

    pub fn tone_map_hdr_to_sdr() -> &'static Self {
        unsafe { kCIImageToneMapHDRtoSDR }
    }

    /// A boolean value specifying whether the image should sampled using "nearest neighbor"
    /// behavior.  If not specified, the image will be sampled using "linear sampling"
    pub fn nearest_sampling() -> &'static Self {
        unsafe { kCIImageNearestSampling }
    }

    pub fn properties() -> &'static Self {
        unsafe { kCIImageProperties }
    }

    pub fn apply_orientation_property() -> &'static Self {
        unsafe { kCIImageApplyOrientationProperty }
    }

    pub fn texture_target() -> &'static Self {
        unsafe { kCIImageTextureTarget }
    }

    pub fn texture_format() -> &'static Self {
        unsafe { kCIImageTextureFormat }
    }

    pub fn auxiliary_depth() -> &'static Self {
        unsafe { kCIImageAuxiliaryDepth }
    }

    pub fn auxiliary_disparity() -> &'static Self {
        unsafe { kCIImageAuxiliaryDisparity }
    }

    pub fn auxiliary_portrait_effects_matte() -> &'static Self {
        unsafe { kCIImageAuxiliaryPortraitEffectsMatte }
    }

    pub fn auxiliary_semantic_segmentation_skin_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationSkinMatte }
    }

    pub fn auxiliary_semantic_segmentation_hair_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationHairMatte }
    }

    pub fn auxiliary_semantic_segmentation_teeth_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationTeethMatte }
    }

    pub fn auxiliary_semantic_segmentation_glasses_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationGlassesMatte }
    }

    pub fn auxiliary_semantic_segmentation_sky_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationSkyMatte }
    }
}

#[link(name = "ci", kind = "static")]
extern "C" {
    fn CIImage_imageWithMTLTexture_options(
        texture: &mtl::Texture,
        options: Option<&cf::Dictionary>,
    ) -> Option<cf::Retained<Image>>;

    static kCIImageColorSpace: &'static ImageOption;
    static kCIImageToneMapHDRtoSDR: &'static ImageOption;
    static kCIImageNearestSampling: &'static ImageOption;
    static kCIImageProperties: &'static ImageOption;
    static kCIImageApplyOrientationProperty: &'static ImageOption;
    static kCIImageTextureTarget: &'static ImageOption;
    static kCIImageTextureFormat: &'static ImageOption;

    static kCIImageAuxiliaryDepth: &'static ImageOption;
    static kCIImageAuxiliaryDisparity: &'static ImageOption;
    static kCIImageAuxiliaryPortraitEffectsMatte: &'static ImageOption;
    static kCIImageAuxiliarySemanticSegmentationSkinMatte: &'static ImageOption;
    static kCIImageAuxiliarySemanticSegmentationHairMatte: &'static ImageOption;
    static kCIImageAuxiliarySemanticSegmentationTeethMatte: &'static ImageOption;
    static kCIImageAuxiliarySemanticSegmentationGlassesMatte: &'static ImageOption;
    static kCIImageAuxiliarySemanticSegmentationSkyMatte: &'static ImageOption;
}
