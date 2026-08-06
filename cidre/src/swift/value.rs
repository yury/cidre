//! Rust-owned storage for Swift values whose layout is only known at runtime.
//!
//! This is interop plumbing rather than API: which pieces are reachable depends
//! on which framework bindings are compiled, so an unused one here is expected
//! rather than a sign of dead code.
#![allow(dead_code)]

use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::NonNull,
};

use core::marker::PhantomData as OptionalMarker;

use super::{FromSwift, SwiftMetadata, abi};

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

    /// Treats the bytes in this allocation as an initialized, owned `T`.
    ///
    /// # Safety
    ///
    /// The allocation must contain a valid initialized value described by
    /// `T::metadata()`.
    #[inline]
    pub(crate) unsafe fn assume_init(self) -> Value<T> {
        Value { storage: self }
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

/// Rust-owned initialized storage for a runtime-sized Swift value.
pub(crate) struct Value<T: SwiftMetadata> {
    storage: Storage<T>,
}

impl<T: SwiftMetadata> Value<T> {
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
        T::metadata()
    }

    /// Deallocates the storage without destroying the value, for when
    /// ownership of the value has already been handed elsewhere.
    ///
    /// # Safety
    ///
    /// The caller must have taken ownership of the value.
    #[inline]
    pub(crate) unsafe fn assume_consumed(self) {
        let this = ManuallyDrop::new(self);
        unsafe { drop(core::ptr::read(&this.storage)) };
    }

    /// Erases the marker type while preserving ownership of the Swift value.
    pub(crate) fn erase(mut self) -> AnyValue {
        unsafe {
            let metadata = T::metadata();
            let mut storage = DynamicStorage::new(metadata);
            abi::initialize_with_take(storage.as_mut_ptr(), self.as_mut_ptr(), metadata);
            self.assume_consumed();
            storage.assume_init()
        }
    }
}

/// Copies through the type's `initializeWithCopy` witness, which is what
/// retains whatever the value owns.
///
/// Without this the wrappers around a `Value` cannot be cloned at all, since
/// none of them can copy runtime-laid-out bytes by hand.
impl<T: SwiftMetadata> Clone for Value<T> {
    fn clone(&self) -> Self {
        unsafe {
            let metadata = T::metadata();
            let mut storage = Storage::<T>::new();
            abi::initialize_with_copy(storage.as_mut_ptr(), self.as_ptr(), metadata);
            storage.assume_init()
        }
    }
}

impl<T: SwiftMetadata> Drop for Value<T> {
    fn drop(&mut self) {
        unsafe { abi::destroy_value(self.storage.as_mut_ptr(), T::metadata()) }
    }
}

/// Calls Swift with an `@owned` indirect argument and then deallocates the
/// argument storage without destroying the value Swift consumed.
///
/// # Safety
///
/// `call` must consume the value at the supplied pointer and must not unwind.
#[inline]
pub(crate) unsafe fn call_with_owned_value<T, R>(
    mut value: Value<T>,
    call: impl FnOnce(*mut ()) -> R,
) -> R
where
    T: SwiftMetadata,
{
    let result = call(value.as_mut_ptr());
    unsafe { value.assume_consumed() };
    result
}

/// Calls Swift with two `@owned` indirect arguments and then deallocates their
/// storage without destroying the values Swift consumed.
///
/// # Safety
///
/// `call` must consume both values at the supplied pointers and must not
/// unwind.
#[inline]
pub(crate) unsafe fn call_with_owned_values<A, B, R>(
    mut first: Value<A>,
    mut second: Value<B>,
    call: impl FnOnce(*mut (), *mut ()) -> R,
) -> R
where
    A: SwiftMetadata,
    B: SwiftMetadata,
{
    let result = call(first.as_mut_ptr(), second.as_mut_ptr());
    unsafe {
        first.assume_consumed();
        second.assume_consumed();
    }
    result
}

/// A wrapper over a Swift value held in storage the wrapper owns.
///
/// A call returning one of these gets the address of that storage in the
/// indirect-result register and hands the wrapper back what Swift wrote, so a
/// generated call needs to name the storage and take it without knowing which
/// wrapper it has.
pub(crate) trait SwiftValue: Sized {
    /// The marker naming the Swift type, which is what sizes the storage.
    type Marker: SwiftMetadata;

