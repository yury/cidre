use crate::blocks;

#[doc(alias = "AVAudioFramePosition")]
pub type FramePos = i64;

#[doc(alias = "AVAudioFrameCount")]
pub type FrameCount = u32;

#[doc(alias = "AVAudioPacketCount")]
pub type PacketCount = u32;

#[doc(alias = "AVAudioChannelCount")]
pub type ChannelCount = u32;

/// AVAudioNode objects potentially have multiple input and/or output busses.
/// AVAudioNodeBus represents a bus as a zero-based index.
#[doc(alias = "AVAudioNodeBus")]
pub type NodeBus = usize;

#[doc(alias = "AVAudioNodeCompletionHandler")]
pub type NodeCh<Attr> = blocks::Block<fn(), Attr>;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Vector3d = Point3d;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector3dOrientation {
    pub forward: Vector3d,
    pub up: Vector3d,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Angular3dOrientation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
