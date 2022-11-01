use crate::{define_obj_type, ns, vn};

define_obj_type!(Region(ns::Id));

impl Region {
    /// The amount of points in a given region. This can be zero if no points for a region could be found.
    pub fn point_count(&self) -> usize {
        unsafe { rsel_pointCount(self) }
    }
}
define_obj_type!(Region2D(Region));

define_obj_type!(FaceLandmarks(ns::Id));
define_obj_type!(FaceLandmarks2D(FaceLandmarks));

impl FaceLandmarks {
    pub fn confidence(&self) -> vn::Confidence {
        unsafe { rsel_confidence(self) }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_pointCount(id: &ns::Id) -> usize;
    fn rsel_confidence(id: &ns::Id) -> vn::Confidence;
}
