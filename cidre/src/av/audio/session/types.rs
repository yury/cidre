use crate::{api, define_obj_type, define_opts, ns};

define_obj_type!(
    #[doc(alias = "AVAudioSessionPort")]
    pub Port(ns::String)
);

/// Input port types
impl Port {
    /// A Continuity Microphone is an iOS device that a user trusts to use for audio input on Apple TV.
    #[doc(alias = "AVAudioSessionPortContinuityMicrophone")]
    #[inline]
    #[api::available(ios = 17.0, watchos = 10.0, tvos = 17.0)]
    pub fn continuity_mic() -> &'static Self {
        unsafe { AVAudioSessionPortContinuityMicrophone }
    }

    /// Line level input on a dock connector
    #[doc(alias = "AVAudioSessionPortLineIn")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn line_in() -> &'static Self {
        unsafe { AVAudioSessionPortLineIn }
    }

    /// Built-in microphone on an iOS device
    #[doc(alias = "AVAudioSessionPortBuiltInMic")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn built_in_mic() -> &'static Self {
        unsafe { AVAudioSessionPortBuiltInMic }
    }

    /// Microphone on a wired headset.  Headset refers to an accessory that has headphone outputs paired with a
    /// microphone.
    #[doc(alias = "AVAudioSessionPortHeadsetMic")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn headset_mic() -> &'static Self {
        unsafe { AVAudioSessionPortHeadsetMic }
    }
}

/// Output port types
impl Port {
    /// Line level output on a dock connector
    #[doc(alias = "AVAudioSessionPortLineOut")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn line_out() -> &'static Self {
        unsafe { AVAudioSessionPortLineOut }
    }

    /// Headphone or headset output
    #[doc(alias = "AVAudioSessionPortHeadphones")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn headphones() -> &'static Self {
        unsafe { AVAudioSessionPortHeadphones }
    }

    /// Output on a Bluetooth A2DP device
    #[doc(alias = "AVAudioSessionPortBluetoothA2DP")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn bluetooth_a2dp() -> &'static Self {
        unsafe { AVAudioSessionPortBluetoothA2DP }
    }

    /// The speaker you hold to your ear when on a phone call
    #[inline]
    #[doc(alias = "AVAudioSessionPortBuiltInReceiver")]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn built_in_receiver() -> &'static Self {
        unsafe { AVAudioSessionPortBuiltInReceiver }
    }

    /// Built-in speaker on an iOS device
    #[doc(alias = "AVAudioSessionPortBuiltInSpeaker")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn built_in_speaker() -> &'static Self {
        unsafe { AVAudioSessionPortBuiltInSpeaker }
    }

    /// Output via High-Definition Multimedia Interface
    #[doc(alias = "AVAudioSessionPortHDMI")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn hdmi() -> &'static Self {
        unsafe { AVAudioSessionPortHDMI }
    }

    /// Output on a remote Air Play device
    #[doc(alias = "AVAudioSessionPortAirPlay")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn air_play() -> &'static Self {
        unsafe { AVAudioSessionPortAirPlay }
    }

    /// Output on a Bluetooth Low Energy device
    #[doc(alias = "AVAudioSessionPortBluetoothLE")]
    #[inline]
    #[api::available(ios = 7.0, watchos = 2.0, tvos = 9.0)]
    pub fn bluetooth_le() -> &'static Self {
        unsafe { AVAudioSessionPortBluetoothLE }
    }
}

/// Port types that refer to either input or output
impl Port {
    /// Input or output on a Bluetooth Hands-Free Profile device
    #[doc(alias = "AVAudioSessionPortBluetoothHFP")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn bluetooth_hfp() -> &'static Self {
        unsafe { AVAudioSessionPortBluetoothHFP }
    }

    /// Input or output on a Universal Serial Bus device
    #[doc(alias = "AVAudioSessionPortUSBAudio")]
    #[inline]
    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    pub fn usb_audio() -> &'static Self {
        unsafe { AVAudioSessionPortUSBAudio }
    }

    /// Input or output via Car Audio
    #[doc(alias = "AVAudioSessionPortCarAudio")]
    #[inline]
    #[api::available(ios = 7.0, watchos = 2.0, tvos = 9.0)]
    pub fn car_audio() -> &'static Self {
        unsafe { AVAudioSessionPortCarAudio }
    }

    /// Input or output that does not correspond to real audio hardware
    #[doc(alias = "AVAudioSessionPortVirtual")]
    #[inline]
    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    pub fn virtual_() -> &'static Self {
        unsafe { AVAudioSessionPortVirtual }
    }

    /// Input or output connected via the PCI (Peripheral Component Interconnect) bus
    #[doc(alias = "AVAudioSessionPortPCI")]
    #[inline]
    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    pub fn pci() -> &'static Self {
        unsafe { AVAudioSessionPortPCI }
    }

    /// Input or output connected via FireWire
    #[doc(alias = "AVAudioSessionPortFireWire")]
    #[inline]
    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    pub fn fire_wire() -> &'static Self {
        unsafe { AVAudioSessionPortFireWire }
    }

    /// Input or output connected via DisplayPort
    #[doc(alias = "AVAudioSessionPortDisplayPort")]
    #[inline]
    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    pub fn display_port() -> &'static Self {
        unsafe { AVAudioSessionPortDisplayPort }
    }

    /// Input or output connected via AVB (Audio Video Bridging)
    #[doc(alias = "AVAudioSessionPortAVB")]
    #[inline]
    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    pub fn avb() -> &'static Self {
        unsafe { AVAudioSessionPortAVB }
    }

    /// Input or output connected via Thunderbolt
    #[doc(alias = "AVAudioSessionPortThunderbolt")]
    #[inline]
    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    pub fn thunderbolt() -> &'static Self {
        unsafe { AVAudioSessionPortThunderbolt }
    }
}

