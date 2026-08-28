//! XPC — inter-process communication.
//!
//! Layout follows `<xpc/*.h>`: this file is `xpc.h`, and every other header
//! gets one module of its own.
//!
//! XPC is an `os_object` family like [`dispatch`](crate::dispatch) and
//! [`nw`](crate::nw): every value is an Objective-C object under the hood, so
//! [`arc::R`](crate::arc::R) manages the reference counts.
//!
//! Three things differ from `dispatch` and are worth knowing before you read
//! on:
//!
//! - C models every value as one `xpc_object_t` and *aborts the process* when
//!   an accessor meets the wrong type. Here each type is a
//!   `#[repr(transparent)]` newtype over [`Object`], and [`Object::try_cast`]
//!   is the checked way down.
//! - [`Type`] is not an object. It is a static type descriptor, only ever
//!   compared by pointer.
//! - Small values — `int64`, `uint64`, `bool`, `null` — are **tagged
//!   pointers**, not allocations. `&`[`Object`] is therefore an address to
//!   hand back to XPC, never a pointer to dereference: build one with
//!   `transmute`, not `&*ptr`, or the debug misaligned-pointer check aborts
//!   the process. (Retain and release are correct on them; the Objective-C
//!   runtime recognises the tag.)
//!
//! ```no_run
//! use cidre::xpc;
//!
//! let con = xpc::Connection::with_mach_service(
//!     c"com.example.service",
//!     None,
//!     xpc::MachServiceFlags::NONE,
//!     |obj| match xpc::Event::from(obj) {
//!         xpc::Event::Msg(msg) => println!("{:?}", msg.i64(c"result")),
//!         xpc::Event::Err(err) => eprintln!("{err}"),
//!         xpc::Event::Other(_) => {}
//!     },
//! );
//!
//! let mut msg = xpc::Dictionary::new();
//! msg.set_cstr(c"op", c"ping");
//! con.send_msg(&msg);
//! ```

use std::{
    ffi::{CStr, CString, c_char, c_void},
    os::fd::{FromRawFd, OwnedFd, RawFd},
};

use crate::{arc, define_obj_type, mach, ns};

#[cfg(feature = "blocks")]
use crate::blocks;

/// `<xpc/connection.h>`
pub mod connection;
pub use connection::Connection;
pub use connection::Event;
pub use connection::Handler;
pub use connection::MachServiceFlags;
pub use connection::ReplyHandler;

/// `<xpc/endpoint.h>`
pub mod endpoint;
pub use endpoint::Endpoint;

// MARK: XPC Object Protocol

define_obj_type!(
    /// The base of every XPC value.
    ///
    /// XPC is an `os_object` family, like `dispatch` and `nw`: under an
    /// Objective-C compiler `xpc_object_t` is an `NSObject` subclass, so
    /// `xpc_retain`/`xpc_release` are `objc_retain`/`objc_release` and
    /// [`arc::R`] manages them.
    ///
    /// Unlike `dispatch`, C gives every value the *same* type, and the
    /// accessors abort the process on a type mismatch. The concrete types in
    /// this module (all `#[repr(transparent)]` over `Object`) restore the
    /// static typing; [`Object::try_cast`] is the checked way back down.
    ///
    /// A reference to one is an address to pass back to XPC, not a pointer to
    /// read: small values are tagged pointers. Reinterpret with `transmute`,
    /// never `&*ptr`.
    #[doc(alias = "xpc_object_t")]
    pub Object(ns::Id)
);

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

/// `xpc_type_t` — a static type descriptor.
///
/// This is **not** an XPC object: it is never retained, never travels in a
/// message, and exists only to be compared by pointer against
/// [`Object::get_type`]. (In the current implementation it is literally the
/// Objective-C class of its instances: `XPC_TYPE_STRING` and
/// `objc_getClass("OS_xpc_string")` are the same address.)
#[doc(alias = "xpc_type_t")]
#[repr(transparent)]
pub struct Type(c_void);

impl PartialEq for Type {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for Type {}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("xpc::Type").field(&self.name()).finish()
    }
}

