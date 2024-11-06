use crate::blocks;

pub type FramePos = i64;
pub type FrameCount = u32;
pub type PacketCount = u32;
#[doc(alias = "AVAudioChannelCount")]
pub type ChannelCount = u32;
pub type NodeBus = usize;

pub type NodeCompletionHandler<Attr> = blocks::Block<fn(), Attr>;

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
