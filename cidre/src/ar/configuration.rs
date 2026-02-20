#[cfg(feature = "av")]
use crate::av;
use crate::{ar, arc, define_cls, define_obj_type, define_opts, ns, objc};

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
    define_cls!(AR_CONFIGURATION);

    /// Returns whether this configuration type is supported on the current device.
    #[objc::msg_send(isSupported)]
    pub fn is_supported() -> bool;

    /// Supported video formats for this configuration on the current device.
    ///
    /// The first element is the default session output format.
    #[objc::msg_send(supportedVideoFormats)]
    #[objc::available(ios = 12.0)]
    pub unsafe fn supported_video_formats_throws() -> arc::R<ns::Array<ar::VideoFormat>>;

    /// Video format used for session output.
    #[objc::msg_send(videoFormat)]
    #[objc::available(ios = 12.0)]
    pub fn video_format(&self) -> arc::R<ar::VideoFormat>;

    /// Sets the video format used for session output.
    #[objc::msg_send(setVideoFormat:)]
    #[objc::available(ios = 12.0)]
    pub fn set_video_format(&mut self, val: &ar::VideoFormat);

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

    /// Returns whether this configuration type supports the requested frame semantics.
    #[objc::msg_send(supportsFrameSemantics:)]
    #[objc::available(ios = 13.0)]
    pub fn supports_frame_semantics(frame_semantics: FrameSemantics) -> bool;

    /// Capture device used for rendering when camera settings are configurable.
    #[cfg(feature = "av")]
    #[objc::msg_send(configurableCaptureDeviceForPrimaryCamera)]
    #[objc::available(ios = 16.0)]
    pub fn configurable_capture_device_for_primary_camera() -> Option<arc::R<av::CaptureDevice>>;

    /// Recommended format using 4K res, when supported.
    #[objc::msg_send(recommendedVideoFormatFor4KResolution)]
    #[objc::available(ios = 16.0)]
    pub fn recommended_video_format_for_4k_res() -> Option<arc::R<ar::VideoFormat>>;

    /// Recommended format for high-res frame capture, when supported.
    #[objc::msg_send(recommendedVideoFormatForHighResolutionFrameCapturing)]
    #[objc::available(ios = 16.0)]
    pub fn recommended_video_format_for_high_res_frame_capturing() -> Option<arc::R<ar::VideoFormat>>;

    /// Whether HDR capture is allowed for HDR-capable video formats.
    #[objc::msg_send(videoHDRAllowed)]
    #[objc::available(ios = 16.0)]
    pub fn video_hdr_allowed(&self) -> bool;

    /// Enables/disables HDR capture for HDR-capable video formats.
    #[objc::msg_send(setVideoHDRAllowed:)]
    #[objc::available(ios = 16.0)]
    pub fn set_video_hdr_allowed(&mut self, val: bool);
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

    #[objc::msg_send(supportedVideoFormats)]
    #[objc::available(ios = 12.0)]
    pub fn supported_video_formats() -> arc::R<ns::Array<ar::VideoFormat>>;

    /// Returns whether this configuration supports `frame_semantics`.
    #[objc::msg_send(supportsFrameSemantics:)]
    #[objc::available(ios = 13.0)]
    pub fn supports_frame_semantics(frame_semantics: FrameSemantics) -> bool;

    /// Returns whether user face tracking is supported with world tracking.
    #[objc::msg_send(supportsUserFaceTracking)]
    #[objc::available(ios = 13.0)]
    pub fn supports_user_face_tracking() -> bool;

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

    /// Whether environment textures are provided with high dynamic range.
    ///
    /// Enabled by default.
    #[objc::msg_send(wantsHDREnvironmentTextures)]
    #[objc::available(ios = 13.0)]
    pub fn wants_hdr_environment_textures(&self) -> bool;

    /// Enables/disables HDR environment textures.
    #[objc::msg_send(setWantsHDREnvironmentTextures:)]
    #[objc::available(ios = 13.0)]
    pub fn set_wants_hdr_environment_textures(&mut self, val: bool);

    /// Whether automatic image scale estimation is enabled.
    #[objc::msg_send(automaticImageScaleEstimationEnabled)]
    #[objc::available(ios = 13.0)]
    pub fn automatic_image_scale_estimation_enabled(&self) -> bool;

    /// Enables/disables automatic image scale estimation.
    #[objc::msg_send(setAutomaticImageScaleEstimationEnabled:)]
    #[objc::available(ios = 13.0)]
    pub fn set_automatic_image_scale_estimation_enabled(&mut self, val: bool);

    /// Maximum number of images to track simultaneously.
    #[objc::msg_send(maximumNumberOfTrackedImages)]
    #[objc::available(ios = 12.0)]
    pub fn maximum_number_of_tracked_images(&self) -> ns::Integer;

    /// Sets the maximum number of images to track simultaneously.
    #[objc::msg_send(setMaximumNumberOfTrackedImages:)]
    #[objc::available(ios = 12.0)]
    pub fn set_maximum_number_of_tracked_images(&mut self, val: ns::Integer);

    /// Whether collaboration is enabled.
    #[objc::msg_send(isCollaborationEnabled)]
    #[objc::available(ios = 13.0)]
    pub fn collaboration_enabled(&self) -> bool;

    /// Enables/disables collaboration.
    #[objc::msg_send(setCollaborationEnabled:)]
    #[objc::available(ios = 13.0)]
    pub fn set_collaboration_enabled(&mut self, val: bool);

    /// Whether App Clip code tracking is enabled.
    #[objc::msg_send(appClipCodeTrackingEnabled)]
    #[objc::available(ios = 14.3)]
    pub fn app_clip_code_tracking_enabled(&self) -> bool;

    /// Enables/disables App Clip code tracking.
    #[objc::msg_send(setAppClipCodeTrackingEnabled:)]
    #[objc::available(ios = 14.3)]
    pub fn set_app_clip_code_tracking_enabled(&mut self, val: bool);

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
    static AR_CONFIGURATION: &'static objc::Class<Cfg>;
    static AR_WORLD_TRACKING_CONFIGURATION: &'static objc::Class<WorldTrackingCfg>;
}
