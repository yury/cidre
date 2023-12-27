use crate::{arc, define_cls, objc};
use crate::{define_obj_type, ns};

mod types;
pub use types::ActivationOpts;
pub use types::Category;
pub use types::CategoryOpts;
pub use types::InterruptionOpts;
pub use types::InterruptionReason;
pub use types::InterruptionType;
pub use types::IoType;
pub use types::Mode;
pub use types::Port;
pub use types::PortOverride;
pub use types::PromptStyle;
pub use types::RecordPermission;
pub use types::RouteChangeReason;
pub use types::RouteSharingPolicy;
pub use types::SetActiveOpts;
pub use types::SilenceSecondaryAudioHintType;
pub use types::StereoOrientation;

mod route;
pub use route::ChannelDesc;
pub use route::DataSrcDesc;
pub use route::Location;
pub use route::Orientation;
pub use route::PolarPattern;
pub use route::PortDesc;
pub use route::RouteDesc;

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
define_obj_type!(pub Session(ns::Id));

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
impl Session {
    define_cls!(AV_AUDIO_SESSION);

    #[objc::cls_msg_send(sharedInstance)]
    pub fn shared() -> &'static mut Self;

    #[objc::msg_send(availableCategories)]
    pub fn available_categories_ar(self) -> arc::Rar<ns::Array<Category>>;

    #[objc::rar_retain]
    pub fn available_categories(self) -> arc::R<ns::Array<Category>>;

    #[objc::msg_send(setCategory:error:)]
    pub unsafe fn set_category_err<'ear>(
        &mut self,
        val: &Category,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_category<'ear>(&mut self, val: &Category) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_category_err(val, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(setCategory:withOptions:error:)]
    pub unsafe fn set_category_with_opts_err<'ear>(
        &mut self,
        val: &Category,
        options: CategoryOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_category_with_opts<'ear>(
        &mut self,
        val: &Category,
        options: CategoryOpts,
    ) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_category_with_opts_err(val, options, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(setCategory:mode:options:error:)]
    pub unsafe fn set_category_mode_opts_err<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        options: CategoryOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_category_mod_opts<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        options: CategoryOpts,
    ) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_category_mode_opts_err(val, mode, options, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(setCategory:mode:routeSharingPolicy:options:error:)]
    pub unsafe fn set_category_mode_policy_opts_err<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        route_sharing_policy: RouteSharingPolicy,
        options: CategoryOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_category_mode_policy_opts<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        route_sharing_policy: RouteSharingPolicy,
        options: CategoryOpts,
    ) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_category_mode_policy_opts_err(
                val,
                mode,
                route_sharing_policy,
                options,
                &mut err,
            ) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(policy)]
    pub fn category(&self) -> &Category;

    #[objc::msg_send(categoryOptions)]
    pub fn category_opts(&self) -> CategoryOpts;

    #[objc::msg_send(routeSharingPolicy)]
    pub fn route_sharing_policy(&self) -> RouteSharingPolicy;

    #[objc::msg_send(availableModes)]
    pub fn available_modes_ar(&self) -> arc::Rar<ns::Array<Mode>>;

    #[objc::rar_retain]
    pub fn available_modes(&self) -> arc::R<ns::Array<Mode>>;

    #[objc::msg_send(setMode:error:)]
    pub unsafe fn set_mode_err<'ear>(
        &mut self,
        val: &Mode,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_mode<'ear>(&mut self, val: &Mode) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_mode_err(val, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(mode)]
    pub fn mode(&self) -> &Mode;

    #[objc::msg_send(setAllowHapticsAndSystemSoundsDuringRecording:error:)]
    pub unsafe fn set_allow_haptics_and_sys_sounds_during_record_err<'ear>(
        &mut self,
        val: bool,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_allow_haptics_and_sys_sounds_during_record<'ear>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_allow_haptics_and_sys_sounds_during_record_err(val, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(allowHapticsAndSystemSoundsDuringRecording)]
    pub fn allow_haptics_and_sys_sounds_during_record(&self) -> bool;

    #[objc::msg_send(overrideOutputAudioPort:error:)]
    pub unsafe fn override_output_audio_port_err<'ear>(
        &mut self,
        val: PortOverride,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Use this method to temporarily override the output to built-in speaker.
    pub fn override_output_audio_port<'ear>(
        &mut self,
        val: PortOverride,
    ) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.override_output_audio_port_err(val, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
#[link(name = "AVFAudio", kind = "framework")]
extern "C" {
    static AV_AUDIO_SESSION: &'static objc::Class<Session>;
}