#[link(name = "AVFAudio", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    #[api::available(ios = 17.0, watchos = 10.0, tvos = 17.0)]
    static AVAudioSessionPortContinuityMicrophone: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortLineIn: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortBuiltInMic: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortHeadsetMic: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortLineOut: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortHeadphones: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortBluetoothA2DP: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortBuiltInReceiver: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortBuiltInSpeaker: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortHDMI: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortAirPlay: &'static Port;

    #[api::available(ios = 7.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortBluetoothLE: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortBluetoothHFP: &'static Port;

    #[api::available(ios = 6.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortUSBAudio: &'static Port;

    #[api::available(ios = 7.0, watchos = 2.0, tvos = 9.0)]
    static AVAudioSessionPortCarAudio: &'static Port;

    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    static AVAudioSessionPortVirtual: &'static Port;

    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    static AVAudioSessionPortPCI: &'static Port;

    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    static AVAudioSessionPortFireWire: &'static Port;

    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    static AVAudioSessionPortDisplayPort: &'static Port;

    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    static AVAudioSessionPortAVB: &'static Port;

    #[api::available(ios = 14.0, watchos = 7.0, tvos = 14.0)]
    static AVAudioSessionPortThunderbolt: &'static Port;

}

define_obj_type!(
    #[doc(alias = "AVAudioSessionCategory")]
    pub Category(ns::String)
);

impl Category {
    /// Use this category for background sounds such as rain, car engine noise, etc.
    /// Mixes with other music
    #[doc(alias = "AVAudioSessionCategoryAmbient")]
    #[inline]
    pub fn ambient() -> &'static Self {
        unsafe { AVAudioSessionCategoryAmbient }
    }

    /// Use this category for background sounds.  Other music will stop playing.
    #[doc(alias = "AVAudioSessionCategorySoloAmbient")]
    #[inline]
    pub fn solo_ambient() -> &'static Self {
        unsafe { AVAudioSessionCategorySoloAmbient }
    }

    /// Use this category for background sounds.  Other music will stop playing.
    #[doc(alias = "AVAudioSessionCategoryPlayback")]
    #[inline]
    pub fn playback() -> &'static Self {
        unsafe { AVAudioSessionCategoryPlayback }
    }

    /// Use this category when recording audio.
    #[doc(alias = "AVAudioSessionCategoryRecord")]
    #[inline]
    pub fn record() -> &'static Self {
        unsafe { AVAudioSessionCategoryRecord }
    }

    /// Use this category when recording and playing back audio.
    ///
    /// The category for recording (input) and playback (output) of audio, such as for a Voice over Internet Protocol (VoIP) app.
    /// Your audio continues with the Silent switch set to silent and with the screen locked. (The switch is called the
    /// Ring/Silent switch on iPhone.) To continue playing audio when your app transitions to the background (for example,
    /// when the screen locks), add the audio value to the UIBackgroundModes key in your information property list file.
    ///
    /// This category is appropriate for simultaneous recording and playback, and also for apps that record and play back,
    /// but not simultaneously.
    ///
    /// By default, using this category implies that your app’s audio is nonmixable—activating your session will interrupt
    /// any other audio sessions which are also nonmixable. To allow mixing for this category, use the
    /// AVAudioSessionCategoryOptionMixWithOthers option.
    ///
    /// The user must grant permission for audio recording.
    ///
    /// This category supports the mirrored version of Airplay. However, AirPlay mirroring will be disabled if the
    /// AVAudioSessionModeVoiceChat mode is used with this category.
    #[doc(alias = "AVAudioSessionCategoryPlayAndRecord")]
    #[inline]
    pub fn play_and_record() -> &'static Self {
        unsafe { AVAudioSessionCategoryPlayAndRecord }
    }

    /// Use this category to customize the usage of available audio accessories and built-in audio hardware.
    /// For example, this category provides an application with the ability to use an available USB output
    /// and headphone output simultaneously for separate, distinct streams of audio data. Use of
    /// this category by an application requires a more detailed knowledge of, and interaction with,
    /// the capabilities of the available audio routes.  May be used for input, output, or both.
    /// Note that not all output types and output combinations are eligible for multi-route. Input is limited
    /// to the last-in input port. Eligible inputs consist of the following:
    /// AVAudioSessionPortUSBAudio, AVAudioSessionPortHeadsetMic, and AVAudioSessionPortBuiltInMic.
    /// Eligible outputs consist of the following:
    /// AVAudioSessionPortUSBAudio, AVAudioSessionPortLineOut, AVAudioSessionPortHeadphones, AVAudioSessionPortHDMI,
    /// and AVAudioSessionPortBuiltInSpeaker.
    /// Note that AVAudioSessionPortBuiltInSpeaker is only allowed to be used when there are no other eligible
    /// outputs connected.
    #[doc(alias = "AVAudioSessionCategoryMultiRoute")]
    #[inline]
    pub fn multi_route() -> &'static Self {
        unsafe { AVAudioSessionCategoryMultiRoute }
    }
}

