use crate::{at, av::audio, cf, define_obj_type, objc::Id};

define_obj_type!(TimeEffect(audio::Unit));

/// Unit that processes audio in non real-time
///
/// An TimeEffect represents an audio unit of type `aufc`.
/// These effects do not process audio in real-time. The varispeed
/// unit is an example of a time effect unit.
///
/// AVAudioUnitTimeEffect
impl TimeEffect {
    pub fn bypass(&self) -> bool {
        unsafe { rsel_bypass(self) }
    }

    pub fn set_bypass(&mut self, value: bool) {
        unsafe { wsel_setBypass(self, value) }
    }

    pub fn with_component_description(
        description: at::audio::ComponentDescription,
    ) -> cf::Retained<Self> {
        unsafe { AVAudioUnitTimeEffect_initWithAudioComponentDescription(description) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVAudioUnitTimeEffect_initWithAudioComponentDescription(
        description: at::audio::ComponentDescription,
    ) -> cf::Retained<TimeEffect>;
    fn rsel_bypass(id: &Id) -> bool;
    fn wsel_setBypass(id: &Id, value: bool);
}
