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
pub use types::RenderingMode;
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
    pub fn available_categories_ar(&self) -> arc::Rar<ns::Array<Category>>;

    #[objc::rar_retain]
    pub fn available_categories(&self) -> arc::R<ns::Array<Category>>;

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
    pub unsafe fn set_category_mode_policy_opts_err_throws<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        route_sharing_policy: RouteSharingPolicy,
        options: CategoryOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub unsafe fn set_category_mode_policy_opts_throws<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        route_sharing_policy: RouteSharingPolicy,
        options: CategoryOpts,
    ) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_category_mode_policy_opts_err_throws(
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

    pub fn set_category_mode_policy_opts<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        route_sharing_policy: RouteSharingPolicy,
        options: CategoryOpts,
    ) -> Result<(), ns::ExErr<'ear>> {
        ns::try_catch_err(|| unsafe {
            self.set_category_mode_policy_opts_throws(val, mode, route_sharing_policy, options)
        })
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

    #[objc::msg_send(preferredInput)]
    pub fn preferred_input(&self) -> Option<&PortDesc>;

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

/// Activation
impl Session {
    #[objc::msg_send(setActive:error:)]
    pub unsafe fn set_active_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_active<'ear>(&mut self, val: bool) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_active_err(val, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(setActive:withOptions:error:)]
    pub unsafe fn set_active_with_opts_err<'ear>(
        &mut self,
        val: bool,
        options: SetActiveOpts,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_active_with_opts<'ear>(
        &mut self,
        val: bool,
        options: SetActiveOpts,
    ) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_active_with_opts_err(val, options, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }
}

/// AVAudioSessionHardwareConfiguration
impl Session {
    #[objc::msg_send(setPreferredSampleRate:error:)]
    pub unsafe fn set_preferred_sample_rate_err<'ear>(
        &mut self,
        val: f64,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// The preferred hardware sample rate for the session. The actual sample rate may be different.
    pub fn set_preferred_sample_rate<'ear>(&mut self, val: f64) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_preferred_sample_rate_err(val, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(preferredSampleRate)]
    pub fn preferred_sample_rate(&self) -> f64;

    #[objc::msg_send(setPreferredIOBufferDuration:error:)]
    pub fn set_preferred_io_buff_duration_err<'ear>(
        &mut self,
        val: ns::TimeInterval,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_preferred_io_buff_duration<'ear>(
        &mut self,
        val: ns::TimeInterval,
    ) -> Result<(), &'ear ns::Error> {
        let mut err = None;
        unsafe {
            if self.set_preferred_io_buff_duration_err(val, &mut err) {
                Ok(())
            } else {
                Err(err.unwrap_unchecked())
            }
        }
    }

    //...
    #[objc::msg_send(inputDataSources)]
    pub fn input_data_sources_ar(&self) -> Option<arc::Rar<ns::Array<DataSrcDesc>>>;

    #[objc::rar_retain]
    pub fn input_data_sources(&self) -> Option<arc::R<ns::Array<DataSrcDesc>>>;
}

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
#[link(name = "AVFAudio", kind = "framework")]
extern "C" {
    static AV_AUDIO_SESSION: &'static objc::Class<Session>;
}

/// Notifications
impl Session {
    #[doc(alias = "AVAudioSessionInterruptionNotification")]
    #[inline]
    pub fn interruption_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionInterruptionNotification }
    }

    #[doc(alias = "AVAudioSessionRouteChangeNotification")]
    #[inline]
    pub fn route_change_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionRouteChangeNotification }
    }

    #[doc(alias = "AVAudioSessionMediaServicesWereLostNotification")]
    #[inline]
    pub fn media_services_where_lost_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionMediaServicesWereLostNotification }
    }

    #[doc(alias = "AVAudioSessionMediaServicesWereResetNotification")]
    #[inline]
    pub fn media_services_where_reset_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionMediaServicesWereResetNotification }
    }

    #[doc(alias = "AVAudioSessionSilenceSecondaryAudioHintNotification")]
    #[inline]
    pub fn silence_secondary_audio_hint_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionSilenceSecondaryAudioHintNotification }
    }

    #[doc(alias = "AVAudioSessionSpatialPlaybackCapabilitiesChangedNotification")]
    #[inline]
    pub fn spatial_playback_capabilities_changed_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionSpatialPlaybackCapabilitiesChangedNotification }
    }

    #[doc(alias = "AVAudioSessionRenderingModeChangeNotification")]
    #[inline]
    pub fn rendering_mode_change_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionRenderingModeChangeNotification }
    }
    #[doc(alias = "AVAudioSessionRenderingCapabilitiesChangeNotification")]
    #[inline]
    pub fn rendering_capabilities_change_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionRenderingCapabilitiesChangeNotification }
    }
}

extern "C" {
    static AVAudioSessionInterruptionNotification: &'static ns::NotificationName;
    static AVAudioSessionRouteChangeNotification: &'static ns::NotificationName;
    static AVAudioSessionMediaServicesWereLostNotification: &'static ns::NotificationName;
    static AVAudioSessionMediaServicesWereResetNotification: &'static ns::NotificationName;
    static AVAudioSessionSilenceSecondaryAudioHintNotification: &'static ns::NotificationName;
    static AVAudioSessionSpatialPlaybackCapabilitiesChangedNotification:
        &'static ns::NotificationName;
    static AVAudioSessionRenderingModeChangeNotification: &'static ns::NotificationName;
    static AVAudioSessionRenderingCapabilitiesChangeNotification: &'static ns::NotificationName;
}