impl Type {
    /// A short name like `"string"` or `"int64"`.
    #[doc(alias = "xpc_type_get_name")]
    #[inline]
    pub fn name(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(xpc_type_get_name(self)) }
    }

    #[doc(alias = "XPC_TYPE_NULL")]
    #[inline]
    pub fn null() -> &'static Self {
        unsafe { &_xpc_type_null }
    }

    #[doc(alias = "XPC_TYPE_BOOL")]
    #[inline]
    pub fn bool() -> &'static Self {
        unsafe { &_xpc_type_bool }
    }

    #[doc(alias = "XPC_TYPE_INT64")]
    #[inline]
    pub fn int64() -> &'static Self {
        unsafe { &_xpc_type_int64 }
    }

    #[doc(alias = "XPC_TYPE_UINT64")]
    #[inline]
    pub fn uint64() -> &'static Self {
        unsafe { &_xpc_type_uint64 }
    }

    #[doc(alias = "XPC_TYPE_DOUBLE")]
    #[inline]
    pub fn double() -> &'static Self {
        unsafe { &_xpc_type_double }
    }

    #[doc(alias = "XPC_TYPE_DATE")]
    #[inline]
    pub fn date() -> &'static Self {
        unsafe { &_xpc_type_date }
    }

    #[doc(alias = "XPC_TYPE_DATA")]
    #[inline]
    pub fn data() -> &'static Self {
        unsafe { &_xpc_type_data }
    }

    #[doc(alias = "XPC_TYPE_STRING")]
    #[inline]
    pub fn string() -> &'static Self {
        unsafe { &_xpc_type_string }
    }

    #[doc(alias = "XPC_TYPE_UUID")]
    #[inline]
    pub fn uuid() -> &'static Self {
        unsafe { &_xpc_type_uuid }
    }

    #[doc(alias = "XPC_TYPE_FD")]
    #[inline]
    pub fn fd() -> &'static Self {
        unsafe { &_xpc_type_fd }
    }

    #[doc(alias = "XPC_TYPE_SHMEM")]
    #[inline]
    pub fn shmem() -> &'static Self {
        unsafe { &_xpc_type_shmem }
    }

    #[doc(alias = "XPC_TYPE_ARRAY")]
    #[inline]
    pub fn array() -> &'static Self {
        unsafe { &_xpc_type_array }
    }

    #[doc(alias = "XPC_TYPE_DICTIONARY")]
    #[inline]
    pub fn dictionary() -> &'static Self {
        unsafe { &_xpc_type_dictionary }
    }

    #[doc(alias = "XPC_TYPE_ERROR")]
    #[inline]
    pub fn error() -> &'static Self {
        unsafe { &_xpc_type_error }
    }

    #[doc(alias = "XPC_TYPE_CONNECTION")]
    #[inline]
    pub fn connection() -> &'static Self {
        unsafe { &_xpc_type_connection }
    }

    #[doc(alias = "XPC_TYPE_ENDPOINT")]
    #[inline]
    pub fn endpoint() -> &'static Self {
        unsafe { &_xpc_type_endpoint }
    }
}

/// A concrete XPC value type, tagged with the [`Type`] the runtime reports for
/// it.
///
/// # Safety
///
/// The implementor must be `#[repr(transparent)]` over [`Object`], so that
/// [`Object::try_cast`] may reinterpret a reference once the type matches.
/// Use [`define_xpc_type!`] rather than implementing this by hand.
pub unsafe trait XpcType: crate::objc::Obj {
    fn xpc_type() -> &'static Type;
}

/// Defines a `#[repr(transparent)]` newtype over [`Object`](Object) and
/// wires it to its `XPC_TYPE_*` descriptor.
///
/// ```ignore
/// define_xpc_type!(pub String, string);   // -> xpc::Type::string()
/// ```
#[macro_export]
macro_rules! define_xpc_type {
    (
        $(#[$outer:meta])*
        $vis:vis $NewType:ident, $ty:ident
    ) => {
        $crate::define_obj_type!(
            $(#[$outer])*
            $vis $NewType($crate::xpc::Object)
        );

        // SAFETY: `define_obj_type!` emits a `#[repr(transparent)]` newtype
        // over `xpc::Object`.
        unsafe impl $crate::xpc::XpcType for $NewType {
            #[inline]
            fn xpc_type() -> &'static $crate::xpc::Type {
                $crate::xpc::Type::$ty()
            }
        }
    };
}

impl Object {
    #[doc(alias = "xpc_get_type")]
    #[inline]
    pub fn get_type(&self) -> &'static Type {
        unsafe { xpc_get_type(self) }
    }

    #[inline]
    pub fn is<T: XpcType>(&self) -> bool {
        self.get_type() == T::xpc_type()
    }

    /// A checked downcast. `None` when the runtime type is something else.
    ///
    /// The C accessors abort the process instead; this is the reason the
    /// concrete types exist.
    #[inline]
    pub fn try_cast<T: XpcType>(&self) -> Option<&T> {
        // SAFETY: `XpcType` guarantees `T` is transparent over `Self`, and the
        // runtime just confirmed the dynamic type.
        //
        // `transmute`, not `&*(ptr as *const T)`: small values are tagged
        // pointers, and building a reference from one trips the debug
        // misaligned-pointer check.
        self.is::<T>()
            .then(|| unsafe { std::mem::transmute::<&Self, &T>(self) })
    }

    /// A deep copy. `None` for the types that do not support copying
    /// (connections, for one).
    #[doc(alias = "xpc_copy")]
    #[inline]
    pub fn copy(&self) -> Option<arc::R<Self>> {
        unsafe { xpc_copy(self) }
    }

    /// Values of different types are never equal.
    ///
    /// This is the same relation `PartialEq` gives you through `-isEqual:`;
    /// it is bound for parity with the C API.
    #[doc(alias = "xpc_equal")]
    #[inline]
    pub fn equal(&self, other: &Self) -> bool {
        unsafe { xpc_equal(self, other) }
    }

    /// Not stable across processes or OS releases — do not persist it.
    #[inline]
    pub fn xpc_hash(&self) -> usize {
        unsafe { xpc_hash(self) }
    }
}

/// Parks the main thread and services incoming connections for a bundled XPC
/// service. Never returns.
#[doc(alias = "xpc_main")]
#[inline]
pub fn main(handler: extern "C-unwind" fn(&mut Connection)) -> ! {
    unsafe { xpc_main(handler) }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    static _xpc_type_null: Type;
    static _xpc_type_bool: Type;
    static _xpc_type_int64: Type;
    static _xpc_type_uint64: Type;
    static _xpc_type_double: Type;
    static _xpc_type_date: Type;
    static _xpc_type_data: Type;
    static _xpc_type_string: Type;
    static _xpc_type_uuid: Type;
    static _xpc_type_fd: Type;
    static _xpc_type_shmem: Type;
    static _xpc_type_array: Type;
    static _xpc_type_dictionary: Type;
    static _xpc_type_error: Type;
    static _xpc_type_connection: Type;
    static _xpc_type_endpoint: Type;

    fn xpc_get_type(object: &Object) -> &'static Type;
    fn xpc_type_get_name(ty: &Type) -> *const c_char;
    fn xpc_copy(object: &Object) -> Option<arc::R<Object>>;
    fn xpc_equal(a: &Object, b: &Object) -> bool;
    fn xpc_hash(object: &Object) -> usize;
    fn xpc_main(handler: extern "C-unwind" fn(&mut Connection)) -> !;
}

