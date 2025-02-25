use crate::{api, arc, cf, os};

/// Errors returned from the cm::Tag routines
pub mod err {
    use crate::os::Error;

    /// When caller passes incorrect input or output parameters.
    #[doc(alias = "kCMTagError_ParamErr")]
    pub const PARAM_ERR: Error = Error::new_unchecked(-15730);

    /// Returned if a necessary allocation failed.
    #[doc(alias = "kCMTagError_AllocationFailed")]
    pub const ALLOC_FAILED: Error = Error::new_unchecked(-15731);
}

/// An enum indicating the available cm::TagCategory identifiers that can be used to distinguish the tag from other kinds.
///
/// Different kinds of cm::Tags may be defined or registered. Each will be associated with a category.
/// cm::Tags with the same CMTagCategory will have the same kinds of values. The category serves as a namespace.
#[doc(alias = "CMTagCategory")]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
#[repr(u32)]
pub enum TagCategory {
    /// Indicates there is no category specified for this value or it could not be determined.
    /// cm::Tags should typically not have this category except as sentinel values.
    #[doc(alias = "kCMTagCategory_Undefined")]
    #[default]
    Undefined = 0,

    /// Value is an os::Type holding a cm::MediaType
    #[doc(alias = "kCMTagCategory_MediaType")]
    MediaType = u32::from_be_bytes(*b"mdia"),

    /// Value is an os::Type holding a media subtype such as a video codec type.
    #[doc(alias = "kCMTagCategory_MediaSubType")]
    MediaSubType = u32::from_be_bytes(*b"msub"),

    /// Value is a cm::PersistentTrackId for a corresponding asset.
    #[doc(alias = "kCMTagCategory_TrackID")]
    TrackId = u32::from_be_bytes(*b"trak"),

    /// Value is the CMVideoTarget/CMVideoReceiver channel identifier.
    #[doc(alias = "kCMTagCategory_ChannelID")]
    ChannelId = u32::from_be_bytes(*b"vchn"),

    /// Value is a i64 specifying the video layer identifier.
    #[doc(alias = "kCMTagCategory_VideoLayerID")]
    VideoLayerId = u32::from_be_bytes(*b"vlay"),

    /// Indicates the pixel format of the buffer or channel, if pixel-based.
    /// The type is an os::Type corresponding to a pixel format (i.e., a kCVPixelFormatType_* type).
    #[doc(alias = "kCMTagCategory_PixelFormat")]
    PixelFormat = u32::from_be_bytes(*b"pixf"),

    /// Indicates this channel is packed in some way (e.g., frame packed, texture atlas).
    /// The value is an OSType carrying kCMPackingType_* constants.
    #[doc(alias = "kCMTagCategory_PackingType")]
    PackingType = u32::from_be_bytes(*b"pack"),

    /// Indicates textures are related to a kind of texture projection (e.g., equirectangular). The value is an OSType carrying one of the kCMProjectionType_* constants.
    #[doc(alias = "kCMTagCategory_ProjectionType")]
    ProjectionType = u32::from_be_bytes(*b"proj"),

    /// Indicates this channel is related to carrying stereographic views. The value is a Flags value carrying one of the kCMTagStereoViewComponent_* constants.
    #[doc(alias = "kCMTagCategory_StereoView")]
    StereoView = u32::from_be_bytes(*b"eyes"),

    /// Indicates this channel has non default stereo view interpretation (e.g., stereo eye view order is reversed.)
    /// Tags with this category will typically be associated with tags of category kCMTagCategory_StereoView.
    /// This tag alone however does not indicate which stereo eyes are present. The value is a Flags value carrying one
    /// of the kCMTagStereoInterpretationOption_* constants.
    #[doc(alias = "kCMTagCategory_StereoViewInterpretation")]
    StereoViewInterpretation = u32::from_be_bytes(*b"eyip"),
}

impl std::fmt::Debug for TagCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        crate::four_cc_fmt_debug(*self as _, "TagCategory", f)
    }
}

#[doc(alias = "CMTagDataType")]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TagDataType {
    /// Value is a sentinel data type indicating it is not valid. The value should not be treated as a value.
    #[doc(alias = "kCMTagDataType_Invalid")]
    #[default]
    Invalid = 0,

    /// Value is a i64.
    #[doc(alias = "kCMTagDataType_SInt64")]
    I64 = 2,

    /// Value is a f64.
    #[doc(alias = "kCMTagDataType_Float64")]
    F64 = 3,

    /// Value is an os::Type in the lower 32 bits of a 64-bit integer (e.g., OSType).
    #[doc(alias = "kCMTagDataType_OSType")]
    OsType = 5,

    /// Value is a u64 holding option bits or flags.
    #[doc(alias = "kCMTagDataType_Flags")]
    Flags = 7,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TagValue(pub u64);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc(alias = "CMTag")]
#[repr(C)]
pub struct Tag {
    pub category: TagCategory,
    dtype: TagDataType,
    val: TagValue,
}
impl Tag {
    pub fn new(category: TagCategory, dtype: TagDataType, val: TagValue) -> Self {
        Self {
            category,
            dtype,
            val,
        }
    }

    pub fn with_i64(category: TagCategory, val: i64) -> Self {
        Self::new(
            category,
            TagDataType::I64,
            TagValue(unsafe { std::mem::transmute(val) }),
        )
    }

    pub fn with_f64(category: TagCategory, val: f64) -> Self {
        Self::new(
            category,
            TagDataType::F64,
            TagValue(unsafe { std::mem::transmute(val) }),
        )
    }

    pub fn with_os_type(category: TagCategory, val: os::Type) -> Self {
        Self::new(category, TagDataType::OsType, TagValue(val as u64))
    }

