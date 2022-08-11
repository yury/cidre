use crate::ns;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Size {
    pub width: ns::UInteger,
    pub height: ns::UInteger,
    pub depth: ns::UInteger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    _impl: u64 
}