pub mod mac_types;
pub use mac_types::FourCharCode;
pub use mac_types::ResType;
pub use mac_types::UniChar;

/// Core Animation
pub mod ca;
/// Core Foundation
pub mod cf;
/// Core Graphics
pub mod cg;
/// Core Media
pub mod cm;
/// Core Video
pub mod cv;
/// Grand Central Dispatch
pub mod dispatch;
/// IOSurface
pub mod io;
/// match
pub mod mach;
/// Metal
pub mod mtl;
/// Foundation
pub mod ns;
pub mod objc;
pub mod os;
pub mod sys;
/// Video Toolbox
pub mod vt;

#[cfg(target_os = "macos")]
pub mod sc;

/// Apple Mobile
#[cfg(target_os = "macos")]
pub mod am;

/// AudioVisual Foundation (AVFoundation)
pub mod av;

/// Audio Toolkit
pub mod at;
/// CoreAudioTypes
pub mod cat;

#[cfg(any(target_os = "ios", target = "aarch64-apple-ios-macabi"))]
pub mod ui;

pub mod time;

#[macro_export]
macro_rules! define_options {
    ($NewType:ident($BaseType:path)) => {
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
        #[repr(transparent)]
        pub struct $NewType(pub $BaseType);

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
    };
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
            let s = cf::String::create_with_bytes_no_copy(
                None,
                &v,
                2,
                cf::StringEncoding::UTF8,
                false,
                cf::Allocator::null(),
            )
            .unwrap();
            // s
            let f = num;

            arr.show();
            // s.show();

            // println!("len {:?}", f.get_length());
            // f.show();
            arr
            // s.retained()
        };

        f.show()
    }
}
