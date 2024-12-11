use crate::ns;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Size {
    pub width: ns::UInteger,
    pub height: ns::UInteger,
    pub depth: ns::UInteger,
}

impl Size {
    #[inline]
    pub fn _2d(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            depth: 1,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Origin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Origin {
    #[inline]
    pub fn zero() -> Self {
        Self::default()
    }
}

/// Handle of the GPU resource suitable for storing in an Argument Buffer
#[doc(alias = "MTLResourceID")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct ResId {
    _impl: usize,
}

#[doc(alias = "MTLRegion")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Region {
    pub origin: Origin,
    pub size: Size,
}

impl Region {
    #[inline]
    pub fn new_1d(x: usize, width: usize) -> Self {
        Self {
            origin: Origin { x, y: 0, z: 0 },
            size: Size {
                width,
                height: 1,
                depth: 1,
            },
        }
    }

    #[inline]
    pub fn new_2d(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            origin: Origin { x, y, z: 0 },
            size: Size {
                width,
                height,
                depth: 1,
            },
        }
    }

    #[inline]
    pub fn new_3d(x: usize, y: usize, z: usize, width: usize, height: usize, depth: usize) -> Self {
        Self {
            origin: Origin { x, y, z },
            size: Size {
                width,
                height,
                depth,
            },
        }
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
