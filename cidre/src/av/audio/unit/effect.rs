use crate::{arc, at, av::audio, define_cls, define_obj_type, objc};

define_obj_type!(pub Effect(audio::Unit));

impl arc::A<Effect> {
    #[objc::msg_send(initWithAudioComponentDescription:)]
    pub fn init_with_audio_component_desc(
        self,
        description: at::audio::ComponentDesc,
    ) -> arc::R<Effect>;
}

impl Effect {
    define_cls!(AV_AUDIO_UNIT_EFFECT);

    pub fn with_component_desc(description: at::audio::ComponentDesc) -> arc::R<Self> {
        Self::alloc().init_with_audio_component_desc(description)
    }

    #[objc::msg_send(bypass)]
    pub fn bypass(&self) -> bool;

    #[objc::msg_send(setBypass:)]
    pub fn set_bypass(&mut self, value: bool);
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_UNIT_EFFECT: &'static objc::Class<Effect>;
}
