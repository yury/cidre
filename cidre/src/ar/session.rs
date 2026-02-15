use crate::{ar, arc, cm, define_obj_type, define_opts, ns, objc};

#[cfg(feature = "dispatch")]
use crate::dispatch;

define_opts!(
    #[doc(alias = "ARSessionRunOptions")]
    pub SessionRunOpts(usize)
);

impl SessionRunOpts {
    /// Reset world tracking state.
    pub const RESET_TRACKING: Self = Self(1 << 0);
    /// Remove all existing anchors from the session.
    pub const REMOVE_EXISTING_ANCHORS: Self = Self(1 << 1);
    /// Stop currently active tracked raycasts.
    pub const STOP_TRACKED_RAYCASTS: Self = Self(1 << 2);
    /// Reset scene reconstruction state.
    pub const RESET_SCENE_RECONSTRUCTION: Self = Self(1 << 3);
}

define_obj_type!(
    #[doc(alias = "ARSession")]
    /// Configures and runs AR techniques on the device.
    pub Session(ns::Id),
    AR_SESSION
);

impl Session {
    /// Unique identifier of the running session.
    ///
    /// May change after `run_with_cfg`/`run_with_cfg_opts`.
    #[objc::msg_send(identifier)]
    #[objc::available(ios = 13.0)]
    pub fn id(&self) -> arc::R<ns::Uuid>;

    /// Delegate receiving session updates.
    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnySessionDelegate>>;

    /// Sets the delegate receiving session updates.
    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: SessionDelegate>(&mut self, val: Option<&D>);

    /// Queue used to invoke delegate callbacks.
    ///
    /// If `None`, callbacks are delivered on the main queue.
    #[cfg(feature = "dispatch")]
    #[objc::msg_send(delegateQueue)]
    pub fn delegate_queue(&self) -> Option<arc::R<dispatch::Queue>>;

    /// Sets the queue used to invoke delegate callbacks.
    ///
    /// If `None`, callbacks are delivered on the main queue.
    #[cfg(feature = "dispatch")]
    #[objc::msg_send(setDelegateQueue:)]
    pub fn set_delegate_queue(&mut self, val: Option<&dispatch::Queue>);

    /// Current frame produced by the session.
    #[objc::msg_send(currentFrame)]
    pub fn current_frame(&self) -> Option<arc::R<ar::Frame>>;

    /// Configuration currently used by the session.
    #[objc::msg_send(configuration)]
    pub fn cfg(&self) -> Option<arc::R<ar::Cfg>>;

    /// Runs the session using `cfg`.
    ///
    /// Running an already started session immediately transitions
    /// to the new configuration.
    #[objc::msg_send(runWithConfiguration:)]
    pub fn run_with_cfg(&mut self, cfg: &ar::Cfg);

    /// Runs the session using `cfg` and run options.
    ///
    /// Running an already started session immediately transitions
    /// to the new configuration.
    #[objc::msg_send(runWithConfiguration:options:)]
    pub fn run_with_cfg_opts(&mut self, cfg: &ar::Cfg, opts: SessionRunOpts);

    /// Pauses the session.
    #[objc::msg_send(pause)]
    pub fn pause(&mut self);

    /// Adds an anchor; it will appear in a subsequent frame update.
    #[objc::msg_send(addAnchor:)]
    pub fn add_anchor(&mut self, anchor: &ar::Anchor);

    /// Removes an anchor from subsequent frame updates.
    #[objc::msg_send(removeAnchor:)]
    pub fn remove_anchor(&mut self, anchor: &ar::Anchor);

    /// Performs a raycast and returns results sorted nearest-to-farthest.
    #[objc::msg_send(raycast:)]
    #[objc::available(ios = 13.0)]
    pub fn raycast(&self, query: &ar::RaycastQuery) -> arc::R<ns::Array<ar::RaycastResult>>;
}

#[objc::protocol(ARSessionObserver)]
/// Observer protocol for AR session state changes.
pub trait SessionObserver: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(session:didFailWithError:)]
    fn session_did_fail_with_error(&mut self, session: &Session, error: &ns::Error);

    #[objc::optional]
    #[objc::msg_send(session:cameraDidChangeTrackingState:)]
    fn session_camera_did_change_tracking_state(&mut self, session: &Session, camera: &ar::Camera);

    #[objc::optional]
    #[objc::msg_send(sessionWasInterrupted:)]
    fn session_was_interrupted(&mut self, session: &Session);

    #[objc::optional]
    #[objc::msg_send(sessionInterruptionEnded:)]
    fn session_interruption_ended(&mut self, session: &Session);

    #[objc::optional]
    #[objc::msg_send(sessionShouldAttemptRelocalization:)]
    fn session_should_attempt_relocalization(&mut self, session: &Session) -> bool;

    #[objc::optional]
    #[objc::msg_send(session:didOutputAudioSampleBuffer:)]
    fn session_did_output_audio_sample_buf(
        &mut self,
        session: &Session,
        audio_sample_buffer: &cm::SampleBuf,
    );

    #[objc::optional]
    #[objc::msg_send(session:didOutputCollaborationData:)]
    fn session_did_output_collaboration_data(
        &mut self,
        session: &Session,
        data: &CollaborationData,
    );

    #[objc::optional]
    #[objc::msg_send(session:didChangeGeoTrackingStatus:)]
    fn session_did_change_geo_tracking_status(
        &mut self,
        session: &Session,
        geo_tracking_status: &GeoTrackingStatus,
    );
}

#[objc::protocol(ARSessionDelegate)]
/// Delegate protocol for per-frame and anchor updates.
pub trait SessionDelegate: SessionObserver {
    #[objc::optional]
    #[objc::msg_send(session:didUpdateFrame:)]
    fn session_did_update_frame(&mut self, session: &Session, frame: &ar::Frame);

    #[objc::optional]
    #[objc::msg_send(session:didAddAnchors:)]
    fn session_did_add_anchors(&mut self, session: &Session, anchors: &ns::Array<ar::Anchor>);

    #[objc::optional]
    #[objc::msg_send(session:didUpdateAnchors:)]
    fn session_did_update_anchors(&mut self, session: &Session, anchors: &ns::Array<ar::Anchor>);

    #[objc::optional]
    #[objc::msg_send(session:didRemoveAnchors:)]
    fn session_did_remove_anchors(&mut self, session: &Session, anchors: &ns::Array<ar::Anchor>);
}

define_obj_type!(
    #[doc(alias = "ARCollaborationData")]
    pub CollaborationData(ns::Id)
);

define_obj_type!(
    #[doc(alias = "ARGeoTrackingStatus")]
    pub GeoTrackingStatus(ns::Id)
);

define_obj_type!(
    pub AnySessionDelegate(ns::Id)
);

impl SessionObserver for AnySessionDelegate {}
impl SessionDelegate for AnySessionDelegate {}

#[link(name = "ar", kind = "static")]
unsafe extern "C" {
    static AR_SESSION: &'static objc::Class<Session>;
}
