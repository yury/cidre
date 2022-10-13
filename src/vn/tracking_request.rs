use crate::{define_obj_type, vn};

#[repr(usize)]
pub enum Level {
    Accurate,
    Fast,
}

define_obj_type!(TrackingRequest(vn::ImageBasedRequest));
