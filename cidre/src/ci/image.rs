use crate::{api, arc, cf, cg, define_cls, define_obj_type, mtl, ns, objc};

define_obj_type!(
    /// A representation of an image to be processed or produced by Core Image filters.
    #[doc(alias = "CIImage")]
    pub Image(ns::Id)
);

impl arc::A<Image> {
    #[objc::msg_send(initWithMTLTexture:options:)]
    pub fn init_with_mlt_texture_options(
        self,
        texture: &mtl::Texture,
        options: Option<&cf::DictionaryOf<ImageOption, cf::Type>>,
    ) -> Option<arc::Retained<Image>>;
}

impl Image {
    define_cls!(CI_IMAGE);

    pub fn with_mtl_texture(
        texture: &mtl::Texture,
        options: Option<&cf::DictionaryOf<ImageOption, cf::Type>>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_mlt_texture_options(texture, options)
    }

    #[objc::msg_send(imageByClampingToRect:)]
    pub fn clamped_to(&self, rect: cg::Rect) -> arc::R<Self>;

    #[objc::msg_send(imageByCroppingToRect:)]
    pub fn cropped_to(&self, rect: cg::Rect) -> arc::R<Self>;

    #[objc::msg_send(blackImage)]
    pub fn black() -> &'static Self;

    #[objc::msg_send(whiteImage)]
    pub fn white() -> &'static Self;

    #[objc::msg_send(grayImage)]
    pub fn gray() -> &'static Self;

    #[objc::msg_send(redImage)]
    pub fn red() -> &'static Self;

    #[objc::msg_send(greenImage)]
    pub fn green() -> &'static Self;

    #[objc::msg_send(blueImage)]
    pub fn blue() -> &'static Self;

    #[objc::msg_send(cyanImage)]
    pub fn cyan() -> &'static Self;

    #[objc::msg_send(mangentaImage)]
    pub fn mangenta() -> &'static Self;

    #[objc::msg_send(yellowImage)]
    pub fn yellow() -> &'static Self;

    #[objc::msg_send(clearImage)]
    pub fn clear() -> &'static Self;

    #[objc::msg_send(emptyImage)]
    pub fn empty() -> &'static Self;

    #[objc::msg_send(colorSpace)]
    pub fn color_space(&self) -> Option<&cg::ColorSpace>;

    #[objc::msg_send(contentHeadroom)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn content_headroom(&self) -> f32;

    #[objc::msg_send(metalTexture)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn mtl_texture(&self) -> Option<arc::R<mtl::Texture>>;
}

/// Pixel data formats for image input, output, and processing.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Format(pub i32);

