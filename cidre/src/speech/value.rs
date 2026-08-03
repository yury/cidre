use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::NonNull,
};

use crate::swift::{SwiftMetadata, abi};

/// Rust-owned uninitialized storage for a runtime-sized Swift value.
pub(super) struct Storage<T: SwiftMetadata> {
    ptr: NonNull<u8>,
    _marker: PhantomData<T>,
}

impl<T: SwiftMetadata> Storage<T> {
    pub(super) fn new() -> Self {
        let metadata = T::metadata();
        assert!(!metadata.is_null(), "Swift type metadata must exist");
        let value_layout = unsafe { abi::value_layout(metadata) };
        let layout = Layout::from_size_align(value_layout.stride.max(1), value_layout.align)
            .expect("valid Swift value layout");
        let ptr = NonNull::new(unsafe { alloc::alloc(layout) })
            .unwrap_or_else(|| alloc::handle_alloc_error(layout));

        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(super) fn as_ptr(&self) -> *const () {
        self.ptr.as_ptr().cast()
    }

    #[inline]
    pub(super) fn as_mut_ptr(&mut self) -> *mut () {
        self.ptr.as_ptr().cast()
    }

    /// Treats the bytes in this allocation as an initialized, owned `T`.
    ///
    /// # Safety
    ///
    /// The allocation must contain a valid initialized value described by
    /// `T::metadata()`.
    #[inline]
    pub(super) unsafe fn assume_init(self) -> Value<T> {
        Value { storage: self }
    }
}

impl<T: SwiftMetadata> Drop for Storage<T> {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr, T::metadata()) }
    }
}

/// Rust-owned initialized storage for a runtime-sized Swift value.
pub(super) struct Value<T: SwiftMetadata> {
    storage: Storage<T>,
}

impl<T: SwiftMetadata> Value<T> {
    pub(super) fn optional_none<P: SwiftMetadata>() -> Self {
        let mut storage = Storage::<T>::new();
        unsafe {
            abi::store_enum_tag_single_payload(storage.as_mut_ptr(), 1, 1, P::metadata());
            storage.assume_init()
        }
    }

    #[inline]
    pub(super) fn as_ptr(&self) -> *const () {
        self.storage.as_ptr()
    }

    #[inline]
    pub(super) fn as_mut_ptr(&mut self) -> *mut () {
        self.storage.as_mut_ptr()
    }

    #[inline]
    #[cfg(feature = "av")]
    pub(super) fn metadata(&self) -> *const abi::TypeMetadata {
        T::metadata()
    }

    #[inline]
    unsafe fn assume_consumed(self) {
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
pub(super) unsafe fn call_with_owned_value<T, R>(
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
pub(super) unsafe fn call_with_owned_values<A, B, R>(
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

/// Type-erased uninitialized storage for metadata discovered from runtime
/// context rather than from a Rust marker type.
pub(super) struct DynamicStorage {
    ptr: NonNull<u8>,
    metadata: *const abi::TypeMetadata,
}

impl DynamicStorage {
    pub(super) unsafe fn new(metadata: *const abi::TypeMetadata) -> Self {
        assert!(!metadata.is_null(), "Swift type metadata must exist");
        let layout = unsafe { layout(metadata) };
        let ptr = NonNull::new(unsafe { alloc::alloc(layout) })
            .unwrap_or_else(|| alloc::handle_alloc_error(layout));
        Self { ptr, metadata }
    }

    #[inline]
    pub(super) fn as_ptr(&self) -> *const () {
        self.ptr.as_ptr().cast()
    }

    #[inline]
    pub(super) fn as_mut_ptr(&mut self) -> *mut () {
        self.ptr.as_ptr().cast()
    }

    #[inline]
    pub(super) fn metadata(&self) -> *const abi::TypeMetadata {
        self.metadata
    }

    /// Treats the bytes in this allocation as an initialized Swift value.
    ///
    /// # Safety
    ///
    /// The allocation must contain a valid initialized value described by its
    /// runtime metadata.
    #[inline]
    pub(super) unsafe fn assume_init(self) -> AnyValue {
        AnyValue { storage: self }
    }
}

impl Drop for DynamicStorage {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr, self.metadata) }
    }
}

pub(super) struct AnyValue {
    storage: DynamicStorage,
}

impl AnyValue {
    #[inline]
    pub(super) fn as_mut_ptr(&mut self) -> *mut () {
        self.storage.as_mut_ptr()
    }

    #[inline]
    pub(super) fn metadata(&self) -> *const abi::TypeMetadata {
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

    #[test]
    fn typed_storage_carries_only_its_allocation_pointer() {
        assert_eq!(
            core::mem::size_of::<Storage<isize>>(),
            core::mem::size_of::<NonNull<u8>>()
        );
        assert_eq!(
            core::mem::size_of::<Value<isize>>(),
            core::mem::size_of::<NonNull<u8>>()
        );
    }
}
