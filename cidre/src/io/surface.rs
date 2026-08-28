use crate::{arc, cf, define_cf_type, define_opts, mach, mach::KernReturn, os, sys::_types::MachPort};

#[cfg(feature = "xpc")]
use crate::xpc;

#[doc(alias = "SurfaceID")]
pub type SurfId = u32;

#[doc(alias = "IOSurfaceComponentName")]
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

#[doc(alias = "IOSurfaceComponentType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum ComponentType {
    Unknown = 0,
    U32 = 1,
    S32 = 2,
    F32 = 3,
    SignedNormalized = 4,
}

#[doc(alias = "IOSurfaceComponentRange")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum ComponentRange {
    Unknown = 0,
    Full = 1,
    Video = 2,
    Wide = 3,
}

#[doc(alias = "IOSurfaceSubsampling")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum Subsampling {
    Unknown = 0,
    None = 1, // Includes "4:4:4"
    _422 = 2, // Chroma downsampled by 2x1
    _420 = 3, // Chroma downsampled by 2x2
    _411 = 4, // Chroma downsampled by 4x1
}

define_opts!(
    #[doc(alias = "IOSurfaceLockOptions")]
    pub LockOpts(u32)
);

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

define_cf_type!(
    #[doc(alias = "IOSurfaceRef")]
    Surf(cf::Type)
);

unsafe impl Send for Surf {}

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
    ///     io::surface::key::width(),
    ///     io::surface::key::height()
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

    /// A send right holding a reference to this surface, ready to travel in a
    /// message.
    ///
    /// [`None`] if the right could not be made. The right releases itself when
    /// the [`mach::SendRight`] drops, so the surface stays referenced for
    /// exactly as long as that lives — the same bargain [`Self::create_xpc_obj`]
    /// offers with an [`arc::R`].
    #[doc(alias = "IOSurfaceCreateMachPort")]
    #[inline]
    pub fn create_mach_port(&self) -> Option<mach::SendRight> {
        // SAFETY: the right is freshly made and ours alone, which is what
        // `try_from_name` is being told.
        unsafe { mach::SendRight::try_from_name(IOSurfaceCreateMachPort(self)) }
    }

    /// Recreates a surface from a send right — one made by
    /// [`Self::create_mach_port`], or copied out of a message by
    /// [`xpc::Dictionary::copy_mach_send`].
    ///
    /// The lookup borrows the right rather than consuming it, so `right` is
    /// still good afterwards and still releases itself on drop.
    #[doc(alias = "IOSurfaceLookupFromMachPort")]
    #[inline]
    pub fn from_mach_port(right: &mach::SendRight) -> Option<arc::R<Surf>> {
        Self::from_mach_port_name(right.name())
    }

    /// As [`Self::from_mach_port`], for a right this task does not own — one
    /// read out of a `mach::MsgHeader`, say, that something else will release.
    ///
    /// A name that does not belong to a surface is [`None`], not a fault, so
    /// this is safe to call with any name.
    #[doc(alias = "IOSurfaceLookupFromMachPort")]
    #[inline]
    pub fn from_mach_port_name(port: MachPort) -> Option<arc::R<Surf>> {
        unsafe { IOSurfaceLookupFromMachPort(port) }
    }

    /// An xpc object holding a reference to this surface, ready to travel in
    /// a message.
    ///
    /// Every live xpc object made this way implicitly raises the surface's
    /// global use count by one until it is released.
    #[doc(alias = "IOSurfaceCreateXPCObject")]
    #[cfg(feature = "xpc")]
    #[inline]
    pub fn create_xpc_obj(&self) -> arc::R<xpc::Object> {
        unsafe { IOSurfaceCreateXPCObject(self) }
    }

    /// Recreates a surface from an object made by [`Self::create_xpc_obj`].
    ///
    /// The object is borrowed, not consumed: releasing `xobj` stays [`arc::R`]'s
    /// job, just as releasing the right [`Self::from_mach_port`] borrows stays
    /// [`mach::SendRight`]'s.
    #[doc(alias = "IOSurfaceLookupFromXPCObject")]
    #[cfg(feature = "xpc")]
    #[inline]
    pub fn from_xpc_obj(xobj: &xpc::Object) -> Option<arc::R<Surf>> {
        unsafe { IOSurfaceLookupFromXPCObject(xobj) }
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
    pub fn inc_use_count(&mut self) {
        unsafe { IOSurfaceIncrementUseCount(self) }
    }

    #[doc(alias = "IOSurfaceDecrementUseCount")]
    #[inline]
    pub fn dec_use_count(&mut self) {
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
        unsafe { std::mem::transmute(ptr) }
    }

    #[doc(alias = "IOSurfaceGetBytesPerRow")]
    #[inline]
    pub fn bytes_per_row(&self) -> usize {
        unsafe { IOSurfaceGetBytesPerRow(self) }
    }

    #[doc(alias = "IOSurfaceGetPixelFormat")]
    #[inline]
    pub fn pixel_format(&self) -> os::Type {
        unsafe { IOSurfaceGetPixelFormat(self) }
    }

    #[doc(alias = "IOSurfaceLock")]
    #[inline]
    pub unsafe fn lock(&mut self, opts: LockOpts, seed: *mut u32) -> os::Result {
        unsafe { IOSurfaceLock(self, opts, seed).result() }
    }

    #[doc(alias = "IOSurfaceUnlock")]
    #[inline]
    pub unsafe fn unlock(&mut self, opts: LockOpts, seed: *mut u32) -> os::Result {
        unsafe { IOSurfaceUnlock(self, opts, seed).result() }
    }
}

