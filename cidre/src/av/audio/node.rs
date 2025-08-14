use crate::{arc, av, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(pub Node(ns::Id));

#[cfg(feature = "blocks")]
pub type AudioNodeTapBlock<Attr> = blocks::Block<fn(&av::AudioPcmBuf, &av::AudioTime), Attr>;

impl Node {
    #[objc::msg_send(reset)]
    pub fn reset(&self);

    #[objc::msg_send(inputFormatForBus:)]
    pub fn input_format_for_bus(&self, bus: av::AudioNodeBus) -> arc::R<av::AudioFormat>;

    #[objc::msg_send(outputFormatForBus:)]
    pub fn output_format_for_bus(&self, bus: av::AudioNodeBus) -> arc::R<av::AudioFormat>;

    /// Gets the name of the input bus you specify.
    #[objc::msg_send(nameForInputBus:)]
    pub fn name_for_input_bus(&self, bus: av::AudioNodeBus) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(engine)]
    pub fn engine(&self) -> Option<arc::R<av::audio::Engine>>;

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

    /// NOTE: `remove_tap_on_bus` if you have already installed tap
    #[cfg(feature = "blocks")]
    #[objc::msg_send(installTapOnBus:bufferSize:format:block:)]
    pub unsafe fn install_tap_on_bus_block_throws(
        &mut self,
        bus: av::AudioNodeBus,
        buffer_size: av::AudioFrameCount,
        format: Option<&av::AudioFormat>,
        tap_block: &mut AudioNodeTapBlock<blocks::Esc>,
    );

    #[cfg(feature = "blocks")]
    #[inline]
    pub unsafe fn install_tap_on_bus_throws(
        &mut self,
        bus: av::AudioNodeBus,
        buffer_size: av::AudioFrameCount,
        format: Option<&av::AudioFormat>,
        tap_block: impl FnMut(&av::AudioPcmBuf, &av::AudioTime) + 'static,
    ) {
        let mut tap_block = AudioNodeTapBlock::<blocks::Esc>::new2(tap_block);
        unsafe { self.install_tap_on_bus_block_throws(bus, buffer_size, format, &mut tap_block) }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn install_tap_on_bus<'ear>(
        &mut self,
        bus: av::AudioNodeBus,
        buffer_size: av::AudioFrameCount,
        format: Option<&av::AudioFormat>,
        tap_block: impl FnMut(&av::AudioPcmBuf, &av::AudioTime) + 'static,
    ) -> ns::ExResult<'ear> {
        let mut tap_block = AudioNodeTapBlock::<blocks::Esc>::new2(tap_block);
        ns::try_catch(|| unsafe {
            self.install_tap_on_bus_block_throws(bus, buffer_size, format, &mut tap_block)
        })
    }

    #[objc::msg_send(removeTapOnBus:)]
    pub unsafe fn remove_tap_on_bus_throws(&mut self, bus: av::AudioNodeBus);

    pub fn remove_tap_on_bus<'ear>(&mut self, bus: av::AudioNodeBus) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.remove_tap_on_bus_throws(bus) })
    }
}
