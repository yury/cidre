use crate::{define_obj_type, ns, objc, vn};

define_obj_type!(Region(ns::Id));

impl Region {
    /// The amount of points in a given region. This can be zero if no points for a region could be found.
    #[objc::msg_send(pointCount)]
    pub fn point_count(&self) -> usize;
}

define_obj_type!(Region2d(Region));

define_obj_type!(FaceLandmarks(ns::Id));
define_obj_type!(FaceLandmarks2d(FaceLandmarks));

impl FaceLandmarks {
    #[objc::msg_send(confidence)]
    pub fn confidence(&self) -> vn::Confidence;
}
