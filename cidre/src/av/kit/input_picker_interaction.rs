use crate::{api, define_obj_type, ns};
#[cfg(target_os = "ios")]
use crate::{arc, av, objc};

#[cfg(target_os = "ios")]
#[objc::protocol(AVInputPickerInteractionDelegate)]
pub trait InputPickerInteractionDelegate: objc::Obj {
    #[api::available(ios = 26.0)]
    #[objc::optional]
    #[objc::msg_send(inputPickerInteractionWillBeginPresenting:)]
    fn input_picker_interaction_will_begin_presenting(
        &mut self,
        interaction: &mut InputPickerInteraction,
    );

    #[api::available(ios = 26.0)]
    #[objc::optional]
    #[objc::msg_send(inputPickerInteractionDidEndPresenting:)]
    fn input_picker_interaction_did_end_presenting(
        &mut self,
        interaction: &mut InputPickerInteraction,
    );

    #[api::available(ios = 26.0)]
    #[objc::optional]
    #[objc::msg_send(inputPickerInteractionWillBeginDismissing:)]
    fn input_picker_interaction_will_begin_dismissing(
        &mut self,
        interaction: &mut InputPickerInteraction,
    );

    #[api::available(ios = 26.0)]
    #[objc::optional]
    #[objc::msg_send(inputPickerInteractionDidEndDismissing:)]
    fn input_picker_interaction_did_end_dismissing(
        &mut self,
        interaction: &mut InputPickerInteraction,
    );
}

#[cfg(target_os = "ios")]
define_obj_type!(pub AnyInputPickerInteractionDelegate(ns::Id));

#[cfg(target_os = "ios")]
impl InputPickerInteractionDelegate for AnyInputPickerInteractionDelegate {}

define_obj_type!(
    #[doc(alias = "AVInputPickerInteraction")]
    pub InputPickerInteraction(ns::Id),
    AV_INPUT_PICKER_INTERACTION,
    #[api::available(ios = 26.0)]
);

#[cfg(target_os = "ios")]
impl arc::A<InputPickerInteraction> {
    #[objc::msg_send(initWithAudioSession:)]
    pub fn init_with_audio_session(
        self,
        val: Option<&av::AudioSession>,
    ) -> arc::R<InputPickerInteraction>;
}

#[cfg(target_os = "ios")]
impl InputPickerInteraction {
    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyInputPickerInteractionDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: InputPickerInteractionDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(isPresented)]
    pub fn is_presented(&self) -> bool;

    #[objc::msg_send(audioSession)]
    pub fn audio_session(&self) -> arc::R<av::AudioSession>;

    #[objc::msg_send(setAudioSession:)]
    pub fn set_audio_session(&mut self, val: &av::AudioSession);

    #[objc::msg_send(present)]
    pub fn present(&mut self);

    #[objc::msg_send(dismiss)]
    pub fn dismiss(&mut self);

    #[api::available(ios = 26.0)]
    pub fn with_audio_session(val: Option<&av::AudioSession>) -> arc::R<Self> {
        Self::alloc().init_with_audio_session(val)
    }
}

#[link(name = "av_kit", kind = "static")]
unsafe extern "C" {
    #[cfg(target_os = "ios")]
    static AV_INPUT_PICKER_INTERACTION: &'static objc::Class<InputPickerInteraction>;
}