// MARK: Primitive Types

define_xpc_type!(
    /// Distinguishes "key present but empty" from "key absent". XPC
    /// dictionaries cannot hold a C `NULL`.
    #[doc(alias = "XPC_TYPE_NULL")]
    pub Null, null
);

define_xpc_type!(
    #[doc(alias = "XPC_TYPE_BOOL")]
    pub Bool, bool
);

define_xpc_type!(
    #[doc(alias = "XPC_TYPE_INT64")]
    pub I64, int64
);

define_xpc_type!(
    #[doc(alias = "XPC_TYPE_UINT64")]
    pub U64, uint64
);

define_xpc_type!(
    #[doc(alias = "XPC_TYPE_DOUBLE")]
    pub F64, double
);

define_xpc_type!(
    /// An interval relative to the Unix epoch, in nanoseconds. Unaware of
    /// local time and of leap seconds.
    #[doc(alias = "XPC_TYPE_DATE")]
    pub Date, date
);

define_xpc_type!(
    #[doc(alias = "XPC_TYPE_DATA")]
    pub Data, data
);

define_xpc_type!(
    /// A NUL-terminated C string.
    #[doc(alias = "XPC_TYPE_STRING")]
    pub String, string
);

define_xpc_type!(
    #[doc(alias = "XPC_TYPE_UUID")]
    pub Uuid, uuid
);

define_xpc_type!(
    /// A POSIX file descriptor. Sending one hands the peer a *duplicate*.
    #[doc(alias = "XPC_TYPE_FD")]
    pub Fd, fd
);

define_xpc_type!(
    #[doc(alias = "XPC_TYPE_SHMEM")]
    pub Shmem, shmem
);

impl Null {
    #[doc(alias = "xpc_null_create")]
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { xpc_null_create() }
    }
}

impl Bool {
    /// `XPC_BOOL_TRUE`.
    ///
    /// The symbol *is* the object — hence `&_xpc_bool_true`, not a pointer
    /// read. Retain and release on it are no-ops.
    #[doc(alias = "XPC_BOOL_TRUE")]
    #[inline]
    pub fn yes() -> &'static Self {
        unsafe { &_xpc_bool_true }
    }

    #[doc(alias = "XPC_BOOL_FALSE")]
    #[inline]
    pub fn no() -> &'static Self {
        unsafe { &_xpc_bool_false }
    }

    /// In practice returns one of the two singletons above.
    #[doc(alias = "xpc_bool_create")]
    #[inline]
    pub fn with_value(val: bool) -> arc::R<Self> {
        unsafe { xpc_bool_create(val) }
    }

    #[doc(alias = "xpc_bool_get_value")]
    #[inline]
    pub fn value(&self) -> bool {
        unsafe { xpc_bool_get_value(self) }
    }
}

impl I64 {
    #[doc(alias = "xpc_int64_create")]
    #[inline]
    pub fn with_value(val: i64) -> arc::R<Self> {
        unsafe { xpc_int64_create(val) }
    }

    #[doc(alias = "xpc_int64_get_value")]
    #[inline]
    pub fn value(&self) -> i64 {
        unsafe { xpc_int64_get_value(self) }
    }
}

impl U64 {
    #[doc(alias = "xpc_uint64_create")]
    #[inline]
    pub fn with_value(val: u64) -> arc::R<Self> {
        unsafe { xpc_uint64_create(val) }
    }

    #[doc(alias = "xpc_uint64_get_value")]
    #[inline]
    pub fn value(&self) -> u64 {
        unsafe { xpc_uint64_get_value(self) }
    }
}

impl F64 {
    #[doc(alias = "xpc_double_create")]
    #[inline]
    pub fn with_value(val: f64) -> arc::R<Self> {
        unsafe { xpc_double_create(val) }
    }

    #[doc(alias = "xpc_double_get_value")]
    #[inline]
    pub fn value(&self) -> f64 {
        unsafe { xpc_double_get_value(self) }
    }
}

impl Date {
    /// `interval` is nanoseconds since the Unix epoch.
    #[doc(alias = "xpc_date_create")]
    #[inline]
    pub fn with_value(interval: i64) -> arc::R<Self> {
        unsafe { xpc_date_create(interval) }
    }

    #[doc(alias = "xpc_date_create_from_current")]
    #[inline]
    pub fn now() -> arc::R<Self> {
        unsafe { xpc_date_create_from_current() }
    }

    #[doc(alias = "xpc_date_get_value")]
    #[inline]
    pub fn value(&self) -> i64 {
        unsafe { xpc_date_get_value(self) }
    }
}

impl Data {
    #[doc(alias = "xpc_data_create")]
    #[inline]
    pub fn with_slice(bytes: &[u8]) -> arc::R<Self> {
        unsafe { xpc_data_create(bytes.as_ptr() as _, bytes.len()) }
    }

    #[cfg(feature = "dispatch")]
    #[doc(alias = "xpc_data_create_with_dispatch_data")]
    #[inline]
    pub fn with_dispatch_data(data: &crate::dispatch::Data) -> arc::R<Self> {
        unsafe { xpc_data_create_with_dispatch_data(data) }
    }

