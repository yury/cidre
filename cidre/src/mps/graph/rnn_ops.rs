use crate::{arc, define_obj_type, mps::graph, ns, objc};

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
define_obj_type!(LSTMDescriptor(ns::Id), MPS_GRAPH_LSTM_DESCRIPTOR);
impl LSTMDescriptor {
    #[objc::msg_send(reverse)]
    pub fn reverse(&self) -> bool;

    #[objc::msg_send(setReverse:)]
    pub fn set_reverse(&mut self, value: bool);

    #[objc::msg_send(bidirectional)]
    pub fn bidirectional(&self) -> bool;

    #[objc::msg_send(setBidirectional:)]
    pub fn set_bidirectional(&mut self, value: bool);

    #[objc::msg_send(produceCell)]
    pub fn produce_cell(&self) -> bool;

    #[objc::msg_send(setProduceCell:)]
    pub fn set_produce_cell(&mut self, value: bool);

    #[objc::msg_send(training)]
    pub fn training(&self) -> bool;

    #[objc::msg_send(setTraining:)]
    pub fn set_training(&mut self, value: bool);

    #[objc::msg_send(forgetGateLast)]
    pub fn forget_gate_last(&self) -> bool;

    #[objc::msg_send(setForgetGateLast:)]
    pub fn set_forget_gate_last(&mut self, value: bool);

    #[objc::msg_send(inputGateActivation)]
    pub fn input_gate_activation(&self) -> RNNActivation;

    #[objc::msg_send(setInputGateActivation:)]
    pub fn set_input_gate_activation(&mut self, value: RNNActivation);

    #[objc::msg_send(forgetGateActivation)]
    pub fn forget_gate_activation(&self) -> RNNActivation;

    #[objc::msg_send(setForgetGateActivation:)]
    pub fn set_forget_gate_activation(&mut self, value: RNNActivation);

    #[objc::msg_send(cellGateActivation)]
    pub fn cell_gate_activation(&self) -> RNNActivation;

    #[objc::msg_send(setCellGateActivation:)]
    pub fn set_cell_gate_activation(&mut self, value: RNNActivation);

    #[objc::msg_send(outputGateActivation)]
    pub fn output_gate_activation(&self) -> RNNActivation;

    #[objc::msg_send(setOutputGateActivation:)]
    pub fn set_output_gate_activation(&mut self, value: RNNActivation);

    #[objc::msg_send(activation)]
    pub fn activation(&self) -> RNNActivation;

    #[objc::msg_send(setActivation:)]
    pub fn set_activation(&mut self, value: RNNActivation);
}

define_obj_type!(GRUDescriptor(ns::Id), MPS_GRAPH_GRU_DESCRIPTOR);
impl GRUDescriptor {
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

impl graph::Graph {
    #[objc::msg_send(singleGateRNNWithSourceTensor:recurrentWeight:inputWeight:bias:initState:mask:descriptor:name:)]
    pub fn single_gate_rnn_bias_mask_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        mask: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn single_gate_rnn_bias_mask(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        mask: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(singleGateRNNWithSourceTensor:recurrentWeight:inputWeight:bias:initState:descriptor:name:)]
    pub fn single_gate_rnn_bias_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn single_gate_rnn_bias(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(singleGateRNNWithSourceTensor:recurrentWeight:initState:descriptor:name:)]
    pub fn single_gate_rnn_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        init_state: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn single_gate_rnn(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        init_state: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(singleGateRNNGradientsWithSourceTensor:recurrentWeight:sourceGradient:
        zState:stageGradient:inputWeight:bias:initState:mask:descriptor:name:)]
    pub fn single_gate_rnn_grad_stage_mask_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        source_gradient: &graph::Tensor,
        z_state: &graph::Tensor,
        stage_gradient: Option<&graph::Tensor>,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        mask: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn single_gate_rnn_grad_stage_mask(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        source_gradient: &graph::Tensor,
        z_state: &graph::Tensor,
        stage_gradient: Option<&graph::Tensor>,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        mask: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(singleGateRNNGradientsWithSourceTensor:recurrentWeight:sourceGradient:
        zState:inputWeight:bias:initState:mask:descriptor:name:)]
    pub fn single_gate_rnn_grad_bias_mask_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        source_gradient: &graph::Tensor,
        z_state: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        mask: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn single_gate_rnn_grad_bias_mask(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        source_gradient: &graph::Tensor,
        z_state: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        mask: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(singleGateRNNGradientsWithSourceTensor:recurrentWeight:sourceGradient:
        zState:inputWeight:bias:initState:descriptor:name:)]
    pub fn single_gate_rnn_grad_bias_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        source_gradient: &graph::Tensor,
        z_state: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn single_gate_rnn_grad_bias(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        source_gradient: &graph::Tensor,
        z_state: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(singleGateRNNGradientsWithSourceTensor:recurrentWeight:sourceGradient:
        zState:initState:descriptor:name:)]
    pub fn single_gate_rnn_grad_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        source_gradient: &graph::Tensor,
        z_state: &graph::Tensor,
        init_state: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn single_gate_rnn_grad(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        source_gradient: &graph::Tensor,
        z_state: &graph::Tensor,
        init_state: Option<&graph::Tensor>,
        descriptor: &SingleGateRNNDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(LSTMWithSourceTensor:recurrentWeight:inputWeight:bias:initState:
        initCell:mask:peephole:descriptor:name:)]
    pub fn lstm_bias_mask_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        init_cell: Option<&graph::Tensor>,
        mask: Option<&graph::Tensor>,
        peephole: Option<&graph::Tensor>,
        descriptor: &LSTMDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn lstm_bias_mask(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        init_cell: Option<&graph::Tensor>,
        mask: Option<&graph::Tensor>,
        peephole: Option<&graph::Tensor>,
        descriptor: &LSTMDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(LSTMWithSourceTensor:recurrentWeight:inputWeight:bias:initState:
        initCell:descriptor:name:)]
    pub fn lstm_bias_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        init_cell: Option<&graph::Tensor>,
        descriptor: &LSTMDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn lstm_bias(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        input_weight: Option<&graph::Tensor>,
        bias: Option<&graph::Tensor>,
        init_state: Option<&graph::Tensor>,
        init_cell: Option<&graph::Tensor>,
        descriptor: &LSTMDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(LSTMWithSourceTensor:recurrentWeight:initState:
        initCell:descriptor:name:)]
    pub fn lstm_ar(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        init_state: Option<&graph::Tensor>,
        init_cell: Option<&graph::Tensor>,
        descriptor: &LSTMDescriptor,
        name: Option<&ns::String>,
    ) -> arc::Rar<ns::Array<graph::Tensor>>;

    #[objc::rar_retain]
    pub fn lstm(
        &self,
        source: &graph::Tensor,
        recurrent_weight: &graph::Tensor,
        init_state: Option<&graph::Tensor>,
        init_cell: Option<&graph::Tensor>,
        descriptor: &LSTMDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<ns::Array<graph::Tensor>>;
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    static MPS_GRAPH_SINGLE_GATE_RNN_DESCRIPTOR: &'static objc::Class<SingleGateRNNDescriptor>;
    static MPS_GRAPH_LSTM_DESCRIPTOR: &'static objc::Class<LSTMDescriptor>;
    static MPS_GRAPH_GRU_DESCRIPTOR: &'static objc::Class<GRUDescriptor>;
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
