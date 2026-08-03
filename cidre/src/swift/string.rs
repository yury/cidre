use core::{ffi::c_char, fmt};
use std::ffi::CStr;

use super::{SwiftMetadata, SwiftType, abi};

/// Raw two-word Swift `String` ABI value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct RawString {
    pub word0: usize,
    pub word1: usize,
}

/// A retained Swift `String` value.
#[repr(transparent)]
pub struct String {
    raw: RawString,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SmallStringError {
    TooLong,
    NonAscii,
}

impl String {
    /// Creates a Swift string from valid UTF-8 through Swift's native
    /// `_uncheckedFromUTF8` standard-library ABI entry.
    #[inline]
    pub fn from_str(str: &str) -> Self {
        let raw = unsafe { abi::string_from_utf8(str.as_bytes()) };
        unsafe { Self::from_raw(raw) }
    }

    #[inline]
    pub fn from_c_str(str: &CStr) -> Self {
        unsafe { Self::from_c_str_ptr(str.as_ptr()) }
    }

    /// Creates a Swift `String` by calling Swift's `String(cString:)` ABI entry.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid pointer to a nul-terminated C string.
    #[inline]
    pub unsafe fn from_c_str_ptr(ptr: *const c_char) -> Self {
        let raw = unsafe { abi::string_from_c_str(ptr) };
        unsafe { Self::from_raw(raw) }
    }

    #[inline]
    pub fn from_small_ascii(str: &str) -> Result<Self, SmallStringError> {
        let bytes = str.as_bytes();
        if bytes.len() > 15 {
            return Err(SmallStringError::TooLong);
        }

        let mut raw = RawString {
            word0: 0,
            word1: (0xe000_0000_0000_0000usize) | (bytes.len() << 56),
        };

        for (index, byte) in bytes.iter().copied().enumerate() {
            if !byte.is_ascii() {
                return Err(SmallStringError::NonAscii);
            }

            if index < 8 {
                raw.word0 |= (byte as usize) << (index * 8);
            } else {
                raw.word1 |= (byte as usize) << ((index - 8) * 8);
            }
        }

        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Takes ownership of a raw Swift `String` ABI value.
    ///
    /// # Safety
    ///
    /// `raw` must be a valid Swift `String` value whose bridge object word can be
    /// released by this value's destructor.
    #[inline]
    pub unsafe fn from_raw(raw: RawString) -> Self {
        Self { raw }
    }

    #[inline]
    pub fn as_raw(&self) -> RawString {
        self.raw
    }

    #[inline]
    pub fn into_raw(self) -> RawString {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    #[inline]
    pub fn count(&self) -> isize {
        unsafe { abi::string_count(self.raw) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Copies this value's UTF-8 representation into a Rust string.
    pub fn to_rust_string(&self) -> std::string::String {
        struct Utf8CString(*mut ());

        impl Drop for Utf8CString {
            fn drop(&mut self) {
                unsafe { abi::bridge_object_release(self.0 as usize) }
            }
        }

        let array = Utf8CString(unsafe { abi::string_utf8_c_string(self.raw) });
        let count =
            unsafe { abi::contiguous_array_count(array.0.cast_const(), abi::int8_metadata()) };
        debug_assert!(count > 0);

        // `utf8CString` includes a trailing nul.
        let len = count.saturating_sub(1) as usize;
        let mut bytes = Vec::with_capacity(len);
        match unsafe { contiguous_int8_elements(array.0, count) } {
            Some(elements) => {
                bytes.extend_from_slice(unsafe { core::slice::from_raw_parts(elements, len) })
            }
            None => {
                for index in 0..len {
                    let mut byte = 0i8;
                    unsafe {
                        abi::contiguous_array_get(
                            array.0.cast_const(),
                            index as isize,
                            core::ptr::from_mut(&mut byte).cast(),
                            abi::int8_metadata(),
                        );
                    }
                    bytes.push(byte as u8);
                }
            }
        }

        // Swift strings always contain valid Unicode scalar values, so their
        // UTF-8 view is valid UTF-8 by construction.
        unsafe { std::string::String::from_utf8_unchecked(bytes) }
    }
}

/// Offset of `_ArrayBody`'s count word inside native array storage, past the
/// heap object header.
const ARRAY_COUNT_OFFSET: usize = 16;

/// Offset of the first element inside native array storage, past the heap
/// object header and `_ArrayBody`. Valid for elements aligned to at most 16.
const ARRAY_ELEMENTS_OFFSET: usize = 32;

/// Returns the element buffer of a `ContiguousArray<Int8>` when its storage
/// matches the standard library's native layout.
///
/// Copying the buffer in one go replaces a generic subscript call per byte,
/// which matters because every transcription result is converted through
/// [`String::to_rust_string`]. The header offsets are an internal standard
/// library detail, so the stored count is checked against the count the array
/// itself reported; a mismatch means the layout moved and the caller falls back
/// to the subscript path.
///
/// # Safety
///
/// `storage` must be a `ContiguousArray<Int8>` value whose `count` is `count`.
unsafe fn contiguous_int8_elements(storage: *mut (), count: isize) -> Option<*const u8> {
    if storage.is_null() {
        return None;
    }

    let base = storage.cast::<u8>().cast_const();
    let stored_count = unsafe { base.add(ARRAY_COUNT_OFFSET).cast::<isize>().read() };
    (stored_count == count).then(|| unsafe { base.add(ARRAY_ELEMENTS_OFFSET) })
}

unsafe impl SwiftMetadata for String {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        abi::string_metadata()
    }
}

unsafe impl SwiftType for String {}

impl Clone for String {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            abi::bridge_object_retain(self.raw.word1);
            Self::from_raw(self.raw)
        }
    }
}

impl Drop for String {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.raw.word1) }
    }
}

