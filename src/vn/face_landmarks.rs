use crate::{define_obj_type, objc, vn};

define_obj_type!(Region(objc::Id));

impl Region {
    /// The amount of points in a given region. This can be zero if no points for a region could be found.
    pub fn point_count(&self) -> usize {
        unsafe { rsel_pointCount(self) }
    }
}
define_obj_type!(Region2D(Region));

define_obj_type!(FaceLandmarks(objc::Id));
define_obj_type!(FaceLandmarks2D(FaceLandmarks));

impl FaceLandmarks {
    pub fn confidence(&self) -> vn::Confidence {
        unsafe { rsel_confidence(self) }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_pointCount(id: &objc::Id) -> usize;
    fn rsel_confidence(id: &objc::Id) -> vn::Confidence;
}
