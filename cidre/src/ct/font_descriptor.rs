use crate::{arc, cf, cg, define_cf_type};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FontOrientation {
    Default = 0,
    Horizontal = 1,
    Vertical = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FontFormat {
    /// The font is not a recognized format
    Unrecognized = 0,

    /// The font is an OpenType format containing PostScript data
    OpenTypePostScript = 1,

    /// The font is an OpenType format containing TrueType data
    OpenTypeTrueType = 2,

    /// The font is a recognized TrueType format
    TrueType = 3,

    /// The font is a recognized PostScript format
    PostScript = 4,

    /// The font is a bitmap only format
    Bitmap = 5,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct FontPriority(pub u32);

impl FontPriority {
    pub const SYSTEM: Self = Self(10000);
    pub const NETWORK: Self = Self(20000);
    pub const COMPUTER: Self = Self(30000);
    pub const USER: Self = Self(40000);
    pub const DYNAMIC: Self = Self(50000);
    pub const PROCESS: Self = Self(60000);
}

// A font descriptor attribute's key.
define_cf_type!(DescAttr(cf::String));

impl DescAttr {
    /// The family name, a `cf::String`: a descriptor with it alone resolves to the family's
    /// regular face.
    #[doc(alias = "kCTFontFamilyNameAttribute")]
    #[inline]
    pub fn family_name() -> &'static Self {
        unsafe { kCTFontFamilyNameAttribute }
    }

    /// The PostScript name, a `cf::String`.
    #[doc(alias = "kCTFontNameAttribute")]
    #[inline]
    pub fn name() -> &'static Self {
        unsafe { kCTFontNameAttribute }
    }
}

define_cf_type!(Desc(cf::Type));

impl Desc {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CTFontDescriptorGetTypeID() }
    }

    #[inline]
    pub fn with_name_size(name: &cf::String, size: cg::Float) -> arc::R<Self> {
        unsafe { CTFontDescriptorCreateWithNameAndSize(name, size) }
    }

    #[inline]
    pub fn with_attributes(attributes: &cf::DictionaryOf<cf::String, cf::Type>) -> arc::R<Self> {
        unsafe { CTFontDescriptorCreateWithAttributes(attributes) }
    }
}

unsafe extern "C" {
    static kCTFontFamilyNameAttribute: &'static DescAttr;
    static kCTFontNameAttribute: &'static DescAttr;
}

unsafe extern "C-unwind" {
    fn CTFontDescriptorGetTypeID() -> cf::TypeId;
    fn CTFontDescriptorCreateWithNameAndSize(name: &cf::String, size: cg::Float) -> arc::R<Desc>;

    fn CTFontDescriptorCreateWithAttributes(
        attributes: &cf::DictionaryOf<cf::String, cf::Type>,
    ) -> arc::R<Desc>;
}

#[cfg(test)]
mod tests {
    use crate::{cf, ct};

    #[test]
    fn basics() {
        let name = cf::str!(c"hello");
        let desc = ct::FontDesc::with_name_size(name, 10.0);
        desc.show();
    }
}