impl PartialEq for String {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { abi::string_equal(self.raw, other.raw) }
    }
}

impl Eq for String {}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rust_string())
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_rust_string(), f)
    }
}

impl Default for String {
    #[inline]
    fn default() -> Self {
        Self::from_str("")
    }
}

impl From<&str> for String {
    #[inline]
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<std::string::String> for String {
    #[inline]
    fn from(value: std::string::String) -> Self {
        Self::from_str(&value)
    }
}

impl From<&std::string::String> for String {
    #[inline]
    fn from(value: &std::string::String) -> Self {
        Self::from_str(value)
    }
}

impl From<String> for std::string::String {
    #[inline]
    fn from(value: String) -> Self {
        value.to_rust_string()
    }
}

#[cfg(test)]
mod tests {
    use super::String;

    #[test]
    fn small_ascii_count_uses_swift_abi() {
        let str = String::from_small_ascii("hello").unwrap();
        assert_eq!(5, str.count());
    }

    #[test]
    fn c_string_count_uses_swift_abi() {
        let str = String::from_c_str(c"hello from rust");
        assert_eq!(15, str.count());
    }

    #[test]
    fn rust_string_roundtrips_utf8_and_embedded_nul() {
        let value = "Swift from Rust: 🦀\0привет";
        let str = String::from(value);

        assert_eq!(value, str.to_rust_string());
        assert_eq!(value.chars().count() as isize, str.count());
    }

    /// Guards the native storage offsets that let `to_rust_string` copy the
    /// UTF-8 buffer in one go instead of one subscript call per byte.
    #[test]
    fn utf8_buffer_reads_native_contiguous_array_storage() {
        use crate::swift::abi;

        let str = String::from("a Swift string long enough to use heap storage 🦀");
        let array = unsafe { abi::string_utf8_c_string(str.as_raw()) };
        let count =
            unsafe { abi::contiguous_array_count(array.cast_const(), abi::int8_metadata()) };
        let elements = unsafe { super::contiguous_int8_elements(array, count) }
            .expect("ContiguousArray<Int8> uses native storage");

        assert_eq!(b'a', unsafe { *elements });
        assert_eq!(0, unsafe { *elements.add(count as usize - 1) });
        unsafe { abi::bridge_object_release(array as usize) };
    }

    #[test]
    fn equality_uses_swift_string_semantics() {
        let composed = String::from("é");
        let decomposed = String::from("é");

        assert_eq!(composed, decomposed);
    }

    #[test]
    fn clone_retains_heap_storage() {
        let original = String::from("a Swift string long enough to use heap storage 🦀");
        let clone = original.clone();
        drop(original);

        assert_eq!(
            "a Swift string long enough to use heap storage 🦀",
            clone.to_rust_string()
        );
    }
}
