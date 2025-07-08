use crate::api;
use crate::{arc, define_cls, define_obj_type, ns, objc};

mod types;
pub use types::ActivationOpts;
pub use types::Category;
pub use types::CategoryOpts;
pub use types::InterruptionOpts;
pub use types::InterruptionReason;
pub use types::InterruptionType;
pub use types::IoType;
pub use types::MicInjectionMode;
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
pub use route::PortExtensionBluetoothMic;
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

    pub fn set_category<'ear>(&mut self, val: &Category) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe { self.set_category_err(val, err) })
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
    ) -> ns::Result<'ear> {
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

    pub fn set_category_mod_opts<'ear>(
        &mut self,
        val: &Category,
        mode: &Mode,
        options: CategoryOpts,
    ) -> ns::Result<'ear> {
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

    pub fn set_mode<'ear>(&mut self, val: &Mode) -> ns::Result<'ear> {
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

    pub fn set_allow_haptics_and_sys_sounds_during_record<'ear>(
        &mut self,
        val: bool,
    ) -> ns::Result<'ear> {
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
    pub fn override_output_audio_port<'ear>(&mut self, val: PortOverride) -> ns::Result<'ear> {
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

    pub fn set_active<'ear>(&mut self, val: bool) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe { self.set_active_err(val, err) })
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
    ) -> ns::Result<'ear> {
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
    pub fn set_preferred_sample_rate<'ear>(&mut self, val: f64) -> ns::Result<'ear> {
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

    pub fn set_preferred_io_buff_duration<'ear>(
        &mut self,
        val: ns::TimeInterval,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe { self.set_preferred_io_buff_duration_err(val, err) })
    }

    #[objc::msg_send(setPreferredInputNumberOfChannels:error:)]
    pub unsafe fn set_preferred_input_channels_num_err<'ear>(
        &mut self,
        val: isize,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Sets the number of input channels that the app would prefer for the current route
    pub fn set_preferred_input_channels_num<'ear>(&mut self, val: isize) -> ns::Result<'ear> {
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

    pub fn set_preferred_input_orientation<'ear>(&mut self, val: &Orientation) -> ns::Result<'ear> {
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

    pub fn set_input_gain<'ear>(&mut self, val: f32) -> ns::Result<'ear> {
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

    pub fn set_input_data_src<'ear>(&mut self, val: Option<&DataSrcDesc>) -> ns::Result<'ear> {
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

    pub fn set_output_data_src<'ear>(&mut self, val: Option<&DataSrcDesc>) -> ns::Result<'ear> {
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
    pub fn set_aggregated_io_preference<'ear>(&mut self, val: IoType) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe { self.set_aggregated_io_preference_err(val, err) })
    }

    #[objc::msg_send(setSupportsMultichannelContent:error:)]
    pub unsafe fn set_supports_multichannel_content_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_supports_multichannel_content<'ear>(&mut self, val: bool) -> ns::Result<'ear> {
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

    #[objc::msg_send(setPrefersNoInterruptionsFromSystemAlerts:error:)]
    pub unsafe fn set_prefers_no_interruptions_from_sys_alerts_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_prefers_no_interruptions_from_sys_alerts<'ear>(
        &mut self,
        val: bool,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe {
            self.set_prefers_no_interruptions_from_sys_alerts_err(val, err)
        })
    }

    #[objc::msg_send(prefersNoInterruptionsFromSystemAlerts)]
    pub fn prefers_no_interruptions_from_sys_alerts();

    /// Use this method to opt in or opt out of interruption on route disconnect policy.
    ///
    /// As described in the Audio Session Programming Guide, most media playback apps are expected
    /// to pause playback if the route change reason is AVAudioSessionRouteChangeReasonOldDeviceUnavailable.
    ///
    /// Starting in iOS 17, by default Now Playing sessions will be interrupted if they are active
    /// when a route change occurs because of a disconnect event. All other sessions will not be
    /// interrupted due to a disconnect event.
    pub fn set_prefers_interruption_on_route_disconnect<'ear>(
        &mut self,
        val: bool,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe {
            self.set_prefers_interruption_on_route_disconnect_err(val, err)
        })
    }

    /// Indicates if session will be interrupted on route disconnect.
    #[objc::msg_send(prefersInterruptionOnRouteDisconnect)]
    pub fn prefers_interruption_on_route_disconnect(&self) -> bool;
}

