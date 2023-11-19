use crate::{av, define_obj_type, ns, objc};

#[objc::obj_trait]
pub trait StereoMixing {
    #[objc::msg_send(pan)]
    fn pan(&self) -> f32;

    #[objc::msg_send(setPan:)]
    fn set_pan(&mut self, value: f32);
}

define_obj_type!(pub MixingDestination(ns::Id));

impl MixingDestination {
    #[objc::msg_send(connectionPoint)]
    pub fn connection_point(&self) -> &av::audio::ConnectionPoint;
}
impl av::audio::StereoMixing for MixingDestination {}
impl av::audio::Mixing for MixingDestination {}

#[objc::obj_trait]
pub trait Mixing: StereoMixing {
    #[objc::msg_send(destinationForMixer:bus:)]
    fn destination_for_mixer(
        &self,
        mixer: av::AudioNode,
        bus: av::AudioNodeBus,
    ) -> Option<&av::audio::MixingDestination>;

    #[objc::msg_send(volume)]
    fn volume(&self) -> f32;

    #[objc::msg_send(setVolume:)]
    fn set_volume(&self, value: f32);
}
