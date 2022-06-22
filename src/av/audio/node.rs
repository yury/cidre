use crate::{define_obj_type, ns};

use super::Engine;

define_obj_type!(Node(ns::Id));

impl Node {
    pub fn reset(&self) {
        unsafe { av_wsel_reset(self) }
    }

    #[inline]
    pub fn engine(&self) -> Option<&Engine> {
        unsafe { rsel_engine(self) }
    }

    /// The node's number of input busses.
    #[inline]
    pub fn number_of_inputs(&self) -> usize {
        unsafe { rsel_numberOfInputs(self) }
    }

    /// The node's number of output busses.
    #[inline]
    pub fn number_of_outputs(&self) -> usize {
        unsafe { rsel_numberOfOutputs(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn av_wsel_reset(id: &ns::Id);
    fn rsel_engine(id: &ns::Id) -> Option<&Engine>;

    fn rsel_numberOfInputs(id: &ns::Id) -> usize;
    fn rsel_numberOfOutputs(id: &ns::Id) -> usize;
}
