use crate::{cf, cg, define_obj_type, objc, vn};

define_obj_type!(Request(objc::Id));

impl Request {
    pub fn revision(&self) -> usize {
        unsafe { todo!() }
    }

    pub fn set_revision(&mut self, value: usize) {
        unsafe { todo!() }
    }

    pub fn uses_cpu_only(&self) -> bool {
        unsafe { todo!() }
    }

    pub fn set_uses_cpu_only(&mut self, value: bool) {
        unsafe {
            todo!();
        }
    }

    pub fn results(&self) -> Option<cf::Retained<cf::ArrayOf<vn::Observation>>> {
        unsafe {
            todo!();
        }
    }
}

define_obj_type!(ImageBasedRequest(Request));

impl ImageBasedRequest {
    pub fn region_of_interest(&self) -> cg::Rect {
        unsafe {
            todo!();
        }
    }

    pub fn set_region_of_interest(&mut self, value: cg::Rect) {
        unsafe {
            todo!();
        }
    }
}
