use std::mem::transmute;

use crate::{cf, cg, define_obj_type, objc, vn};

define_obj_type!(Request(objc::Id));

impl Request {
    /// The specific algorithm or implementation revision that is to be used to perform the request.
    pub fn revision(&self) -> usize {
        unsafe { rsel_revision(self) }
    }

    pub fn set_revision(&mut self, value: usize) {
        unsafe { wsel_setRevision(self, value) }
    }

    pub fn uses_cpu_only(&self) -> bool {
        unsafe { rsel_usesCPUOnly(self) }
    }

    pub fn set_uses_cpu_only(&mut self, value: bool) {
        unsafe { wsel_setUsesCPUOnly(self, value) }
    }

    #[inline]
    pub fn results(&self) -> Option<cf::Retained<cf::ArrayOf<vn::Observation>>> {
        unsafe { rsel_results(self) }
    }

    pub const REVISION_UNSPECIFIED: usize = 0;
}

define_obj_type!(ImageBasedRequest(Request));

impl ImageBasedRequest {
    /// The region of the image in which the request will be performed.
    /// The rectangle is normalized to the dimensions of the image being
    /// processed and has its origin specified relative to the image's lower-left corner.
    ///
    /// The default value for this property is { { 0, 0 }, { 1, 1 } }.  Setting this
    /// property to a rectangle that is outside of the normalized coordinate space will
    /// be accepted but result in the request failing to be performed.
    #[inline]
    pub fn region_of_interest(&self) -> cg::Rect {
        unsafe { rsel_regionOfInterest(self) }
    }

    #[inline]
    pub fn set_region_of_interest(&mut self, value: cg::Rect) {
        unsafe { wsel_setRegionOfIntereset(self, value) }
    }
}

define_obj_type!(DetectHorizonRequest(ImageBasedRequest));

impl DetectHorizonRequest {
    pub const REVISION_1: usize = 1;

    pub fn results(&self) -> Option<cf::Retained<cf::ArrayOf<vn::HorizonObservation>>> {
        unsafe { transmute(rsel_results(self)) }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_revision(id: &objc::Id) -> usize;
    fn wsel_setRevision(id: &objc::Id, value: usize);

    fn rsel_usesCPUOnly(id: &objc::Id) -> bool;
    fn wsel_setUsesCPUOnly(id: &objc::Id, value: bool);

    fn rsel_results(id: &objc::Id) -> Option<cf::Retained<cf::ArrayOf<vn::Observation>>>;

    fn rsel_regionOfInterest(id: &objc::Id) -> cg::Rect;
    fn wsel_setRegionOfIntereset(id: &mut objc::Id, value: cg::Rect);
}