    #[doc(alias = "xpc_data_get_length")]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { xpc_data_get_length(self) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Borrowed for as long as `self` lives.
    #[doc(alias = "xpc_data_get_bytes_ptr")]
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        let ptr = unsafe { xpc_data_get_bytes_ptr(self) };
        if ptr.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(ptr as *const u8, self.len()) }
    }
}

impl String {
    #[doc(alias = "xpc_string_create")]
    #[inline]
    pub fn with_cstr(str: &CStr) -> arc::R<Self> {
        unsafe { xpc_string_create(str.as_ptr()) }
    }

    /// Copies through a `CString`; fails if `str` contains an interior NUL.
    #[doc(alias = "xpc_string_create")]
    #[inline]
    pub fn with_str(str: &str) -> Option<arc::R<Self>> {
        let str = CString::new(str).ok()?;
        Some(Self::with_cstr(&str))
    }

    /// Byte length, not counting the NUL.
    #[doc(alias = "xpc_string_get_length")]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { xpc_string_get_length(self) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Borrowed for as long as `self` lives.
    #[doc(alias = "xpc_string_get_string_ptr")]
    #[inline]
    pub fn as_cstr(&self) -> Option<&CStr> {
        let ptr = unsafe { xpc_string_get_string_ptr(self) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr) })
    }

    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        self.as_cstr()?.to_str().ok()
    }
}

impl Uuid {
    #[doc(alias = "xpc_uuid_create")]
    #[inline]
    pub fn with_bytes(uuid: &[u8; 16]) -> arc::R<Self> {
        unsafe { xpc_uuid_create(uuid) }
    }

    #[doc(alias = "xpc_uuid_get_bytes")]
    #[inline]
    pub fn bytes(&self) -> Option<&[u8; 16]> {
        let ptr = unsafe { xpc_uuid_get_bytes(self) };
        (!ptr.is_null()).then(|| unsafe { &*(ptr as *const [u8; 16]) })
    }
}

impl Fd {
    /// Duplicates `fd`. `None` if it is not a valid descriptor.
    #[doc(alias = "xpc_fd_create")]
    #[inline]
    pub fn with_raw_fd(fd: RawFd) -> Option<arc::R<Self>> {
        unsafe { xpc_fd_create(fd) }
    }

    /// A fresh descriptor for the same resource, owned by the caller.
    #[doc(alias = "xpc_fd_dup")]
    #[inline]
    pub fn dup(&self) -> Option<OwnedFd> {
        match unsafe { xpc_fd_dup(self) } {
            -1 => None,
            fd => Some(unsafe { OwnedFd::from_raw_fd(fd) }),
        }
    }
}

impl Shmem {
    /// # Safety
    ///
    /// `region` must point to `len` bytes obtained from `mmap`/`vm_allocate`,
    /// and stay mapped while the object lives.
    #[doc(alias = "xpc_shmem_create")]
    #[inline]
    pub unsafe fn with_region(region: *mut c_void, len: usize) -> arc::R<Self> {
        unsafe { xpc_shmem_create(region, len) }
    }

    /// Maps the region into this process and returns the mapping and its
    /// length. `None` on failure.
    ///
    /// # Safety
    ///
    /// The caller owns the mapping and must `munmap` it.
    #[doc(alias = "xpc_shmem_map")]
    #[inline]
    pub unsafe fn map(&self) -> Option<(*mut c_void, usize)> {
        let mut region = std::ptr::null_mut();
        match unsafe { xpc_shmem_map(self, &mut region) } {
            0 => None,
            len => Some((region, len)),
        }
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    static _xpc_bool_true: Bool;
    static _xpc_bool_false: Bool;

    fn xpc_null_create() -> arc::R<Null>;

    fn xpc_bool_create(val: bool) -> arc::R<Bool>;
    fn xpc_bool_get_value(xbool: &Bool) -> bool;

    fn xpc_int64_create(val: i64) -> arc::R<I64>;
    fn xpc_int64_get_value(xint: &I64) -> i64;

    fn xpc_uint64_create(val: u64) -> arc::R<U64>;
    fn xpc_uint64_get_value(xuint: &U64) -> u64;

    fn xpc_double_create(val: f64) -> arc::R<F64>;
    fn xpc_double_get_value(xdouble: &F64) -> f64;

    fn xpc_date_create(interval: i64) -> arc::R<Date>;
    fn xpc_date_create_from_current() -> arc::R<Date>;
    fn xpc_date_get_value(xdate: &Date) -> i64;

    fn xpc_data_create(bytes: *const c_void, len: usize) -> arc::R<Data>;
    #[cfg(feature = "dispatch")]
    fn xpc_data_create_with_dispatch_data(data: &crate::dispatch::Data) -> arc::R<Data>;
    fn xpc_data_get_length(xdata: &Data) -> usize;
    fn xpc_data_get_bytes_ptr(xdata: &Data) -> *const c_void;

    fn xpc_string_create(str: *const c_char) -> arc::R<String>;
    fn xpc_string_get_length(xstring: &String) -> usize;
    fn xpc_string_get_string_ptr(xstring: &String) -> *const c_char;

    fn xpc_uuid_create(uuid: &[u8; 16]) -> arc::R<Uuid>;
    fn xpc_uuid_get_bytes(xuuid: &Uuid) -> *const u8;

    fn xpc_fd_create(fd: RawFd) -> Option<arc::R<Fd>>;
    fn xpc_fd_dup(xfd: &Fd) -> RawFd;

    fn xpc_shmem_create(region: *mut c_void, len: usize) -> arc::R<Shmem>;
    fn xpc_shmem_map(xshmem: &Shmem, region: *mut *mut c_void) -> usize;
}
// MARK: Arrays

/// Returns `false` to stop iterating.
#[cfg(feature = "blocks")]
#[doc(alias = "xpc_array_applier_t")]
pub type ArrayApplier = blocks::NoEscBlock<fn(index: usize, val: &Object) -> bool>;

define_xpc_type!(
    /// A contiguous, growable array. It cannot hold a C `NULL` — use
    /// [`Null`](Null) for an empty slot.
    #[doc(alias = "XPC_TYPE_ARRAY")]
    pub Array, array
);

impl Array {
    #[doc(alias = "xpc_array_create")]
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { xpc_array_create(std::ptr::null(), 0) }
    }

