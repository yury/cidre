use crate::{define_obj_type, ns};

/// Represents the current state of the device with reference to a region.
// #[cfg(any(target_os = "ios", target_os = "macos"))]
#[doc(alias = "CLRegionState")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum RegionState {
    Unknown,
    Inside,
    Outside,
}

/// Represents the current proximity of an entity.
// #[cfg(any(target_os = "ios", target_os = "macos"))]
#[doc(alias = "CLProximity")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum Proximity {
    Unknown,
    Immediate,
    Near,
    Far,
}

define_obj_type!(
    pub Region(ns::Id)
);
