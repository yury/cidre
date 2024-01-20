pub mod mac_types;

pub use mac_types::FourCharCode;
pub use mac_types::ResType;
pub use mac_types::UniChar;

/// Apple Mobile
#[cfg(all(target_os = "macos", feature = "am"))]
pub mod am;

/// Audio Toolkit
#[cfg(feature = "at")]
pub mod at;

/// AudioVisual Foundation (AVFoundation)
#[cfg(feature = "av")]
pub mod av;

/// Core Animation
#[cfg(not(target_os = "watchos"))]
#[cfg(feature = "ca")]
pub mod ca;

/// Core Image
#[cfg(feature = "ci")]
pub mod ci;

/// Core Foundation
#[cfg(feature = "cf")]
pub mod cf;

/// Core Text
#[cfg(feature = "ct")]
pub mod ct;

/// Core Graphics
#[cfg(feature = "cg")]
pub mod cg;

/// Core Media
#[cfg(feature = "cm")]
pub mod cm;

/// Core Motion
#[cfg(not(target_os = "tvos"))]
#[cfg(feature = "core_motion")]
pub mod core_motion;

/// Core Video
#[cfg(feature = "cv")]
pub mod cv;

/// Grand Central Dispatch
#[cfg(feature = "dispatch")]
pub mod dispatch;

/// IOSurface
#[cfg(feature = "io")]
pub mod io;

/// cidre vision of obj-c blocks impl in rust
#[cfg(feature = "blocks")]
pub mod blocks;

/// mach
pub mod mach;

/// MultipeerConnectivity
#[cfg(not(target_os = "watchos"))]
#[cfg(feature = "mc")]
pub mod mc;

/// Metal
#[cfg(feature = "mtl")]
pub mod mtl;

/// MLCompute
#[cfg(feature = "mlc")]
pub mod mlc;

/// Metal Performance Shaders
#[cfg(feature = "mps")]
pub mod mps;

/// Foundation
#[cfg(feature = "ns")]
pub mod ns;

/// Network
#[cfg(feature = "nw")]
pub mod nw;

#[cfg(feature = "ns")]
pub mod objc;

/// Game Controller
#[cfg(feature = "gc")]
pub mod gc;

pub mod os;
pub mod sys;

/// Video Toolbox
#[cfg(feature = "vt")]
pub mod vt;

/// Screen Capture
#[cfg(all(target_os = "macos", feature = "sc"))]
pub mod sc;

/// Sound Analysis
#[cfg(feature = "sn")]
pub mod sn;

/// CoreAudioTypes
#[cfg(feature = "cat")]
pub mod cat;

#[cfg(all(
    any(
        target_os = "ios",
        target = "aarch64-apple-ios-macabi",
        target_os = "tvos"
    ),
    feature = "ui"
))]
pub mod ui;

/// UniformTypeIdentifiers
#[cfg(feature = "ut")]
pub mod ut;

pub mod time;

pub mod dns_sd;

#[cfg(feature = "simd")]
pub mod simd;

/// Vision
#[cfg(feature = "vn")]
pub mod vn;

/// Web Kit
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
#[cfg(feature = "wk")]
pub mod wk;

pub mod arc;

#[macro_export]
macro_rules! define_opts {
    (
        $(#[$outer:meta])*
        $vis:vis
        $NewType:ident($BaseType:path)
    ) => {
        $(#[$outer])*
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
        #[repr(transparent)]
        $vis struct $NewType(pub $BaseType);

        impl $NewType {
            #[inline]
            pub fn contains(&self, rhs: Self) -> bool {
                *self & rhs == rhs
            }
        }

        impl ::std::ops::BitAndAssign for $NewType {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                *self = Self(self.0 & rhs.0)
            }
        }

        impl ::std::ops::BitAnd for $NewType {
            type Output = $NewType;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl ::std::ops::BitOr for $NewType {
            type Output = $NewType;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl ::std::ops::BitOrAssign for $NewType {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                *self = Self(self.0 | rhs.0)
            }
        }

        impl ::std::convert::From<$BaseType> for $NewType {
            #[inline]
            fn from(value: $BaseType) -> Self {
                Self(value)
            }
        }

        impl ::std::fmt::Binary for $NewType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                ::std::fmt::Binary::fmt(&self.0, f)
            }
        }
    };
}

pub trait Dyn {}

impl std::fmt::Debug for dyn Dyn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn")
    }
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    fn it_works() {
        let f = {
            let null = cf::Null::value();
            null.show();

            let num = cf::Number::from_i16(0);
            let arr = cf::Array::from_type_refs(&[null, &num]).unwrap();

            let v = b"he".to_vec();
            let _s = cf::String::create_with_bytes_no_copy_in(
                &v,
                cf::StringEncoding::UTF8,
                false,
                cf::Allocator::null(),
                None,
            )
            .unwrap();

            let _f = num;

            arr.show();
            arr
        };

        f.show()
    }
}
