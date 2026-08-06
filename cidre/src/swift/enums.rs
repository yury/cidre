//! Binding the two ways Swift publishes the cases of a type.
//!
//! Neither is a Rust enum. A resilient `enum`'s cases are exported as one-byte
//! tag descriptors, and its layout may change between OS releases, so a case is
//! read from its symbol rather than written as a literal. A frozen `struct`'s
//! cases — the presets and analysis types the frameworks use as enums — are
//! static properties whose values are only reachable by calling their getters.
//!
//! Each framework had grown its own spelling of one of these. Both are here
//! now, so a binding declares the cases and nothing else.

/// Declares a resilient Swift `enum` whose cases are exported tag descriptors.
///
/// The value is the tag byte the runtime published, which is what makes this
/// survive a case being inserted in a later OS release. `hash` is the type's
/// `hashValue` getter; `debug` is `debugDescription`, for the types that have
/// one — without it, [`Debug`](core::fmt::Debug) names whichever case matches.
///
/// Each case's symbol is declared inside the function that reads it, so a
/// binding writes the mangled name once and invents no identifiers for it.
#[macro_export]
macro_rules! define_swift_tag_enum {
    (
        $(#[$meta:meta])*
        $vis:vis $ty:ident in $framework:literal {
            hash = $hash:literal,
            $(debug = $debug:literal,)?
            cases { $($case:ident = $symbol:literal),+ $(,)? }
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        #[repr(transparent)]
        $vis struct $ty(u8);

        impl $ty {
            $(
                #[inline]
                pub fn $case() -> Self {
                    #[link(name = $framework, kind = "framework")]
                    unsafe extern "C" {
                        #[link_name = $symbol]
                        static TAG: u8;
                    }
                    unsafe { Self(TAG) }
                }
            )+

            /// The address Swift passes a value of this type at, since a
            /// resilient enum is passed indirectly.
            #[inline]
            pub fn as_abi_ptr(&self) -> *const () {
                core::ptr::from_ref(self).cast()
            }

            #[inline]
            pub fn hash_value(&self) -> isize {
                #[link(name = $framework, kind = "framework")]
                unsafe extern "C" {
                    #[link_name = $hash]
                    fn hash_value();
                }
                unsafe {
                    $crate::swift::abi::call_value_to_int(
                        hash_value as *const (),
                        self.as_abi_ptr(),
                    )
                }
            }

            $(
                /// Swift's own `debugDescription`.
                #[inline]
                pub fn debug_desc(&self) -> $crate::swift::String {
                    #[link(name = $framework, kind = "framework")]
                    unsafe extern "C" {
                        #[link_name = $debug]
                        fn debug_description();
                    }
                    unsafe {
                        $crate::swift::String::from_raw($crate::swift::abi::call_value_to_string(
                            debug_description as *const (),
                            self.as_abi_ptr(),
                        ))
                    }
                }
            )?
        }

        $crate::define_swift_tag_enum!(@debug $ty $(, $debug)?; $($case),+);
    };
    // With a `debugDescription`, print what Swift prints.
    (@debug $ty:ident, $debug:literal; $($case:ident),+) => {
        impl core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(&self.debug_desc().to_string())
            }
        }
    };
    // Without one, the tag is matched against the cases the framework exports.
    (@debug $ty:ident; $($case:ident),+) => {
        impl core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                $(
                    if *self == Self::$case() {
                        return f.write_str(stringify!($case));
                    }
                )+
                write!(f, concat!(stringify!($ty), "({})"), self.0)
            }
        }
    };
}

/// Declares a Swift type whose cases are static properties, as a Rust enum.
///
/// The value of a case is only reachable by calling its getter, so the Rust
/// enum is a choice of getter and [`ToSwift`](crate::swift::ToSwift) is what
/// turns it back into the Swift value — which is what lets one be a set
/// element, a dictionary key, or an argument.
#[macro_export]
macro_rules! define_swift_getter_enum {
    (
        $(#[$meta:meta])*
        $vis:vis $ty:ident in $framework:literal = accessor $metadata:literal {
            $($(#[$case_meta:meta])* $case:ident = $getter:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        #[non_exhaustive]
        $vis enum $ty {
            $($(#[$case_meta])* $case,)+
        }

        impl $ty {
            /// The static property this case reads its value from.
            fn getter(self) -> *const () {
                #[allow(non_snake_case)]
                #[link(name = $framework, kind = "framework")]
                unsafe extern "C" {
                    $(
                        $(#[$case_meta])*
                        #[link_name = $getter]
                        fn $case();
                    )+
                }

                match self {
                    $($(#[$case_meta])* Self::$case => $case as *const (),)+
                }
            }

            /// Every case this build knows about, in declaration order.
            #[allow(dead_code)]
            $vis fn all() -> Vec<Self> {
                let mut cases = Vec::new();
                $(
                    $(#[$case_meta])*
                    cases.push(Self::$case);
                )+
                cases
            }
        }

        unsafe impl $crate::swift::SwiftMetadata for $ty {
            fn metadata() -> *const $crate::swift::abi::TypeMetadata {
                static CACHE: $crate::swift::abi::MetadataCache =
                    $crate::swift::abi::MetadataCache::new();
                CACHE.get(|| {
                    #[link(name = $framework, kind = "framework")]
                    unsafe extern "C" {
                        #[link_name = $metadata]
                        fn metadata();
                    }
                    unsafe {
                        $crate::swift::abi::call_int_to_int(metadata as *const (), 0)
                            as *const $crate::swift::abi::TypeMetadata
                    }
                })
            }
        }

        /// A case is a static property of the Swift type rather than a tag this
        /// binding could write itself, so making the value means calling its
        /// getter straight into the destination.
        unsafe impl $crate::swift::ToSwift for $ty {
            #[inline]
            unsafe fn copy_to_swift(&self, dst: *mut ()) {
                unsafe { $crate::swift::abi::call0_value(self.getter(), dst) }
            }
        }
    };
}
