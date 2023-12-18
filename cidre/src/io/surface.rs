use crate::{arc, cf, define_cf_type, define_options, sys::_types::MachPort};

#[doc(alias = "SurfaceID")]
pub type SurfId = u32;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum ComponentType {
    Unknown = 0,
    U32 = 1,
    S32 = 2,
    F32 = 3,
    SignedNormalized = 4,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum ComponentRange {
    Unknown = 0,
    Full = 1,
    Video = 2,
    Wide = 3,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum Subsampling {
    Unknown = 0,
    None = 1, // Includes "4:4:4"
    _422 = 2, // Chroma downsampled by 2x1
    _420 = 3, // Chroma downsampled by 2x2
    _411 = 4, // Chroma downsampled by 4x1
}

define_options!(pub LockOpts(u32));

impl LockOpts {
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

define_cf_type!(Surf(cf::Type));

impl Surf {
    /// ```
    /// use cidre::io;
    ///
    /// let type_id = io::Surf::type_id();
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
    /// let surf = io::Surf::create(&properties).unwrap();
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
    #[inline]
    pub fn create(properties: &cf::Dictionary) -> Option<arc::R<Surf>> {
        unsafe { IOSurfaceCreate(properties) }
    }

    #[doc(alias = "IOSurfaceGetID")]
    #[inline]
    pub fn id(&self) -> SurfId {
        unsafe { IOSurfaceGetID(self) }
    }

    #[doc(alias = "IOSurfaceGetWidth")]
    #[inline]
    pub fn width(&self) -> usize {
        unsafe { IOSurfaceGetWidth(self) }
    }

    #[doc(alias = "IOSurfaceGetHeight")]
    #[inline]
    pub fn height(&self) -> usize {
        unsafe { IOSurfaceGetHeight(self) }
    }

    #[doc(alias = "IOSurfaceGetPlaneCount")]
    #[inline]
    pub fn plane_count(&self) -> usize {
        unsafe { IOSurfaceGetPlaneCount(self) }
    }

    #[doc(alias = "IOSurfaceGetWidthOfPlane")]
    #[inline]
    pub fn plane_width(&self, plane_index: usize) -> usize {
        unsafe { IOSurfaceGetWidthOfPlane(self, plane_index) }
    }

    #[doc(alias = "IOSurfaceGetHeightOfPlane")]
    #[inline]
    pub fn plane_height(&self, plane_index: usize) -> usize {
        unsafe { IOSurfaceGetHeightOfPlane(self, plane_index) }
    }

    /// ```
    /// use cidre::io;
    ///
    /// let surf = io::Surf::lookup(0);
    ///
    /// assert!(surf.is_none());
    /// ```
    #[doc(alias = "IOSurfaceLookup")]
    #[inline]
    pub fn lookup(csid: SurfId) -> Option<arc::R<Surf>> {
        unsafe { IOSurfaceLookup(csid) }
    }

    #[doc(alias = "IOSurfaceCopyAllValues")]
    #[inline]
    pub fn all_values(&self) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>> {
        unsafe { IOSurfaceCopyAllValues(self) }
    }

    #[doc(alias = "IOSurfaceRemoveAllValues")]
    #[inline]
    pub fn remove_all_values(&mut self) {
        unsafe { IOSurfaceRemoveAllValues(self) }
    }

    #[doc(alias = "IOSurfaceCreateMachPort")]
    #[inline]
    pub fn create_mach_port(&self) -> MachPort {
        unsafe { IOSurfaceCreateMachPort(self) }
    }

    /// This call takes a mach_port_t created via io::Surface::create_mach_port() and recreates an io::Surface from it.
    ///
    /// This call does NOT destroy the port.
    #[doc(alias = "IOSurfaceLookupFromMachPort")]
    #[inline]
    pub fn from_mach_port(port: MachPort) -> Option<arc::R<Surf>> {
        unsafe { IOSurfaceLookupFromMachPort(port) }
    }

    /// Returns true of an io::Surface is in use by any process in the system, otherwise false.
    #[doc(alias = "IOSurfaceIsInUse")]
    #[inline]
    pub fn is_in_use(&self) -> bool {
        unsafe { IOSurfaceIsInUse(self) }
    }

    #[doc(alias = "IOSurfaceGetUseCount")]
    #[inline]
    pub fn use_count(&self) -> i32 {
        unsafe { IOSurfaceGetUseCount(self) }
    }

    #[doc(alias = "IOSurfaceIncrementUseCount")]
    #[inline]
    pub fn increment_use_count(&mut self) {
        unsafe { IOSurfaceIncrementUseCount(self) }
    }

    #[doc(alias = "IOSurfaceDecrementUseCount")]
    #[inline]
    pub fn decrement_use_count(&mut self) {
        unsafe { IOSurfaceDecrementUseCount(self) }
    }

    #[doc(alias = "IOSurfaceAllowsPixelSizeCasting")]
    #[inline]
    pub fn allows_pixel_size_casting(&self) -> bool {
        unsafe { IOSurfaceAllowsPixelSizeCasting(self) }
    }

    /// This will return the current seed value of the buffer and is a cheap call to make to see
    /// if the contents of the buffer have changed since the last lock/unlock.
    #[doc(alias = "IOSurfaceGetSeed")]
    #[inline]
    pub fn seed(&self) -> u32 {
        unsafe { IOSurfaceGetSeed(self) }
    }

    /// Returns the total allocation size of the buffer including all planes.
    #[doc(alias = "IOSurfaceGetAllocSize")]
    #[inline]
    pub fn alloc_size(&self) -> usize {
        unsafe { IOSurfaceGetAllocSize(self) }
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut u8) -> arc::R<Self> {
        std::mem::transmute(ptr)
    }
}

extern "C" {
    fn IOSurfaceGetTypeID() -> cf::TypeId;
    fn IOSurfaceCreate(properties: &cf::Dictionary) -> Option<arc::R<Surf>>;
    fn IOSurfaceLookup(csid: SurfId) -> Option<arc::R<Surf>>;
    fn IOSurfaceGetID(buffer: &Surf) -> SurfId;
    fn IOSurfaceGetWidth(buffer: &Surf) -> usize;
    fn IOSurfaceGetHeight(buffer: &Surf) -> usize;
    fn IOSurfaceGetPlaneCount(buffer: &Surf) -> usize;
    fn IOSurfaceGetWidthOfPlane(buffer: &Surf, plane_index: usize) -> usize;
    fn IOSurfaceGetHeightOfPlane(buffer: &Surf, plane_index: usize) -> usize;

    fn IOSurfaceCopyAllValues(
        buffer: &Surf,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>>;

    fn IOSurfaceCreateMachPort(buffer: &Surf) -> MachPort;
    fn IOSurfaceLookupFromMachPort(port: MachPort) -> Option<arc::R<Surf>>;

    fn IOSurfaceIsInUse(buffer: &Surf) -> bool;
    fn IOSurfaceGetUseCount(buffer: &Surf) -> i32;
    fn IOSurfaceIncrementUseCount(buffer: &mut Surf);
    fn IOSurfaceDecrementUseCount(buffer: &mut Surf);

    fn IOSurfaceAllowsPixelSizeCasting(buffer: &Surf) -> bool;

    fn IOSurfaceGetSeed(buffer: &Surf) -> u32;
    fn IOSurfaceGetAllocSize(buffer: &Surf) -> usize;

    fn IOSurfaceRemoveAllValues(buffer: &mut Surf);

}

/// The following list of properties are used with the cf::Dictionary passed to io::Surface::create
pub mod keys {
    use crate::cf::String;

    /// cf::Number of the total allocation size of the buffer including all planes.    
    /// Defaults to BufferHeight * BytesPerRow if not specified. Must be specified for
    /// dimensionless buffers.
    #[doc(alias = "kIOSurfaceAllocSize")]
    #[inline]
    pub fn alloc_size() -> &'static String {
        unsafe { kIOSurfaceAllocSize }
    }

    /// cf::Number for the width of the io::Surface buffer in pixels. Required for planar io::Surfaces
    #[doc(alias = "kIOSurfaceWidth")]
    #[inline]
    pub fn width() -> &'static String {
        unsafe { kIOSurfaceWidth }
    }

    /// cf::Number for the height of the io::Surface buffer in pixels. Required for planar io::Surfaces
    #[doc(alias = "kIOSurfaceHeight")]
    #[inline]
    pub fn height() -> &'static String {
        unsafe { kIOSurfaceHeight }
    }

    #[doc(alias = "kIOSurfaceBytesPerRow")]
    #[inline]
    pub fn bytes_per_row() -> &'static String {
        unsafe { kIOSurfaceBytesPerRow }
    }

    /// cf::Number for the total number of bytes in an element.
    #[doc(alias = "kIOSurfaceBytesPerElement")]
    #[inline]
    pub fn bytes_per_element() -> &'static String {
        unsafe { kIOSurfaceBytesPerElement }
    }

    /// cf::Number for how many pixels wide each element is.
    #[doc(alias = "kIOSurfaceElementWidth")]
    #[inline]
    pub fn element_width() -> &'static String {
        unsafe { kIOSurfaceElementWidth }
    }

    /// cf::Number for how many pixels high each element is.
    #[doc(alias = "kIOSurfaceElementHeight")]
    #[inline]
    pub fn element_height() -> &'static String {
        unsafe { kIOSurfaceElementHeight }
    }

    #[doc(alias = "kIOSurfaceOffset")]
    #[inline]
    pub fn offset() -> &'static String {
        unsafe { kIOSurfaceOffset }
    }

    #[doc(alias = "kIOSurfacePlaneInfo")]
    #[inline]
    pub fn plane_info() -> &'static String {
        unsafe { kIOSurfacePlaneInfo }
    }

    #[doc(alias = "kIOSurfacePlaneWidth")]
    #[inline]
    pub fn plane_width() -> &'static String {
        unsafe { kIOSurfacePlaneWidth }
    }

    #[doc(alias = "kIOSurfacePlaneHeight")]
    #[inline]
    pub fn plane_height() -> &'static String {
        unsafe { kIOSurfacePlaneHeight }
    }

    #[doc(alias = "kIOSurfacePlaneBytesPerRow")]
    #[inline]
    pub fn plane_bytes_per_row() -> &'static String {
        unsafe { kIOSurfacePlaneBytesPerRow }
    }

    #[doc(alias = "kIOSurfacePlaneOffset")]
    #[inline]
    pub fn plane_offset() -> &'static String {
        unsafe { kIOSurfacePlaneOffset }
    }

    #[doc(alias = "kIOSurfacePlaneSize")]
    #[inline]
    pub fn plane_size() -> &'static String {
        unsafe { kIOSurfacePlaneSize }
    }

    #[doc(alias = "kIOSurfacePlaneBase")]
    #[inline]
    pub fn plane_base() -> &'static String {
        unsafe { kIOSurfacePlaneBase }
    }

    #[doc(alias = "kIOSurfacePlaneBitsPerElement")]
    #[inline]
    pub fn plane_bits_per_element() -> &'static String {
        unsafe { kIOSurfacePlaneBitsPerElement }
    }

    #[doc(alias = "kIOSurfacePlaneBytesPerElement")]
    #[inline]
    pub fn plane_bytes_per_element() -> &'static String {
        unsafe { kIOSurfacePlaneBytesPerElement }
    }

    #[doc(alias = "kIOSurfacePlaneElementWidth")]
    #[inline]
    pub fn plane_element_width() -> &'static String {
        unsafe { kIOSurfacePlaneElementWidth }
    }

    #[doc(alias = "kIOSurfacePlaneElementHeight")]
    #[inline]
    pub fn plane_element_height() -> &'static String {
        unsafe { kIOSurfacePlaneElementHeight }
    }

    #[doc(alias = "kIOSurfaceCacheMode")]
    #[inline]
    pub fn cache_mode() -> &'static String {
        unsafe { kIOSurfaceCacheMode }
    }

    #[doc(alias = "kIOSurfacePixelFormat")]
    #[inline]
    pub fn pixel_format() -> &'static String {
        unsafe { kIOSurfacePixelFormat }
    }

    #[doc(alias = "kIOSurfacePixelSizeCastingAllowed")]
    #[inline]
    pub fn pixel_size_casting_allowed() -> &'static String {
        unsafe { kIOSurfacePixelSizeCastingAllowed }
    }

    #[doc(alias = "kIOSurfacePlaneComponentBitDepths")]
    #[inline]
    pub fn plane_component_bit_depths() -> &'static String {
        unsafe { kIOSurfacePlaneComponentBitDepths }
    }

    #[doc(alias = "kIOSurfacePlaneComponentBitOffsets")]
    #[inline]
    pub fn plane_component_bit_offsets() -> &'static String {
        unsafe { kIOSurfacePlaneComponentBitOffsets }
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
        static kIOSurfacePlaneBytesPerElement: &'static String;
        static kIOSurfacePlaneElementWidth: &'static String;
        static kIOSurfacePlaneElementHeight: &'static String;
        static kIOSurfaceCacheMode: &'static String;
        static kIOSurfacePixelFormat: &'static String;
        static kIOSurfacePixelSizeCastingAllowed: &'static String;
        static kIOSurfacePlaneComponentBitDepths: &'static String;
        static kIOSurfacePlaneComponentBitOffsets: &'static String;
    }
}

#[cfg(test)]
mod test {
    use crate::{cf, io};

    #[test]
    fn basics() {
        let width = cf::Number::from_i32(100);
        let height = cf::Number::from_i32(200);

        let properties = cf::Dictionary::with_keys_values(
            &[io::surface::keys::width(), io::surface::keys::height()],
            &[&width, &height],
        )
        .unwrap();

        let surf = io::Surf::create(&properties).unwrap();
        let port = surf.create_mach_port();
        let surf2 = io::Surf::from_mach_port(port).unwrap();
        port.task_self_deallocate();
        assert!(surf.equal(&surf2));
        assert_eq!(false, surf.is_in_use());
        assert_eq!(false, surf2.is_in_use());
        let vals = surf2.all_values().unwrap();
        vals.show();
    }
}
