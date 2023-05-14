use crate::{av, blocks::Block, define_obj_type, ns, objc};

define_obj_type!(Node(ns::Id));

impl Node {
    #[objc::msg_send(reset)]
    pub fn reset(&self);

    #[objc::msg_send(inputFormatForBus:)]
    pub fn input_format_for_bus(&self, bus: av::AudioNodeBus) -> &av::AudioFormat;

    /// Gets the name of the input bus you specify.
    #[objc::msg_send(nameForInputBus:)]
    pub fn name_for_input_bus(&self, bus: av::AudioNodeBus) -> Option<&ns::String>;

    #[objc::msg_send(engine)]
    pub fn engine(&self) -> Option<&av::audio::Engine>;

    /// The node's number of input busses.
    #[objc::msg_send(numberOfInputs)]
    pub fn number_of_inputs(&self) -> usize;

    /// The node's number of output busses.
    #[objc::msg_send(numberOfOutputs)]
    pub fn number_of_outputs(&self) -> usize;

    /// Obtain the time for which the node most recently rendered.
    ///
    /// Will return None if the engine is not running or if the node is not connected to an input or
    /// output node.
    #[objc::msg_send(lastRenderTime)]
    pub fn last_render_time(&self) -> Option<av::AudioTime>;

    /// The processing latency of the node, in seconds.
    ///
    /// This property reflects the delay between when an impulse in the audio stream arrives at the
    /// input vs. output of the node. This should reflect the delay due to signal processing
    /// (e.g. filters, FFT's, etc.), not delay or reverberation which is being applied as an effect.
    /// A value of zero indicates either no latency or an unknown latency.
    #[objc::msg_send(latency)]
    pub fn latency(&self) -> ns::TimeInterval;

    /// The maximum render pipeline latency downstream of the node, in seconds.
    #[objc::msg_send(outputPresentationLatency)]
    pub fn output_presentation_latency(&self) -> ns::TimeInterval;

    #[objc::msg_send(installTapOnBus:bufferSize:format:block:)]
    pub fn install_tap_on_bus<'b, B>(
        &mut self,
        bus: av::AudioNodeBus,
        buffer_size: av::AudioFrameCount,
        format: Option<&av::AudioFormat>,
        tap_block: &'static Block<B>,
    ) where
        B: FnMut(&'b av::AudioPCMBuffer, &'b av::AudioTime);

    #[objc::msg_send(removeTapOnBus:)]
    pub fn remove_tap_on_bus(&mut self, bus: av::AudioNodeBus);
}
