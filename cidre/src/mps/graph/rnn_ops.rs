use crate::{arc, define_obj_type, ns, objc};

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum RNNActivation {
    None = 0,
    Relu,
    Tahn,
    Sigmoid,
    HardSigmoid,
}

define_obj_type!(
    SingleGateRNNDescriptor(ns::Id),
    MPS_GRAPH_SINGLE_GATE_RNN_DESCRIPTOR
);
impl SingleGateRNNDescriptor {
    /// If set then the input sequence is passed in reverse time order to the layer.
    /// Ignored when `bidirectional = true`
    /// Default value `false`
    #[objc::msg_send(reverse)]
    pub fn reverse(&self) -> bool;

    #[objc::msg_send(setReverse:)]
    pub fn set_reverse(&mut self, value: bool);

    /// If set then the input sequence is traversed in both directions and the two results
    /// are concatenated together on the channel-axis.
    /// Default value `false`
    #[objc::msg_send(bidirectional)]
    pub fn bidirectional(&self) -> bool;

    #[objc::msg_send(setBidirectional:)]
    pub fn set_bidirectional(&mut self, value: bool);

    /// If set then the layer will produce training state tensor as a secondary output.
    /// Default value `false`
    #[objc::msg_send(training)]
    pub fn training(&self) -> bool;

    #[objc::msg_send(setTraining:)]
    pub fn set_training(&mut self, value: bool);

    /// Activation function to use with the RNN op.
    /// Default value `None`
    #[objc::msg_send(activation)]
    pub fn activation(&self) -> RNNActivation;

    #[objc::msg_send(setActivation:)]
    pub fn set_activation(&mut self, value: RNNActivation);
}

define_obj_type!(GRUDDescriptor(ns::Id), MPS_GRAPH_GRUD_DESCRITPTOR);
impl GRUDDescriptor {
    #[objc::msg_send(reverse)]
    pub fn reverse(&self) -> bool;

    #[objc::msg_send(setReverse:)]
    pub fn set_reverse(&mut self, value: bool);

    #[objc::msg_send(bidirectional)]
    pub fn bidirectional(&self) -> bool;

    #[objc::msg_send(setBidirectional:)]
    pub fn set_bidirectional(&mut self, value: bool);

    #[objc::msg_send(training)]
    pub fn training(&self) -> bool;

    #[objc::msg_send(setTraining:)]
    pub fn set_training(&mut self, value: bool);

    /// If set then the layer will use the gate-ordering `[ r, z, o ]` instead of default `[ z, r, o ]`.
    #[objc::msg_send(resetGateFirst)]
    pub fn reset_gate_first(&self) -> bool;

    #[objc::msg_send(setResetGateFirst:)]
    pub fn set_reset_gate_first(&mut self, value: bool);

    #[objc::msg_send(resetAfter)]
    pub fn reset_after(&self) -> bool;

    #[objc::msg_send(setResetAfter:)]
    pub fn set_reset_after(&mut self, value: bool);

    #[objc::msg_send(updateGateActivation)]
    pub fn update_gate_activation(&self) -> RNNActivation;

    #[objc::msg_send(setUpdateGateActivation:)]
    pub fn set_update_gate_activation(&mut self, value: RNNActivation);

    #[objc::msg_send(resetGateActivation)]
    pub fn reset_gate_activation(&self) -> RNNActivation;

    #[objc::msg_send(setResetGateActivation:)]
    pub fn set_reset_gate_activation(&mut self, value: RNNActivation);

    #[objc::msg_send(outputGateActivation)]
    pub fn output_gate_activation(&self) -> RNNActivation;

    #[objc::msg_send(setOutputGateActivation:)]
    pub fn set_output_gate_activation(&mut self, value: RNNActivation);
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    static MPS_GRAPH_SINGLE_GATE_RNN_DESCRIPTOR: &'static objc::Class<SingleGateRNNDescriptor>;
    static MPS_GRAPH_GRUD_DESCRITPTOR: &'static objc::Class<GRUDDescriptor>;
}

#[cfg(test)]
mod test {
    use crate::mps::graph;

    #[test]
    fn basics() {
        let desc = graph::SingleGateRNNDescriptor::new();
        assert_eq!(desc.activation(), graph::RNNActivation::None);
    }
}
