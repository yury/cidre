use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::NonNull,
};

use core::marker::PhantomData as OptionalMarker;

use super::{SwiftMetadata, abi};

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
    let value_layout = unsafe { abi::value_layout(metadata) };
    if value_layout.stride <= INLINE_CAPACITY
        && value_layout.align <= 16
        && unsafe { abi::is_bitwise_takable(metadata) }
    {
        return None;
    }

    let layout = Layout::from_size_align(value_layout.stride.max(1), value_layout.align)
        .expect("valid Swift value layout");
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

/// `Swift.Optional<T>`, whose metadata comes from the standard library's
/// generic accessor rather than a hand-written mangled name.
pub(crate) struct Optional<T: SwiftMetadata>(OptionalMarker<T>);

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
}

/// Type-erased uninitialized storage for metadata discovered from runtime
/// context rather than from a Rust marker type.
pub(crate) struct DynamicStorage {
    ptr: NonNull<u8>,
    metadata: *const abi::TypeMetadata,
}

impl DynamicStorage {
    pub(crate) unsafe fn new(metadata: *const abi::TypeMetadata) -> Self {
        assert!(!metadata.is_null(), "Swift type metadata must exist");
        let layout = unsafe { layout(metadata) };
        let ptr = NonNull::new(unsafe { alloc::alloc(layout) })
            .unwrap_or_else(|| alloc::handle_alloc_error(layout));
        Self { ptr, metadata }
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
        self.metadata
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
        unsafe { dealloc(self.ptr, self.metadata) }
    }
}

pub(crate) struct AnyValue {
    storage: DynamicStorage,
}

impl AnyValue {
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
        unsafe { abi::destroy_value(self.storage.as_mut_ptr(), self.storage.metadata) }
    }
}

#[inline]
unsafe fn dealloc(ptr: NonNull<u8>, metadata: *const abi::TypeMetadata) {
    unsafe { alloc::dealloc(ptr.as_ptr(), layout(metadata)) }
}

#[inline]
unsafe fn layout(metadata: *const abi::TypeMetadata) -> Layout {
    let value_layout = unsafe { abi::value_layout(metadata) };
    Layout::from_size_align(value_layout.stride.max(1), value_layout.align)
        .expect("valid Swift value layout")
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
