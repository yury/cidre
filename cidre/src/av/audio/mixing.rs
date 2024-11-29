use crate::{arc, av, define_obj_type, ns, objc};

#[objc::protocol(AVStereoMixing)]
pub trait StereoMixing {
    #[objc::msg_send(pan)]
    fn pan(&self) -> f32;

    #[objc::msg_send(setPan:)]
    fn set_pan(&mut self, value: f32);
}

define_obj_type!(
    #[doc(alias = "AVAudioMixingDestination")]
    pub MixingDestination(ns::Id)
);

impl MixingDestination {
    #[objc::msg_send(connectionPoint)]
    pub fn connection_point(&self) -> arc::R<av::audio::ConnectionPoint>;
}
impl av::audio::StereoMixing for MixingDestination {}
impl av::audio::Mixing for MixingDestination {}

#[objc::protocol(AVAudioMixing)]
pub trait Mixing: StereoMixing {
    #[objc::msg_send(destinationForMixer:bus:)]
    fn destination_for_mixer(
        &self,
        mixer: av::AudioNode,
        bus: av::AudioNodeBus,
    ) -> Option<arc::R<av::audio::MixingDestination>>;

    #[objc::msg_send(volume)]
    fn volume(&self) -> f32;

    #[objc::msg_send(setVolume:)]
    fn set_volume(&mut self, value: f32);
}
