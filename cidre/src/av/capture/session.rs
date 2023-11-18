use crate::{arc, av, cm, define_cls, define_obj_type, ns, objc};

/// Constants indicating video orientation, for use with av::CaptureVideoPreviewLayer and av::CaptureConnection.
///
/// [Apple Documentation](https://developer.apple.com/documentation/avfoundation/avcapturevideoorientation?language=objc)
#[doc(alias = "AVCaptureVideoOrientation")]
#[deprecated(since = "0.1.0", note = "See videoRotationAngle instead.")]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(isize)]
pub enum VideoOrienation {
    /// Indicates that video should be oriented vertically, home button on the bottom.
    Portrait = 1,

    /// Indicates that video should be oriented vertically, home button on the top.
    PortraitUpsideDown = 2,

    /// Indicates that video should be oriented horizontally, home button on the right.
    LandscapeRight = 3,

    /// Indicates that video should be oriented horizontally, home button on the left.
    LandscapeLeft = 4,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(isize)]
pub enum InterruptionReason {
    VideoDeviceNotAvailableInBackground = 1,
    AudioDeviceInUseByAnotherClient = 2,
    VideoDeviceInUseByAnotherClient = 3,
    VideoDeviceNotAvailableWithMultipleForegroundApps = 4,
    VideoDeviceNotAvailableDueToSystemPressure = 5,
}

define_obj_type!(
    /// An object that configures capture behavior and coordinates the flow of data
    /// from input devices to capture outputs.
    #[doc(alias = "AVCaptureSession")]
    Session(ns::Id),
    AV_CAPTURE_SESSION
);

impl Session {
    #[objc::msg_send(canSetSessionPreset:)]
    pub fn can_set_session_preset(&self, preset: &av::CaptureSessionPreset) -> bool;

    #[objc::msg_send(sessionPreset)]
    pub fn session_preset(&self) -> &av::CaptureSessionPreset;

    #[objc::msg_send(setSessionPreset:)]
    pub fn set_session_preset(&self, value: &av::CaptureSessionPreset);

    #[objc::msg_send(inputs)]
    pub fn inputs(&self) -> &ns::Array<av::CaptureInput>;

    #[objc::msg_send(canAddInput:)]
    pub fn can_add_input(&self, input: &av::CaptureInput) -> bool;

    #[objc::msg_send(addInput:)]
    pub fn add_input(&mut self, input: &av::CaptureInput);

    #[objc::msg_send(removeInput:)]
    pub fn remove_input(&mut self, input: &av::CaptureInput);

    #[objc::msg_send(outputs)]
    pub fn outputs(&self) -> &ns::Array<av::CaptureOutput>;

    #[objc::msg_send(canAddOutput:)]
    pub fn can_add_output(&self, output: &av::CaptureOutput) -> bool;

    #[objc::msg_send(addOutput:)]
    pub fn add_output(&mut self, output: &av::CaptureOutput);

    #[objc::msg_send(removeOutput:)]
    pub fn remove_output(&mut self, output: &av::CaptureOutput);

    #[objc::msg_send(addInputWithNoConnections:)]
    pub fn add_input_without_connections(&mut self, input: &av::CaptureInput);

    #[objc::msg_send(addOutputWithNoConnections:)]
    pub fn add_output_without_connections(&mut self, output: &av::CaptureOutput);

    #[objc::msg_send(connections)]
    pub fn connections(&self) -> &ns::Array<av::CaptureConnection>;

    #[objc::msg_send(canAddConnection:)]
    pub fn can_add_connection(&self, connection: &av::CaptureConnection) -> bool;

    #[objc::msg_send(addConnection:)]
    pub fn add_connection(&mut self, connection: &av::CaptureConnection);

    #[objc::msg_send(removeConnection:)]
    pub fn remove_connection(&mut self, connection: &av::CaptureConnection);

    #[objc::msg_send(beginConfiguration)]
    pub fn begin_configuration(&mut self);

    #[objc::msg_send(commitConfiguration)]
    pub fn commit_configuration(&mut self);

    pub fn configure<F: FnMut(&mut Self)>(&mut self, mut config: F) {
        self.begin_configuration();
        config(self);
        self.commit_configuration();
    }

    #[objc::msg_send(startRunning)]
    pub fn start_running(&mut self);

    #[objc::msg_send(stopRunning)]
    pub fn stop_running(&mut self);

    #[objc::msg_send(usesApplicationAudioSession)]
    pub fn uses_app_audio_session(&self) -> bool;

    #[objc::msg_send(setUsesApplicationAudioSession:)]
    pub fn set_uses_app_audio_session(&mut self, value: bool);

    #[objc::msg_send(synchronizationClock)]
    pub fn sync_clock(&self) -> Option<&cm::Clock>;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(hardwareCost)]
    pub fn hw_cost(&self) -> f32;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_SESSION: &'static objc::Class<Session>;
    static AV_CAPTURE_MULTI_CAM_SESSION: &'static objc::Class<MultiCamSession>;
    static AV_CAPTURE_CONNECTION: &'static objc::Class<Connection>;
}

