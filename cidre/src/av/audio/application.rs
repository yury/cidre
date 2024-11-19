use crate::{api, arc, av, blocks, define_obj_type, ns, objc};

#[doc(alias = "AVAudioApplicationRecordPermission")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum RecordPermission {
    /// The user has not yet been asked for permission.
    #[doc(alias = "AVAudioApplicationRecordPermissionUndetermined")]
    Undetermined = i32::from_be_bytes(*b"undt") as _,

    /// The user has been asked and has denied permission.
    #[doc(alias = "AVAudioApplicationRecordPermissionDenied")]
    Denied = i32::from_be_bytes(*b"deny") as _,

    /// The user has been asked and has granted permission.
    #[doc(alias = "AVAudioApplicationRecordPermissionGranted")]
    Granted = i32::from_be_bytes(*b"grnt") as _,
}

#[doc(alias = "AVAudioApplicationMicrophoneInjectionPermission")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum MicInjectionPermission {
    /// The user has disabled this service for all apps.
    #[doc(alias = "AVAudioApplicationMicrophoneInjectionPermissionServiceDisabled")]
    ServiceDisabled = i32::from_be_bytes(*b"srds") as _,

    /// The user has not yet been asked for permission.
    #[doc(alias = "AVAudioApplicationMicrophoneInjectionPermissionUndetermined")]
    Undetermined = i32::from_be_bytes(*b"undt") as _,

    /// The user has been asked and has denied permission.
    #[doc(alias = "AVAudioApplicationMicrophoneInjectionPermissionDenied")]
    Denied = i32::from_be_bytes(*b"deny") as _,

    /// The user has been asked and has granted permission.
    #[doc(alias = "AVAudioApplicationMicrophoneInjectionPermissionGranted")]
    Granted = i32::from_be_bytes(*b"grnt") as _,
}

impl ns::NotificationName {
    #[doc(alias = "AVAudioApplicationInputMuteStateChangeNotification")]
    #[inline]
    #[api::available(
        macos = 14.0,
        ios = 17.0,
        maccatalyst = 17.0,
        tvos = 17.0,
        visionos = 1.0,
        watchos = 10.0
    )]
    pub fn audio_app_input_mute_state_change() -> &'static Self {
        unsafe { AVAudioApplicationInputMuteStateChangeNotification }
    }
}

define_obj_type!(
    #[doc(alias = "AVAudioApplication")]
    pub App(ns::Id)
);

impl App {
    #[api::available(
        macos = 14.0,
        ios = 17.0,
        maccatalyst = 17.0,
        tvos = 17.0,
        visionos = 1.0,
        watchos = 10.0
    )]
    crate::define_cls!(AV_AUDIO_APPLICATION);

    #[doc(alias = "AVAudioApplicationInputMuteStateChangeNotification")]
    #[doc(alias = "AVAudioApplication.inputMuteStateChangeNotification")]
    #[inline]
    #[api::available(
        macos = 14.0,
        ios = 17.0,
        maccatalyst = 17.0,
        tvos = 17.0,
        visionos = 1.0,
        watchos = 10.0
    )]
    pub fn input_change_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioApplicationInputMuteStateChangeNotification }
    }

    #[doc(alias = "AVAudioApplicationMuteStateKey")]
    #[doc(alias = "AVAudioApplication.muteStateKey")]
    #[inline]
    #[api::available(
        macos = 14.0,
        ios = 17.0,
        maccatalyst = 17.0,
        tvos = 17.0,
        visionos = 1.0,
        watchos = 10.0
    )]
    pub fn mute_state_key() -> &'static ns::String {
        unsafe { AVAudioApplicationMuteStateKey }
    }

    #[objc::msg_send(sharedInstance)]
    pub fn shared() -> arc::R<Self>;

    #[objc::msg_send(isInputMuted)]
    pub fn is_input_mutted(&self) -> bool;

    #[objc::msg_send(setInputMuted:error:)]
    pub unsafe fn set_input_muted_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn set_input_muted<'ear>(&mut self, val: bool) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe { self.set_input_muted_err(val, err) })
    }

    #[objc::msg_send(recordPermission)]
    pub fn record_permission(&self) -> av::AudioAppRecordPermission;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(setInputMuteStateChangeHandler:)]
    pub fn set_input_mute_state_change_handler(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(input_should_be_muted: bool) -> bool>>,
    );

    #[objc::msg_send(requestRecordPermissionWithCompletionHandler:)]
    pub fn request_record_permission_ch_block(handler: &mut blocks::SendBlock<fn(bool)>);

    pub fn request_record_permission_ch(handler: impl FnMut(bool) + 'static + std::marker::Send) {
        let mut handler = blocks::SendBlock::new1(handler);
        Self::request_record_permission_ch_block(&mut handler)
    }

    #[cfg(feature = "async")]
    pub async fn request_record_permission() -> bool {
        let (future, mut block) = blocks::comp1();
        Self::request_record_permission_ch_block(&mut block);
        future.await
    }

    /// Returns an enum indicating whether the user has granted or denied permission to inject audio into input,
    /// or has not been asked
    #[objc::msg_send(microphoneInjectionPermission)]
    #[objc::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    pub fn mic_injection_permission(&self) -> MicInjectionPermission;

    #[objc::msg_send(requestMicrophoneInjectionPermissionWithCompletionHandler:)]
    #[objc::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    pub fn request_mic_injection_permission_ch_block(
        handler: &mut blocks::SendBlock<fn(MicInjectionPermission)>,
    );

    #[objc::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    pub fn request_mic_injection_permission_ch(
        handler: impl FnMut(MicInjectionPermission) + 'static + std::marker::Send,
    ) {
        let mut handler = blocks::SendBlock::new1(handler);
        Self::request_mic_injection_permission_ch_block(&mut handler)
    }

    #[cfg(feature = "async")]
    #[objc::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    pub async fn request_mic_injection_permission() -> MicInjectionPermission {
        let (future, mut block) = blocks::comp1();
        Self::request_mic_injection_permission_ch_block(&mut block);
        future.await
    }
}

#[link(name = "AVFAudio", kind = "framework")]
#[api::weak]
extern "C" {
    #[api::available(
        macos = 14.0,
        ios = 17.0,
        maccatalyst = 17.0,
        tvos = 17.0,
        visionos = 1.0,
        watchos = 10.0
    )]
    static AVAudioApplicationInputMuteStateChangeNotification: &'static ns::NotificationName;

    #[api::available(
        macos = 14.0,
        ios = 17.0,
        maccatalyst = 17.0,
        tvos = 17.0,
        visionos = 1.0,
        watchos = 10.0
    )]
    static AVAudioApplicationMuteStateKey: &'static ns::String;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_APPLICATION: &'static objc::Class<App>;
}

#[cfg(test)]
mod tests {
    use crate::av;
    #[test]
    fn basics() {
        let app = av::AudioApp::shared();
        assert_eq!(app.is_input_mutted(), false);
    }
}