unsafe extern "C-unwind" {
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

    #[cfg(feature = "xpc")]
    fn IOSurfaceCreateXPCObject(buffer: &Surf) -> arc::R<xpc::Object>;
    #[cfg(feature = "xpc")]
    fn IOSurfaceLookupFromXPCObject(xobj: &xpc::Object) -> Option<arc::R<Surf>>;

    fn IOSurfaceIsInUse(buffer: &Surf) -> bool;
    fn IOSurfaceGetUseCount(buffer: &Surf) -> i32;
    fn IOSurfaceIncrementUseCount(buffer: &mut Surf);
    fn IOSurfaceDecrementUseCount(buffer: &mut Surf);

    fn IOSurfaceAllowsPixelSizeCasting(buffer: &Surf) -> bool;

    fn IOSurfaceGetSeed(buffer: &Surf) -> u32;
    fn IOSurfaceGetAllocSize(buffer: &Surf) -> usize;

    fn IOSurfaceRemoveAllValues(buffer: &mut Surf);

    fn IOSurfaceGetBytesPerRow(buffer: &Surf) -> usize;
    fn IOSurfaceGetPixelFormat(buffer: &Surf) -> os::Type;

    fn IOSurfaceLock(buffer: &mut Surf, options: LockOpts, seed: *mut u32) -> KernReturn;
    fn IOSurfaceUnlock(buffer: &mut Surf, options: LockOpts, seed: *mut u32) -> KernReturn;

}

/// The following list of properties are used with the cf::Dictionary passed to io::Surface::create
pub mod key {
    use crate::{api, cf::String};

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

    #[doc(alias = "kIOSurfaceColorSpace")]
    #[inline]
    pub fn color_space() -> &'static String {
        unsafe { kIOSurfaceColorSpace }
    }

    #[doc(alias = "kIOSurfaceICCProfile")]
    #[inline]
    pub fn icc_profile() -> &'static String {
        unsafe { kIOSurfaceICCProfile }
    }

    /// cf::Number representation of the content headroom, which is defined as the ratio of nominal peak luminance
    /// ("peak white") to nominal diffuse luminance ("reference white" or "diffuse white").
    #[doc(alias = "kIOSurfaceContentHeadroom")]
    #[inline]
    #[api::available(
        macos = 15.0,
        maccatalyst = 18.0,
        ios = 18.0,
        watchos = 11.0,
        tvos = 18.0
    )]
    pub fn content_headroom() -> &'static String {
        unsafe { kIOSurfaceContentHeadroom }
    }

    #[api::weak]
    unsafe extern "C" {
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
        static kIOSurfaceColorSpace: &'static String;
        static kIOSurfaceICCProfile: &'static String;
        #[api::available(
            macos = 15.0,
            maccatalyst = 18.0,
            ios = 18.0,
            watchos = 11.0,
            tvos = 18.0
        )]
        static kIOSurfaceContentHeadroom: &'static String;
    }
}

#[cfg(test)]
mod test {
    #[cfg(not(feature = "macos_15_0"))]
    use crate::api;
    use crate::{cf, io};