#[link(name = "AVFAudio", kind = "framework")]
unsafe extern "C" {
    static AVAudioSessionCategoryAmbient: &'static Category;
    static AVAudioSessionCategorySoloAmbient: &'static Category;
    static AVAudioSessionCategoryPlayback: &'static Category;
    static AVAudioSessionCategoryRecord: &'static Category;
    static AVAudioSessionCategoryPlayAndRecord: &'static Category;
    static AVAudioSessionCategoryMultiRoute: &'static Category;
}

define_obj_type!(
    #[doc(alias = "AVAudioSessionMode")]
    pub Mode(ns::String)
);

/// Modes modify the audio category in order to introduce behavior that is tailored to the specific
/// use of audio within an application. Available in iOS 5.0 and greater.
impl Mode {
    #[doc(alias = "AVAudioSessionModeDefault")]
    #[inline]
    pub fn default() -> &'static Self {
        unsafe { AVAudioSessionModeDefault }
    }

    /// Only valid with [`av::AudioSessionCategory::play_and_record`]. Appropriate for Voice over IP
    /// (VoIP) applications. Reduces the number of allowable audio routes to be only those
    /// that are appropriate for VoIP applications and may engage appropriate system-supplied
    /// signal processing.  Has the side effect of setting [`av::AudioSessionCategoryOpts::allow_bluetooth`]
    #[doc(alias = "AVAudioSessionModeVoiceChat")]
    #[inline]
    pub fn voice_chat() -> &'static Self {
        unsafe { AVAudioSessionModeVoiceChat }
    }

    /// Set by Game Kit on behalf of an application that uses a GKVoiceChat object; valid
    /// only with the [`av::AudioSessionCategory::play_and_record()`] category.
    /// Do not set this mode directly. If you need similar behavior and are not using
    /// a GKVoiceChat object, use [`av::AudioSessionMode::voice_chat()`] instead.
    #[doc(alias = "AVAudioSessionModeGameChat")]
    #[inline]
    pub fn game_chat() -> &'static Self {
        unsafe { AVAudioSessionModeGameChat }
    }

    /// Only valid with AVAudioSessionCategoryPlayAndRecord or AVAudioSessionCategoryRecord.
    /// Modifies the audio routing options and may engage appropriate system-supplied signal processing.
    #[doc(alias = "AVAudioSessionModeVideoRecording")]
    #[inline]
    pub fn video_recording() -> &'static Self {
        unsafe { AVAudioSessionModeVideoRecording }
    }

    /// Appropriate for applications that wish to minimize the effect of system-supplied signal
    /// processing for input and/or output audio signals.
    #[doc(alias = "AVAudioSessionModeMeasurement")]
    #[inline]
    pub fn measurement() -> &'static Self {
        unsafe { AVAudioSessionModeMeasurement }
    }

    /// Only valid with kAudioSessionCategory_PlayAndRecord. Reduces the number of allowable audio
    /// routes to be only those that are appropriate for video chat applications. May engage appropriate
    /// system-supplied signal processing. Has the side effect of setting
    /// AVAudioSessionCategoryOptionAllowBluetooth and AVAudioSessionCategoryOptionDefaultToSpeaker
    #[doc(alias = "AVAudioSessionModeVideoChat")]
    #[inline]
    pub fn video_chat() -> &'static Self {
        unsafe { AVAudioSessionModeVideoChat }
    }

    /// Engages appropriate output signal processing for movie playback scenarios.
    /// Currently only applied during playback over built-in speaker.
    #[doc(alias = "AVAudioSessionModeMoviePlayback")]
    #[inline]
    pub fn movie_playback() -> &'static Self {
        unsafe { AVAudioSessionModeMoviePlayback }
    }

    /// Appropriate for applications which play spoken audio and wish to be paused (via audio session interruption) rather than ducked
    /// if another app (such as a navigation app) plays a spoken audio prompt.  Examples of apps that would use this are podcast players and
    /// audio books. For more information, see the related category option AVAudioSessionCategoryOptionInterruptSpokenAudioAndMixWithOthers
    #[doc(alias = "AVAudioSessionModeSpokenAudio")]
    #[inline]
    pub fn spoken_audio() -> &'static Self {
        unsafe { AVAudioSessionModeSpokenAudio }
    }

    /// Appropriate for applications which play audio using text to speech. Setting this mode allows for different routing behaviors when
    /// connected to certain audio devices such as CarPlay. An example of an app that would use this mode is a turn by turn navigation app that
    /// plays short prompts to the user. Typically, these same types of applications would also configure their session to use
    /// AVAudioSessionCategoryOptionDuckOthers and AVAudioSessionCategoryOptionInterruptSpokenAudioAndMixWithOthers
    #[doc(alias = "AVAudioSessionModeVoicePrompt")]
    #[inline]
    pub fn voice_prompt() -> &'static Self {
        unsafe { AVAudioSessionModeVoicePrompt }
    }

    #[doc(alias = "AVAudioSessionModeShortFormVideo")]
    #[api::available(ios = 26.0)]
    pub fn short_form_video() -> &'static Self {
        unsafe { AVAudioSessionModeShortFormVideo }
    }
}

