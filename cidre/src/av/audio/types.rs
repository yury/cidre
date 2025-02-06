#[cfg(feature = "blocks")]
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
#[cfg(feature = "blocks")]
pub type NodeCh<Attr> = blocks::Block<fn(), Attr>;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[doc(alias = "AVAudio3DVector")]
pub type Vec3d = Point3d;

#[doc(alias = "AVAudio3DVectorOrientation")]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vec3dOrientation {
    pub forward: Vec3d,
    pub up: Vec3d,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Angular3dOrientation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
