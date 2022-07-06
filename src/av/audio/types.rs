pub type FramePosition = i64;
pub type FrameCount = u32;
pub type PacketCount = u32;
pub type ChannelCount = u32;
pub type NodeBus = usize;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Vector3D = Point3D;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector3DOrientation {
    pub forward: Vector3D,
    pub up: Vector3D,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Angular3DOrientation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
