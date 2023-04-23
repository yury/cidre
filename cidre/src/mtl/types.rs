use crate::ns;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Size {
    pub width: ns::UInteger,
    pub height: ns::UInteger,
    pub depth: ns::UInteger,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Origin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
/// Handle of the GPU resource suitable for storing in an Argument Buffer
pub struct ResourceID {
    _impl: u64,
}

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
            origin: Origin {
                x,
                ..Default::default()
            },
            size: Size {
                width,
                ..Default::default()
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
                depth: 0,
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
