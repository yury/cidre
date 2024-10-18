use crate::{arc, cf, cg, define_cf_type};

define_cf_type!(OptKey(cf::String));

impl OptKey {
    /// The desired compression quality to use when writing to an image
    /// destination. If present, the value of this key is a 'cf::Number'
    /// in the range 0.0 to 1.0. A value of 1.0 implies lossless
    /// compression is desired if destination format supports it.
    /// A value of 0.0 implies that that maximum compression is
    /// desired.
    #[doc(alias = "kCGImageDestinationLossyCompressionQuality")]
    #[inline]
    pub fn lossy_compression_quality() -> &'static Self {
        unsafe { kCGImageDestinationLossyCompressionQuality }
    }

    /// The desired background color to composite against when writing
    /// an image with alpha to a destination format that does not support
    /// alpha. If present, the value of this key is a 'cg::Color' without
    /// any alpha component of its own.  If not present a white color
    /// will be used if needed.
    #[doc(alias = "kCGImageDestinationBackgroundColor")]
    #[inline]
    pub fn bg_color() -> &'static Self {
        unsafe { kCGImageDestinationBackgroundColor }
    }

    /// Rescale the image to the maximum width and height in pixels.
    /// If present, this value of this key must be a 'cf::Number'.
    #[doc(alias = "kCGImageDestinationImageMaxPixelSize")]
    #[inline]
    pub fn image_max_pixel_size() -> &'static Self {
        unsafe { kCGImageDestinationImageMaxPixelSize }
    }

    /// Enable or disable thumbnail embedding for JPEG and HEIF.
    /// The value should be 'cf::Boolean::value_true()' or 'cf::Boolean::value_false()'.
    /// Defaults to 'cf::Boolean::value_false()'
    #[doc(alias = "kCGImageDestinationEmbedThumbnail")]
    #[inline]
    pub fn embed_thumbnail() -> &'static Self {
        unsafe { kCGImageDestinationEmbedThumbnail }
    }
    /// Create an image using a colorspace, that has is compatible with older devices
    /// The value should be 'cf::Boolean::value_true()' or 'cf::Boolean::value_false()'
    /// Defaults to 'cf::Boolean::value_false()' = don't do any color conversion
    #[doc(alias = "kCGImageDestinationOptimizeColorForSharing")]
    #[inline]
    pub fn optimize_color_for_sharing() -> &'static Self {
        unsafe { kCGImageDestinationOptimizeColorForSharing }
    }
}

define_cf_type!(AddOptKey(cf::String));

impl AddOptKey {
    /// For CGImageDestinationAddImageFromSource: when set to 'cf::Boolean::value_true()', a HEIF-embedded GainMap will be preserved.
    /// If the destination image is scaled (using kCGImageDestinationImageMaxPixelSize), the GainMap will be scaled accordingly.
    /// The value should be 'cf::Boolean::value_true()' or 'cf::Boolean::value_false()'
    /// Defaults to 'cf::Boolean::value_false()'
    #[doc(alias = "kCGImageDestinationPreserveGainMap")]
    #[inline]
    pub fn preserve_gain_map() -> &'static Self {
        unsafe { kCGImageDestinationPreserveGainMap }
    }
}
define_cf_type!(CopyOptKey(cf::String));

impl CopyOptKey {
    #[doc(alias = "kCGImageDestinationMetadata")]
    #[inline]
    pub fn metadata() -> &'static Self {
        unsafe { kCGImageDestinationMetadata }
    }

    #[doc(alias = "kCGImageDestinationMergeMetadata")]
    #[inline]
    pub fn merge_metadata() -> &'static Self {
        unsafe { kCGImageDestinationMergeMetadata }
    }

    #[doc(alias = "kCGImageMetadataShouldExcludeXMP")]
    #[inline]
    pub fn exclude_xmp() -> &'static Self {
        unsafe { kCGImageMetadataShouldExcludeXMP }
    }

    #[doc(alias = "kCGImageMetadataShouldExcludeGPS")]
    #[inline]
    pub fn exclude_gps() -> &'static Self {
        unsafe { kCGImageMetadataShouldExcludeGPS }
    }

    #[doc(alias = "kCGImageDestinationDateTime")]
    #[inline]
    pub fn date_time() -> &'static Self {
        unsafe { kCGImageDestinationDateTime }
    }

    #[doc(alias = "kCGImageDestinationOrientation")]
    #[inline]
    pub fn orientation() -> &'static Self {
        unsafe { kCGImageDestinationOrientation }
    }
}

define_cf_type!(Dst(cf::Type));