#[link(name = "AVFAudio", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    static AVAudioSessionModeDefault: &'static Mode;
    static AVAudioSessionModeVoiceChat: &'static Mode;
    static AVAudioSessionModeGameChat: &'static Mode;
    static AVAudioSessionModeVideoRecording: &'static Mode;
    static AVAudioSessionModeMeasurement: &'static Mode;
    static AVAudioSessionModeMoviePlayback: &'static Mode;
    static AVAudioSessionModeVideoChat: &'static Mode;
    static AVAudioSessionModeSpokenAudio: &'static Mode;
    static AVAudioSessionModeVoicePrompt: &'static Mode;
    #[api::available(ios = 26.0)]
    static AVAudioSessionModeShortFormVideo: &'static Mode;

}

#[doc(alias = "AVAudioSessionActivationOptions")]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum ActivationOpts {
    None = 0,
}

/// For use with overrideOutputAudioPort:error:
#[doc(alias = "AVAudioSessionPortOverride")]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum PortOverride {
    /// No override. Return audio routing to the default state for the current audio category.
    None = 0,
    /// Route audio output to speaker. Use this override with av::AudioSessionCategory::PlayAndRecord,
    /// which by default routes the output to the receiver.
    Speaker = u32::from_be_bytes(*b"spkr") as _,
}

/// Values for AVAudioSessionRouteChangeReasonKey in AVAudioSessionRouteChangeNotification's
/// userInfo dictionary
#[doc(alias = "AVAudioSessionRouteChangeReason")]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum RouteChangeReason {
    /// The reason is unknown.
    #[doc(alias = "AVAudioSessionRouteChangeReasonUnknown")]
    Unknown = 0,

    /// A new device became available (e.g. headphones have been plugged in).
    #[doc(alias = "AVAudioSessionRouteChangeReasonNewDeviceAvailable")]
    NewDeviceAvailable = 1,

    /// The old device became unavailable (e.g. headphones have been unplugged).
    #[doc(alias = "AVAudioSessionRouteChangeReasonOldDeviceUnavailable")]
    OldDeviceUnavailable = 2,

    /// The audio category has changed (e.g. AVAudioSessionCategoryPlayback has been changed to
    /// AVAudioSessionCategoryPlayAndRecord).
    #[doc(alias = "AVAudioSessionRouteChangeReasonCategoryChange")]
    CategoryChange = 3,

    /// The route has been overridden (e.g. category is AVAudioSessionCategoryPlayAndRecord and
    /// the output has been changed from the receiver, which is the default, to the speaker).
    #[doc(alias = "AVAudioSessionRouteChangeReasonOverride")]
    Override = 4,

    /// The device woke from sleep.
    #[doc(alias = "AVAudioSessionRouteChangeReasonWakeFromSleep")]
    WakeFromSleep = 6,

    /// Returned when there is no route for the current category (for instance, the category is
    /// AVAudioSessionCategoryRecord but no input device is available).
    #[doc(alias = "AVAudioSessionRouteChangeReasonNoSuitableRouteForCategory")]
    NoSuitableRouteForCategory = 7,

    /// Indicates that the set of input and/our output ports has not changed, but some aspect of
    /// their configuration has changed.  For example, a port's selected data source has changed.
    /// (Introduced in iOS 7.0, watchOS 2.0, tvOS 9.0).
    #[doc(alias = "AVAudioSessionRouteChangeReasonRouteConfigurationChange")]
    RouteCfgChange = 8,
}

impl From<u32> for RouteChangeReason {
    fn from(value: u32) -> Self {
        if value <= 8 {
            let v = value as usize;
            unsafe { std::mem::transmute(v) }
        } else {
            Self::Unknown
        }
    }
}

define_opts!(
    #[doc(alias = "AVAudioSessionCategoryOptions")]
    pub CategoryOpts(usize)
);

/// Applications must be prepared for changing category options to fail as behavior may change
/// in future releases. If an application changes its category, it should reassert the options,
/// since they are not sticky across category changes. Introduced in iOS 6.0 / watchOS 2.0 /
/// tvOS 9.0.
impl CategoryOpts {
    /// Controls whether other active audio apps will be interrupted or mixed with when your app's
    /// audio session goes active. Details depend on the category.
    ///
    /// AVAudioSessionCategoryPlayAndRecord or AVAudioSessionCategoryMultiRoute:
    ///      MixWithOthers defaults to false, but can be set to true, allowing other applications to
    ///      play in the background while your app has both audio input and output enabled.
    ///
    /// AVAudioSessionCategoryPlayback:
    ///      MixWithOthers defaults to false, but can be set to true, allowing other applications to
    ///      play in the background. Your app will still be able to play regardless of the setting
    ///      of the ringer switch.
    ///
    /// Other categories:
    ///      MixWithOthers defaults to false and cannot be changed.
    ///
    /// MixWithOthers is only valid with AVAudioSessionCategoryPlayAndRecord,
    /// AVAudioSessionCategoryPlayback, and AVAudioSessionCategoryMultiRoute.
    #[doc(alias = "AVAudioSessionCategoryOptionMixWithOthers")]
    pub const MIX_WITH_OTHERS: Self = Self(0x1);

