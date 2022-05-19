use crate::{define_obj_type, ns};

pub mod item;
pub use item::Item as PlayerItem;
pub use item::Status as ItemStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    Unknown = 0,
    ReadyToPlay = 1,
    Failed = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum TimeControlStatus {
    Paused = 0,
    WaitingToPlayAtSpecifiedRate = 1,
    Playing = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum ActionAtItemEnd {
    ActionAtItemEndAdvance = 0,
    ActionAtItemEndPause = 1,
    ActionAtItemEndNone = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum HDRMode {
    HLG = 0x1,
    HDR10 = 0x2,
    DolbyVision = 0x4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum AudiovisualBackgroundPlaybackPolicy {
    AudiovisualBackgroundPlaybackPolicyAutomatic = 1,
    AudiovisualBackgroundPlaybackPolicyPauses = 2,
    AudiovisualBackgroundPlaybackPolicyContinuesIfPossible = 3,
}

define_obj_type!(Player(ns::Id));
define_obj_type!(QueuePlayer(Player));