    pub fn with_flags(category: TagCategory, val: u64) -> Self {
        Self::new(category, TagDataType::Flags, TagValue(val))
    }

    pub fn with_cf_dictionary(dict: &cf::Dictionary) -> Self {
        unsafe { CMTagMakeFromDictionary(dict) }
    }

    pub fn dtype(&self) -> TagDataType {
        self.dtype
    }

    pub fn val(&self) -> TagValue {
        self.val
    }

    pub fn val_i64(&self) -> Option<i64> {
        if self.dtype == TagDataType::I64 {
            unsafe { Some(std::mem::transmute(self.val.0)) }
        } else {
            None
        }
    }

    pub fn val_f64(&self) -> Option<f64> {
        if self.dtype == TagDataType::F64 {
            unsafe { Some(std::mem::transmute(self.val.0)) }
        } else {
            None
        }
    }

    pub fn val_os_type(&self) -> Option<os::Type> {
        if self.dtype == TagDataType::OsType {
            unsafe { Some(std::mem::transmute(self.val.0 as u32)) }
        } else {
            None
        }
    }

    pub fn val_flags(&self) -> Option<u64> {
        if self.dtype == TagDataType::Flags {
            Some(self.val.0)
        } else {
            None
        }
    }

    pub fn set_val_i64(&mut self, val: i64) {
        self.dtype = TagDataType::I64;
        self.val = TagValue(unsafe { std::mem::transmute(val) });
    }

    pub fn set_val_f64(&mut self, val: f64) {
        self.dtype = TagDataType::F64;
        self.val = TagValue(unsafe { std::mem::transmute(val) });
    }

    pub fn set_val_os_type(&mut self, val: os::Type) {
        self.dtype = TagDataType::OsType;
        self.val = TagValue(val as u64);
    }

    pub fn set_val_flags(&mut self, val: u64) {
        self.dtype = TagDataType::Flags;
        self.val = TagValue(val);
    }

    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.dtype as u32 != TagDataType::Invalid as u32
    }

    #[inline]
    pub const fn is_invalid(&self) -> bool {
        self.dtype as u32 == TagDataType::Invalid as u32
    }

    #[doc(alias = "CMTagCopyDescription")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn desc_in(&self, allocator: Option<&cf::Allocator>) -> Option<arc::R<cf::String>> {
        unsafe { CMTagCopyDescription(allocator, *self) }
    }

    #[doc(alias = "CMTagCopyDescription")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn desc(&self) -> arc::R<cf::String> {
        unsafe { std::mem::transmute(self.desc_in(None)) }
    }

    #[doc(alias = "CMTagCopyAsDictionary")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn to_cf_dictionary_in(
        &self,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>> {
        unsafe { CMTagCopyAsDictionary(*self, allocator) }
    }

    #[doc(alias = "CMTagCopyAsDictionary")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn to_cf_dictionary(&self) -> arc::R<cf::DictionaryOf<cf::String, cf::Type>> {
        unsafe { std::mem::transmute(self.to_cf_dictionary_in(None)) }
    }
}

#[cfg(any(
    feature = "macos_14_0",
    feature = "ios_17_0",
    feature = "tvos_17_0",
    feature = "watchos_10_0",
    feature = "visionos_1_0"
))]
impl Into<arc::R<cf::DictionaryOf<cf::String, cf::Type>>> for Tag {
    #[inline]
    fn into(self) -> arc::R<cf::DictionaryOf<cf::String, cf::Type>> {
        self.to_cf_dictionary()
    }
}

#[cfg(any(
    feature = "macos_14_0",
    feature = "ios_17_0",
    feature = "tvos_17_0",
    feature = "watchos_10_0",
    feature = "visionos_1_0"
))]

impl From<&cf::Dictionary> for Tag {
    fn from(value: &cf::Dictionary) -> Self {
        unsafe { CMTagMakeFromDictionary(&value) }
    }
}

#[cfg(any(
    feature = "macos_14_0",
    feature = "ios_17_0",
    feature = "tvos_17_0",
    feature = "watchos_10_0",
    feature = "visionos_1_0"
))]
impl From<&cf::DictionaryOf<cf::String, cf::Type>> for Tag {
    fn from(value: &cf::DictionaryOf<cf::String, cf::Type>) -> Self {
        unsafe { CMTagMakeFromDictionary(&value) }
    }
}

#[link(name = "CoreMedia", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCopyDescription(
        allocator: Option<&cf::Allocator>,
        tag: Tag,
    ) -> Option<arc::R<cf::String>>;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCopyAsDictionary(
        tag: Tag,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>>;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagMakeFromDictionary(dict: &cf::Dictionary) -> Tag;
}

#[cfg(test)]
mod tests {
    use crate::{arc, cf, cm};

    #[test]
    fn basics() {
        let mut tag = cm::Tag::with_i64(cm::TagCategory::TrackId, 0);
        assert_eq!(tag.dtype(), cm::TagDataType::I64);
        assert_eq!(tag.val_i64(), Some(0));
        assert_eq!(tag.val_f64(), None);
        assert_eq!(tag.val_os_type(), None);

        let desc = tag.desc();
        assert_eq!(desc.as_ref(), "{category:'trak' value:0 <int64>}");

        let dict: arc::R<cf::DictionaryOf<cf::String, cf::Type>> = tag.into();
        dict.show();

        let tag_a = cm::Tag::with_cf_dictionary(&dict);
        assert_eq!(tag, tag_a);

        let tag_b = cm::Tag::from(cf::Dictionary::new().as_ref());
        assert!(tag_b.is_invalid());

        let tag_b = cm::Tag::from(dict.as_ref());
        assert!(tag_b.is_valid());
        assert_eq!(tag, tag_b);

        tag.set_val_flags(2);
        assert_ne!(tag, tag_b);
    }
}
