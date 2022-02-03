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

/// ```
/// use cidre::io;
///
/// // assert_eq!(8896801736, io::surface_get_type_id());
/// ```
#[inline]
pub fn surface_get_type_id() -> cf::TypeID {
    unsafe { IOSurfaceGetTypeID() }
}

define_cf_type!(Surface(Type));

impl Surface {
    /// ```
    /// use cidre::cf;
    /// use cidre::io;
    ///
    /// let properties = cf::Dictionary::create(None, std::ptr::null(), std::ptr::null(), 0, None, None).unwrap();
    /// let surf = io::Surface::create(&properties);
    /// ```
    pub fn create(properties: &cf::Dictionary) -> Option<Retained<Surface>> {
        unsafe { IOSurfaceCreate(properties) }
    }
}

#[link(name = "IOSurface", kind = "framework")]
extern "C" {
    fn IOSurfaceGetTypeID() -> cf::TypeID;
    fn IOSurfaceCreate(properties: &cf::Dictionary) -> Option<Retained<Surface>>;
}
