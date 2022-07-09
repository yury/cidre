use crate::{
    cf::{self, Retained, Type},
    define_cf_type, define_options,
    sys::_types::MachPort,
};

pub type Id = u32;

#[repr(i32)]
pub enum ComponentName {
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
pub enum ComponentType {
    Unknown = 0,
    UnsignedInteger = 1,
    SignedInteger = 2,
    Float = 3,
    SignedNormalized = 4,
}

#[repr(i32)]
pub enum ComponentRange {
    Unknown = 0,
    Full = 1,
    Video = 2,
    Wide = 3,
}

#[repr(i32)]
pub enum Subsampling {
    Unknown = 0,
    None = 1, // Includes "4:4:4"
    _422 = 2, // Chroma downsampled by 2x1
    _420 = 3, // Chroma downsampled by 2x2
    _411 = 4, // Chroma downsampled by 4x1
}

// #[repr(transparent)]
// pub struct LockOptions(pub u32);
define_options!(LockOptions(u32));

impl LockOptions {
    /// If you are not going to modify the data while you hold the lock,
    /// you should set this flag to avoid invalidating
    /// any existing caches of the buffer contents.
    ///
    /// This flag should be passed both to the lock and unlock functions.
    /// Non-symmetrical usage of this flag will result in undefined behavior.
    pub const READ_ONLY: Self = Self(1);

    /// If you want to detect/avoid a potentially expensive paging operation
    /// (such as readback from a GPU to system memory)
    /// when you lock the buffer, you may include this flag.
    /// If locking the buffer requires a readback, the lock will
    /// fail with an error return of kIOReturnCannotLock.
    pub const AVOID_SYNC: Self = Self(2);
}

define_cf_type!(Surface(Type));

impl Surface {
    /// ```
    /// use cidre::io;
    ///
    /// let type_id = io::Surface::type_id();
    ///
    /// assert_ne!(type_id, 0);
    /// ```
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { IOSurfaceGetTypeID() }
    }
    /// ```
    /// use cidre::{cf, io};
    ///
    ///
    /// let width = cf::Number::from_i32(100);
    /// let height = cf::Number::from_i32(200);
    ///
    /// let properties = cf::Dictionary::with_keys_values(
    ///   &[
    ///     io::surface::keys::width(),
    ///     io::surface::keys::height()
    ///   ],
    ///   &[
    ///     &width,
    ///     &height
    ///   ]
    /// ).unwrap();
    ///
    /// let surf = io::Surface::create(&properties).unwrap();
    ///
    /// assert_eq!(100, surf.width());
    /// assert_eq!(200, surf.height());
    /// assert_eq!(0, surf.plane_count());
    /// assert_ne!(0, surf.id());
    ///
    /// let props = surf.all_values().unwrap();
    /// props.show();
    /// assert!(props.len() >= 1);
    /// ```
    pub fn create(properties: &cf::Dictionary) -> Option<Retained<Surface>> {
        unsafe { IOSurfaceCreate(properties) }
    }

    #[inline]
    pub fn id(&self) -> Id {
        unsafe { IOSurfaceGetID(&self) }
    }

    #[inline]
    pub fn width(&self) -> usize {
        unsafe { IOSurfaceGetWidth(self) }
    }

    #[inline]
    pub fn height(&self) -> usize {
        unsafe { IOSurfaceGetHeight(self) }
    }

    #[inline]
    pub fn plane_count(&self) -> usize {
        unsafe { IOSurfaceGetPlaneCount(self) }
    }

    #[inline]
    pub fn plane_width(&self, plane_index: usize) -> usize {
        unsafe { IOSurfaceGetWidthOfPlane(self, plane_index) }
    }

    #[inline]
    pub fn plane_height(&self, plane_index: usize) -> usize {
        unsafe { IOSurfaceGetHeightOfPlane(self, plane_index) }
    }

    /// ```
    /// use cidre::io;
    ///
    /// let surf = io::Surface::lookup(0);
    ///
    /// assert!(surf.is_none());
    /// ```
    pub fn lookup(csid: Id) -> Option<Retained<Surface>> {
        unsafe { IOSurfaceLookup(csid) }
    }