impl Format {
    /// A 32-bit-per-pixel, fixed-point pixel format in which
    /// the alpha value precedes the red, green, and blue color components.
    #[doc(alias = "kCIFormatARGB8")]
    #[inline]
    pub fn argb8() -> Self {
        unsafe { kCIFormatARGB8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format in which
    /// the blue, green, and red color components precede the alpha value.
    #[doc(alias = "kCIFormatBGRA8")]
    #[inline]
    pub fn bgra8() -> Self {
        unsafe { kCIFormatBGRA8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format in which
    /// the red, green, and blue color components precede the alpha value.
    #[doc(alias = "kCIFormatRGBA8")]
    #[inline]
    pub fn rgba8() -> Self {
        unsafe { kCIFormatRGBA8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format in which
    /// the red, green, and blue color components precede the alpha value.
    #[doc(alias = "kCIFormatABGR8")]
    #[inline]
    pub fn abgr8() -> Self {
        unsafe { kCIFormatABGR8 }
    }

    /// A 64-bit-per-pixel, floating-point pixel format.
    #[doc(alias = "kCIFormatRGBAh")]
    #[inline]
    pub fn rgbah() -> Self {
        unsafe { kCIFormatRGBAh }
    }

    /// A 64-bit-per-pixel, fixed-point pixel format.
    #[doc(alias = "kCIFormatRGBA16")]
    #[inline]
    pub fn rgba16() -> Self {
        unsafe { kCIFormatRGBA16 }
    }

    /// A 128-bit-per-pixel, floating-point pixel format.
    #[doc(alias = "kCIFormatRGBAf")]
    #[inline]
    pub fn rgbaf() -> Self {
        unsafe { kCIFormatRGBAf }
    }

    /// An 8-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is alpha.
    #[doc(alias = "kCIFormatA8")]
    #[inline]
    pub fn a8() -> Self {
        unsafe { kCIFormatA8 }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is alpha.
    #[doc(alias = "kCIFormatA16")]
    #[inline]
    pub fn a16() -> Self {
        unsafe { kCIFormatA16 }
    }

    /// A 16-bit-per-pixel, half-width floating-point pixel format in which
    /// the sole component is alpha.
    #[doc(alias = "kCIFormatAh")]
    #[inline]
    pub fn ah() -> Self {
        unsafe { kCIFormatAh }
    }

    /// A 32-bit-per-pixel, full-width floating-point pixel format in which
    /// the sole component is alpha.
    #[doc(alias = "kCIFormatAf")]
    #[inline]
    pub fn af() -> Self {
        unsafe { kCIFormatAf }
    }

    /// An 8-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is a red color value.
    #[doc(alias = "kCIFormatR8")]
    #[inline]
    pub fn r8() -> Self {
        unsafe { kCIFormatR8 }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is a red color value.
    #[doc(alias = "kCIFormatR16")]
    #[inline]
    pub fn r16() -> Self {
        unsafe { kCIFormatR16 }
    }

    /// A 16-bit-per-pixel, floating-point pixel format in which
    /// the sole component is a red color value.
    #[doc(alias = "kCIFormatRh")]
    #[inline]
    pub fn rh() -> Self {
        unsafe { kCIFormatRh }
    }

    /// A 32-bit-per-pixel, floating-point pixel format in which
    /// the sole component is a red color value.
    #[inline]
    #[doc(alias = "kCIFormatRf")]
    pub fn rf() -> Self {
        unsafe { kCIFormatRf }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format
    /// with only red and green color components.
    #[doc(alias = "kCIFormatRG8")]
    #[inline]
    pub fn rg8() -> Self {
        unsafe { kCIFormatRG8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format
    /// with only red and green color components.
    #[doc(alias = "kCIFormatRG16")]
    #[inline]
    pub fn rg16() -> Self {
        unsafe { kCIFormatRG16 }
    }

    /// A 32-bit-per-pixel, floating-point pixel format
    /// with only red and green color components.
    #[doc(alias = "kCIFormatRGh")]
    #[inline]
    pub fn rgh() -> Self {
        unsafe { kCIFormatRGh }
    }

    /// A 64-bit-per-pixel, floating-point pixel format
    /// with only red and green color components.
    #[doc(alias = "kCIFormatRGf")]
    #[inline]
    pub fn rgf() -> Self {
        unsafe { kCIFormatRGf }
    }

    /// An 8-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is luminance.
    #[doc(alias = "kCIFormatL8")]
    #[inline]
    pub fn l8() -> Self {
        unsafe { kCIFormatL8 }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format in which
    /// the sole component is luminance.
    #[doc(alias = "kCIFormatL16")]
    #[inline]
    pub fn l16() -> Self {
        unsafe { kCIFormatL16 }
    }

    /// A 16-bit-per-pixel, floating-point pixel format in which
    /// the sole component is luminance.
    #[doc(alias = "kCIFormatLh")]
    #[inline]
    pub fn lh() -> Self {
        unsafe { kCIFormatLh }
    }

    /// A 32-bit-per-pixel, floating-point pixel format in which
    /// the sole component is luminance.
    #[doc(alias = "kCIFormatLf")]
    #[inline]
    pub fn lf() -> Self {
        unsafe { kCIFormatLf }
    }

    /// A 16-bit-per-pixel, fixed-point pixel format
    /// with only 8-bit luminance and alpha components.
    #[doc(alias = "kCIFormatLA8")]
    #[inline]
    pub fn la8() -> Self {
        unsafe { kCIFormatLA8 }
    }

    /// A 32-bit-per-pixel, fixed-point pixel format with
    /// only 16-bit luminance and alpha components.
    #[doc(alias = "kCIFormatLA16")]
    #[inline]
    pub fn la16() -> Self {
        unsafe { kCIFormatLA16 }
    }

    /// A 32-bit-per-pixel, half-width floating-point pixel format
    /// with 16-bit luminance and alpha components.
    #[doc(alias = "kCIFormatLAh")]
    #[inline]
    pub fn lah() -> Self {
        unsafe { kCIFormatLAh }
    }

    /// A 64-bit-per-pixel, full-width floating-point pixel format
    /// with 32-bit luminance and alpha components.
    #[doc(alias = "kCIFormatLAf")]
    #[inline]
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

define_obj_type!(
    #[doc(alias = "CIImageOption")]
    pub ImageOption(ns::String)
);

impl ImageOption {
    /// A &cg::ColorSpace defining the color space of the image. This value
    /// overrides the image's implicit color space.
    /// If cf::Null::null() then dont color manage the image.
    #[doc(alias = "kCIImageColorSpace")]
    #[inline]
    pub fn color_space() -> &'static Self {
        unsafe { kCIImageColorSpace }
    }

    #[doc(alias = "kCIImageToneMapHDRtoSDR")]
    #[inline]
    pub fn tone_map_hdr_to_sdr() -> &'static Self {
        unsafe { kCIImageToneMapHDRtoSDR }
    }

    /// A boolean value specifying whether the image should sampled using "nearest neighbor"
    /// behavior.  If not specified, the image will be sampled using "linear sampling"
    #[doc(alias = "kCIImageNearestSampling")]
    #[inline]
    pub fn nearest_sampling() -> &'static Self {
        unsafe { kCIImageNearestSampling }
    }

    #[doc(alias = "kCIImageProperties")]
    #[inline]
    pub fn props() -> &'static Self {
        unsafe { kCIImageProperties }
    }

    #[doc(alias = "kCIImageApplyOrientationProperty")]
    #[inline]
    pub fn apply_orientation_prop() -> &'static Self {
        unsafe { kCIImageApplyOrientationProperty }
    }

    #[doc(alias = "kCIImageTextureTarget")]
    #[inline]
    pub fn texture_target() -> &'static Self {
        unsafe { kCIImageTextureTarget }
    }

    #[doc(alias = "kCIImageTextureFormat")]
    #[inline]
    pub fn texture_format() -> &'static Self {
        unsafe { kCIImageTextureFormat }
    }

    #[doc(alias = "kCIImageAuxiliaryDepth")]
    #[inline]
    pub fn auxiliary_depth() -> &'static Self {
        unsafe { kCIImageAuxiliaryDepth }
    }

    #[doc(alias = "kCIImageAuxiliaryDisparity")]
    #[inline]
    pub fn auxiliary_disparity() -> &'static Self {
        unsafe { kCIImageAuxiliaryDisparity }
    }

    #[doc(alias = "kCIImageAuxiliaryPortraitEffectsMatte")]
    #[inline]
    pub fn auxiliary_portrait_effects_matte() -> &'static Self {
        unsafe { kCIImageAuxiliaryPortraitEffectsMatte }
    }

    #[doc(alias = "kCIImageAuxiliarySemanticSegmentationSkinMatte")]
    #[inline]
    pub fn auxiliary_semantic_segmentation_skin_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationSkinMatte }
    }

    #[doc(alias = "kCIImageAuxiliarySemanticSegmentationHairMatte")]
    #[inline]
    pub fn auxiliary_semantic_segmentation_hair_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationHairMatte }
    }

    #[doc(alias = "kCIImageAuxiliarySemanticSegmentationTeethMatte")]
    #[inline]
    pub fn auxiliary_semantic_segmentation_teeth_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationTeethMatte }
    }

    #[doc(alias = "kCIImageAuxiliarySemanticSegmentationGlassesMatte")]
    #[inline]
    pub fn auxiliary_semantic_segmentation_glasses_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationGlassesMatte }
    }

    #[doc(alias = "kCIImageAuxiliarySemanticSegmentationSkyMatte")]
    #[inline]
    pub fn auxiliary_semantic_segmentation_sky_matte() -> &'static Self {
        unsafe { kCIImageAuxiliarySemanticSegmentationSkyMatte }
    }

    #[doc(alias = "kCIImageContentHeadroom")]
    #[inline]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn content_headeroom() -> &'static Self {
        unsafe { kCIImageContentHeadroom }
    }
}

#[link(name = "ci", kind = "static")]
#[api::weak]
extern "C" {
    static CI_IMAGE: &'static objc::Class<Image>;

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

    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    static kCIImageContentHeadroom: &'static ImageOption;
}

#[cfg(test)]
mod tests {
    use crate::{api, ci};

    #[test]
    fn basics() {
        {
            let black = ci::Image::black();
            let rc = black.as_type_ref().retain_count();
            assert_eq!(rc, 1);
        }
        {
            let black = ci::Image::black();
            let rc = black.as_type_ref().retain_count();
            assert_eq!(rc, 1);
        }
        {
            let empty = ci::Image::empty();
            let rc = empty.as_type_ref().retain_count();
            assert_eq!(rc, 1);
        }
    }

    #[test]
    fn versioning() {
        let black = ci::Image::black();
        if api::version!(macos = 15.0) {
            let headroom = unsafe { black.content_headroom() };
            assert_eq!(0.0f32, headroom);

            let key = unsafe { ci::ImageOption::content_headeroom() };
            assert!(key.is_some());
        } else {
            let key = unsafe { ci::ImageOption::content_headeroom() };
            assert!(key.is_none());
        }
    }
}
