pub type FramePosition = i64;
pub type FrameCount = u32;
pub type PacketCount = u32;
pub type ChannelCount = u32;
pub type NodeBus = usize;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Vector3d = Point3d;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector3dOrientation {
    pub forward: Vector3d,
    pub up: Vector3d,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Angular3dOrientation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