    #[doc(alias = "xpc_array_create")]
    #[inline]
    pub fn with_values(values: &[&Object]) -> arc::R<Self> {
        unsafe { xpc_array_create(values.as_ptr() as _, values.len()) }
    }

    #[doc(alias = "xpc_array_get_count")]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { xpc_array_get_count(self) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Borrowed for as long as `self` lives.
    ///
    /// `xpc_array_get_value` aborts the process out of bounds; this returns
    /// `None` instead.
    #[doc(alias = "xpc_array_get_value")]
    #[inline]
    pub fn get(&self, index: usize) -> Option<&Object> {
        (index < self.len()).then(|| unsafe { xpc_array_get_value(self, index) })
    }

    /// # Panics
    ///
    /// If `index` is out of bounds — where C would abort the process.
    #[doc(alias = "xpc_array_set_value")]
    #[inline]
    pub fn set(&mut self, index: usize, val: &Object) {
        assert!(index < self.len(), "index {index} out of bounds");
        unsafe { xpc_array_set_value(self, index, val) }
    }

    #[doc(alias = "xpc_array_append_value")]
    #[inline]
    pub fn push(&mut self, val: &Object) {
        unsafe { xpc_array_append_value(self, val) }
    }

    /// A borrowed C string, or `None` if the slot is not a string.
    #[doc(alias = "xpc_array_get_string")]
    #[inline]
    pub fn get_cstr(&self, index: usize) -> Option<&CStr> {
        let ptr = unsafe { xpc_array_get_string(self, index) };
        (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) })
    }

    /// Returns `false` if the applier stopped early.
    #[cfg(feature = "blocks")]
    #[doc(alias = "xpc_array_apply")]
    #[inline]
    pub fn apply_block(&self, applier: &mut ArrayApplier) -> bool {
        unsafe { xpc_array_apply(self, applier) }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn apply(&self, mut f: impl FnMut(usize, &Object) -> bool) -> bool {
        let mut block = unsafe { ArrayApplier::stack2(&mut f) };
        self.apply_block(&mut block)
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Object;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            array: self,
            index: 0,
        }
    }
}

pub struct Iter<'a> {
    array: &'a Array,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Object;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.array.get(self.index)?;
        self.index += 1;
        Some(val)
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    fn xpc_array_create(objects: *const c_void, count: usize) -> arc::R<Array>;
    fn xpc_array_get_count(xarray: &Array) -> usize;
    fn xpc_array_get_value<'a>(xarray: &'a Array, index: usize) -> &'a Object;
    fn xpc_array_set_value(xarray: &mut Array, index: usize, val: &Object);
    fn xpc_array_append_value(xarray: &mut Array, val: &Object);
    fn xpc_array_get_string(xarray: &Array, index: usize) -> *const c_char;

    #[cfg(feature = "blocks")]
    fn xpc_array_apply(xarray: &Array, applier: &mut ArrayApplier) -> bool;
}
// MARK: Dictionaries

/// Returns `false` to stop iterating.
#[cfg(feature = "blocks")]
#[doc(alias = "xpc_dictionary_applier_t")]
pub type DictionaryApplier = blocks::NoEscBlock<fn(key: *const c_char, val: &Object) -> bool>;

define_xpc_type!(
    /// A dictionary keyed by C strings. This is the type of every XPC message.
    #[doc(alias = "XPC_TYPE_DICTIONARY")]
    pub Dictionary, dictionary
);

