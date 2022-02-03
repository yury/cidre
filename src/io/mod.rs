use crate::{
    cf::{self, Retained, Type},
    define_cf_type,
};

#[repr(i32)]
pub enum SurfaceComponentName {
    Unkown = 0,
    Alpha = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    Luma = 5,
    ChromaRed = 6,
    ChromeBlue = 7,
}

#[repr(i32)]
pub enum SurfaceComponentType {
    Unknown = 0,
    UnsignedInteger = 1,
    SignedInteger = 2,
    Float = 3,
    SignedNormalized = 4,
}

#[repr(i32)]
pub enum SurfaceComponentRange {
    Unknown = 0,
    Full = 1,
    Video = 2,
    Wide = 3,
}

#[repr(i32)]
pub enum SurfaceSubsampling {
    Unknown = 0,
    None = 1, // Includes "4:4:4"
    _422 = 2, // Chroma downsampled by 2x1
    _420 = 3, // Chroma downsampled by 2x2
    _411 = 4, // Chroma downsampled by 4x1
}

define_cf_type!(Surface(Type));

impl Surface {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { IOSurfaceGetTypeID() }
    }
    /// ```
    /// use cidre::cf;
    /// use cidre::io;
    ///
    /// let properties = cf::Dictionary::from_pairs(&[], &[]).unwrap();
    /// let surf = io::Surface::create(&properties);
    /// ```
    pub fn create<'a>(properties: &cf::Dictionary) -> Option<Retained<'a, Surface>> {
        unsafe { IOSurfaceCreate(properties) }
    }
}

#[link(name = "IOSurface", kind = "framework")]
extern "C" {
    fn IOSurfaceGetTypeID() -> cf::TypeId;
    fn IOSurfaceCreate<'a>(properties: &cf::Dictionary) -> Option<Retained<'a, Surface>>;
}
