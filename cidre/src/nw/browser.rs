use crate::{define_obj_type, ns};

#[doc(alias = "nw_browser_state_t")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum State {
    Invalid = 0,
    Ready = 1,
    Failed = 2,
    Cancelled = 3,
    Waiting = 4,
}

define_obj_type!(
    #[doc(alias = "nw_browser")]
    #[doc(alias = "nw_browser_t")]
    pub Browser(ns::Id)
);

impl Browser {}

#[link(name = "Network", kind = "framework")]
extern "C" {}