    /// Controls whether or not other active audio apps will be ducked when when your app's audio
    /// session goes active. An example of this is a workout app, which provides periodic updates to
    /// the user. It reduces the volume of any music currently being played while it provides its
    /// status.
    ///
    /// Defaults to off. Note that the other audio will be ducked for as long as the current session
    /// is active. You will need to deactivate your audio session when you want to restore full
    /// volume playback (un-duck) other sessions.
    ///
    /// Setting this option will also make your session mixable with others
    /// (AVAudioSessionCategoryOptionMixWithOthers will be set).
    ///
    /// DuckOthers is only valid with AVAudioSessionCategoryAmbient,
    /// AVAudioSessionCategoryPlayAndRecord, AVAudioSessionCategoryPlayback, and
    /// AVAudioSessionCategoryMultiRoute.
    #[doc(alias = "AVAudioSessionCategoryOptionDuckOthers")]
    pub const DUCK_OTHERS: Self = Self(0x2);

    /// Allows an application to change the default behavior of some audio session categories with
    /// regard to whether Bluetooth Hands-Free Profile (HFP) devices are available for routing. The
    /// exact behavior depends on the category.
    ///
    /// AVAudioSessionCategoryPlayAndRecord:
    ///     AllowBluetooth defaults to false, but can be set to true, allowing a paired bluetooth
    ///     HFP device to appear as an available route for input, while playing through the
    ///     category-appropriate output.
    ///
    /// AVAudioSessionCategoryRecord:
    ///     AllowBluetooth defaults to false, but can be set to true, allowing a paired Bluetooth
    ///     HFP device to appear as an available route for input
    ///
    /// Other categories:
    ///     AllowBluetooth defaults to false and cannot be changed. Enabling Bluetooth for input in
    ///     these categories is not allowed.
    #[doc(alias = "AVAudioSessionCategoryOptionAllowBluetooth")]
    #[doc(alias = "AVAudioSessionCategoryOptionAllowBluetoothHFP")]
    pub const ALLOW_BLUETOOTH_HFP: Self = Self(0x4);

    /// Allows an application to change the default behavior of some audio session categories with
    /// regard to the audio route. The exact behavior depends on the category.
    ///
    /// AVAudioSessionCategoryPlayAndRecord:
    ///     DefaultToSpeaker will default to false, but can be set to true, routing to Speaker
    ///     (instead of Receiver) when no other audio route is connected.
    ///
    /// Other categories:
    ///     DefaultToSpeaker is always false and cannot be changed.
    #[doc(alias = "AVAudioSessionCategoryOptionDefaultToSpeaker")]
    pub const DEFAULT_TO_SPEAKER: Self = Self(0x8);

    /// When a session with InterruptSpokenAudioAndMixWithOthers set goes active, then if there is
    /// another playing app whose session mode is AVAudioSessionModeSpokenAudio (for podcast
    /// playback in the background, for example), then the spoken-audio session will be
    /// interrupted. A good use of this is for a navigation app that provides prompts to its user:
    /// it pauses any spoken audio currently being played while it plays the prompt.
    ///
    /// InterruptSpokenAudioAndMixWithOthers defaults to off. Note that the other app's audio will
    /// be paused for as long as the current session is active. You will need to deactivate your
    /// audio session to allow the other session to resume playback. Setting this option will also
    /// make your category mixable with others (AVAudioSessionCategoryOptionMixWithOthers will be
    /// set). If you want other non-spoken audio apps to duck their audio when your app's session
    /// goes active, also set AVAudioSessionCategoryOptionDuckOthers.
    ///
    /// Only valid with AVAudioSessionCategoryPlayAndRecord, AVAudioSessionCategoryPlayback, and
    /// AVAudioSessionCategoryMultiRoute. Introduced in iOS 9.0 / watchOS 2.0 / tvOS 9.0.    
    #[doc(alias = "AVAudioSessionCategoryOptionInterruptSpokenAudioAndMixWithOthers")]
    pub const INTERRUPT_SPOKEN_AUDIO_AND_MIX_WITH_OTHERS: Self = Self(0x11);

