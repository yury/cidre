use crate::{arc, av, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "cm")]
use crate::cm;

define_obj_type!(
    #[doc(alias = "AVCaptureInput")]
    pub Input(ns::Id)
);

define_obj_type!(
    #[doc(alias = "AVCaptureDeviceInput")]
    pub DeviceInput(Input)
);

define_obj_type!(
    #[doc(alias = "AVCaptureInputPort")]
    pub Port(ns::Id)
);

/// Constants indicating the modes of multichannel audio.
#[doc(alias = "AVCaptureMultichannelAudioMode")]
#[repr(isize)]
pub enum MultichannelAudioMode {
    /// Indicates that no multichannel audio should be used.
    #[doc(alias = "AVCaptureMultichannelAudioModeNone")]
    None = 0,

    /// Indicates that the audio should be recorded using stereo.
    #[doc(alias = "AVCaptureMultichannelAudioModeStereo")]
    Stereo = 1,

    /// Indicates that the audio should be recorded using first-order ambisonics.
    /// When recording a QuickTime movie file, a stereo audio track will be recorded alongside the FOA
    /// track for backward playback compatibility.
    #[doc(alias = "AVCaptureMultichannelAudioModeFirstOrderAmbisonics")]
    FirstOrderAmbisonics = 2,
}

impl Input {
    #[objc::msg_send(ports)]
    pub fn ports(&self) -> arc::R<ns::Array<Port>>;
}

impl arc::A<DeviceInput> {
    #[objc::msg_send(initWithDevice:error:)]
    pub fn init_with_device_err<'ear>(
        self,
        device: &av::CaptureDevice,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<DeviceInput>>;
}

#[cfg(not(target_os = "watchos"))]
impl DeviceInput {
    define_cls!(AV_CAPTURE_DEVICE_INPUT);

    pub fn with_device<'ear>(device: &av::CaptureDevice) -> Result<arc::R<Self>, &'ear ns::Error> {
        let mut error = None;
        unsafe {
            let res = Self::alloc().init_with_device_err(device, &mut error);
            match error {
                Some(e) => Err(e),
                None => Ok(res.unwrap_unchecked()),
            }
        }
    }

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<av::CaptureDevice>;

    #[objc::msg_send(unifiedAutoExposureDefaultsEnabled)]
    pub fn unified_auto_exposure_defaults_enabled(&self) -> bool;

    #[objc::msg_send(setUnifiedAutoExposureDefaultsEnabled:)]
    pub fn set_unified_auto_exposure_defaults_enabled(&mut self, val: bool);

    #[objc::msg_send(portsWithMediaType:sourceDeviceType:sourceDevicePosition:)]
    pub fn ports_with(
        &self,
        media_type: Option<&av::MediaType>,
        src_device_type: Option<&av::CaptureDeviceType>,
        src_device_pos: av::CaptureDevicePos,
    ) -> arc::R<ns::Array<Port>>;

    /// A property that acts as a modifier to the [`av::CaptureDevice`]'s activeVideoMinFrameDuration property.
    /// Default value is cm::Time::invalid().
    #[cfg(feature = "cm")]
    #[objc::msg_send(videoMinFrameDurationOverride)]
    #[objc::available(ios = 13.0, maccatalyst = 14.0, tvos = 17.0)]
    pub fn video_min_frame_duration_override(&self) -> cm::Time;

    #[cfg(feature = "cm")]
    #[objc::msg_send(setVideoMinFrameDurationOverride:)]
    #[objc::available(ios = 13.0, maccatalyst = 14.0, tvos = 17.0)]
    pub fn set_video_min_frame_duration_override(&mut self, val: cm::Time);

    /// Returns whether the receiver supports the given multichannel audio mode.
    ///
    /// Multichannel audio modes are not supported when used in conjunction with av::CaptureMultiCamSession.
    #[objc::msg_send(isMultichannelAudioModeSupported:)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn is_multichannel_audio_mode_supported(&self, val: MultichannelAudioMode) -> bool;

    /// Indicates the multichannel audio mode to apply when recording audio.
    ///
    /// This property only takes effect when audio is being routed through the built-in microphone, and is ignored if an external microphone is in use.
    /// The default value is av::capture::MultichannelAudioMode::None, in which case the default single channel audio recording is used.
    #[objc::msg_send(multichannelAudioMode)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn multichannel_audio_mode(&self) -> MultichannelAudioMode;

    #[objc::msg_send(setMultichannelAudioMode:)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn set_multichannel_audio_mode(&mut self, val: MultichannelAudioMode);

