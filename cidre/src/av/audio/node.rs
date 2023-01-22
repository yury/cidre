use crate::{av, define_obj_type, ns, objc};

define_obj_type!(Node(ns::Id));

impl Node {
    #[objc::msg_send(reset)]
    pub fn reset(&self);

    #[objc::msg_send(engine)]
    pub fn engine(&self) -> Option<&av::audio::Engine>;

    /// The node's number of input busses.
    #[objc::msg_send(numberOfInputs)]
    pub fn number_of_inputs(&self) -> usize;

    /// The node's number of output busses.
    #[objc::msg_send(numberOfOutputs)]
    pub fn number_of_outputs(&self) -> usize;
}
