use crate::{arc, at, av::audio, define_obj_type, objc::Id};

define_obj_type!(Effect(audio::Unit));

impl Effect {
    pub fn with_component_description(
        description: at::audio::ComponentDescription,
    ) -> arc::R<Self> {
        unsafe { AVAudioUnitEffect_initWithAudioComponentDescription(description) }
    }

    pub fn bypass(&self) -> bool {
        unsafe { rsel_bypass(self) }
    }

    pub fn set_bypass(&mut self, value: bool) {
        unsafe { wsel_setBypass(self, value) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVAudioUnitEffect_initWithAudioComponentDescription(
        description: at::audio::ComponentDescription,
    ) -> arc::R<Effect>;
    fn rsel_bypass(id: &Id) -> bool;
    fn wsel_setBypass(id: &Id, value: bool);
}
