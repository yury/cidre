//! Swift ABI interop.
//!
//! This module calls Swift entry points directly, with no C or Objective-C
//! wrapper functions in between. A direct call goes through a small naked
//! thunk that fills the registers Swift's convention adds to C's — `self`,
//! the error, and the indirect result — so the caller makes an ordinary C
//! call; a suspending one runs on a Swift task through the trampolines in
//! `concurrency`.

pub mod abi;
mod array;
pub(crate) mod concurrency;
mod dictionary;
mod duration;
mod enums;
mod set;
mod string;
mod types;
pub(crate) mod value;

#[cfg(test)]
mod symbol_audit;

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

/// StoreKit.framework (StoreKit 2).
#[cfg(all(
    not(target_os = "watchos"),
    feature = "sk",
    any(feature = "macos_15_0", feature = "ios_18_0")
))]
pub mod store_kit;

pub use array::{Array, ArrayIter};
pub use dictionary::Dictionary;
pub use duration::Duration;
pub use set::Set;
pub use string::{RawString, SmallStringError, String};
pub use types::{
    AbiClass, FromSwift, FromSwiftDoubles, SwiftAbi, SwiftClass, SwiftConsume, SwiftError,
    SwiftHashable, SwiftMetadata, SwiftOptional, SwiftSelf, SwiftSendable, SwiftType, ToSwift,
    ToSwiftDoubles,
};

/// Calls a Swift entry point instead of writing out the call.
///
/// The attribute names the Swift declaration; the Rust signature it is written
/// on says how each value is represented, which is what picks the registers.
pub use cidre_macros::swift_call as call;

/// The address of the Swift entry point a declaration names, for handing to
/// something that will call it rather than calling it here.
pub use cidre_macros::swift_symbol as symbol;

/// The address of a protocol conformance's descriptor, named `Type: Protocol`.
pub use cidre_macros::swift_conformance as conformance;

/// The address of the witness table a conformance publishes, named
/// `Type: Protocol`, for one that needs no instantiation.
pub use cidre_macros::swift_witness_table as witness_table;

/// A suspending function's entry point and the async function pointer that
/// sizes its context, for a call built by hand rather than generated.
pub use cidre_macros::swift_async_symbols as async_symbols;

/// The address of a Swift enum case's descriptor, the constant a resilient
/// enum publishes a case's tag through, named as Swift spells the case.
pub use cidre_macros::swift_enum_case as enum_case;

/// The address of a Swift type's metadata accessor, named as Swift spells the
/// type rather than as the symbol spells it.
pub use cidre_macros::swift_metadata_accessor as metadata_accessor;

/// Declares the Rust type standing for a Swift one, given the Swift type's
/// name.
///
/// The name is what the metadata accessor's symbol is made of, so a binding
/// states the type once, in the form a reader can check against the framework's
/// own documentation, instead of a mangled symbol and a hand-written `extern`
/// block that no longer resembles it.
///
/// ```ignore
/// define_swift!(#[swift::class("DockKit.DockAccessoryManager")] pub AccessoryManager);
/// define_swift!(#[swift::struct("Foundation.UUID")] pub Uuid, UuidValue);
/// define_swift!(#[swift::enum("DockKit.DockAccessory(class).TrackedSubjectType")] pub(crate) TrackedSubjectTypeValue);
/// ```
///
/// A type nested inside another carries the enclosing type's kind in
/// parentheses, since the symbol encodes it and the Swift spelling does not.
#[macro_export]
macro_rules! define_swift {
    (
        #[swift::class($name:literal)]
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $crate::define_swift_class!(
            $(#[$outer])*
            $vis $ty = accessor $crate::swift::metadata_accessor!(class, $name)
        );
    };
    // A generic instantiation, whose metadata the runtime resolves from the
    // mangled name because no accessor symbol names it.
    (
        #[swift::mangled($name:literal, size($size:literal), align($align:literal), trivial, sendable)]
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $crate::swift_value!(@build_mangled $(#[$outer])* $vis $ty = $name, $size, $align, true);
        $crate::swift_value!(@trivial $ty);
        $crate::swift_value!(@shared $ty);
    };
    // A Swift value type, held inline at the layout it declares.
    (
        #[swift::struct($name:literal, size($size:literal), align($align:literal), trivial, sendable)]
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $crate::swift_value!(@build $(#[$outer])* $vis $ty = $name, $size, $align, true);
        $crate::swift_value!(@trivial $ty);
        $crate::swift_value!(@shared $ty);
    };
    (
        #[swift::struct($name:literal, size($size:literal), align($align:literal), trivial)]
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $crate::swift_value!(@build $(#[$outer])* $vis $ty = $name, $size, $align, true);
        $crate::swift_value!(@trivial $ty);
    };
    (
        #[swift::struct($name:literal, size($size:literal), align($align:literal), sendable)]
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $crate::swift_value!(@build $(#[$outer])* $vis $ty = $name, $size, $align, false);
        $crate::swift_value!(@owned $ty);
        $crate::swift_value!(@shared $ty);
    };
    (
        #[swift::struct($name:literal, size($size:literal), align($align:literal))]
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $crate::swift_value!(@build $(#[$outer])* $vis $ty = $name, $size, $align, false);
        $crate::swift_value!(@owned $ty);
    };
    (
        #[swift::enum($name:literal)]
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $crate::define_swift_marker!(
            $(#[$outer])*
            $vis $ty = accessor $crate::swift::metadata_accessor!(enum, $name)
        );
    };
    (
        #[swift::struct($name:literal)]
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $crate::define_swift_marker!(
            $(#[$outer])*
            $vis $ty = accessor $crate::swift::metadata_accessor!(struct, $name)
        );
    };
}

/// Defines an opaque native Swift class marker type and implements Cidre's
/// shared retain/release ownership traits for it.
///
/// Given the class's metadata accessor, it also implements
/// [`SwiftMetadata`](swift::SwiftMetadata) and [`SwiftClass`](swift::SwiftClass),
/// which is what lets `arc::R<Self>` stand in as the class's ABI value wherever
/// a Swift value is expected.
#[macro_export]
macro_rules! define_swift_class {
    (
        $(#[$outer:meta])*
        $vis:vis $ty:ident = accessor $accessor:expr
    ) => {
        $crate::define_swift_class!($(#[$outer])* $vis $ty);

        unsafe impl $crate::swift::SwiftMetadata for $ty {
            #[inline]
            fn metadata() -> *const $crate::swift::abi::TypeMetadata {
                static CACHE: $crate::swift::abi::MetadataCache =
                    $crate::swift::abi::MetadataCache::new();
                CACHE.get(|| unsafe {
                    $crate::swift::abi::call_metadata_accessor($accessor as *const ())
                })
            }
        }

        $crate::impl_swift_optional!($ty);

        unsafe impl $crate::swift::SwiftClass for $ty {}
    };
    (
        $(#[$outer:meta])*
        $vis:vis $ty:ident
    ) => {
        $(#[$outer])*
        #[repr(C)]
        $vis struct $ty {
            _priv: [u8; 0],
        }

        /// A class-typed value *is* the reference, so that is what `self` is.
        unsafe impl $crate::swift::SwiftSelf for $ty {
            #[inline]
            fn swift_self_ptr(&self) -> *const () {
                (self as *const Self).cast()
            }
        }

        impl $crate::arc::Release for $ty {
            #[inline]
            unsafe fn release(ptr: std::ptr::NonNull<Self>) {
                unsafe {
                    $crate::swift::abi::object_release(ptr.as_ptr().cast());
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