impl Dictionary {
    #[doc(alias = "xpc_dictionary_create")]
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { xpc_dictionary_create(std::ptr::null(), std::ptr::null(), 0) }
    }

    /// A reply addressed back to the sender of `self`.
    ///
    /// `None` unless `self` is a message that expects a reply.
    #[doc(alias = "xpc_dictionary_create_reply")]
    #[inline]
    pub fn reply(&self) -> Option<arc::R<Self>> {
        unsafe { xpc_dictionary_create_reply(self) }
    }

    #[doc(alias = "xpc_dictionary_get_count")]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { xpc_dictionary_get_count(self) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Borrowed for as long as `self` lives.
    #[doc(alias = "xpc_dictionary_get_value")]
    #[inline]
    pub fn get(&self, key: &CStr) -> Option<&Object> {
        unsafe { xpc_dictionary_get_value(self, key.as_ptr()) }
    }

    /// `None` removes the key.
    #[doc(alias = "xpc_dictionary_set_value")]
    #[inline]
    pub fn set(&mut self, key: &CStr, val: Option<&Object>) {
        unsafe { xpc_dictionary_set_value(self, key.as_ptr(), val) }
    }

    /// A checked read: `None` for a missing key *or* a value of another type.
    ///
    /// The `*_raw` accessors below match C and cannot tell those apart from a
    /// zero value.
    #[inline]
    pub fn get_as<T: XpcType>(&self, key: &CStr) -> Option<&T> {
        self.get(key)?.try_cast()
    }

    #[doc(alias = "xpc_dictionary_set_bool")]
    #[inline]
    pub fn set_bool(&mut self, key: &CStr, val: bool) {
        unsafe { xpc_dictionary_set_bool(self, key.as_ptr(), val) }
    }

    /// `false` when the key is missing or holds another type.
    #[doc(alias = "xpc_dictionary_get_bool")]
    #[inline]
    pub fn bool_raw(&self, key: &CStr) -> bool {
        unsafe { xpc_dictionary_get_bool(self, key.as_ptr()) }
    }

    #[inline]
    pub fn bool(&self, key: &CStr) -> Option<bool> {
        Some(self.get_as::<Bool>(key)?.value())
    }

    #[doc(alias = "xpc_dictionary_set_int64")]
    #[inline]
    pub fn set_i64(&mut self, key: &CStr, val: i64) {
        unsafe { xpc_dictionary_set_int64(self, key.as_ptr(), val) }
    }

    /// `0` when the key is missing or holds another type.
    #[doc(alias = "xpc_dictionary_get_int64")]
    #[inline]
    pub fn i64_raw(&self, key: &CStr) -> i64 {
        unsafe { xpc_dictionary_get_int64(self, key.as_ptr()) }
    }

    #[inline]
    pub fn i64(&self, key: &CStr) -> Option<i64> {
        Some(self.get_as::<I64>(key)?.value())
    }

    #[doc(alias = "xpc_dictionary_set_uint64")]
    #[inline]
    pub fn set_u64(&mut self, key: &CStr, val: u64) {
        unsafe { xpc_dictionary_set_uint64(self, key.as_ptr(), val) }
    }

    /// `0` when the key is missing or holds another type.
    #[doc(alias = "xpc_dictionary_get_uint64")]
    #[inline]
    pub fn u64_raw(&self, key: &CStr) -> u64 {
        unsafe { xpc_dictionary_get_uint64(self, key.as_ptr()) }
    }

    #[inline]
    pub fn u64(&self, key: &CStr) -> Option<u64> {
        Some(self.get_as::<U64>(key)?.value())
    }

    #[doc(alias = "xpc_dictionary_set_double")]
    #[inline]
    pub fn set_f64(&mut self, key: &CStr, val: f64) {
        unsafe { xpc_dictionary_set_double(self, key.as_ptr(), val) }
    }

    /// `NAN` when the key is missing or holds another type.
    #[doc(alias = "xpc_dictionary_get_double")]
    #[inline]
    pub fn f64_raw(&self, key: &CStr) -> f64 {
        unsafe { xpc_dictionary_get_double(self, key.as_ptr()) }
    }

    #[inline]
    pub fn f64(&self, key: &CStr) -> Option<f64> {
        Some(self.get_as::<F64>(key)?.value())
    }

    #[doc(alias = "xpc_dictionary_set_string")]
    #[inline]
    pub fn set_cstr(&mut self, key: &CStr, val: &CStr) {
        unsafe { xpc_dictionary_set_string(self, key.as_ptr(), val.as_ptr()) }
    }

    /// Borrowed for as long as `self` lives. `None` when the key is missing or
    /// holds another type.
    #[doc(alias = "xpc_dictionary_get_string")]
    #[inline]
    pub fn cstr(&self, key: &CStr) -> Option<&CStr> {
        let ptr = unsafe { xpc_dictionary_get_string(self, key.as_ptr()) };
        (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) })
    }

    #[inline]
    pub fn str(&self, key: &CStr) -> Option<&str> {
        self.cstr(key)?.to_str().ok()
    }

    #[doc(alias = "xpc_dictionary_set_data")]
    #[inline]
    pub fn set_data(&mut self, key: &CStr, val: &[u8]) {
        unsafe { xpc_dictionary_set_data(self, key.as_ptr(), val.as_ptr() as _, val.len()) }
    }

    #[inline]
    pub fn data(&self, key: &CStr) -> Option<&[u8]> {
        Some(self.get_as::<Data>(key)?.as_slice())
    }

    #[inline]
    pub fn dictionary(&self, key: &CStr) -> Option<&Dictionary> {
        self.get_as(key)
    }

    #[inline]
    pub fn array(&self, key: &CStr) -> Option<&Array> {
        self.get_as(key)
    }

    /// Sends a *duplicate* of `fd` to the peer.
    #[doc(alias = "xpc_dictionary_set_fd")]
    #[inline]
    pub fn set_fd(&mut self, key: &CStr, fd: std::os::fd::RawFd) {
        unsafe { xpc_dictionary_set_fd(self, key.as_ptr(), fd) }
    }

    /// A fresh descriptor owned by the caller.
    #[doc(alias = "xpc_dictionary_dup_fd")]
    #[inline]
    pub fn dup_fd(&self, key: &CStr) -> Option<std::os::fd::OwnedFd> {
        use std::os::fd::FromRawFd;
        match unsafe { xpc_dictionary_dup_fd(self, key.as_ptr()) } {
            -1 => None,
            fd => Some(unsafe { std::os::fd::OwnedFd::from_raw_fd(fd) }),
        }
    }

    /// Inserts a send right. XPC consumes the caller's reference.
    #[doc(alias = "xpc_dictionary_set_mach_send")]
    #[inline]
    pub fn set_mach_send(&mut self, key: &CStr, port: mach::Port) {
        unsafe { xpc_dictionary_set_mach_send(self, key.as_ptr(), port) }
    }

    /// A send right the caller must deallocate. [`mach::Port::NULL`] if absent.
    #[doc(alias = "xpc_dictionary_copy_mach_send")]
    #[inline]
    pub fn copy_mach_send(&self, key: &CStr) -> mach::Port {
        unsafe { xpc_dictionary_copy_mach_send(self, key.as_ptr()) }
    }

    /// The connection a received message arrived on. `None` for a dictionary
    /// you built yourself.
    #[doc(alias = "xpc_dictionary_get_remote_connection")]
    #[inline]
    pub fn remote_connection(&self) -> Option<&Connection> {
        unsafe { xpc_dictionary_get_remote_connection(self) }
    }

    /// Returns `false` if the applier stopped early.
    #[cfg(feature = "blocks")]
    #[doc(alias = "xpc_dictionary_apply")]
    #[inline]
    pub fn apply_block(&self, applier: &mut DictionaryApplier) -> bool {
        unsafe { xpc_dictionary_apply(self, applier) }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn apply(&self, mut f: impl FnMut(&CStr, &Object) -> bool) -> bool {
        let mut shim = |key: *const c_char, val: &Object| f(unsafe { CStr::from_ptr(key) }, val);
        let mut block = unsafe { DictionaryApplier::stack2(&mut shim) };
        self.apply_block(&mut block)
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    fn xpc_dictionary_create(
        keys: *const *const c_char,
        values: *const c_void,
        count: usize,
    ) -> arc::R<Dictionary>;
    fn xpc_dictionary_create_reply(original: &Dictionary) -> Option<arc::R<Dictionary>>;
    fn xpc_dictionary_get_count(xdict: &Dictionary) -> usize;

    fn xpc_dictionary_get_value<'a>(
        xdict: &'a Dictionary,
        key: *const c_char,
    ) -> Option<&'a Object>;
    fn xpc_dictionary_set_value(xdict: &mut Dictionary, key: *const c_char, val: Option<&Object>);

    fn xpc_dictionary_set_bool(xdict: &mut Dictionary, key: *const c_char, val: bool);
    fn xpc_dictionary_get_bool(xdict: &Dictionary, key: *const c_char) -> bool;

    fn xpc_dictionary_set_int64(xdict: &mut Dictionary, key: *const c_char, val: i64);
    fn xpc_dictionary_get_int64(xdict: &Dictionary, key: *const c_char) -> i64;

    fn xpc_dictionary_set_uint64(xdict: &mut Dictionary, key: *const c_char, val: u64);
    fn xpc_dictionary_get_uint64(xdict: &Dictionary, key: *const c_char) -> u64;

    fn xpc_dictionary_set_double(xdict: &mut Dictionary, key: *const c_char, val: f64);
    fn xpc_dictionary_get_double(xdict: &Dictionary, key: *const c_char) -> f64;

    fn xpc_dictionary_set_string(xdict: &mut Dictionary, key: *const c_char, val: *const c_char);
    fn xpc_dictionary_get_string(xdict: &Dictionary, key: *const c_char) -> *const c_char;

    fn xpc_dictionary_set_data(
        xdict: &mut Dictionary,
        key: *const c_char,
        bytes: *const c_void,
        len: usize,
    );

    fn xpc_dictionary_set_fd(xdict: &mut Dictionary, key: *const c_char, fd: std::os::fd::RawFd);
    fn xpc_dictionary_dup_fd(xdict: &Dictionary, key: *const c_char) -> std::os::fd::RawFd;

    fn xpc_dictionary_set_mach_send(xdict: &mut Dictionary, key: *const c_char, port: mach::Port);
    fn xpc_dictionary_copy_mach_send(xdict: &Dictionary, key: *const c_char) -> mach::Port;

    fn xpc_dictionary_get_remote_connection<'a>(xdict: &'a Dictionary) -> Option<&'a Connection>;

    #[cfg(feature = "blocks")]
    fn xpc_dictionary_apply(xdict: &Dictionary, applier: &mut DictionaryApplier) -> bool;
}