    /// Allows an application to change the default behavior of some audio session categories with
    /// regard to whether Bluetooth Advanced Audio Distribution Profile (A2DP) devices are
    /// available for routing. The exact behavior depends on the category.
    ///
    /// AVAudioSessionCategoryPlayAndRecord:
    ///     AllowBluetoothA2DP defaults to false, but can be set to true, allowing a paired
    ///     Bluetooth A2DP device to appear as an available route for output, while recording
    ///      through the category-appropriate input.
    ///
    ///  AVAudioSessionCategoryMultiRoute and AVAudioSessionCategoryRecord:
    ///      AllowBluetoothA2DP is false, and cannot be set to true.
    ///
    /// Other categories:
    ///       AllowBluetoothA2DP is always implicitly true and cannot be changed; Bluetooth A2DP ports
    ///        are always supported in output-only categories.
    ///
    /// Setting both AVAudioSessionCategoryOptionAllowBluetooth and
    ///   AVAudioSessionCategoryOptionAllowBluetoothA2DP is allowed. In cases where a single
    ///      Bluetooth device supports both HFP and A2DP, the HFP ports will be given a higher priority
    ///      for routing. For HFP and A2DP ports on separate hardware devices, the last-in wins rule
    ///      applies.
    ///
    /// Introduced in iOS 10.0 / watchOS 3.0 / tvOS 10.0.
    #[doc(alias = "AVAudioSessionCategoryOptionAllowBluetoothA2DP")]
    pub const ALLOW_BLUETOOTH_A2_DP: Self = Self(0x20);

    /// Allows an application to change the default behavior of some audio session categories with
    /// with regard to showing AirPlay devices as available routes. This option applies to
    /// various categories in the same way as AVAudioSessionCategoryOptionAllowBluetoothA2DP;
    /// see above for details.
    ///
    /// Only valid with AVAudioSessionCategoryPlayAndRecord. Introduced in iOS 10.0 / tvOS 10.0.
    ///
    #[doc(alias = "AVAudioSessionCategoryOptionAllowAirPlay")]
    pub const ALLOW_AIR_PLAY: Self = Self(0x40);
    /// Some devices include a privacy feature that mutes the built-in microphone at a hardware level
    /// under certain conditions e.g. when the Smart Folio of an iPad is closed. The default behavior is
    /// to interrupt the session using the built-in microphone when that microphone is muted in hardware.
    /// This option allows an application to opt out of the default behavior if it is using a category that
    /// supports both input and output, such as AVAudioSessionCategoryPlayAndRecord, and wants to
    /// allow its session to stay activated even when the microphone has been muted. The result would be
    /// that playback continues as normal, and microphone sample buffers would continue to be produced
    /// but all microphone samples would have a value of zero.
    ///
    /// This may be useful if an application knows that it wants to allow playback to continue and
    /// recording/monitoring a muted microphone will not lead to a poor user experience. Attempting to use
    /// this option with a session category that doesn't support the use of audio input will result in an error.
    ///
    /// Note that under the default policy, a session will be interrupted if it is running input at the time when
    /// the microphone is muted in hardware. Similarly, attempting to start input when the microphone is
    /// muted will fail.
    /// Note that this option has no relation to the recordPermission property, which indicates whether or
    /// not the user has granted permission to use microphone input.
    #[doc(alias = "AVAudioSessionCategoryOptionOverrideMutedMicrophoneInterruption")]
    pub const OVERRIDE_MUTED_MICROPHONE_INTERRUPTION: Self = Self(0x80);

    #[doc(alias = "AVAudioSessionCategoryOptionBluetoothHighQualityRecording")]
    #[cfg(target_os = "ios")]
    pub const BLUETOOTH_HIGHT_QUALITY_RECORDING: Self = Self(1 << 19);
}

/// Values for AVAudioSessionInterruptionTypeKey in AVAudioSessionInterruptionNotification's
/// userInfo dictionary.
#[doc(alias = "AVAudioSessionInterruptionType")]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum InterruptionType {
    /// the system has interrupted your audio session
    Began = 1,
    /// the interruption has ended
    Ended = 0,
}

/// Values for AVAudioSessionInterruptionOptionKey in AVAudioSessionInterruptionNotification's
/// userInfo dictionary.
#[doc(alias = "AVAudioSessionInterruptionOptions")]
#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum InterruptionOpts {
    #[default]
    None = 0,
    /// Indicates that you should resume playback now that the interruption has ended.
    ShouldResume = 1,
}

/// Values for AVAudioSessionInterruptionReasonKey in AVAudioSessionInterruptionNotification's userInfo dictionary.
#[doc(alias = "AVAudioSessionInterruptionReason")]
#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum InterruptionReason {
    /// The audio session was interrupted because another session was activated.
    #[default]
    Default = 0,
    /// The audio session was interrupted due to the app being suspended by the operating sytem.
    ///
    /// Starting in iOS 10, the system will deactivate the audio session of most apps in response to the
    /// app process being suspended. When the app starts running again, it will receive the notification
    /// that its session has been deactivated by the system. Note that the notification is necessarily
    /// delayed in time, due to the fact that the application was suspended at the time the session was
    /// deactivated by the system and the notification can only be delivered once the app is running
    /// again.
    AppWasSuspended = 1,
    /// The audio session was interrupted due to the built-in mic being muted e.g. due to an iPad's Smart Folio being closed.
    BuiltInMicMuted = 2,
}

/// options for use when calling setActive:withOptions:error:
#[doc(alias = "AVAudioSessionSetActiveOptions")]
#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum SetActiveOpts {
    #[default]
    None = 0,
    /// Notify an interrupted app that the interruption has ended and it may resume playback. Only
    /// valid on session deactivation.
    NotifyOthersOnDeactivation = 1,
}

