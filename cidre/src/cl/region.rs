use crate::{define_obj_type, ns};

/// Represents the current state of the device with reference to a region.
#[doc(alias = "CLRegionState")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum RegionState {
    Unknown,
    Inside,
    Outside,
}

/// Represents the current proximity of an entity.
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
    #[doc(alias = "CLRegion")]
    pub Region(ns::Id)
);
