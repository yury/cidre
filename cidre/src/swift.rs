//! Swift ABI interop.
//!
//! This module intentionally calls Swift ABI entry points directly from Rust
//! inline assembly. It does not use C or Objective-C wrapper functions.

pub mod abi;
mod array;
pub(crate) mod concurrency;
mod string;
mod types;
pub(crate) mod value;

/// DockKit.
#[cfg(all(
    any(target_os = "macos", all(target_os = "ios", not(target_abi = "sim"))),
    feature = "dk"
))]
pub mod dock_kit;

/// Foundation's Swift-native value types.
#[cfg(feature = "foundation")]
pub mod foundation;

/// MusicUnderstanding.framework.
#[cfg(all(
    not(target_os = "watchos"),
    feature = "music_understanding",
    any(feature = "macos_27_0", feature = "ios_27_0")
))]
pub mod music_understanding;

/// Speech.framework.
#[cfg(all(
    not(target_os = "watchos"),
    feature = "speech",
    any(feature = "macos_26_0", feature = "ios_26_0")
))]
pub mod speech;

pub use array::{Array, ArrayIter};
pub use string::{RawString, SmallStringError, String};
pub use types::{SwiftMetadata, SwiftType};

/// Defines an opaque native Swift class marker type and implements Cidre's
/// shared retain/release ownership traits for it.
#[macro_export]
macro_rules! define_swift_class {
    (
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $(#[$outer])*
        #[repr(C)]
        $vis struct $ty {
            _priv: [u8; 0],
        }

        impl $crate::arc::Release for $ty {
            #[inline]
            unsafe fn release(&mut self) {
                unsafe {
                    $crate::swift::abi::object_release((self as *mut Self).cast());
                }
            }
        }

        impl $crate::arc::Retain for $ty {
            #[inline]
            fn retained(&self) -> $crate::arc::R<Self> {
                unsafe {
                    let ptr = $crate::swift::abi::object_retain((self as *const Self).cast())
                        .cast_mut()
                        .cast();
                    $crate::arc::R::from_raw(ptr)
                }
            }
        }
    };
}
