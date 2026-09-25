use crate::{arc, cf, cg, define_cf_type};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum Status {
    UnexpectedEof = -5,
    InvalidData = -4,
    UnknownType = -3,
    ReadingHeader = -2,
    Incomplete = -1,
    Complete = 0,
}

define_cf_type!(
    #[doc(alias = "CGImageSourceRef")]
    Src(cf::Type)
);

impl Src {
    pub fn type_id() -> cf::TypeId {
        unsafe { CGImageSourceGetTypeID() }
    }

    /// Return an array of supported type identifiers.
    #[doc(alias = "CGImageSourceCopyTypeIdentifiers")]
    #[inline]
    pub fn supported_type_ids() -> arc::R<cf::ArrayOf<cf::String>> {
        unsafe { CGImageSourceCopyTypeIdentifiers() }
    }

    #[doc(alias = "CGImageSourceCreateWithData")]
    #[inline]
    pub fn with_data(data: &cf::Data, options: Option<&cf::Dictionary>) -> Option<arc::R<Src>> {
        unsafe { CGImageSourceCreateWithData(data, options) }
    }

    #[doc(alias = "CGImageSourceCreateWithURL")]
    #[inline]
    pub fn with_url(url: &cf::Url, options: Option<&cf::Dictionary>) -> Option<arc::R<Src>> {
        unsafe { CGImageSourceCreateWithURL(url, options) }
    }

    #[inline]
    pub fn get_type(&self) -> Option<&cf::String> {
        unsafe { CGImageSourceGetType(self) }
    }

    /// Return the number of images (not including thumbnails) in the image source
    #[doc(alias = "CGImageSourceGetCount")]
    #[inline]
    pub fn count(&self) -> usize {
        unsafe { CGImageSourceGetCount(self) }
    }

    #[doc(alias = "CGImageSourceGetStatus")]
    #[inline]
    pub fn status(&self) -> Status {
        unsafe { CGImageSourceGetStatus(self) }
    }

    #[doc(alias = "CGImageSourceCopyProperties")]
    #[inline]
    pub fn props(
        &self,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Plist>>> {
        unsafe { CGImageSourceCopyProperties(self, options) }
    }

    #[doc(alias = "CGImageSourceCopyPropertiesAtIndex")]
    #[inline]
    pub fn props_at(
        &self,
        index: usize,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Plist>>> {
        unsafe { CGImageSourceCopyPropertiesAtIndex(self, index, options) }
    }

    #[doc(alias = "CGImageSourceCreateImageAtIndex")]
    #[inline]
    pub fn image_at(
        &self,
        index: usize,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<cg::Image>> {
        unsafe { CGImageSourceCreateImageAtIndex(self, index, options) }
    }

    #[doc(alias = "CGImageSourceCreateThumbnailAtIndex")]
    #[inline]
    pub fn thumbnail_at(
        &self,
        index: usize,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<cg::Image>> {
        unsafe { CGImageSourceCreateThumbnailAtIndex(self, index, options) }
    }
}

unsafe extern "C-unwind" {
    fn CGImageSourceGetTypeID() -> cf::TypeId;
    fn CGImageSourceCopyTypeIdentifiers() -> arc::R<cf::ArrayOf<cf::String>>;

    fn CGImageSourceCreateWithData(
        data: &cf::Data,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<Src>>;

    fn CGImageSourceCreateWithURL(
        data: &cf::Url,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<Src>>;

    fn CGImageSourceGetType(isrc: &Src) -> Option<&cf::String>;

    fn CGImageSourceGetCount(isrc: &Src) -> usize;

    fn CGImageSourceGetStatus(isrc: &Src) -> Status;

    fn CGImageSourceCopyProperties(
        isrc: &Src,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Plist>>>;

    fn CGImageSourceCopyPropertiesAtIndex(
        isrc: &Src,
        index: usize,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Plist>>>;

    fn CGImageSourceCreateImageAtIndex(
        isrc: &Src,
        index: usize,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<cg::Image>>;

    fn CGImageSourceCreateThumbnailAtIndex(
        isrc: &Src,
        index: usize,
        options: Option<&cf::Dictionary>,
    ) -> Option<arc::R<cg::Image>>;
}

/// Keys of the options of [`Src::image_at`] and [`Src::thumbnail_at`].
pub mod opt_keys {
    use crate::cf;

    /// A cf::Boolean: decode and cache the image when it is created, not when drawn.
    #[doc(alias = "kCGImageSourceShouldCacheImmediately")]
    #[inline]
    pub fn should_cache_immediately() -> &'static cf::String {
        unsafe { kCGImageSourceShouldCacheImmediately }
    }

    /// A cf::Boolean: make the thumbnail from the full image if the file has none, or none
    /// as big as asked.
    #[doc(alias = "kCGImageSourceCreateThumbnailFromImageIfAbsent")]
    #[inline]
    pub fn create_thumbnail_from_image_if_absent() -> &'static cf::String {
        unsafe { kCGImageSourceCreateThumbnailFromImageIfAbsent }
    }

    /// A cf::Boolean: make the thumbnail from the full image always, whatever the file has.
    #[doc(alias = "kCGImageSourceCreateThumbnailFromImageAlways")]
    #[inline]
    pub fn create_thumbnail_from_image_always() -> &'static cf::String {
        unsafe { kCGImageSourceCreateThumbnailFromImageAlways }
    }

    /// A cf::Number: the thumbnail's longer side at most this many pixels.
    #[doc(alias = "kCGImageSourceThumbnailMaxPixelSize")]
    #[inline]
    pub fn thumbnail_max_pixel_size() -> &'static cf::String {
        unsafe { kCGImageSourceThumbnailMaxPixelSize }
    }

    /// A cf::Boolean: the thumbnail turned and mirrored as the image's orientation (and pixel
    /// aspect) says, upright.
    #[doc(alias = "kCGImageSourceCreateThumbnailWithTransform")]
    #[inline]
    pub fn create_thumbnail_with_transform() -> &'static cf::String {
        unsafe { kCGImageSourceCreateThumbnailWithTransform }
    }

    unsafe extern "C" {
        static kCGImageSourceShouldCacheImmediately: &'static cf::String;
        static kCGImageSourceCreateThumbnailFromImageIfAbsent: &'static cf::String;
        static kCGImageSourceCreateThumbnailFromImageAlways: &'static cf::String;
        static kCGImageSourceThumbnailMaxPixelSize: &'static cf::String;
        static kCGImageSourceCreateThumbnailWithTransform: &'static cf::String;
    }
}

#[cfg(test)]
mod tests {
    use crate::cg;

    #[test]
    fn basics() {
        assert!(cg::ImageSrc::supported_type_ids().len() > 10);
    }
}