    /// # Safety
    ///
    /// `storage` must hold an initialized value of this type.
    unsafe fn from_storage(storage: Storage<Self::Marker>) -> Self;

    fn from_value(value: Value<Self::Marker>) -> Self;
}

/// A wrapper a Swift `Optional` result can be read back into.
///
/// The two flavours of wrapper differ in what they hold — the payload, or the
/// optional itself — so the storage a call allocates is described here and the
/// wrapper decides what to make of it.
pub(crate) trait SwiftOptionalValue: Sized {
    /// The marker naming the payload's Swift type.
    type Marker: SwiftMetadata;

    /// # Safety
    ///
    /// `storage` must hold an initialized `Optional<Self::Marker>`.
    unsafe fn from_optional_storage(storage: Storage<Optional<Self::Marker>>) -> Option<Self>;
}

/// Defines a wrapper over a Swift value type whose layout is only known at
/// runtime, together with the marker naming it.
///
/// Every one of these needs the same pieces: a marker, a newtype holding a
/// [`Value`], the pointer accessors the ABI shims take, and a `Clone` that goes
/// through the copy witness. The `optional` forms are for getters that return
/// `Payload?`, where the wrapper holds the optional itself — an optional's
/// payload starts at offset 0, so the same storage doubles as the unwrapped
/// value once the tag says `.some`.
macro_rules! define_swift_value {
    ($(#[$meta:meta])* $vis:vis $ty:ident, $marker:ident = accessor $accessor:expr) => {
        $crate::define_swift_marker!(pub(crate) $marker = accessor $accessor);
        $crate::swift::value::define_swift_value!(@wrap $(#[$meta])* $vis $ty, $marker);
        $crate::swift::value::define_swift_value!(@read $ty, $marker);
        $crate::swift::value::define_swift_value!(@value $ty, $marker);
    };
    ($(#[$meta:meta])* $vis:vis $ty:ident, $marker:ident = mangled $name:literal) => {
        $crate::define_swift_marker!(pub(crate) $marker = mangled $name);
        $crate::swift::value::define_swift_value!(@wrap $(#[$meta])* $vis $ty, $marker);
        $crate::swift::value::define_swift_value!(@read $ty, $marker);
        $crate::swift::value::define_swift_value!(@value $ty, $marker);
    };
    // The wrapper's own storage, named so a generated call can allocate the
    // result buffer and take what Swift wrote into it without either side
    // spelling out the marker type.
    (@value $ty:ident, $marker:ident) => {
        impl $crate::swift::value::SwiftValue for $ty {
            type Marker = $marker;

            #[inline]
            unsafe fn from_storage(
                storage: $crate::swift::value::Storage<Self::Marker>,
            ) -> Self {
                Self(unsafe { storage.assume_init() })
            }

            #[inline]
            fn from_value(value: $crate::swift::value::Value<Self::Marker>) -> Self {
                Self(value)
            }
        }

        /// The wrapper holds the payload, so an optional result is unwrapped
        /// into it and the `.none` case is destroyed with the optional.
        impl $crate::swift::value::SwiftOptionalValue for $ty {
            type Marker = $marker;

            #[inline]
            unsafe fn from_optional_storage(
                storage: $crate::swift::value::Storage<
                    $crate::swift::value::Optional<Self::Marker>,
                >,
            ) -> Option<Self> {
                unsafe { storage.assume_init().take_value().map(Self::from_value) }
            }
        }

        /// A value type is passed by address, which is what the wrapper holds.
        unsafe impl $crate::swift::SwiftSelf for $ty {
            #[inline]
            fn swift_self_ptr(&self) -> *const () {
                self.0.as_ptr()
            }
        }

        /// Its layout is only known at runtime, so Swift returns it into
        /// storage the caller provides.
        unsafe impl $crate::swift::SwiftAbi for $ty {
            const CLASS: $crate::swift::AbiClass = $crate::swift::AbiClass::Indirect;
        }
    };
    // Only the unwrapped forms get this: an `Optional<Payload>` wrapper names
    // the payload's type but does not have its layout, so it can be neither
    // read as one nor written where one is expected.
    (@read $ty:ident, $marker:ident) => {
        /// The wrapper stands for the Swift type its marker names, which is
        /// what lets it be an element, a key, or an argument like any other.
        unsafe impl $crate::swift::SwiftMetadata for $ty {
            #[inline]
            fn metadata() -> *const $crate::swift::abi::TypeMetadata {
                <$marker as $crate::swift::SwiftMetadata>::metadata()
            }
        }

        unsafe impl $crate::swift::FromSwift for $ty {
            #[inline]
            unsafe fn copy_swift(value: *const ()) -> Self {
                unsafe {
                    let mut storage = Self::storage();
                    $crate::swift::abi::initialize_with_copy(
                        storage.as_mut_ptr(),
                        value,
                        <$marker as $crate::swift::SwiftMetadata>::metadata(),
                    );
                    Self::from_storage(storage)
                }
            }

            /// Moving the bytes out is what the wrapper's own storage is for,
            /// so an owned value never goes through a copy and a destroy.
            #[inline]
            unsafe fn take_swift(value: *mut ()) -> Self {
                unsafe {
                    let mut storage = Self::storage();
                    $crate::swift::abi::initialize_with_take(
                        storage.as_mut_ptr(),
                        value,
                        <$marker as $crate::swift::SwiftMetadata>::metadata(),
                    );
                    Self::from_storage(storage)
                }
            }
        }

        unsafe impl $crate::swift::ToSwift for $ty {
            #[inline]
            unsafe fn copy_to_swift(&self, dst: *mut ()) {
                unsafe {
                    $crate::swift::abi::initialize_with_copy(
                        dst,
                        self.as_ptr(),
                        <$marker as $crate::swift::SwiftMetadata>::metadata(),
                    )
                };
            }
        }
    };
    ($(#[$meta:meta])* $vis:vis $ty:ident, $marker:ident = optional accessor $accessor:expr) => {
        $crate::define_swift_marker!(pub(crate) $marker = accessor $accessor);
        $crate::swift::value::define_swift_value!(@wrap $(#[$meta])* $vis $ty, $crate::swift::value::Optional<$marker>);
        $crate::swift::value::define_swift_value!(@optional $ty, $marker);
    };
    ($(#[$meta:meta])* $vis:vis $ty:ident, $marker:ident = optional mangled $name:literal) => {
        $crate::define_swift_marker!(pub(crate) $marker = mangled $name);
        $crate::swift::value::define_swift_value!(@wrap $(#[$meta])* $vis $ty, $crate::swift::value::Optional<$marker>);
        $crate::swift::value::define_swift_value!(@optional $ty, $marker);
    };
    (@optional $ty:ident, $marker:ident) => {
        /// The wrapper holds the optional itself, so the storage becomes the
        /// wrapper unchanged once the tag says `.some`.
        impl $crate::swift::value::SwiftOptionalValue for $ty {
            type Marker = $marker;

            #[inline]
            unsafe fn from_optional_storage(
                storage: $crate::swift::value::Storage<
                    $crate::swift::value::Optional<Self::Marker>,
                >,
            ) -> Option<Self> {
                let value = unsafe { storage.assume_init() };
                value.is_some().then(|| Self(value))
            }
        }

        /// A value type is passed by address, which is what the wrapper holds.
        unsafe impl $crate::swift::SwiftSelf for $ty {
            #[inline]
            fn swift_self_ptr(&self) -> *const () {
                self.0.as_ptr()
            }
        }

        unsafe impl $crate::swift::SwiftAbi for $ty {
            const CLASS: $crate::swift::AbiClass = $crate::swift::AbiClass::Indirect;
        }

        impl $ty {
            /// Reads what Swift wrote into `storage`, or `None` when the tag
            /// says the getter had nothing to return.
            #[allow(dead_code)]
            #[inline]
            pub(crate) unsafe fn from_optional_storage(
                storage: $crate::swift::value::Storage<$crate::swift::value::Optional<$marker>>,
            ) -> Option<Self> {
                let value = unsafe { storage.assume_init() };
                value.is_some().then(|| Self(value))
            }
        }
    };
    (@wrap $(#[$meta:meta])* $vis:vis $ty:ident, $value:ty) => {
        $(#[$meta])*
        #[derive(Clone)]
        $vis struct $ty($crate::swift::value::Value<$value>);

        #[allow(dead_code)]
        impl $ty {
            /// Uninitialized storage for a getter to write into.
            #[inline]
            pub(crate) fn storage() -> $crate::swift::value::Storage<$value> {
                $crate::swift::value::Storage::new()
            }

            /// Takes what Swift wrote into `storage`.
            ///
            /// # Safety
            ///
            /// `storage` must hold an initialized value of this type.
            #[inline]
            pub(crate) unsafe fn from_storage(
                storage: $crate::swift::value::Storage<$value>,
            ) -> Self {
                Self(unsafe { storage.assume_init() })
            }

            #[inline]
            pub(crate) fn from_value(value: $crate::swift::value::Value<$value>) -> Self {
                Self(value)
            }

            #[inline]
            pub(crate) fn as_ptr(&self) -> *const () {
                self.0.as_ptr()
            }

            #[inline]
            pub(crate) fn as_mut_ptr(&mut self) -> *mut () {
                self.0.as_mut_ptr()
            }

            /// Deliberately not `value`: a wrapper's own API often has a
            /// Swift property by that name.
            #[inline]
            pub(crate) fn swift_value(&self) -> &$crate::swift::value::Value<$value> {
                &self.0
            }

            /// Surrenders the Swift value, for handing to something that
            /// consumes it.
            #[inline]
            pub(crate) fn into_value(self) -> $crate::swift::value::Value<$value> {
                self.0
            }
        }
    };
}

pub(crate) use define_swift_value;

/// `Swift.Optional<T>`, whose metadata comes from the standard library's
/// generic accessor rather than a hand-written mangled name.
pub(crate) struct Optional<T: SwiftMetadata>(OptionalMarker<T>);

/// An optional is as sendable as what it wraps.
unsafe impl<T: super::SwiftSendable> super::SwiftSendable for Optional<T> {}

unsafe impl<T: SwiftMetadata> SwiftMetadata for Optional<T> {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        let wrapped = T::metadata();
        assert!(!wrapped.is_null(), "Swift type metadata must exist");
        unsafe { abi::optional_metadata(wrapped) }
    }
}

impl<T: SwiftMetadata> Value<Optional<T>> {
    /// Builds `Optional<T>.none`.
    pub(crate) fn none() -> Self {
        let mut storage = Storage::<Optional<T>>::new();
        unsafe {
            abi::store_enum_tag_single_payload(storage.as_mut_ptr(), 1, 1, T::metadata());
            storage.assume_init()
        }
    }

    /// Returns whether the value is `.some`, reading the tag through `T`'s
    /// witnesses.
    #[inline]
    pub(crate) fn is_some(&self) -> bool {
        unsafe { abi::get_enum_tag_single_payload(self.as_ptr(), 1, T::metadata()) == 0 }
    }

    /// Decodes the payload, or `None` when the optional is `.none`.
    ///
    /// A single-payload enum lays its payload out at offset zero, so `.some`
    /// storage already holds the wrapped value; taking it leaves the optional
    /// with nothing to destroy. A `.none` is destroyed through its witness like
    /// any other value.
    pub(crate) fn take(mut self) -> Option<T>
    where
        T: FromSwift,
    {
        if !self.is_some() {
            return None;
        }
        unsafe {
            let value = T::take_swift(self.as_mut_ptr());
            self.assume_consumed();
            Some(value)
        }
    }

    /// Moves the payload into storage of its own, or `None` when the optional
    /// is `.none`.
    ///
    /// The counterpart of [`take`](Self::take) for a payload that stays a Swift
    /// value instead of decoding into a Rust one. Moving rather than copying is
    /// what keeps this off the type's copy witness, which would retain whatever
    /// the payload owns only for the optional to release it again.
    pub(crate) fn take_value(mut self) -> Option<Value<T>> {
        if !self.is_some() {
            return None;
        }
        unsafe {
            let mut storage = Storage::<T>::new();
            abi::initialize_with_take(storage.as_mut_ptr(), self.as_mut_ptr(), T::metadata());
            self.assume_consumed();
            Some(storage.assume_init())
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
