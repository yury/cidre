use crate::{arc, define_cls, define_obj_type, ns, objc};

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
define_obj_type!(
    #[doc(alias = "AVAudioSession")]
    pub Session(ns::Id)
);

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
impl Session {
    define_cls!(AV_AUDIO_SESSION);

    #[objc::msg_send(sharedInstance)]
    pub fn shared() -> &'static mut Self;

    #[objc::msg_send(availableCategories)]
    pub fn available_categories(&self) -> arc::R<ns::Array<Category>>;

    #[objc::msg_send(setCategory:error:)]
    pub unsafe fn set_category_err<'ear>(
        &mut self,
        val: &Category,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_category(&mut self, val: &Category) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_category_err(val, err) })
    }

    #[objc::msg_send(setCategory:withOptions:error:)]
    pub unsafe fn set_category_with_opts_err<'ear>(
        &mut self,
        val: &Category,
        options: CategoryOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_category_with_opts(&mut self, val: &Category, options: CategoryOpts) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_category_with_opts_err(val, options, err) })
    }

    #[objc::msg_send(setCategory:mode:options:error:)]
    pub unsafe fn set_category_mode_opts_err<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        options: CategoryOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_category_mod_opts(
        &mut self,
        val: &Category,
        mode: &Mode,
        options: CategoryOpts,
    ) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_category_mode_opts_err(val, mode, options, err) })
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
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe {
            self.set_category_mode_policy_opts_err_throws(
                val,
                mode,
                route_sharing_policy,
                options,
                err,
            )
        })
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
    pub fn available_modes(&self) -> arc::R<ns::Array<Mode>>;

    #[objc::msg_send(setMode:error:)]
    pub unsafe fn set_mode_err<'ear>(
        &mut self,
        val: &Mode,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_mode(&mut self, val: &Mode) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_mode_err(val, err) })
    }

    #[objc::msg_send(mode)]
    pub fn mode(&self) -> &Mode;

    #[objc::msg_send(setAllowHapticsAndSystemSoundsDuringRecording:error:)]
    pub unsafe fn set_allow_haptics_and_sys_sounds_during_record_err<'ear>(
        &mut self,
        val: bool,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_allow_haptics_and_sys_sounds_during_record(&mut self, val: bool) -> ns::Result {
        ns::if_false(|err| unsafe {
            self.set_allow_haptics_and_sys_sounds_during_record_err(val, err)
        })
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
    pub fn override_output_audio_port<'ear>(&mut self, val: PortOverride) -> ns::Result {
        ns::if_false(|err| unsafe { self.override_output_audio_port_err(val, err) })
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

    pub fn set_active(&mut self, val: bool) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_active_err(val, err) })
    }

    #[objc::msg_send(setActive:withOptions:error:)]
    pub unsafe fn set_active_with_opts_err<'ear>(
        &mut self,
        val: bool,
        options: SetActiveOpts,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_active_with_opts(&mut self, val: bool, options: SetActiveOpts) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_active_with_opts_err(val, options, err) })
    }
}

/// AVAudioSessionHardwareConfiguration
///
/// This category deals with the set of properties that reflect the current state of
/// audio hardware in the current route. Applications whose functionality depends on these
/// properties should reevaluate them any time the route changes.
impl Session {
    #[objc::msg_send(setPreferredSampleRate:error:)]
    pub unsafe fn set_preferred_sample_rate_err<'ear>(
        &mut self,
        val: f64,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// The preferred hardware sample rate for the session. The actual sample rate may be different.
    pub fn set_preferred_sample_rate(&mut self, val: f64) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_preferred_sample_rate_err(val, err) })
    }

    #[objc::msg_send(preferredSampleRate)]
    pub fn preferred_sample_rate(&self) -> f64;

    #[objc::msg_send(setPreferredIOBufferDuration:error:)]
    pub unsafe fn set_preferred_io_buff_duration_err<'ear>(
        &mut self,
        val: ns::TimeInterval,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_preferred_io_buff_duration(&mut self, val: ns::TimeInterval) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_preferred_io_buff_duration_err(val, err) })
    }

    #[objc::msg_send(setPreferredInputNumberOfChannels:error:)]
    pub unsafe fn set_preferred_input_channels_num_err<'ear>(
        &mut self,
        val: isize,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Sets the number of input channels that the app would prefer for the current route
    pub fn set_preferred_input_channels_num(&mut self, val: isize) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_preferred_input_channels_num_err(val, err) })
    }

    #[objc::msg_send(preferredOutputNumberOfChannels)]
    pub fn preferred_output_channels_num(&self) -> isize;

    #[objc::msg_send(setPreferredInputOrientation:error:)]
    pub unsafe fn set_preferred_input_orientation_err<'ear>(
        &mut self,
        val: &Orientation,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_preferred_input_orientation(&mut self, val: &Orientation) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_preferred_input_orientation_err(val, err) })
    }

    #[objc::msg_send(preferredInputOrientation)]
    pub fn preferred_input_orientation(&self) -> &Orientation;

    #[objc::msg_send(maximumInputNumberOfChannels)]
    pub fn max_input_channels_num(&self) -> isize;

    #[objc::msg_send(maximumOutputNumberOfChannels)]
    pub fn max_output_channels_num(&self) -> isize;

    #[objc::msg_send(setInputGain:error:)]
    pub unsafe fn set_input_gain_err<'ear>(
        &mut self,
        val: f32,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_input_gain(&mut self, val: f32) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_input_gain_err(val, err) })
    }

    #[objc::msg_send(isInputGainSettable)]
    pub fn is_input_gain_settable(&self) -> bool;

    #[objc::msg_send(isInputAvailable)]
    pub fn is_input_available(&self) -> bool;

    #[objc::msg_send(inputDataSources)]
    pub fn input_data_srcs(&self) -> Option<arc::R<ns::Array<DataSrcDesc>>>;

    #[objc::msg_send(setInputDataSource:error:)]
    pub unsafe fn set_input_data_src_err<'ear>(
        &mut self,
        val: Option<&DataSrcDesc>,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_input_data_src<'ear>(&mut self, val: Option<&DataSrcDesc>) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_input_data_src_err(val, err) })
    }

    #[objc::msg_send(outputDataSources)]
    pub fn output_data_srcs(&self) -> Option<arc::R<ns::Array<DataSrcDesc>>>;

    #[objc::msg_send(outputDataSource)]
    pub fn output_data_src(&self) -> Option<&DataSrcDesc>;

    #[objc::msg_send(setOutputDataSource:error:)]
    pub unsafe fn set_output_data_src_err<'ear>(
        &mut self,
        val: Option<&DataSrcDesc>,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_output_data_src(&mut self, val: Option<&DataSrcDesc>) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_output_data_src_err(val, err) })
    }

    /// The current hardware sample rate
    #[objc::msg_send(sampleRate)]
    pub fn sample_rate(&self) -> f64;

    #[objc::msg_send(inputNumberOfChannels)]
    pub fn input_channels_num(&self) -> isize;

    #[objc::msg_send(outputNumberOfChannels)]
    pub fn output_channels_num(&self) -> isize;

    #[objc::msg_send(inputLatency)]
    pub fn input_latency(&self) -> ns::TimeInterval;

    #[objc::msg_send(outputLatency)]
    pub fn output_latency(&self) -> ns::TimeInterval;

    #[objc::msg_send(IOBufferDuration)]
    pub fn io_buf_duration(&self) -> ns::TimeInterval;
}

