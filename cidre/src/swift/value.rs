//! Rust-owned storage for Swift values whose layout is only known at runtime.
//!
//! This is interop plumbing rather than API: which pieces are reachable depends
//! on which framework bindings are compiled, so an unused one here is expected
//! rather than a sign of dead code.
#![allow(dead_code)]

use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    ptr::NonNull,
};

use core::marker::PhantomData as OptionalMarker;

use super::{FromSwift, SwiftMetadata, SwiftOptional, abi};

/// How many bytes a [`Storage`] keeps inline before falling back to the heap.
///
/// Sized so the Swift values these bindings pass around most — locales,
/// presets, options, analysis results — stay off the heap entirely.
const INLINE_CAPACITY: usize = 96;

/// Swift caps the alignment it asks of a value at 16, which
/// [`abi::value_layout`] already reflects.
#[repr(C, align(16))]
struct Inline([core::mem::MaybeUninit<u8>; INLINE_CAPACITY]);

/// Rust-owned uninitialized storage for a runtime-sized Swift value.
///
/// Small values live in the struct itself, which is what Swift's own callers
/// get from a dynamic stack allocation. The address is derived on demand rather
/// than stored, so moving a `Storage` stays sound, and the inline path is taken
/// only for values the runtime says may be relocated by copying their bytes.
pub(crate) struct Storage<T: SwiftMetadata> {
    inline: Inline,
    heap: Option<NonNull<u8>>,
    _marker: PhantomData<T>,
}