impl Dst {
    #[doc(alias = "CGImageDestinationGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGImageDestinationGetTypeID() }
    }

    #[doc(alias = "CGImageDestinationCopyTypeIdentifiers")]
    #[inline]
    pub fn supported_type_ids() -> arc::R<cf::ArrayOf<cf::String>> {
        unsafe { CGImageDestinationCopyTypeIdentifiers() }
    }

    #[doc(alias = "CGImageDestinationCreateWithData")]
    #[inline]
    pub fn with_data(
        data: &mut cf::DataMut,
        ty: &cf::String,
        count: usize,
    ) -> Option<arc::R<Self>> {
        unsafe { CGImageDestinationCreateWithData(data, ty, count, None) }
    }

    #[doc(alias = "CGImageDestinationCreateWithURL")]
    #[inline]
    pub fn with_url(url: &cf::Url, ty: &cf::String, count: usize) -> Option<arc::R<Self>> {
        unsafe { CGImageDestinationCreateWithURL(url, ty, count, None) }
    }

    #[doc(alias = "CGImageDestinationCopyImageSource")]
    #[inline]
    pub unsafe fn copy_image_source(
        &mut self,
        isrc: &cg::ImageSrc,
        options: Option<&cf::DictionaryOf<CopyOptKey, cf::Type>>,
        err: *mut Option<arc::R<cf::Error>>,
    ) -> bool {
        unsafe { CGImageDestinationCopyImageSource(self, isrc, options, err) }
    }

    #[doc(alias = "CGImageDestinationCopyImageSource")]
    #[inline]
    pub fn copy_image_src(
        &mut self,
        isrc: &cg::ImageSrc,
        options: Option<&cf::DictionaryOf<CopyOptKey, cf::Type>>,
    ) -> Result<(), arc::R<cf::Error>> {
        cf::if_false(|err| unsafe { self.copy_image_source(isrc, options, err) })
    }

    #[doc(alias = "CGImageDestinationSetProperties")]
    #[inline]
    pub fn set_props(&mut self, properies: Option<&cf::Dictionary>) {
        unsafe { CGImageDestinationSetProperties(self, properies) }
    }

    #[doc(alias = "CGImageDestinationAddImage")]
    #[inline]
    pub fn add_image(
        &mut self,
        image: &cg::Image,
        options: Option<&cf::DictionaryOf<AddOptKey, cf::Type>>,
    ) {
        unsafe { CGImageDestinationAddImage(self, image, options) }
    }

    #[doc(alias = "CGImageDestinationAddImageFromSource")]
    pub fn add_image_from_src(
        &mut self,
        isrc: &cg::ImageSrc,
        index: usize,
        properties: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
    ) {
        unsafe { CGImageDestinationAddImageFromSource(self, isrc, index, properties) }
    }

    #[doc(alias = "CGImageDestinationFinalize")]
    #[inline]
    pub fn finalize(&mut self) -> bool {
        unsafe { CGImageDestinationFinalize(self) }
    }
}

impl arc::R<Dst> {
    #[doc(alias = "CGImageDestinationFinalize")]
    #[inline]
    pub fn finalize(mut self) -> bool {
        self.as_mut().finalize()
    }
}

extern "C" {
    static kCGImageDestinationLossyCompressionQuality: &'static OptKey;
    static kCGImageDestinationBackgroundColor: &'static OptKey;
    static kCGImageDestinationImageMaxPixelSize: &'static OptKey;
    static kCGImageDestinationEmbedThumbnail: &'static OptKey;
    static kCGImageDestinationOptimizeColorForSharing: &'static OptKey;
    static kCGImageDestinationPreserveGainMap: &'static AddOptKey;
    static kCGImageDestinationMetadata: &'static CopyOptKey;
    static kCGImageDestinationMergeMetadata: &'static CopyOptKey;
    static kCGImageMetadataShouldExcludeXMP: &'static CopyOptKey;
    static kCGImageMetadataShouldExcludeGPS: &'static CopyOptKey;
    static kCGImageDestinationDateTime: &'static CopyOptKey;
    static kCGImageDestinationOrientation: &'static CopyOptKey;

    fn CGImageDestinationGetTypeID() -> cf::TypeId;
    fn CGImageDestinationCopyTypeIdentifiers() -> arc::R<cf::ArrayOf<cf::String>>;

    fn CGImageDestinationCreateWithData(
        data: &mut cf::DataMut,
        ty: &cf::String,
        count: usize,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<Dst>>;

    fn CGImageDestinationCreateWithURL(
        url: &cf::Url,
        ty: &cf::String,
        count: usize,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<Dst>>;

    fn CGImageDestinationSetProperties(idst: &mut Dst, properties: Option<&cf::Dictionary>);

    fn CGImageDestinationCopyImageSource(
        idst: &mut Dst,
        isrc: &cg::ImageSrc,
        options: Option<&cf::DictionaryOf<CopyOptKey, cf::Type>>,
        err: *mut Option<arc::R<cf::Error>>,
    ) -> bool;

    fn CGImageDestinationAddImage(
        idst: &mut Dst,
        image: &cg::Image,
        options: Option<&cf::DictionaryOf<AddOptKey, cf::Type>>,
    );

    fn CGImageDestinationAddImageFromSource(
        idst: &mut Dst,
        isrc: &cg::ImageSrc,
        index: usize,
        properties: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
    );

    fn CGImageDestinationFinalize(dst: &mut Dst) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::{cf, cg};

    #[test]
    fn basics() {
        let type_ids = cg::ImageDst::supported_type_ids();
        type_ids.show();
        assert!(type_ids.len() > 10);

        let mut data = cf::DataMut::with_capacity(10);
        let ty = cf::String::from_str("public.utf8-plain-text");
        let dst = cg::ImageDst::with_data(&mut data, &ty, 1);
        assert!(dst.is_none());
    }
}
