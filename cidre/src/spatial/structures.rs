/// A three-dimensional vector (`SPVector3D`, Swift `Spatial.Vector3D`).
#[doc(alias = "SPVector3D")]
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    _padding: f64,
}

impl Vector3D {
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            _padding: 0.0,
        }
    }
}

/// A quaternion rotation (`SPRotation3D`, Swift `Spatial.Rotation3D`).
#[doc(alias = "SPRotation3D")]
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Rotation3D {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    #[inline]
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
}

impl Default for Rotation3D {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// The `swift` module carries hand-written aarch64 assembly, so it exists only
/// where [`crate::swift`] itself does. The condition has to match that one.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64", feature = "swift"))]
mod swift_interop {
    use super::{Rotation3D, Vector3D};
    use crate::swift::{SwiftMetadata, SwiftType, abi};

    unsafe impl SwiftMetadata for Vector3D {
        #[inline]
        fn metadata() -> *const abi::TypeMetadata {
            unsafe { abi::type_by_mangled_name("So10SPVector3Da") }
        }
    }

    unsafe impl SwiftType for Vector3D {}

    unsafe impl SwiftMetadata for Rotation3D {
        #[inline]
        fn metadata() -> *const abi::TypeMetadata {
            unsafe { abi::type_by_mangled_name("So12SPRotation3Da") }
        }
    }

    unsafe impl SwiftType for Rotation3D {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imported_layouts_match_spatial_headers() {
        assert_eq!(32, size_of::<Vector3D>());
        assert_eq!(16, align_of::<Vector3D>());
        assert_eq!(32, size_of::<Rotation3D>());
        assert_eq!(16, align_of::<Rotation3D>());
    }

    #[cfg(feature = "swift")]
    #[test]
    fn imported_layouts_match_swift_metadata() {
        use crate::swift::{SwiftMetadata, abi};

        unsafe {
            let vector = abi::value_layout(Vector3D::metadata());
            assert_eq!(size_of::<Vector3D>(), vector.size);
            assert_eq!(size_of::<Vector3D>(), vector.stride);

            let rotation = abi::value_layout(Rotation3D::metadata());
            assert_eq!(size_of::<Rotation3D>(), rotation.size);
            assert_eq!(size_of::<Rotation3D>(), rotation.stride);
        }
    }
}