/// Observation
impl Session {
    /// True when another application is playing audio.
    ///
    /// As of iOS 8.0, Apple recommends that most applications use
    /// secondaryAudioShouldBeSilencedHint instead of this property. The otherAudioPlaying property
    /// will be true if any other audio (including audio from an app using
    /// AVAudioSessionCategoryAmbient) is playing, whereas the secondaryAudioShouldBeSilencedHint
    /// property is more restrictive in its consideration of whether primary audio from another
    /// application is playing.
    #[objc::msg_send(isOtherAudioPlaying)]
    pub fn is_other_audio_playing(&self) -> bool;

    #[objc::msg_send(secondaryAudioShouldBeSilencedHint)]
    pub fn secondary_audio_should_be_silenced_hint(&self) -> bool;

    /// The current output volume. Value in range \[0.0, 1.0\]. Is key-value observable.
    #[objc::msg_send(outputVolume)]
    pub fn output_volume(&self) -> f32;

    #[objc::msg_send(promptStyle)]
    pub fn prompt_style(&self) -> PromptStyle;
}

/// RoutingConfiguration
impl Session {
    /// Note that this property only applies to the session's current category and mode. For
    /// example, if the session's current category is AVAudioSessionCategoryPlayback, there will be
    /// no available inputs.
    #[objc::msg_send(availableInputs)]
    pub fn available_inputs(&self) -> Option<arc::R<ns::Array<PortDesc>>>;

    /// A description of the current route, consisting of zero or more input ports and zero or more
    /// output ports
    #[objc::msg_send(currentRoute)]
    pub fn current_route(&self) -> arc::R<RouteDesc>;

    #[objc::msg_send(setAggregatedIOPreference:error:)]
    pub unsafe fn set_aggregated_io_preference_err<'ear>(
        &mut self,
        val: IoType,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Controls whether audio input and output are aggregated. Only valid in combination with
    /// AVAudioSessionCategoryPlayAndRecord or AVAudioSessionCategoryMultiRoute.
    ///
    /// See the AVAudioSessionIOType documentation for a more detailed explanation of why a client may
    /// want to change the IO type.
    pub fn set_aggregated_io_preference(&mut self, val: IoType) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_aggregated_io_preference_err(val, err) })
    }

    #[objc::msg_send(setSupportsMultichannelContent:error:)]
    pub unsafe fn set_supports_multichannel_content_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_supports_multichannel_content(&mut self, val: bool) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_supports_multichannel_content_err(val, err) })
    }

    #[objc::msg_send(supportsMultichannelContent)]
    pub fn supports_multichannel_content(&self) -> bool;

    #[objc::msg_send(setPrefersInterruptionOnRouteDisconnect:error:)]
    pub unsafe fn set_prefers_interruption_on_route_disconnect_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Use this method to opt in or opt out of interruption on route disconnect policy.
    ///
    /// As described in the Audio Session Programming Guide, most media playback apps are expected
    /// to pause playback if the route change reason is AVAudioSessionRouteChangeReasonOldDeviceUnavailable.
    ///
    /// Starting in iOS 17, by default Now Playing sessions will be interrupted if they are active
    /// when a route change occurs because of a disconnect event. All other sessions will not be
    /// interrupted due to a disconnect event.
    pub fn set_prefers_interruption_on_route_disconnect(&mut self, val: bool) -> ns::Result {
        ns::if_false(|err| unsafe {
            self.set_prefers_interruption_on_route_disconnect_err(val, err)
        })
    }

    /// Indicates if session will be interrupted on route disconnect.
    #[objc::msg_send(prefersInterruptionOnRouteDisconnect)]
    pub fn prefers_interruption_on_route_disconnect(&self) -> bool;
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