impl<T: SwiftMetadata> Storage<T> {
    pub(crate) fn new() -> Self {
        let metadata = T::metadata();
        assert!(!metadata.is_null(), "Swift type metadata must exist");

        let heap = unsafe { allocate_if_large(metadata) };
        Self {
            inline: Inline([core::mem::MaybeUninit::uninit(); INLINE_CAPACITY]),
            heap,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const () {
        match self.heap {
            Some(ptr) => ptr.as_ptr().cast(),
            None => self.inline.0.as_ptr().cast(),
        }
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut () {
        match self.heap {
            Some(ptr) => ptr.as_ptr().cast(),
            None => self.inline.0.as_mut_ptr().cast(),
        }
    }
}

impl<T: SwiftMetadata> Storage<T> {
    /// Destroys the value this storage holds, through its own witness.
    ///
    /// A `Storage` only owns the allocation, so a scratch buffer that was
    /// filled with a value has to say when the value goes. Handing the buffer
    /// to a callee that consumes it means *not* calling this.
    ///
    /// # Safety
    ///
    /// The storage must hold an initialized value, and must not be used again.
    #[inline]
    pub(crate) unsafe fn destroy(&mut self) {
        unsafe { abi::destroy_value(self.as_mut_ptr(), T::metadata()) }
    }
}

/// A `Sendable` Swift value is safe to move and to share, and the storage is
/// only a private allocation holding one, so it inherits both.
unsafe impl<T: super::SwiftSendable> Send for Storage<T> {}
unsafe impl<T: super::SwiftSendable> Sync for Storage<T> {}

impl<T: super::SwiftClass> Storage<crate::arc::R<T>> {
    /// Moves a class reference into storage for the class's Swift value.
    ///
    /// The reference is the value, so this is a one-word write rather than a
    /// call through the runtime.
    pub(crate) fn from_class_ref(value: crate::arc::R<T>) -> Self {
        let mut storage = Self::new();
        unsafe {
            storage
                .as_mut_ptr()
                .cast::<*mut T>()
                .write(value.into_raw())
        };
        storage
    }
}

impl<T: SwiftMetadata> Drop for Storage<T> {
    fn drop(&mut self) {
        if let Some(ptr) = self.heap {
            unsafe { dealloc(ptr, T::metadata()) }
        }
    }
}

/// Allocates only when a value cannot live inside a [`Storage`].
///
/// # Safety
///
/// `metadata` must be non-null.
unsafe fn allocate_if_large(metadata: *const abi::TypeMetadata) -> Option<NonNull<u8>> {
    // One witness table load answers both questions.
    let witnesses = unsafe { abi::ValueWitnesses::new(metadata) };
    let value_layout = witnesses.layout();
    if value_layout.stride <= INLINE_CAPACITY
        && value_layout.align <= 16
        && witnesses.is_bitwise_takable()
    {
        return None;
    }

    let layout = rust_layout(value_layout);
    Some(
        NonNull::new(unsafe { alloc::alloc(layout) })
            .unwrap_or_else(|| alloc::handle_alloc_error(layout)),
    )
}

/// Calls Swift with an `@owned` indirect argument and then deallocates the
/// argument storage without destroying the value Swift consumed.
///
/// # Safety
///
/// `call` must consume the value at the supplied pointer and must not unwind.
#[inline]
pub(crate) unsafe fn call_with_owned_value<T, R>(
    mut value: Storage<T>,
    call: impl FnOnce(*mut ()) -> R,
) -> R
where
    T: SwiftMetadata,
{
    // The storage is only an allocation; the value inside it is Swift's now, so
    // nothing here destroys it.
    call(value.as_mut_ptr())
}

/// Calls Swift with two `@owned` indirect arguments and then deallocates their
/// storage without destroying the values Swift consumed.
///
/// `Swift.Optional<T>`, whose metadata comes from the standard library's
/// generic accessor rather than a hand-written mangled name.
pub(crate) struct Optional<T: SwiftOptional>(OptionalMarker<T>);

/// An optional is as sendable as what it wraps.
unsafe impl<T: SwiftOptional + super::SwiftSendable> super::SwiftSendable for Optional<T> {}

unsafe impl<T: SwiftOptional> SwiftMetadata for Optional<T> {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        <T as SwiftOptional>::optional_metadata()
    }
}

/// A wrapper a Swift `Optional` result can be read back into.
pub(crate) trait SwiftOptionalValue: Sized {
    /// The marker naming the payload's Swift type.
    type Marker: SwiftOptional;

    /// # Safety
    ///
    /// `storage` must hold an initialized `Optional<Self::Marker>`.
    unsafe fn from_optional_storage(storage: Storage<Optional<Self::Marker>>) -> Option<Self>;
}

/// Where a call writes a value returned through the indirect-result register.
///
/// A wrapper whose size is declared is its own buffer, so the call writes
/// straight into it; a sequence, whose layout is only known at runtime, needs
/// storage the runtime sizes. Both are inlined away, so this costs nothing
/// beyond naming which of the two a type has.
pub(crate) trait SwiftOut: Sized {
    type Buf;

    fn out_buf() -> Self::Buf;
    fn out_ptr(buf: &mut Self::Buf) -> *mut ();

    /// # Safety
    ///
    /// The buffer must hold an initialized value of this type.
    unsafe fn out_take(buf: Self::Buf) -> Self;
}

/// Asserts, once, that a declaration really matches what Swift lays out.
///
/// Called from the metadata cache's initializer, so it runs on first use and
/// never again. A declaration that is the wrong size, too weakly aligned, or
/// wrong about triviality fails here rather than corrupting memory later.
#[track_caller]
pub(crate) fn check_declared_layout(
    name: &str,
    metadata: *const abi::TypeMetadata,
    size: usize,
    align: usize,
    trivial: bool,
) {
    let witnesses = unsafe { abi::ValueWitnesses::new(metadata) };
    let layout = witnesses.layout();
    assert_eq!(
        layout.stride, size,
        "{name}: declared {size} bytes, Swift strides {}",
        layout.stride
    );
    assert!(
        layout.align <= align,
        "{name}: declared align {align}, Swift wants {}",
        layout.align
    );
    // A Rust move is a `memcpy` with no hook, so a value that cannot be
    // relocated that way must not be held inline whatever its size.
    assert!(
        witnesses.is_bitwise_takable(),
        "{name}: not bitwise-takable, so it cannot be held inline"
    );
    assert_eq!(
        trivial,
        unsafe { abi::is_pod(metadata) },
        "{name}: declared trivial = {trivial}, Swift disagrees"
    );
}

/// A `T?` written into scratch storage, which is what a Swift getter returning
/// an optional hands back.
impl<T: SwiftOptional> Storage<Optional<T>> {
    /// Builds `Optional<T>.none`.
    pub(crate) fn none() -> Self {
        let mut storage = Self::new();
        unsafe { abi::store_enum_tag_single_payload(storage.as_mut_ptr(), 1, 1, T::metadata()) };
        storage
    }

    /// Whether the value is `.some`, reading the tag through `T`'s witnesses.
    #[inline]
    pub(crate) fn is_some(&self) -> bool {
        unsafe { abi::get_enum_tag_single_payload(self.as_ptr(), 1, T::metadata()) == 0 }
    }

    /// Decodes the payload, or `None` when the optional is `.none`.
    ///
    /// A single-payload enum lays its payload out at offset zero, so `.some`
    /// storage already holds the wrapped value and moving its bytes out is the
    /// whole of taking it. A `.none` has nothing at offset zero, so it is
    /// destroyed through its witness instead.
    pub(crate) fn take(mut self) -> Option<T>
    where
        T: FromSwift,
    {
        unsafe {
            if !self.is_some() {
                abi::destroy_value(
                    self.as_mut_ptr(),
                    <Optional<T> as SwiftMetadata>::metadata(),
                );
                return None;
            }
            Some(T::take_swift(self.as_mut_ptr()))
        }
    }
}

/// Type-erased uninitialized storage for metadata discovered from runtime
/// context rather than from a Rust marker type.
pub(crate) struct DynamicStorage {
    ptr: NonNull<u8>,
    /// Held rather than just the metadata, so destroying and deallocating the
    /// value need no further runtime lookups.
    witnesses: abi::ValueWitnesses,
}

impl DynamicStorage {
    pub(crate) unsafe fn new(metadata: *const abi::TypeMetadata) -> Self {
        let witnesses = unsafe { abi::ValueWitnesses::new(metadata) };
        let layout = rust_layout(witnesses.layout());
        let ptr = NonNull::new(unsafe { alloc::alloc(layout) })
            .unwrap_or_else(|| alloc::handle_alloc_error(layout));
        Self { ptr, witnesses }
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const () {
        self.ptr.as_ptr().cast()
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut () {
        self.ptr.as_ptr().cast()
    }

    #[inline]
    pub(crate) fn metadata(&self) -> *const abi::TypeMetadata {
        self.witnesses.metadata()
    }

    /// Treats the bytes in this allocation as an initialized Swift value.
    ///
    /// # Safety
    ///
    /// The allocation must contain a valid initialized value described by its
    /// runtime metadata.
    #[inline]
    pub(crate) unsafe fn assume_init(self) -> AnyValue {
        AnyValue { storage: self }
    }
}

impl Drop for DynamicStorage {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.ptr.as_ptr(), rust_layout(self.witnesses.layout())) }
    }
}

pub(crate) struct AnyValue {
    storage: DynamicStorage,
}

impl AnyValue {
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const () {
        self.storage.as_ptr()
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut () {
        self.storage.as_mut_ptr()
    }

    #[inline]
    pub(crate) fn metadata(&self) -> *const abi::TypeMetadata {
        self.storage.metadata()
    }
}

impl Drop for AnyValue {
    fn drop(&mut self) {
        let witnesses = self.storage.witnesses;
        unsafe { witnesses.destroy(self.storage.as_mut_ptr()) }
    }
}

#[inline]
unsafe fn dealloc(ptr: NonNull<u8>, metadata: *const abi::TypeMetadata) {
    unsafe { alloc::dealloc(ptr.as_ptr(), rust_layout(abi::value_layout(metadata))) }
}

/// The Rust allocation that backs a Swift value of this layout.
#[inline]
fn rust_layout(value: abi::ValueLayout) -> Layout {
    Layout::from_size_align(value.stride.max(1), value.align).expect("valid Swift value layout")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_wrappers_and_auto_traits() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<crate::swift::foundation::Date>();
        assert_sync::<crate::swift::foundation::Date>();
        println!("Date is Send + Sync purely by inference");
    }

