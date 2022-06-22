use crate::{define_obj_type, ns};

use super::Engine;

define_obj_type!(Node(ns::Id));

impl Node {
    pub fn reset(&self) {
        unsafe { av_wsel_reset(self) }
    }

    pub fn engine(&self) -> Option<&Engine> {
        unsafe { rsel_engine(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn av_wsel_reset(id: &ns::Id);
    fn rsel_engine(id: &ns::Id) -> Option<&Engine>;
}