    /// Returns whether or not the device supports wind noise removal during audio capture.
    #[objc::msg_send(isWindNoiseRemovalSupported)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn is_wind_noise_removal_supported(&self) -> bool;

    /// Specifies whether or not wind noise is removed during audio capture.
    ///
    /// Wind noise removal is available when the AVCaptureDeviceInput multichannelAudioMode property is set to any value other than av::CaptureMultichannelAudioModeNone.
    #[objc::msg_send(isWindNoiseRemovalEnabled)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn is_wind_noise_removal_enabled(&self) -> bool;

    /// Specifies whether or not wind noise is removed during audio capture.
    #[objc::msg_send(setWindNoiseRemovalEnabled:)]
    #[objc::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn set_wind_noise_removal_enabled(&mut self, val: bool);

    #[objc::msg_send(isCinematicVideoCaptureSupported)]
    #[objc::available(macos = 26.0, ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub fn is_cinematic_video_capture_supported(&self) -> bool;

    #[objc::msg_send(isCinematicVideoCaptureEnabled)]
    #[objc::available(macos = 26.0, ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub fn is_cinematic_video_capture_enabled(&self) -> bool;

    #[objc::msg_send(setCinematicVideoCaptureEnabled:)]
    #[objc::available(macos = 26.0, ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub fn set_cinematic_video_capture_enabled(&mut self, val: bool);

    #[objc::msg_send(simulatedAperture)]
    #[objc::available(macos = 26.0, ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub fn simulated_aperture(&self) -> f32;

    #[objc::msg_send(setSimulatedAperture:)]
    #[objc::available(macos = 26.0, ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub unsafe fn set_simulated_aperture_throws(&mut self, val: f32);

    /// Shallow depth of field simulated aperture.
    ///
    /// When capturing a Cinematic Video, use this property to control the amount of blur in the simulated depth of field effect.
    #[objc::available(macos = 26.0, ios = 26.0, maccatalyst = 26.0, tvos = 26.0)]
    pub fn set_simultated_aperture<'ear>(&mut self, val: f32) -> ns::ExResult<'ear> {
        unsafe { ns::try_catch(|| self.set_simulated_aperture_throws(val)) }
    }
}

#[cfg(any(target_os = "ios", target_os = "tvos"))]
define_obj_type!(
    #[doc(alias = "AVCaptureMetadataInput")]
    pub MetadataInput(Input),
    AV_CAPTURE_METADATA_INPUT
);

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    #[cfg(not(target_os = "watchos"))]
    static AV_CAPTURE_DEVICE_INPUT: &'static objc::Class<DeviceInput>;

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    static AV_CAPTURE_METADATA_INPUT: &'static objc::Class<MetadataInput>;
}

impl Port {
    /// The input that owns the receiver.
    #[objc::msg_send(input)]
    pub fn input(&self) -> &Input;

    /// The media type of the data provided by the receiver.
    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> av::MediaType;

    /// The format of the data provided by the receiver.
    #[cfg(feature = "cm")]
    #[objc::msg_send(formatDescription)]
    pub fn format_desc(&self) -> Option<&cm::FormatDesc>;

    #[objc::msg_send(isEnabled)]
    pub fn enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    /// Provides access to the "native" clock used by the input port.
    #[cfg(feature = "cm")]
    #[objc::msg_send(clock)]
    pub fn clock(&self) -> Option<&cm::Clock>;

    /// The source device providing input through this port.
    #[objc::msg_send(sourceDeviceType)]
    pub fn src_device_type(&self) -> Option<&av::CaptureDeviceType>;

    /// Position of the source device providing input through this port.
    #[objc::msg_send(sourceDevicePosition)]
    pub fn src_device_pos(&self) -> av::CaptureDevicePos;
}

pub mod port_notifications {
    use crate::{api, ns};

    #[doc(alias = "AVCaptureInputPortFormatDescriptionDidChangeNotification")]
    #[api::available(macos = 10.7, ios = 4.0, maccatalyst = 14.0, tvos = 17.0)]
    pub fn format_desc_did_change() -> &'static ns::NotificationName {
        unsafe { AVCaptureInputPortFormatDescriptionDidChangeNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    #[api::weak]
    unsafe extern "C" {
        #[api::available(macos = 10.7, ios = 4.0, maccatalyst = 14.0, tvos = 17.0)]
        static AVCaptureInputPortFormatDescriptionDidChangeNotification:
            &'static ns::NotificationName;
    }
}