// MARK: Errors

define_xpc_type!(
    /// An error handed to a connection's event handler.
    ///
    /// Errors are dictionaries underneath — the dictionary getters work on
    /// them — but [`Object::get_type`] reports [`Type::error`], so
    /// they never `try_cast` to [`Dictionary`].
    ///
    /// The values below are process-lifetime constants, so an event handler
    /// identifies them by pointer, as C does.
    #[doc(alias = "XPC_TYPE_ERROR")]
    pub Error, error
);

#[cfg(test)]
mod tests {
    use crate::xpc;

    #[test]
    fn bools_are_singletons() {
        let yes = xpc::Bool::yes();
        let no = xpc::Bool::no();

        assert!(yes.value());
        assert!(!no.value());

        // `XPC_BOOL_TRUE` is the symbol itself, not a pointer to it. If the
        // extern were declared `&'static Bool` this would read the isa.
        assert!(std::ptr::eq(yes, &*xpc::Bool::with_value(true)));
        assert!(std::ptr::eq(no, &*xpc::Bool::with_value(false)));
    }

    #[test]
    fn small_values_are_tagged_pointers() {
        // `xpc_int64_create` hands back a tagged pointer rather than an
        // object, and the same tagged value comes back out of a dictionary.
        // Nothing may dereference it.
        let boxed = xpc::I64::with_value(1);
        let ptr = &*boxed as *const xpc::I64 as usize;
        assert_ne!(ptr % align_of::<xpc::I64>(), 0, "expected a tagged pointer");

        let mut dict = xpc::Dictionary::new();
        dict.set_i64(c"a", 1);
        let from_dict = dict.get(c"a").unwrap();
        assert_eq!(from_dict as *const xpc::Object as usize, ptr);
        assert_eq!(from_dict.get_type(), xpc::Type::int64());
        assert_eq!(from_dict.try_cast::<xpc::I64>().unwrap().value(), 1);

        // Allocated types are ordinary pointers.
        let str = xpc::String::with_cstr(c"hello");
        assert_eq!(
            &*str as *const xpc::String as usize % align_of::<xpc::String>(),
            0
        );
    }

