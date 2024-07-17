use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLAllocation")]
    pub Allocation(ns::Id)
);

impl Allocation {
    #[objc::msg_send(allocatedSize)]
    #[objc::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn allocated_size(&self) -> usize;
}