    #[inline]
    pub fn all_values(&self) -> Option<Retained<cf::Dictionary>> {
        unsafe { IOSurfaceCopyAllValues(self) }
    }

    /// ```
    /// use cidre::{cf, io, mach};
    ///
    /// let width = cf::Number::from_i32(100);
    /// let height = cf::Number::from_i32(200);
    ///
    /// let properties = cf::Dictionary::with_keys_values(
    ///   &[
    ///     io::surface::keys::width(),
    ///     io::surface::keys::height()
    ///   ],
    ///   &[
    ///     &width,
    ///     &height
    ///   ]
    /// ).unwrap();
    ///
    /// let surf = io::Surface::create(&properties).unwrap();
    /// let port = surf.create_mach_port();
    /// let surf2 = io::Surface::from_mach_port(port).unwrap();
    /// port.task_self_deallocate();
    /// assert!(surf.equal(&surf2));
    /// assert_eq!(false, surf.is_in_use());
    /// assert_eq!(false, surf2.is_in_use());
    /// ```
    #[inline]
    pub fn create_mach_port(&self) -> MachPort {
        unsafe { IOSurfaceCreateMachPort(self) }
    }

    /// This call takes a mach_port_t created via io::Surface::create_mach_port() and recreates an io::Surface from it.
    ///
    /// This call does NOT destroy the port.
    #[inline]
    pub fn from_mach_port(port: MachPort) -> Option<Retained<Surface>> {
        unsafe { IOSurfaceLookupFromMachPort(port) }
    }

    /// Returns true of an io::Surface is in use by any process in the system, otherwise false.
    #[inline]
    pub fn is_in_use(&self) -> bool {
        unsafe { IOSurfaceIsInUse(self) }
    }

    #[inline]
    pub fn use_count(&self) -> i32 {
        unsafe { IOSurfaceGetUseCount(self) }
    }

    #[inline]
    pub fn increment_use_count(&mut self) {
        unsafe { IOSurfaceIncrementUseCount(self) }
    }

    #[inline]
    pub fn decrement_use_count(&mut self) {
        unsafe { IOSurfaceDecrementUseCount(self) }
    }

    #[inline]
    pub fn allows_pixel_size_casting(&self) -> bool {
        unsafe { IOSurfaceAllowsPixelSizeCasting(self) }
    }

    /// This will return the current seed value of the buffer and is a cheap call to make to see
    /// if the contents of the buffer have changed since the last lock/unlock.
    #[inline]
    pub fn seed(&self) -> u32 {
        unsafe { IOSurfaceGetSeed(self) }
    }
}

extern "C" {
    fn IOSurfaceGetTypeID() -> cf::TypeId;
    fn IOSurfaceCreate(properties: &cf::Dictionary) -> Option<Retained<Surface>>;
    fn IOSurfaceLookup(csid: Id) -> Option<Retained<Surface>>;
    fn IOSurfaceGetID(buffer: &Surface) -> Id;
    fn IOSurfaceGetWidth(buffer: &Surface) -> usize;
    fn IOSurfaceGetHeight(buffer: &Surface) -> usize;
    fn IOSurfaceGetPlaneCount(buffer: &Surface) -> usize;
    fn IOSurfaceGetWidthOfPlane(buffer: &Surface, plane_index: usize) -> usize;
    fn IOSurfaceGetHeightOfPlane(buffer: &Surface, plane_index: usize) -> usize;

    fn IOSurfaceCopyAllValues(buffer: &Surface) -> Option<Retained<cf::Dictionary>>;

    fn IOSurfaceCreateMachPort(buffer: &Surface) -> MachPort;
    fn IOSurfaceLookupFromMachPort(port: MachPort) -> Option<Retained<Surface>>;

    fn IOSurfaceIsInUse(buffer: &Surface) -> bool;
    fn IOSurfaceGetUseCount(buffer: &Surface) -> i32;
    fn IOSurfaceIncrementUseCount(buffer: &mut Surface);
    fn IOSurfaceDecrementUseCount(buffer: &mut Surface);

    fn IOSurfaceAllowsPixelSizeCasting(buffer: &Surface) -> bool;

    fn IOSurfaceGetSeed(buffer: &Surface) -> u32;

}

