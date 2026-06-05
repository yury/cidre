use core::ffi::c_char;
use std::ffi::CStr;

use super::{Int, abi};

/// Raw two-word Swift `String` ABI value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct RawString {
    pub word0: usize,
    pub word1: usize,
}

/// A retained Swift `String` value.
#[derive(Debug, Eq, PartialEq)]
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
    pub fn count(&self) -> Int {
        unsafe { abi::string_count(self.raw) }
    }
}

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
}
