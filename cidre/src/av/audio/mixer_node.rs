use crate::{arc, av::audio, define_obj_type, objc};

define_obj_type!(
    #[doc(alias = "AVAudioMixerNode")]
    pub MixerNode(audio::Node),
    AV_AUDIO_MIXER_NODE
);

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

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_AUDIO_MIXER_NODE: &'static objc::Class<MixerNode>;
}
