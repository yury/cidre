use crate::{av::audio, define_obj_type, objc};

// AVAudioMixerNode
define_obj_type!(MixerNode(audio::Node));
impl MixerNode {
    /// The mixer's output volume.
    ///
    /// This accesses the mixer's output volume (0.0-1.0, inclusive).
    #[objc::msg_send(outputVolume)]
    pub fn output_volume(&self) -> f32;

    #[objc::msg_send(setOutputVolume:)]
    pub fn set_output_volume(&mut self, value: f32);

    /// Find an unused input bus.
    ///
    /// This will find and return the first input bus to which no other node is connected.
    #[objc::msg_send(nextAvailableInputBus)]
    pub fn next_available_input_bus(&self) -> audio::NodeBus;
}
