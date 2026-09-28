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
/// survive a case being inserted in a later OS release. The enum is named the
/// way Swift spells it, and every symbol — each case's descriptor, the
/// `hashValue` getter, and `debugDescription` for the types that list `debug`
/// — is derived from that name. Without `debug`, [`Debug`](core::fmt::Debug)
/// names whichever case matches.
///
/// ```ignore
/// define_swift_tag_enum!(
///     pub State = swift "DockKit.DockAccessory(class).State" {
///         debug,
///         cases { undocked = "undocked", docked = "docked" }
///     }
/// );
/// ```
#[macro_export]
macro_rules! define_swift_tag_enum {
    (
        $(#[$meta:meta])*
        $vis:vis $ty:ident = swift $path:literal {
            debug,
            cases { $($case:ident = $swift_case:literal),+ $(,)? }
        }
    ) => {
        $crate::define_swift_tag_enum!(@impl [debug] $(#[$meta])* $vis $ty = $path {
            $($case = $swift_case),+
        });
    };
    (
        $(#[$meta:meta])*
        $vis:vis $ty:ident = swift $path:literal {
            cases { $($case:ident = $swift_case:literal),+ $(,)? }
        }
    ) => {
        $crate::define_swift_tag_enum!(@impl [] $(#[$meta])* $vis $ty = $path {
            $($case = $swift_case),+
        });
    };
    (
        @impl [$($debug:ident)?]
        $(#[$meta:meta])*
        $vis:vis $ty:ident = $path:literal {
            $($case:ident = $swift_case:literal),+
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
                    unsafe { Self(*$crate::swift::enum_case!($path, "(enum).", $swift_case)) }
                }
            )+

            /// The address Swift passes a value of this type at, since a
            /// resilient enum is passed indirectly.
            #[inline]
            pub fn as_abi_ptr(&self) -> *const () {
                core::ptr::from_ref(self).cast()
            }

            #[$crate::swift::call($path, "(enum).hashValue: Int { get }")]
            pub fn hash_value(&self) -> isize;
        }

        /// Passed indirectly, so what a call hands over is the value's address.
        /// That is also what a generated call asks a value type for, which is
        /// what lets one of these be an argument without a hand-written call.
        unsafe impl $crate::swift::SwiftSelf for $ty {
            #[inline]
            fn swift_self_ptr(&self) -> *const () {
                self.as_abi_ptr()
            }
        }

        /// The value is plain data, so taking it leaves nothing to release.
        unsafe impl $crate::swift::SwiftConsume for $ty {}

        /// A resilient enum comes back indirectly, into the tag byte itself.
        unsafe impl $crate::swift::SwiftAbi for $ty {
            const CLASS: $crate::swift::AbiClass = $crate::swift::AbiClass::Indirect;
        }

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

        $(
            $crate::define_swift_tag_enum!(@debug_desc $debug $ty $path);
        )?

        $crate::define_swift_tag_enum!(@debug $ty $(, $debug)?; $($case),+);
    };
    (@debug_desc debug $ty:ident $path:literal) => {
        impl $ty {
            /// Swift's own `debugDescription`.
            #[$crate::swift::call($path, "(enum).debugDescription: String { get }")]
            pub fn debug_desc(&self) -> $crate::swift::String;
        }
    };
    // With a `debugDescription`, print what Swift prints.
    (@debug $ty:ident, debug; $($case:ident),+) => {
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
///
/// The type is named the way Swift spells it, and each case by the static
/// property it reads, so the metadata accessor and every getter's symbol are
/// derived rather than written out:
///
/// ```ignore
/// define_swift_getter_enum!(
///     pub TranscriberPreset = swift "Speech.SpeechTranscriber(class).Preset" {
///         Transcription = "transcription",
///     }
/// );
/// ```
#[macro_export]
macro_rules! define_swift_getter_enum {
    (
        $(#[$meta:meta])*
        $vis:vis $ty:ident = swift $path:literal {
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
                match self {
                    $(
                        $(#[$case_meta])*
                        Self::$case => $crate::swift::symbol!(
                            "static ", $path, "(struct).", $getter, ": ", $path, "(struct) { get }"
                        ),
                    )+
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
                CACHE.get(|| unsafe {
                    $crate::swift::abi::call_metadata_accessor(
                        $crate::swift::metadata_accessor!(struct, $path),
                    )
                })
            }
        }

        /// A case is a static property of the Swift type rather than a tag this
        /// binding could write itself, so making the value means calling its
        /// getter straight into the destination.
        unsafe impl $crate::swift::ToSwift for $ty {
            #[inline]
            unsafe fn copy_to_swift(&self, dst: *mut ()) {
                unsafe { $crate::swift::abi::call::to_value(self.getter(), dst) }
            }
        }
    };
}
