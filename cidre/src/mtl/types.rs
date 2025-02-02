use crate::ns;

/// A set of dimensions to declare the size of an object, such as an image, texture, threadgroup, or grid.
#[doc(alias = "MTLSize")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Size {
    pub width: ns::UInteger,
    pub height: ns::UInteger,
    pub depth: ns::UInteger,
}

impl Size {
    #[inline]
    pub const fn _1d(width: usize) -> Self {
        Self {
            width,
            height: 1,
            depth: 1,
        }
    }

    #[inline]
    pub const fn _2d(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            depth: 1,
        }
    }
}

/// Identify a pixel in an image. MTLOrigin is ususally used as the upper-left corner of a region of a texture.
#[doc(alias = "MTLOrigin")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Origin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Origin {
    #[inline]
    pub const fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

/// Handle of the GPU resource suitable for storing in an Argument Buffer
#[doc(alias = "MTLResourceID")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct ResId {
    _impl: usize,
}

/// Identify a region in an image or texture.
#[doc(alias = "MTLRegion")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Region {
    pub origin: Origin,
    pub size: Size,
}

impl Region {
    #[doc(alias = "MTLRegionMake1D")]
    #[inline]
    pub const fn new_1d(x: usize, width: usize) -> Self {
        Self {
            origin: Origin { x, y: 0, z: 0 },
            size: Size::_1d(width),
        }
    }

    #[doc(alias = "MTLRegionMake2D")]
    #[inline]
    pub const fn new_2d(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            origin: Origin { x, y, z: 0 },
            size: Size::_2d(width, height),
        }
    }

    #[doc(alias = "MTLRegionMake3D")]
    #[inline]
    pub const fn new_3d(
        x: usize,
        y: usize,
        z: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) -> Self {
        Self {
            origin: Origin { x, y, z },
            size: Size {
                width,
                height,
                depth,
            },
        }
    }

    pub fn with(origin: Origin, size: Size) -> Self {
        Self { origin, size }
    }
}

/// Identify a sample within a pixel. Origin is top-left with a range [0,1) for both x and y.
#[doc(alias = "MTLSamplePosition")]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SamplePos {
    pub x: f32,
    pub y: f32,
}

/// A floating point coordinate in an abstract 2D space.
///
/// Refer to location of use for concrete information on the space in which the coordinate exists.
pub type Coordinate2d = SamplePos;
