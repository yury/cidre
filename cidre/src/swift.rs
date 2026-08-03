//! Swift ABI interop.
//!
//! This module intentionally calls Swift ABI entry points directly from Rust
//! inline assembly. It does not use C or Objective-C wrapper functions.

pub mod abi;
mod array;
pub(crate) mod async_task;
mod string;
mod types;

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