/// Values for AVAudioSessionSilenceSecondaryAudioHintTypeKey in
/// AVAudioSessionSilenceSecondaryAudioHintNotification's userInfo dictionary, to indicate whether
/// optional secondary audio muting should begin or end.
#[doc(alias = "AVAudioSessionSilenceSecondaryAudioHintType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum SilenceSecondaryAudioHintType {
    /// Another application's primary audio has started.
    Begin = 1,

    /// Another application's primary audio has stopped.
    End = 0,
}

/// Starting in iOS 10, applications that use AVCaptureSession on iPads and iPhones that
/// support taking Live Photos, will have non-aggregated audio I/O unless the app opts out by
/// setting its AVAudioSessionIOType to Aggregated. Non-aggregated audio I/O means that separate
/// threads will be used to service audio I/O for input and output directions.
///  
/// Note that in cases where the I/O is not aggregated, the sample rate and IO buffer duration
/// properties will map to the output audio device. In this scenario, the input and
/// output audio hardware may be running at different sample rates and with different IO buffer
/// durations. If your app requires input and output audio to be presented in the same realtime
/// I/O callback, or requires that input and output audio have the same sample rate or IO buffer
/// duration, or if your app requires the ability to set a preferred sample rate or IO buffer duration
/// for audio input, set the [`av::AudioSessionIoType`] to Aggregated.
///  
/// Apps that don't use AVCaptureSession and use [`av::AudioSessionCategory::play_and_record()`] will continue
/// to have aggregated audio I/O, as in previous versions of iOS.
#[doc(alias = "AVAudioSessionIOType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum IoType {
    /// The default value. If your app does not use [`av::CaptureSession`] or does not have any specific
    /// requirement for aggregating input and output audio in the same realtime I/O callback, use this
    /// value. Note that if your app does not use [`av::CaptureSession`], it will get aggregated I/O when using
    /// [`av::AudioSessionCategory::play_and_record()`].
    ///
    /// If your app does utilize [`av::CaptureSession`], use of this value will allow [`av::CaptureSession`] to
    /// start recording without glitching already running output audio and will allow the system to
    /// utilize power-saving optimizations.
    #[doc(alias = "AVAudioSessionIOTypeNotSpecified")]
    NotSpecified = 0,

    /// Use this value if your session uses [`av::AudioSessionCategory::play_and_record()`] and requires input and
    /// output audio to be presented in the same realtime I/O callback. For example, if your app will be using
    /// a RemoteIO with both input and output enabled.
    ///
    /// Note that your session's preference to use aggregated IO will not be honored if it specifies
    /// AVAudioSessionCategoryOptionMixWithOthers AND another app's audio session was already active
    /// with non-mixable, non-aggregated input/output.
    #[doc(alias = "AVAudioSessionIOTypeAggregated")]
    Aggregated = 1,
}

/// Starting in iOS 11, tvOS 11, and watchOS 5, the route sharing policy allows a session
/// to specify that its output audio should be routed somewhere other than the default system output,
/// when appropriate alternative routes are available.
#[doc(alias = "AVAudioSessionRouteSharingPolicy")]
#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum RouteSharingPolicy {
    /// Follow normal rules for routing audio output.
    #[doc(alias = "AVAudioSessionRouteSharingPolicyDefault")]
    #[default]
    Default = 0,

    /// Route output to the shared long-form audio output. A session whose primary use case is as a
    /// music or podcast player may use this value to play to the same output as the built-in Music (iOS),
    /// Podcasts, or iTunes (macOS) applications. Typically applications that use this policy will also
    /// want sign up for remote control events as documented in “Event Handling Guide for UIKit Apps”
    /// and will want to utilize MediaPlayer framework’s MPNowPlayingInfoCenter class. All applications
    /// on the system that use the long-form audio route sharing policy will have their audio routed to the
    /// same location.
    /// Apps running on watchOS using this policy will also be able to play audio in the background,
    /// as long as an eligible audio route can be activated. Apps running on watchOS using this policy
    /// must use -activateWithOptions:completionHandler: instead of -setActive:withOptions:error: in
    /// order to ensure that the user will be given the opportunity to pick an appropriate audio route
    /// in cases where the system is unable to automatically pick the route.
    #[doc(alias = "AVAudioSessionRouteSharingPolicyLongFormAudio")]
    LongFormAudio = 1,

    /// Applications should not attempt to set this value directly. On iOS, this value will be set by
    /// the system in cases where route picker UI is used to direct video to a wireless route.
    #[doc(alias = "AVAudioSessionRouteSharingPolicyIndependent")]
    Independent = 2,

    /// Route output to the shared long-form video output. A session whose primary use case is as a
    /// movie or other long-form video content player may use this value to play to the same output as
    /// other long-form video content applications such as the built-in TV (iOS) application. Applications
    /// that use this policy will also want to also set the AVInitialRouteSharingPolicy key
    /// in their Info.plist to "LongFormVideo". All applications on the system that use the long-form video
    /// route sharing policy will have their audio and video routed to the same location (e.g. AppleTV when
    /// an AirPlay route is selected). Video content not using this route sharing policy will remain local
    /// to the playback device even when long form video content is being routed to AirPlay.
    #[doc(alias = "AVAudioSessionRouteSharingPolicyLongFormVideo")]
    LongFormVideo = 3,
}