/// MicrophoneInjection
impl Session {
    #[objc::msg_send(setPreferredMicrophoneInjectionMode:error:)]
    #[api::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    pub unsafe fn set_preferred_mic_injection_mode_err<'ear>(
        &mut self,
        val: MicInjectionMode,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Set the preferred form of audio injection into another app's input stream
    #[objc::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    pub fn set_preferred_mic_injection_mode<'ear>(&mut self, val: MicInjectionMode) -> ns::Result {
        let if_false =
            ns::if_false(|err| unsafe { self.set_preferred_mic_injection_mode_err(val, err) });
        let if_false = if_false;
        if_false
    }

    #[objc::msg_send(preferredMicrophoneInjectionMode)]
    #[api::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    pub fn preferred_mic_injection_mode(&self) -> MicInjectionMode;

    #[objc::msg_send(isMicrophoneInjectionAvailable)]
    #[api::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    pub fn is_mic_injection_available(&self) -> bool;

    #[objc::msg_send(setPrefersEchoCancelledInput:error:)]
    #[api::available(ios = 18.2, maccatalyst = 18.2)]
    pub unsafe fn set_prefers_echo_cancelled_input_err<'ear>(
        &mut self,
        val: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Set a preference to enable echo cancelled input on supported hardware
    ///
    /// Applications might want to record the built-in microphone's input while also playing audio out via the built-in speaker.
    /// Enabling echo cancelled input is useful when the application needs the input signal to be clear of any echoes
    /// from the audio playing out of the built-in speaker.
    ///
    /// Audio sessions using Voice Processor don't need this option as echo cancellation is implicitly applied for those routes.
    /// The Voice Processor solution is tuned for voice signals, unlike this option, which is tuned for better capture
    /// of wider range of audio signals in the presence of built-in speaker echo.
    ///
    /// This option is valid only when used with AVAudioSessionCategoryPlayAndRecord and AVAudioSessionModeDefault and is only available
    /// on certain 2024 or later iPhone models. Support can be queried using property `isEchoCancelledInputAvailable`.
    /// Other recording sessions might be interrupted if this option is not compatible with sessions that are already recording.
    ///
    /// After an audio session goes active, `isEchoCancelledInputEnabled` property can be queried to check if the option was honored.
    /// Note that the enabled state may change after route changes, e.g. if user plugs in a headset, that route might not support echo cancellation.
    #[api::available(ios = 18.2, maccatalyst = 18.2)]
    pub fn set_prefers_echo_cancelled_input(&mut self, val: bool) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_prefers_echo_cancelled_input_err(val, err) })
    }

    #[objc::msg_send(prefersEchoCancelledInput)]
    #[api::available(ios = 18.2, maccatalyst = 18.2)]
    pub fn prefers_echo_cancelled_input(&self) -> bool;

    #[objc::msg_send(isEchoCancelledInputEnabled)]
    #[api::available(ios = 18.2, maccatalyst = 18.2)]
    pub fn is_echo_cancelled_input_enabled(&self) -> bool;

    #[objc::msg_send(isEchoCancelledInputAvailable)]
    #[api::available(ios = 18.2, maccatalyst = 18.2)]
    pub fn is_echo_cancelled_input_available(&self) -> bool;
}

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
#[link(name = "av", kind = "static")]
unsafe extern "C" {
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

    #[doc(alias = "AVAudioSessionMicrophoneInjectionCapabilitiesChangeNotification")]
    #[api::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    #[inline]
    pub fn mic_injection_capabilities_change_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionMicrophoneInjectionCapabilitiesChangeNotification }
    }

    #[doc(alias = "AVAudioSessionOutputMuteStateChangeNotification")]
    #[api::available(macos = 26.0, ios = 26.0)]
    pub fn output_mute_state_change_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionOutputMuteStateChangeNotification }
    }

    #[doc(alias = "AVAudioSessionUserIntentToUnmuteOutputNotification")]
    #[api::available(ios = 26.0)]
    pub fn user_intent_to_unmute_output_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionUserIntentToUnmuteOutputNotification }
    }

    #[doc(alias = "AVAudioSessionAvailableInputsChangeNotification")]
    #[api::available(ios = 26.0)]
    pub fn available_inputes_change_notification() -> &'static ns::NotificationName {
        unsafe { AVAudioSessionAvailableInputsChangeNotification }
    }
}

/// Keys for ns::Notification user_info dictionaries
pub mod keys {
    use crate::{api, ns};