    #[test]
    fn types_are_descriptors_not_objects() {
        assert_eq!(xpc::Type::string(), xpc::Type::string());
        assert_ne!(xpc::Type::string(), xpc::Type::int64());
        assert_eq!(xpc::Type::string().name(), c"string");
        assert_eq!(xpc::Type::int64().name(), c"int64");

        let str = xpc::String::with_cstr(c"hello");
        assert_eq!(str.get_type(), xpc::Type::string());
    }

    #[test]
    fn try_cast_is_checked() {
        let str = xpc::String::with_cstr(c"hello");
        let obj: &xpc::Object = &str;

        assert!(obj.is::<xpc::String>());
        assert_eq!(
            obj.try_cast::<xpc::String>().unwrap().as_str(),
            Some("hello")
        );
        // C would abort here.
        assert!(obj.try_cast::<xpc::I64>().is_none());
        assert!(obj.try_cast::<xpc::Dictionary>().is_none());
    }

    #[test]
    fn values() {
        assert_eq!(xpc::I64::with_value(-7).value(), -7);
        assert_eq!(xpc::U64::with_value(7).value(), 7);
        assert_eq!(xpc::F64::with_value(0.5).value(), 0.5);
        assert_eq!(xpc::String::with_str("hi").unwrap().len(), 2);
        assert_eq!(xpc::Data::with_slice(b"abc").as_slice(), b"abc");
        assert_eq!(xpc::Uuid::with_bytes(&[7u8; 16]).bytes(), Some(&[7u8; 16]));
    }

    #[test]
    fn dictionary() {
        let mut dict = xpc::Dictionary::new();
        assert!(dict.is_empty());

        dict.set_i64(c"answer", 42);
        dict.set_cstr(c"greeting", c"hello");
        dict.set(c"nothing", Some(&xpc::Null::new()));

        assert_eq!(dict.len(), 3);
        assert_eq!(dict.i64(c"answer"), Some(42));
        assert_eq!(dict.str(c"greeting"), Some("hello"));

        // Missing and mistyped both read as zero through the C accessor...
        assert_eq!(dict.i64_raw(c"missing"), 0);
        assert_eq!(dict.i64_raw(c"greeting"), 0);
        // ...and are distinguishable through the checked one.
        assert_eq!(dict.i64(c"missing"), None);
        assert_eq!(dict.i64(c"greeting"), None);

        dict.set(c"answer", None);
        assert_eq!(dict.i64(c"answer"), None);
    }

    #[test]
    fn dictionary_apply() {
        let mut dict = xpc::Dictionary::new();
        dict.set_i64(c"a", 1);
        dict.set_i64(c"b", 2);

        let mut sum = 0;
        assert!(dict.apply(|_key, val| {
            sum += val.try_cast::<xpc::I64>().unwrap().value();
            true
        }));
        assert_eq!(sum, 3);

        // A `false` stops the walk.
        let mut seen = 0;
        assert!(!dict.apply(|_key, _val| {
            seen += 1;
            false
        }));
        assert_eq!(seen, 1);
    }

    #[test]
    fn array() {
        let mut arr = xpc::Array::new();
        assert!(arr.get(0).is_none()); // C aborts out of bounds

        arr.push(&xpc::I64::with_value(1));
        arr.push(&xpc::I64::with_value(2));
        assert_eq!(arr.len(), 2);

        let vals: Vec<i64> = arr
            .into_iter()
            .map(|v| v.try_cast::<xpc::I64>().unwrap().value())
            .collect();
        assert_eq!(vals, [1, 2]);
    }

    #[test]
    fn errors_are_dictionaries_of_their_own_type() {
        let err = xpc::Error::connection_invalid();
        assert_eq!(err.desc(), Some(c"Connection invalid"));
        assert_eq!(err.get_type(), xpc::Type::error());
        // An error never casts to a dictionary, even though it is one.
        let obj: &xpc::Object = err;
        assert!(obj.try_cast::<xpc::Dictionary>().is_none());
        assert!(matches!(xpc::Event::from(obj), xpc::Event::Err(_)));
    }

    #[test]
    fn abandoned_connection_does_not_abort() {
        // `xpc_connection_create` returns a *suspended* connection, and
        // releasing one is fatal: "Release of last reference on a suspended
        // connection." `Inactive::drop` walks it out through a legal path.
        let con = xpc::connection::Inactive::with_name(Some(c"com.example.nope"), None);
        drop(con);
    }

    #[test]
    fn connection_roundtrip() {
        let con = xpc::Connection::with_name(Some(c"com.example.nope"), None, |_| {});
        assert_eq!(con.name(), Some(c"com.example.nope"));
        con.cancel();
    }
}