/// The following list of properties are used with the cf::Dictionary passed to io::Surface::create
pub mod keys {
    use crate::cf::String;

    /// cf::Number of the total allocation size of the buffer including all planes.    
    /// Defaults to BufferHeight * BytesPerRow if not specified. Must be specified for
    /// dimensionless buffers.
    #[inline]
    pub fn alloc_size() -> &'static String {
        unsafe { kIOSurfaceAllocSize }
    }

    /// cf::Number for the width of the io::Surface buffer in pixels. Required for planar io::Surfaces
    #[inline]
    pub fn width() -> &'static String {
        unsafe { kIOSurfaceWidth }
    }

    /// cf::Number for the height of the io::Surface buffer in pixels. Required for planar io::Surfaces
    #[inline]
    pub fn height() -> &'static String {
        unsafe { kIOSurfaceHeight }
    }
    #[inline]
    pub fn bytes_per_row() -> &'static String {
        unsafe { kIOSurfaceBytesPerRow }
    }

    /// cf::Number for the total number of bytes in an element.
    #[inline]
    pub fn bytes_per_element() -> &'static String {
        unsafe { kIOSurfaceBytesPerElement }
    }

    /// cf::Number for how many pixels wide each element is.
    #[inline]
    pub fn element_width() -> &'static String {
        unsafe { kIOSurfaceElementWidth }
    }

    /// cf::Number for how many pixels high each element is.
    #[inline]
    pub fn element_height() -> &'static String {
        unsafe { kIOSurfaceElementHeight }
    }
    #[inline]
    pub fn offset() -> &'static String {
        unsafe { kIOSurfaceOffset }
    }
    #[inline]
    pub fn plane_info() -> &'static String {
        unsafe { kIOSurfacePlaneInfo }
    }
    #[inline]
    pub fn plane_width() -> &'static String {
        unsafe { kIOSurfacePlaneWidth }
    }
    #[inline]
    pub fn plane_height() -> &'static String {
        unsafe { kIOSurfacePlaneHeight }
    }
    #[inline]
    pub fn plane_bytes_per_row() -> &'static String {
        unsafe { kIOSurfacePlaneBytesPerRow }
    }
    #[inline]
    pub fn plane_offset() -> &'static String {
        unsafe { kIOSurfacePlaneOffset }
    }
    #[inline]
    pub fn plane_size() -> &'static String {
        unsafe { kIOSurfacePlaneSize }
    }
    #[inline]
    pub fn plane_base() -> &'static String {
        unsafe { kIOSurfacePlaneBase }
    }
    #[inline]
    pub fn plane_bits_per_element() -> &'static String {
        unsafe { kIOSurfacePlaneBitsPerElement }
    }

    extern "C" {
        static kIOSurfaceAllocSize: &'static String;
        static kIOSurfaceWidth: &'static String;
        static kIOSurfaceHeight: &'static String;
        static kIOSurfaceBytesPerRow: &'static String;
        static kIOSurfaceBytesPerElement: &'static String;
        static kIOSurfaceElementWidth: &'static String;
        static kIOSurfaceElementHeight: &'static String;
        static kIOSurfaceOffset: &'static String;
        static kIOSurfacePlaneInfo: &'static String;
        static kIOSurfacePlaneWidth: &'static String;
        static kIOSurfacePlaneHeight: &'static String;
        static kIOSurfacePlaneBytesPerRow: &'static String;
        static kIOSurfacePlaneOffset: &'static String;
        static kIOSurfacePlaneSize: &'static String;
        static kIOSurfacePlaneBase: &'static String;
        static kIOSurfacePlaneBitsPerElement: &'static String;
        // static kIOSurfacePlaneBytesPerElement: &'static String;
        // static kIOSurfacePlaneElementWidth: &'static String;
        // static kIOSurfacePlaneElementHeight: &'static String;
        // static kIOSurfaceCacheMode: &'static String;
        // static kIOSurfacePixelFormat: &'static String;
        // static kIOSurfacePixelSizeCastingAllowed: &'static String;
        // static kIOSurfacePlaneComponentBitDepths: &'static String;
        // static kIOSurfacePlaneComponentBitOffsets: &'static String;
    }
}