    /// value is an ns::Number whose boolean value indicates if spatial audio enabled.
    ///
    /// key for AVAudioSessionSpatialPlaybackCapabilitiesChangedNotification
    #[doc(alias = "AVAudioSessionSpatialAudioEnabledKey")]
    #[api::available(ios = 15.0, watchos = 8.0, tvos = 15.0)]
    pub fn spacial_audio_enabled() -> &'static ns::String {
        unsafe { AVAudioSessionSpatialAudioEnabledKey }
    }

    /// Value is an ns::Number representing an av::AudioSessionInterruptionType
    #[doc(alias = "AVAudioSessionInterruptionTypeKey")]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn interruption_type() -> &'static ns::String {
        unsafe { AVAudioSessionInterruptionTypeKey }
    }

    /// Only present for end interruption events. Value is of type av::AudioSessionInterruptionOptions.
    #[doc(alias = "AVAudioSessionInterruptionOptionKey")]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn interruption_option() -> &'static ns::String {
        unsafe { AVAudioSessionInterruptionOptionKey }
    }

    /// Only present in begin interruption events. Value is of type av::AudioSessionInterruptionReason.
    #[doc(alias = "AVAudioSessionInterruptionReasonKey")]
    #[api::available(ios = 14.5, watchos = 7.3)]
    pub fn interruption_reason() -> &'static ns::String {
        unsafe { AVAudioSessionInterruptionReasonKey }
    }

    /// Value is an ns::Number representing an av::AudioSessionRouteChangeReason
    #[doc(alias = "AVAudioSessionRouteChangeReasonKey")]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn route_change_reason() -> &'static ns::String {
        unsafe { AVAudioSessionRouteChangeReasonKey }
    }

    /// Value is av::AudioSessionRouteDesc
    #[doc(alias = "AVAudioSessionRouteChangePreviousRouteKey")]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn route_change_previuous_route() -> &'static ns::String {
        unsafe { AVAudioSessionRouteChangePreviousRouteKey }
    }

    /// Value is an ns::Number representing an AVAudioSessionSilenceSecondaryAudioHintType
    #[doc(alias = "AVAudioSessionSilenceSecondaryAudioHintTypeKey")]
    #[api::available(ios = 8.0, watchos = 2.0, tvos = 9.0)]
    pub fn silence_secondary_audio_hint_type() -> &'static ns::String {
        unsafe { AVAudioSessionSilenceSecondaryAudioHintTypeKey }
    }

    /// Contains a payload of ns::Integer representing the new resolved rendering mode
    #[doc(alias = "AVAudioSessionRenderingModeNewRenderingModeKey")]
    #[api::available(ios = 17.2, tvos = 17.2)]
    pub fn rendering_mode_new_rendering_mode() -> &'static ns::String {
        unsafe { AVAudioSessionRenderingModeNewRenderingModeKey }
    }

    /// Indicates if microphone injection is available.
    ///
    /// Value is an ns::Number whose boolean value indicates if microphone injection is available.
    #[doc(alias = "AVAudioSessionMicrophoneInjectionIsAvailableKey")]
    #[api::available(ios = 18.2, visionos = 2.2)]
    pub fn mic_injection_is_available() -> &'static ns::String {
        unsafe { AVAudioSessionMicrophoneInjectionIsAvailableKey }
    }

    #[doc(alias = "AVAudioSessionMuteStateKey")]
    #[api::available(macos = 26.0, ios = 26.0)]
    pub fn mute_state() -> &'static ns::String {
        unsafe { AVAudioSessionMuteStateKey }
    }

    #[link(name = "AVFAudio", kind = "framework")]
    #[api::weak]
    unsafe extern "C" {
        #[api::available(ios = 15.0, watchos = 8.0, tvos = 15.0)]
        static AVAudioSessionSpatialAudioEnabledKey: &'static ns::String;

        #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
        static AVAudioSessionInterruptionTypeKey: &'static ns::String;

        #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
        static AVAudioSessionInterruptionOptionKey: &'static ns::String;

        #[api::available(ios = 14.5, watchos = 7.3)]
        static AVAudioSessionInterruptionReasonKey: &'static ns::String;

        #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
        static AVAudioSessionRouteChangeReasonKey: &'static ns::String;

        #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
        static AVAudioSessionRouteChangePreviousRouteKey: &'static ns::String;

        #[api::available(ios = 8.0, watchos = 2.0, tvos = 9.0)]
        static AVAudioSessionSilenceSecondaryAudioHintTypeKey: &'static ns::String;

        #[api::available(ios = 17.2, tvos = 17.2)]
        static AVAudioSessionRenderingModeNewRenderingModeKey: &'static ns::String;

        #[api::available(ios = 18.2, visionos = 2.2)]
        static AVAudioSessionMicrophoneInjectionIsAvailableKey: &'static ns::String;

        #[api::available(macos = 26.0, ios = 26.0)]
        static AVAudioSessionMuteStateKey: &'static ns::String;

    }
}

#[link(name = "AVFAudio", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    static AVAudioSessionInterruptionNotification: &'static ns::NotificationName;
    static AVAudioSessionRouteChangeNotification: &'static ns::NotificationName;
    static AVAudioSessionMediaServicesWereLostNotification: &'static ns::NotificationName;
    static AVAudioSessionMediaServicesWereResetNotification: &'static ns::NotificationName;
    static AVAudioSessionSilenceSecondaryAudioHintNotification: &'static ns::NotificationName;
    static AVAudioSessionSpatialPlaybackCapabilitiesChangedNotification:
        &'static ns::NotificationName;
    static AVAudioSessionRenderingModeChangeNotification: &'static ns::NotificationName;
    static AVAudioSessionRenderingCapabilitiesChangeNotification: &'static ns::NotificationName;

    #[api::available(ios = 18.2, maccatalyst = 18.2, visionos = 2.2)]
    static AVAudioSessionMicrophoneInjectionCapabilitiesChangeNotification:
        &'static ns::NotificationName;

    #[api::available(macos = 26.0, ios = 26.0)]
    static AVAudioSessionOutputMuteStateChangeNotification: &'static ns::NotificationName;

    #[api::available(ios = 26.0)]
    static AVAudioSessionUserIntentToUnmuteOutputNotification: &'static ns::NotificationName;

    #[api::available(ios = 26.0)]
    static AVAudioSessionAvailableInputsChangeNotification: &'static ns::NotificationName;
}
