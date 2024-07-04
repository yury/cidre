use crate::{av, blocks, define_cls, define_obj_type, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum RecordPermission {
    Undetermined = i32::from_be_bytes(*b"undt") as _,
    Denied = i32::from_be_bytes(*b"deny") as _,
    Granted = i32::from_be_bytes(*b"grnt") as _,
}

impl ns::NotificationName {
    #[inline]
    pub fn audio_app_input_mute_state_change() -> &'static Self {
        unsafe { AVAudioApplicationInputMuteStateChangeNotification }
    }
}

define_obj_type!(
    #[doc(alias = "AVAudioApplication")]
    pub App(ns::Id)
);

impl App {
    define_cls!(AV_AUDIO_APPLICATION);

    #[inline]
    pub fn input_change_notification() -> &'static ns::NotificationName {
        ns::NotificationName::audio_app_input_mute_state_change()
    }

    #[doc(alias = "AVAudioApplicationMuteStateKey")]
    #[inline]
    pub fn mute_state_key() -> &'static ns::String {
        unsafe { AVAudioApplicationMuteStateKey }
    }

    #[objc::msg_send(sharedInstance)]
    pub fn shared() -> &'static mut Self;

    #[objc::msg_send(isInputMuted)]
    pub fn is_input_mutted(&self) -> bool;

    #[objc::msg_send(setInputMuted:error:)]
    pub unsafe fn set_input_muted_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn set_input_muted<'ear>(&mut self, val: bool) -> Result<(), &'ear ns::Error> {
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
}

#[link(name = "AVFAudio", kind = "framework")]
extern "C" {
    static AVAudioApplicationInputMuteStateChangeNotification: &'static ns::NotificationName;
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
