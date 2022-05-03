use crate::{define_obj_type, objc::Id};

define_obj_type!(Output(Id));

#[repr(isize)]
pub enum DataDroppedReason {
    None = 0,
    LateData = 1,
    OutOfBuffers = 2,
    Discontinuity = 3,
}
