use crate::{av, cf, define_obj_type, objc::Id};

/// Constants indicating video orientation, for use with av::CaptureVideoPreviewLayer and av::CaptureConnection.
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

#[repr(isize)]
pub enum InterruptionReason {
    VideoDeviceNotAvailableInBackground = 1,
    AudioDeviceInUseByAnotherClient = 2,
    VideoDeviceInUseByAnotherClient = 3,
    VideoDeviceNotAvailableWithMultipleForegroundApps = 4,
    VideoDeviceNotAvailableDueToSystemPressure = 5,
}

define_obj_type!(Session(Id));

impl Session {
    #[inline]
    pub fn can_set_session_preset(&self, preset: &av::CaptureSessionPreset) -> bool {
        unsafe { rsel_canSetSessionPreset(self, preset) }
    }

    #[inline]
    pub fn session_preset(&self) -> &av::CaptureSessionPreset {
        unsafe { rsel_sessionPreset(self) }
    }

    #[inline]
    pub fn set_session_preset(&self, value: &av::CaptureSessionPreset) {
        unsafe { wsel_setSessionPreset(self, value) }
    }

    #[inline]
    pub fn inputs(&self) -> &cf::ArrayOf<av::CaptureInput> {
        unsafe { rsel_inputs(self) }
    }

    #[inline]
    pub fn can_add_input(&self, input: &av::CaptureInput) -> bool {
        unsafe { rsel_canAddInput(self, input) }
    }

    #[inline]
    pub fn add_input(&mut self, input: &av::CaptureInput) {
        unsafe { wsel_addInput(self, input) }
    }

    #[inline]
    pub fn remove_input(&mut self, input: &av::CaptureInput) {
        unsafe { wsel_removeInput(self, input) }
    }

    #[inline]
    pub fn outputs(&self) -> &cf::ArrayOf<av::CaptureOutput> {
        unsafe { rsel_outputs(self) }
    }

    #[inline]
    pub fn can_add_output(&self, output: &av::CaptureOutput) -> bool {
        unsafe { rsel_canAddOutput(self, output) }
    }

    #[inline]
    pub fn add_output(&mut self, output: &av::CaptureOutput) {
        unsafe { wsel_addOutput(self, output) }
    }

    #[inline]
    pub fn remove_output(&mut self, output: &av::CaptureOutput) {
        unsafe { wsel_removeOutput(self, output) }
    }

    #[inline]
    pub fn add_input_without_connections(&mut self, input: &av::CaptureInput) {
        unsafe { wsel_addInputWithNoConnections(self, input) }
    }

    #[inline]
    pub fn add_output_without_connections(&mut self, output: &av::CaptureOutput) {
        unsafe { wsel_addOutputWithNoConnections(self, output) }
    }

    #[inline]
    pub fn connections(&self) -> &cf::ArrayOf<av::CaptureConnection> {
        unsafe { rsel_connections(self) }
    }

    #[inline]
    pub fn can_add_connection(&self, connection: &av::CaptureConnection) -> bool {
        unsafe { rsel_canAddConnection(self, connection) }
    }

    #[inline]
    pub fn add_connection(&mut self, connection: &av::CaptureConnection) {
        unsafe { wsel_addConnection(self, connection) }
    }

    #[inline]
    pub fn remove_connection(&mut self, connection: &av::CaptureConnection) {
        unsafe { wsel_removeConnection(self, connection) }
    }

    #[inline]
    pub fn begin_configuration(&mut self) {
        unsafe { wsel_beginConfiguration(self) }
    }

    #[inline]
    pub fn commit_configuration(&mut self) {
        unsafe { wsel_commitConfiguration(self) }
    }

    #[inline]
    pub fn start_running(&mut self) {
        unsafe { wsel_startRunning(self) }
    }

    #[inline]
    pub fn stop_running(&mut self) {
        unsafe { wsel_stopRunning(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_canSetSessionPreset(session: &Session, preset: &av::CaptureSessionPreset) -> bool;
    fn rsel_sessionPreset(session: &Session) -> &av::CaptureSessionPreset;
    fn wsel_setSessionPreset(session: &Session, value: &av::CaptureSessionPreset);

    fn rsel_inputs(session: &Session) -> &cf::ArrayOf<av::CaptureInput>;
    fn rsel_canAddInput(session: &Session, input: &av::CaptureInput) -> bool;
    fn wsel_addInput(session: &mut Session, input: &av::CaptureInput);
    fn wsel_removeInput(session: &mut Session, input: &av::CaptureInput);

    fn rsel_outputs(session: &Session) -> &cf::ArrayOf<av::CaptureOutput>;
    fn rsel_canAddOutput(session: &Session, output: &av::CaptureOutput) -> bool;
    fn wsel_addOutput(session: &mut Session, output: &av::CaptureOutput);
    fn wsel_removeOutput(session: &mut Session, output: &av::CaptureOutput);

    fn wsel_addInputWithNoConnections(session: &mut Session, input: &av::CaptureInput);
    fn wsel_addOutputWithNoConnections(session: &mut Session, output: &av::CaptureOutput);

    fn rsel_connections(id: &Id) -> &cf::ArrayOf<av::CaptureConnection>;

    fn rsel_canAddConnection(session: &Session, connection: &av::CaptureConnection) -> bool;
    fn wsel_addConnection(session: &mut Session, connection: &av::CaptureConnection);
    fn wsel_removeConnection(session: &mut Session, connection: &av::CaptureConnection);

    fn wsel_beginConfiguration(session: &mut Session);
    fn wsel_commitConfiguration(session: &mut Session);

    fn wsel_startRunning(session: &mut Session);
    fn wsel_stopRunning(session: &mut Session);
}

define_obj_type!(MultiCamSession(Session));

impl MultiCamSession {
    /// ```
    /// use cidre::av;
    ///
    /// assert!(!av::CaptureMultiCamSession::multicam_supported());
    /// ```
    pub fn multicam_supported() -> bool {
        unsafe { is_mutlicam_supported() }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn is_mutlicam_supported() -> bool;
}

define_obj_type!(Connection(Id));

pub mod notifications {
    use crate::cf;

    pub fn runtime_error() -> &'static cf::NotificationName {
        unsafe { AVCaptureSessionRuntimeErrorNotification }
    }

    pub fn did_start_running() -> &'static cf::NotificationName {
        unsafe { AVCaptureSessionDidStartRunningNotification }
    }

    pub fn did_stop_running() -> &'static cf::NotificationName {
        unsafe { AVCaptureSessionDidStopRunningNotification }
    }

    pub fn was_interrupted() -> &'static cf::NotificationName {
        unsafe { AVCaptureSessionWasInterruptedNotification }
    }

    pub fn interruption_ended() -> &'static cf::NotificationName {
        unsafe { AVCaptureSessionInterruptionEndedNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureSessionRuntimeErrorNotification: &'static cf::NotificationName;
        static AVCaptureSessionDidStartRunningNotification: &'static cf::NotificationName;
        static AVCaptureSessionDidStopRunningNotification: &'static cf::NotificationName;
        static AVCaptureSessionWasInterruptedNotification: &'static cf::NotificationName;
        static AVCaptureSessionInterruptionEndedNotification: &'static cf::NotificationName;
    }
}
