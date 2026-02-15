use crate::{ar, arc, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "ARFrameSemantics")]
    /// Semantic understanding types produced for each frame.
    pub FrameSemantics(usize)
);

impl FrameSemantics {
    /// No semantic operation is run.
    pub const NONE: Self = Self(0);
    /// Person segmentation.
    pub const PERSON_SEGMENTATION: Self = Self(1 << 0);
    /// Person segmentation with depth.
    pub const PERSON_SEGMENTATION_WITH_DEPTH: Self = Self((1 << 1) | (1 << 0));
    /// Body detection.
    pub const BODY_DETECTION: Self = Self(1 << 2);
    /// Scene depth.
    pub const SCENE_DEPTH: Self = Self(1 << 3);
    /// Temporally smoothed scene depth.
    pub const SMOOTHED_SCENE_DEPTH: Self = Self(1 << 4);
}

#[doc(alias = "ARWorldAlignment")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum WorldAlignment {
    /// Aligns world up-vector to gravity.
    Gravity,
    /// Aligns world to gravity and heading (True North).
    GravityAndHeading,
    /// Aligns world with the camera orientation.
    Camera,
}

#[doc(alias = "AREnvironmentTexturing")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum EnvironmentTexturing {
    /// No environment texturing.
    None,
    /// Texture data is gathered for explicitly added probes.
    Manual,
    /// Texture data is gathered with automatically placed probes.
    Automatic,
}

define_opts!(
    #[doc(alias = "ARSceneReconstruction")]
    /// Scene reconstruction modes.
    pub SceneReconstruction(usize)
);

impl SceneReconstruction {
    /// No scene reconstruction is run.
    pub const NONE: Self = Self(0);
    /// Generate a scene mesh.
    pub const MESH: Self = Self(1 << 0);
    /// Generate a scene mesh with per-face classification.
    pub const MESH_WITH_CLASSIFICATION: Self = Self((1 << 1) | (1 << 0));
}

define_obj_type!(
    #[doc(alias = "ARConfiguration")]
    /// Base configuration describing AR techniques to run in an `ar::Session`.
    pub Cfg(ns::Id)
);

impl Cfg {
    /// World coordinate alignment.
    ///
    /// Default is `ar::WorldAlignment::Gravity`.
    #[objc::msg_send(worldAlignment)]
    pub fn world_alignment(&self) -> WorldAlignment;

    /// Sets world coordinate alignment.
    #[objc::msg_send(setWorldAlignment:)]
    pub fn set_world_alignment(&mut self, val: WorldAlignment);

    /// Whether light estimation is enabled.
    ///
    /// Enabled by default.
    #[objc::msg_send(isLightEstimationEnabled)]
    pub fn is_light_estimation_enabled(&self) -> bool;

    /// Enables/disables light estimation.
    #[objc::msg_send(setLightEstimationEnabled:)]
    pub fn set_light_estimation_enabled(&mut self, val: bool);

    /// Whether audio data is captured and provided by the session.
    ///
    /// Disabled by default.
    #[objc::msg_send(providesAudioData)]
    pub fn provides_audio_data(&self) -> bool;

    /// Enables/disables audio data capture.
    #[objc::msg_send(setProvidesAudioData:)]
    pub fn set_provides_audio_data(&mut self, val: bool);

    /// Frame semantics requested for produced frames.
    #[objc::msg_send(frameSemantics)]
    #[objc::available(ios = 13.0)]
    pub fn frame_semantics(&self) -> FrameSemantics;

    /// Sets frame semantics requested for produced frames.
    ///
    /// Use `supports_frame_semantics` on the concrete configuration type
    /// to verify support before setting.
    #[objc::msg_send(setFrameSemantics:)]
    #[objc::available(ios = 13.0)]
    pub fn set_frame_semantics(&mut self, val: FrameSemantics);
}

define_obj_type!(
    #[doc(alias = "ARWorldTrackingConfiguration")]
    /// World-tracking configuration (6DoF tracking).
    pub WorldTrackingCfg(Cfg),
    AR_WORLD_TRACKING_CONFIGURATION
);

impl WorldTrackingCfg {
    /// Returns whether world tracking is supported on the current device.
    #[objc::msg_send(isSupported)]
    pub fn is_supported() -> bool;

    /// Returns whether this configuration supports `frame_semantics`.
    #[objc::msg_send(supportsFrameSemantics:)]
    #[objc::available(ios = 13.0)]
    pub fn supports_frame_semantics(frame_semantics: FrameSemantics) -> bool;

    /// Plane types currently detected by the session.
    #[objc::msg_send(planeDetection)]
    pub fn plane_detection(&self) -> ar::PlaneDetection;

    /// Sets plane types to detect.
    #[objc::msg_send(setPlaneDetection:)]
    pub fn set_plane_detection(&mut self, val: ar::PlaneDetection);

    /// Environment texturing mode.
    ///
    /// Default is `ar::EnvironmentTexturing::None`.
    #[objc::msg_send(environmentTexturing)]
    #[objc::available(ios = 12.0)]
    pub fn environment_texturing(&self) -> EnvironmentTexturing;

    /// Sets environment texturing mode.
    #[objc::msg_send(setEnvironmentTexturing:)]
    #[objc::available(ios = 12.0)]
    pub fn set_environment_texturing(&mut self, val: EnvironmentTexturing);

    /// Scene reconstruction mode.
    #[objc::msg_send(sceneReconstruction)]
    #[objc::available(ios = 14.0)]
    pub fn scene_reconstruction(&self) -> SceneReconstruction;

    /// Sets scene reconstruction mode.
    #[objc::msg_send(setSceneReconstruction:)]
    #[objc::available(ios = 14.0)]
    pub fn set_scene_reconstruction(&mut self, val: SceneReconstruction);
}

#[link(name = "ar", kind = "static")]
unsafe extern "C" {
    static AR_WORLD_TRACKING_CONFIGURATION: &'static objc::Class<WorldTrackingCfg>;
}