/// The prompt style is a hint to sessions that use AVAudioSessionModeVoicePrompt to modify the type of
/// prompt they play in response to other audio activity on the system, such as Siri or phone calls.
/// Sessions that issue voice prompts are encouraged to pay attention to changes in the prompt style and
/// modify their prompts in response. Apple encourages the use of non-verbal prompts when the Short
/// style is requested.
#[doc(alias = "AVAudioSessionPromptStyle")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum PromptStyle {
    /// Indicates that another session is actively using microphone input and would be negatively impacted
    /// by having prompts play at that time. For example if Siri is recognizing speech, having navigation or
    /// exercise prompts play, could interfere with its ability to accurately recognize the user’s speech.
    /// Client sessions should refrain from playing any prompts while the prompt style is None.
    #[doc(alias = "AVAudioSessionPromptStyleNone")]
    None = u32::from_be_bytes(*b"none") as _,

    /// Indicates one of three states: Siri is active but not recording, voicemail playback is active, or
    /// voice call is active. Short, non-verbal versions of prompts should be used.
    #[doc(alias = "AVAudioSessionPromptStyleShort")]
    Short = u32::from_be_bytes(*b"shrt") as _,

    /// Indicates that normal (long, verbal) versions of prompts may be used.
    #[doc(alias = "AVAudioSessionPromptStyleNormal")]
    Normal = u32::from_be_bytes(*b"nrml") as _,
}

/// Constants indicating stereo input audio orientation,
/// for use with built-in mic input data sources with
/// a stereo polar pattern selected.
#[doc(alias = "AVAudioStereoOrientation")]
#[doc(alias = "AVAudioSession.StereoOrientation")]
#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum StereoOrientation {
    /// Indicates that audio capture orientation is not applicable
    /// (on mono capture, for instance).
    #[doc(alias = "AVAudioStereoOrientationNone")]
    #[default]
    None = 0,

    /// Indicates that audio capture should be oriented vertically,
    /// Lightning connector on the bottom.
    #[doc(alias = "AVAudioStereoOrientationPortrait")]
    Portrait = 1,

    /// Indicates that audio capture should be oriented vertically,
    /// Lightning connector on the top.
    #[doc(alias = "AVAudioStereoOrientationPortraitUpsideDown")]
    PortraitUpsideDown = 2,

    /// Indicates that audio capture should be oriented horizontally,
    /// Lightning connector on the right.
    #[doc(alias = "AVAudioStereoOrientationLandscapeRight")]
    LandscapeRight = 3,

    /// Indicates that audio capture should be oriented horizontally,
    /// Lightning connector on the left.
    #[doc(alias = "AVAudioStereoOrientationLandscapeLeft")]
    LandscapeLeft = 4,
}

#[doc(alias = "AVAudioSessionRecordPermission")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum RecordPermission {
    /// The user has not yet been asked for permission.
    Undetermined = u32::from_be_bytes(*b"undt") as _,
    /// The user has been asked and has denied permission.
    Denied = u32::from_be_bytes(*b"deny") as _,
    /// The user has been asked and has granted permission.
    Granted = u32::from_be_bytes(*b"grnt") as _,
}

#[doc(alias = "AVAudioSessionRenderingMode")]
#[doc(alias = "AVAudioSession.RenderingMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum RenderingMode {
    /// Default Mode when no asset is loaded or playing
    #[doc(alias = "AVAudioSessionRenderingModeNotApplicable")]
    NotApplicable = 0,

    /// Default mode for non multi-channel cases
    #[doc(alias = "AVAudioSessionRenderingModeMonoStereo")]
    MonoStereo = 1,

    /// Default mode for multi-channel cases that do not fall into the modes below
    #[doc(alias = "AVAudioSessionRenderingModeSurround")]
    Surround = 2,

    /// Fallback mode if provided content is Dolby variant but hardware capabilities don't support it
    #[doc(alias = "AVAudioSessionRenderingModeSpatialAudio")]
    SpatialAudio = 3,

    /// Dolby Audio mode
    #[doc(alias = "AVAudioSessionRenderingModeDolbyAudio")]
    DolbyAudio = 4,

    /// Dolby Atmos mode
    #[doc(alias = "AVAudioSessionRenderingModeDolbyAtmos")]
    DolbyAtmos = 5,
}

/// Various modes to inject audio coming from a session to another app’s input stream
///
/// Applications can state their intent to mix locally generated audio, which should consist primarily of
/// synthesized speech, to another app's input stream. This feature is intended to be used by accessibility apps
/// implementing augmentative and alternative communication systems that enable users with disabilities to
/// communicate with synthesized speech. When input is muted, microphone injection will also be muted.
#[doc(alias = "AVAudioSessionMicrophoneInjectionMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum MicInjectionMode {
    /// Default state, microphone injection is not preferred
    #[doc(alias = "AVAudioSessionMicrophoneInjectionModeNone")]
    None = 0,

    /// Inject Spoken Audio, like synthesized speech, with microphone audio
    #[doc(alias = "AVAudioSessionMicrophoneInjectionModeSpokenAudio")]
    SpokenAudio = 1,
}