define_obj_type!(MultiCamSession(Session), AV_CAPTURE_MULTI_CAM_SESSION);

impl MultiCamSession {
    /// ```no_run
    /// use cidre::av;
    ///
    /// assert!(!av::CaptureMultiCamSession::is_multicam_supported());
    /// ```
    #[cfg(not(target_os = "macos"))]
    #[objc::cls_msg_send(isMultiCamSupported)]
    pub fn is_multicam_supported() -> bool;

    /// The value of this property is a float from 0.0 => 1.0 indicating
    /// how much of the session's available hardware is in use as a percentage,
    /// given the currently connected inputs and outputs and the features for
    /// which you've opted in. When your hardwareCost is greater than 1.0,
    /// the capture session cannot run your desired configuration due to
    /// hardware constraints, so you receive an AVCaptureSessionRuntimeErrorNotification
    /// when attempting to start it running. Default value is 0.
    /// Contributors to hardwareCost include:
    ///
    /// - Whether the source devices' active formats use the full
    ///   sensor (4:3) or a crop (16:9). Cropped formats require lower
    ///   hardware bandwidth, and therefore lower the cost.
    /// - The max frame rate supported by the source devices' active formats.
    ///   The higher the max frame rate, the higher the cost.
    /// - Whether the source devices' active formats are binned or not.
    ///   Binned formats require substantially less hardware bandwidth,
    ///   and therefore result in a lower cost.
    /// - The number of sources configured to deliver streaming
    ///   disparity / depth via AVCaptureDepthDataOutput. The higher the number
    ///   of cameras configured to produce depth, the higher the cost.
    ///   In order to reduce hardwareCost, consider picking a sensor-cropped
    ///   activeFormat, or a binned format.
    ///   You may also use AVCaptureDeviceInput's videoMinFrameDurationOverride
    ///   property to artificially limit the max frame rate (which is the
    ///   reciprocal of the min frame duration) of a source device to a lower value.
    ///   By doing so, you only pay the hardware cost for the max frame rate you intend to use.
    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(hardwareCost)]
    pub fn hw_cost(&self) -> f32;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(systemPressureCost)]
    pub fn system_pressure_cost(&self) -> f32;
}

define_obj_type!(
    /// An object that represents a connection from a capture input to a capture output.
    #[doc(alias = "AVCaptureConnection")]
    Connection(ns::Id)
);

impl Connection {
    define_cls!(AV_CAPTURE_CONNECTION);

    /// An array of audio channels that the connection provides.
    #[objc::msg_send(audioChannels)]
    pub fn audio_channels(&self) -> &ns::Array<AudioChannel>;
}

define_obj_type!(
    /// Represents a single channel of audio flowing through an [`av::CaptureSession`]
    ///
    /// An [`av::CaptureConnection`] from an input producing audio to an output receiving
    /// audio exposes an array of [`av::CaptureAudioChannel`] objects, one for each channel of audio available.
    /// Iterating through these audio channel objects, a client may poll for audio levels.
    /// Instances of [`av::CaptureAudioChannel`] cannot be created directly.
    #[doc(alias = "AVCaptureAudioChannel")]
    AudioChannel(ns::Id)
);

impl AudioChannel {
    /// A measurement of the instantaneous average power level of the audio flowing through the receiver.
    ///
    /// A client may poll an [`av::CaptureAudioChannel`] object for its current averagePowerLevel
    /// to get its instantaneous average power level in decibels.
    /// This property is not key-value observable.
    #[objc::msg_send(averagePowerLevel)]
    pub fn average_power_level(&self) -> f32;

    /// A measurement of the peak/hold level of the audio flowing through the receiver.
    ///
    /// A client may poll an [`av::CaptureAudioChannel`] object for its current peakHoldLevel to get its most
    /// recent peak hold level in decibels.
    /// This property is not key-value observable.
    #[objc::msg_send(peakHoldLevel)]
    pub fn peak_hold_level(&self) -> f32;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(volume)]
    pub fn volume(&self) -> f32;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(setVolume:)]
    pub fn set_volume(&mut self, val: f32);

    /// A property indicating whether the receiver is currently enabled for data capture.
    #[cfg(target_os = "macos")]
    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&self, val: bool);
}

pub mod notifications {
    use crate::ns;

    pub fn runtime_error() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionRuntimeErrorNotification }
    }

    pub fn did_start_running() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionDidStartRunningNotification }
    }

    pub fn did_stop_running() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionDidStopRunningNotification }
    }

    pub fn was_interrupted() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionWasInterruptedNotification }
    }

    pub fn interruption_ended() -> &'static ns::NotificationName {
        unsafe { AVCaptureSessionInterruptionEndedNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureSessionRuntimeErrorNotification: &'static ns::NotificationName;
        static AVCaptureSessionDidStartRunningNotification: &'static ns::NotificationName;
        static AVCaptureSessionDidStopRunningNotification: &'static ns::NotificationName;
        static AVCaptureSessionWasInterruptedNotification: &'static ns::NotificationName;
        static AVCaptureSessionInterruptionEndedNotification: &'static ns::NotificationName;
    }
}
