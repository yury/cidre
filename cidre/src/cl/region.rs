use crate::{define_cls, define_obj_type, ns, objc};

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

#[cfg(not(target_os = "visionos"))]
define_obj_type!(
    #[doc(alias = "CLRegion")]
    pub Region(ns::Id)
);

#[cfg(not(target_os = "visionos"))]
impl Region {
    define_cls!(CL_REGION);
}

#[link(name = "cl", kind = "static")]
unsafe extern "C" {
    #[cfg(not(target_os = "visionos"))]
    static CL_REGION: &'static objc::Class<Region>;
}