    #[test]
    fn basics() {
        let width = cf::Number::from_i32(100);
        let height = cf::Number::from_i32(200);

        let properties = cf::Dictionary::with_keys_values(
            &[io::surface::key::width(), io::surface::key::height()],
            &[&width, &height],
        )
        .unwrap();

        let surf = io::Surf::create(&properties).unwrap();
        let port = surf.create_mach_port().unwrap();
        let surf2 = io::Surf::from_mach_port(&port).unwrap();
        // The lookup borrows the right rather than consuming it, so the same
        // one serves again.
        assert!(io::Surf::from_mach_port(&port).is_some());
        assert!(surf.equal(&surf2));

        // A live send right holds a use count, exactly as a live xpc object
        // does, and dropping it hands that back — no `task_self_deallocate`
        // for the caller to remember.
        assert!(surf.is_in_use());
        drop(port);
        assert_eq!(false, surf.is_in_use());
        assert_eq!(false, surf2.is_in_use());
        let vals = surf2.all_values().unwrap();
        vals.show();
    }

    #[cfg(feature = "xpc")]
    #[test]
    fn xpc_obj_roundtrip() {
        use crate::xpc;

        let width = cf::Number::from_i32(100);
        let height = cf::Number::from_i32(200);
        let properties = cf::Dictionary::with_keys_values(
            &[io::surface::key::width(), io::surface::key::height()],
            &[&width, &height],
        )
        .unwrap();
        let surf = io::Surf::create(&properties).unwrap();

        let xobj = surf.create_xpc_obj();
        // A live xpc object holds a use count on the surface.
        assert!(surf.is_in_use());

        let surf2 = io::Surf::from_xpc_obj(&xobj).unwrap();
        assert!(surf.equal(&surf2));

        // The object survives a round trip through a message.
        let mut msg = xpc::Dictionary::new();
        msg.set(c"surface", Some(&xobj));
        let surf3 = io::Surf::from_xpc_obj(msg.get(c"surface").unwrap()).unwrap();
        assert!(surf.equal(&surf3));

        drop(xobj);
        drop(msg);
        assert!(!surf.is_in_use());
    }

    /// `xpc_dictionary_set_mach_send` inserts with `MACH_MSG_TYPE_COPY_SEND`,
    /// so the dictionary takes a reference of its own and the caller keeps
    /// theirs. Both sides then hold a right that works on its own, and the
    /// surface only comes free once both are gone.
    #[cfg(feature = "xpc")]
    #[test]
    fn mach_send_roundtrip() {
        use crate::xpc;

        let width = cf::Number::from_i32(100);
        let height = cf::Number::from_i32(200);
        let properties = cf::Dictionary::with_keys_values(
            &[io::surface::key::width(), io::surface::key::height()],
            &[&width, &height],
        )
        .unwrap();
        let surf = io::Surf::create(&properties).unwrap();

        let right = surf.create_mach_port().unwrap();
        let mut msg = xpc::Dictionary::new();
        msg.set_mach_send(c"surface", &right);

        // Inserting copied the right; ours still resolves.
        assert!(surf.equal(&io::Surf::from_mach_port(&right).unwrap()));

        // And the dictionary's copy resolves independently of ours. Same task
        // and same port, so mach hands back the name we already have, carrying
        // one more user reference rather than a second name.
        let copied = msg.copy_mach_send(c"surface").unwrap();
        assert_eq!(copied, right);
        assert!(surf.equal(&io::Surf::from_mach_port(&copied).unwrap()));

        // Dropping one gives back one reference, leaving the other good.
        drop(right);
        assert!(surf.equal(&io::Surf::from_mach_port(&copied).unwrap()));
        assert!(surf.is_in_use());

        drop(copied);
        drop(msg);
        assert_eq!(false, surf.is_in_use());

        // A key that holds no send right is `None`, not a null name.
        assert!(xpc::Dictionary::new().copy_mach_send(c"absent").is_none());
    }

    #[cfg(not(feature = "macos_15_0"))]
    #[test]
    fn versioning() {
        if api::version!(macos = 15.0) {
            let _ = unsafe { io::surface::key::content_headroom().unwrap() };
        } else {
            let k = unsafe { io::surface::key::content_headroom() };
            assert!(k.is_none());
        }
    }

    #[cfg(feature = "macos_15_0")]
    #[test]
    fn versioning() {
        let _ = io::surface::key::content_headroom();
    }
}