    /// The cached `T?` must be exactly what an uncached instantiation resolves,
    /// and two payloads must not share a cache — a `static` in a generic
    /// function would have given every type the first one's answer, which is
    /// silent memory corruption rather than a failure.
    #[test]
    fn each_optional_caches_its_own_metadata() {
        use crate::swift::{String, SwiftOptional};

        let int = unsafe { abi::optional_metadata(<isize as SwiftMetadata>::metadata()) };
        let string = unsafe { abi::optional_metadata(<String as SwiftMetadata>::metadata()) };
        assert!(!int.is_null() && !string.is_null(), "`T?` must resolve");
        assert_ne!(int, string, "different payloads are different types");

        // The first call fills each cache; the second must read it back.
        for _ in 0..2 {
            assert_eq!(int, <isize as SwiftOptional>::optional_metadata());
            assert_eq!(int, <Optional<isize> as SwiftMetadata>::metadata());
            assert_eq!(string, <String as SwiftOptional>::optional_metadata());
            assert_eq!(string, <Optional<String> as SwiftMetadata>::metadata());
        }
    }

    /// Small values must stay off the heap, and the address must survive a
    /// move, since that is what makes the inline buffer sound.
    #[test]
    fn small_values_live_inline_and_survive_a_move() {
        let mut storage = Storage::<isize>::new();
        assert!(storage.heap.is_none(), "isize must fit inline");

        unsafe { storage.as_mut_ptr().cast::<isize>().write(0x5EED) };
        let moved = storage;
        assert_eq!(0x5EED, unsafe { moved.as_ptr().cast::<isize>().read() });
    }
}

/// Declares the Rust type that *is* a Swift value.
///
/// A Swift value type has a size, an alignment, and witnesses that copy and
/// destroy it. Given the first two, the Rust type can be the value: a byte
/// array of exactly its stride, held inline, with no allocation and no
/// indirection. What Swift is still needed for — copying and destroying a value
/// that owns something — stays behind the witnesses, and a value Swift treats
/// as plain bytes needs neither, so it is `Copy` and costs nothing to hold.
///
/// The declaration is checked against Swift's own layout the first time the
/// type is used. It states the stride exactly rather than a budget, which is
/// what lets a `&[T]` be a run of Swift values.
///
/// Written through [`define_swift!`](crate::define_swift):
///
/// ```ignore
/// define_swift!(#[swift::struct("Foundation.Date", size(8), align(8), trivial, sendable)] pub Date);
/// define_swift!(#[swift::struct("Foundation.Locale", size(16), align(8), sendable)] pub Locale);
/// ```
#[macro_export]
macro_rules! swift_value {
    (
        @build_mangled $(#[$meta:meta])*
        $vis:vis $ty:ident = $name:literal,
        $size:literal, $align:literal, $trivial:literal
    ) => {
        $crate::swift_value!(
            @common $(#[$meta])* $vis $ty = $name, $size, $align, $trivial,
            unsafe { $crate::swift::abi::type_by_mangled_name($name) }
        );
    };
    (
        @build $(#[$meta:meta])*
        $vis:vis $ty:ident = $name:literal,
        $size:literal, $align:literal, $trivial:literal
    ) => {
        $crate::swift_value!(
            @common $(#[$meta])* $vis $ty = $name, $size, $align, $trivial,
            unsafe {
                $crate::swift::abi::call_metadata_accessor(
                    $crate::swift::metadata_accessor!(struct, $name),
                )
            }
        );
    };
    (
        @common $(#[$meta:meta])*
        $vis:vis $ty:ident = $name:literal,
        $size:literal, $align:literal, $trivial:literal, $resolve:expr
    ) => {
        #[doc = concat!("Swift `", $name, "`.")]
        ///
        $(#[$meta])*
        #[doc(alias = $name)]
        #[repr(C, align($align))]
        $vis struct $ty {
            bytes: [::core::mem::MaybeUninit<u8>; $size],
            /// A Swift value is only as shareable as Swift says it is, and a
            /// byte array would otherwise inherit `Send` and `Sync` from Rust
            /// for free.
            _not_shared: ::core::marker::PhantomData<*const ()>,
        }

        impl $ty {
            #[inline]
            pub(crate) fn as_ptr(&self) -> *const () {
                self.bytes.as_ptr().cast()
            }

        }

        /// A `T?` result is written into storage for the optional, and the
        /// payload moves out of it when the tag says there is one.
        impl $crate::swift::value::SwiftOptionalValue for $ty {
            type Marker = Self;

            #[inline]
            unsafe fn from_optional_storage(
                storage: $crate::swift::value::Storage<
                    $crate::swift::value::Optional<Self>,
                >,
            ) -> Option<Self> {
                storage.take()
            }
        }

        /// The value is the wrapper, so a call writes straight into it — no
        /// separate buffer and nothing to move afterwards.
        impl $crate::swift::value::SwiftOut for $ty {
            type Buf = ::core::mem::MaybeUninit<Self>;

            #[inline]
            fn out_buf() -> Self::Buf {
                ::core::mem::MaybeUninit::uninit()
            }

            #[inline]
            fn out_ptr(buf: &mut Self::Buf) -> *mut () {
                buf.as_mut_ptr().cast()
            }

            #[inline]
            unsafe fn out_take(buf: Self::Buf) -> Self {
                unsafe { buf.assume_init() }
            }
        }

        unsafe impl $crate::swift::SwiftMetadata for $ty {
            #[inline]
            fn metadata() -> *const $crate::swift::abi::TypeMetadata {
                static CACHE: $crate::swift::abi::MetadataCache =
                    $crate::swift::abi::MetadataCache::new();
                CACHE.get(|| {
                    let metadata = $resolve;
                    $crate::swift::value::check_declared_layout(
                        stringify!($ty), metadata, $size, $align, $trivial,
                    );
                    metadata
                })
            }
        }

        /// The Rust type has the Swift value's exact layout and ownership.
        unsafe impl $crate::swift::SwiftType for $ty {}

        $crate::impl_swift_optional!($ty);

        unsafe impl $crate::swift::SwiftSelf for $ty {
            #[inline]
            fn swift_self_ptr(&self) -> *const () {
                self.as_ptr()
            }
        }

        unsafe impl $crate::swift::SwiftAbi for $ty {
            const CLASS: $crate::swift::AbiClass = $crate::swift::AbiClass::Indirect;
        }

        unsafe impl $crate::swift::FromSwift for $ty {
            #[inline]
            unsafe fn copy_swift(value: *const ()) -> Self {
                unsafe {
                    let mut copy = ::core::mem::MaybeUninit::<Self>::uninit();
                    $crate::swift::abi::initialize_with_copy(
                        copy.as_mut_ptr().cast(),
                        value,
                        <Self as $crate::swift::SwiftMetadata>::metadata(),
                    );
                    copy.assume_init()
                }
            }

            /// The layout check proves the value is bitwise-takable, so taking
            /// it is a move of the bytes.
            #[inline]
            unsafe fn take_swift(value: *mut ()) -> Self {
                unsafe { value.cast::<Self>().read() }
            }
        }

        unsafe impl $crate::swift::ToSwift for $ty {
            /// The declared stride is Swift's own, so a slice is already a run
            /// of Swift values.
            const IS_CONTIGUOUS: bool = true;

            #[inline]
            fn as_swift_ptr(&self) -> Option<*const ()> {
                Some(self.as_ptr())
            }

            #[inline]
            unsafe fn copy_to_swift(&self, dst: *mut ()) {
                unsafe {
                    $crate::swift::abi::initialize_with_copy(
                        dst,
                        self.as_ptr(),
                        <Self as $crate::swift::SwiftMetadata>::metadata(),
                    );
                }
            }
        }

    };

    // Swift copies and destroys this by moving bytes, so Rust may too.
    (@trivial $ty:ident) => {
        impl Clone for $ty {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl Copy for $ty {}
    };
    // It owns something, so both go through its own witnesses.
    (@owned $ty:ident) => {
        impl Clone for $ty {
            #[inline]
            fn clone(&self) -> Self {
                unsafe { <Self as $crate::swift::FromSwift>::copy_swift(self.as_ptr()) }
            }
        }

        impl Drop for $ty {
            #[inline]
            fn drop(&mut self) {
                unsafe {
                    $crate::swift::abi::destroy_value(
                        self.bytes.as_mut_ptr().cast(),
                        <Self as $crate::swift::SwiftMetadata>::metadata(),
                    )
                }
            }
        }
    };

    // Swift says its values may cross threads, which a byte array cannot say
    // for itself — Rust would hand out both auto traits regardless.
    (@shared $ty:ident) => {
        unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {}
    };
}
