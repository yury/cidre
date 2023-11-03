use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use crate::{arc, av, cg, cm, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVCaptureDeviceTypeExternal: &'static Type;
    static AVCaptureDeviceTypeBuiltInMicrophone: &'static Type;
    static AVCaptureDeviceTypeBuiltInWideAngleCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTelephotoCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInUltraWideCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInDualCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInDualWideCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTripleCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTrueDepthCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInLiDARDepthCamera: &'static Type;
    static AVCaptureDeviceTypeContinuityCamera: &'static Type;
    #[cfg(target_os = "macos")]
    static AVCaptureDeviceTypeDeskViewCamera: &'static Type;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    static AVCaptureLensPositionCurrent: f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    static AVCaptureISOCurrent: f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    static AVCaptureExposureDurationCurrent: cm::Time;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    static AVCaptureExposureTargetBiasCurrent: f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    static AVCaptureWhiteBalanceGainsCurrent: WbGains;

    static AVCaptureMaxAvailableTorchLevel: f32;
}

define_obj_type!(Device(ns::Id));

impl Device {
    define_cls!(AV_CAPTURE_DEVICE);

    #[objc::cls_msg_send(defaultDeviceWithDeviceType:mediaType:position:)]
    pub fn with_device_type_media_and_position_ar(
        device_type: &Type,
        media_type: Option<&av::MediaType>,
        position: Position,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_device_type_media_and_position(
        device_type: &Type,
        media_type: Option<&av::MediaType>,
        position: Position,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(devices)]
    pub fn devices_ar() -> arc::Rar<ns::Array<Self>>;

    #[objc::cls_rar_retain]
    pub fn devices() -> arc::R<ns::Array<Self>>;

    #[objc::cls_msg_send(deviceWithUniqueID:)]
    pub fn with_unique_id_ar(unique_id: ns::String) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_unique_id(unique_id: ns::String) -> Option<arc::R<Self>>;

    #[objc::msg_send(uniqueID)]
    pub fn unique_id(&self) -> &ns::String;

    /// The model ID of the receiver.
    ///
    /// The value of this property is an identifier unique to all devices of the same model.
    /// The value is persistent across device connections and disconnections,
    /// and across different systems. For example, the model ID of the camera built in
    /// to two identical iPhone models will be the same even though they are different
    /// physical devices.
    #[objc::msg_send(modelID)]
    pub fn model_id(&self) -> &ns::String;

    /// A localized human-readable name for the receiver.
    ///
    /// This property can be used for displaying the name of a capture device in a user interface.
    #[objc::msg_send(localizedName)]
    pub fn localized_name(&self) -> &ns::String;

    /// The human-readable manufacturer name for the receiver.
    ///
    /// This property can be used to identify capture devices from a particular manufacturer.
    /// All Apple devices return "Apple Inc.". Devices from third party manufacturers may
    /// return an empty string.
    #[objc::msg_send(manufacturer)]
    pub fn manufacturer(&self) -> &ns::String;

    /// Returns whether the receiver provides media with the given media type.
    ///
    /// 'true' if the device outputs the given media type, 'false' otherwise.
    #[objc::msg_send(hasMediaType:)]
    pub fn has_media_type(&self, media_type: &av::MediaType) -> bool;

    #[objc::msg_send(formats)]
    pub fn formats(&self) -> &ns::Array<Format>;

    #[objc::msg_send(supportsAVCaptureSessionPreset:)]
    pub fn supports_preset(&self, preset: &av::CaptureSessionPreset) -> bool;

    #[objc::msg_send(activeFormat)]
    pub fn active_format(&self) -> &Format;

    /// Indicates whether the device is connected and available to the system.
    ///
    /// The value of this property is a 'bool' indicating whether the device represented
    /// by the receiver is connected and available for use as a capture device.
    /// Clients can key value observe the value of this property to be notified when a device
    /// is no longer available. When the value of this property becomes 'false' for a given
    /// instance, it will not become 'true' again. If the same physical device again becomes
    /// available to the system, it will be represented using a new instance of 'av::CaptureDevice'.
    #[objc::msg_send(isConnected)]
    pub fn is_connected(&self) -> bool;

    #[objc::msg_send(activeVideoMinFrameDuration)]
    pub fn active_video_min_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(activeVideoMaxFrameDuration)]
    pub fn active_video_max_frame_duration(&self) -> cm::Time;

    pub fn config_lock(&mut self) -> Result<ConfigLockGuard, arc::R<ns::Error>> {
        let mut error = None;
        unsafe {
            let result = self.lock_for_config(&mut error);
            if let Some(error) = error.take() {
                return Err(error);
            }

            debug_assert!(result);

            Ok(ConfigLockGuard { device: self })
        }
    }

    #[objc::msg_send(lockForConfiguration:)]
    pub unsafe fn lock_for_config(&mut self, error: &mut Option<arc::R<ns::Error>>) -> bool;

    #[objc::msg_send(unlockForConfiguration)]
    pub unsafe fn unlock_for_config(&mut self);

    #[objc::msg_send(setActiveFormat:)]
    pub unsafe fn set_active_format(&mut self, val: &Format);

    #[objc::msg_send(setActiveVideoMinFrameDuration:)]
    pub unsafe fn set_active_video_min_frame_duration(&mut self, val: cm::Time);

    #[objc::msg_send(setActiveVideoMaxFrameDuration:)]
    pub unsafe fn set_active_video_max_frame_duration(&mut self, val: cm::Time);
}

#[doc(alias = "AVCaptureTorchMode")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum TorchMode {
    Off = 0,
    On = 1,
    Auto = 2,
}

/// AVCaptureDeviceTorch
impl Device {
    #[objc::msg_send(hasTorch)]
    pub fn has_torch(&self) -> bool;

    /// Indicates whether the receiver's torch is currently available for use.
    #[objc::msg_send(isTorchAvailable)]
    pub fn is_torch_available(&self) -> bool;

    #[objc::msg_send(isTorchActive)]
    pub fn is_torch_active(&self) -> bool;

    #[objc::msg_send(torchLevel)]
    pub fn torch_level(&self) -> f32;

    #[objc::msg_send(isTorchModeSupported:)]
    pub fn is_torch_mode_supported(&self, val: TorchMode) -> bool;

    #[objc::msg_send(torchMode)]
    pub fn torch_mode(&self) -> TorchMode;

    #[objc::msg_send(setTorchMode:)]
    unsafe fn set_torch_mode_throws(&self, val: TorchMode);

    #[objc::msg_send(setTorchModeOnWithLevel:error:)]
    unsafe fn set_torch_mode_on_with_level_err(
        &mut self,
        torch_level: f32,
        error: *mut Option<arc::R<ns::Error>>,
    ) -> bool;
}

/// AVCaptureDeviceTorch
impl<'a> ConfigLockGuard<'a> {
    pub unsafe fn set_torch_mode_throws(&mut self, val: TorchMode) {
        self.device.set_torch_mode_throws(val)
    }

    pub fn set_torch_mode<'ar>(&mut self, val: TorchMode) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_torch_mode_throws(val) })
    }

    pub fn set_torch_mode_on_with_level_err(
        &mut self,
        torch_level: f32,
        error: *mut Option<arc::R<ns::Error>>,
    ) -> bool {
        unsafe {
            self.device
                .set_torch_mode_on_with_level_err(torch_level, error)
        }
    }

    pub fn set_torch_mode_on_with_level(
        &mut self,
        torch_level: f32,
    ) -> Result<(), arc::R<ns::Error>> {
        let mut error = None;
        if self.set_torch_mode_on_with_level_err(torch_level, &mut error) {
            return Ok(());
        } else {
            unsafe { Err(error.unwrap_unchecked()) }
        }
    }
}

/// AVCaptureDeviceReactionEffects
impl Device {
    #[objc::cls_msg_send(reactionEffectsEnabled)]
    pub fn reaction_effects_enabled() -> bool;

    #[objc::cls_msg_send(reactionEffectGesturesEnabled)]
    pub fn reaction_effect_gestures_enabled() -> bool;

    #[objc::msg_send(canPerformReactionEffects)]
    pub fn can_perform_reaction_effects(&self) -> bool;

    /// Returns a list of reaction types which can be passed to perform_effect_for_reaction.
    ///
    /// The list may differ between devices, or be affected by changes to active format,
    /// and can be key-value observed.
    #[objc::msg_send(availableReactionTypes)]
    pub fn available_reaction_types(&self) -> &ns::Set<av::CaptureReactionType>;

    /// Triggers a specified reaction on the video stream.
    #[objc::msg_send(performEffectForReaction:)]
    pub fn perform_effect_for_reaction(&mut self, reaction_type: &av::CaptureReactionType);

    #[objc::msg_send(reactionEffectsInProgress)]
    pub fn reaction_effects_in_progress(&self) -> &ns::Array<av::CaptureReactionEffectState>;
}

/// AVCaptureDeviceContinuityCamera
impl Device {
    /// A property that reports YES if the receiver is a Continuity Camera.
    ///
    /// Access this property to discover if the receiver is
    /// a Continuity Camera (external iPhone webcam).
    #[objc::msg_send(isContinuityCamera)]
    pub fn is_continuity_camera(&self) -> bool;
}

#[doc(alias = "AVCaptureMicrophoneMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum MicMode {
    Standard = 0,
    WideSpectrum = 1,
    VoiceIsolation = 2,
}

/// AVCaptureMicrophoneMode
impl Device {
    /// Indicates the microphone mode that has been selected by the user in Control Center.
    ///
    /// This readonly property returns the microphone mode selected by the user in Control Center. It is key-value observable.
    #[objc::cls_msg_send(preferredMicrophoneMode)]
    pub fn preferred_mic_mode() -> MicMode;

    /// Indicates the currently active microphone mode.
    ///
    /// This readonly property returns the currently active microphone mode,
    /// which may differ from the 'preferred_mic_mode()' if the application's
    /// active audio route does not support the preferred microphone mode.
    /// This property is key-value observable.
    #[objc::cls_msg_send(activeMicrophoneMode)]
    pub fn active_mic_mode() -> MicMode;
}

#[doc(alias = "AVCaptureCenterStageControlMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CenterStageControlMode {
    /// Indicates that the application is unaware of the Center Stage feature.
    /// Its enablement is entirely under user control in Control Center.
    User = 0,

    /// Indicates that the application controls the Center Stage feature,
    /// disallowing input from the user in Control Center.
    App = 1,

    /// Indicates that both the user and application cooperatively share
    /// control of the Center Stage feature.
    Cooperative = 2,
}

impl Device {
    /// Current mode of Center Stage control (user, app, or cooperative).
    ///
    /// This class property determines how the Center Stage feature is controlled.
    /// When set to the default value of 'av::CaptureCenterStageControlMode::User',
    /// 'set_center_stage_enabled' may not be set programmatically and throws an
    /// NSInvalidArgumentException. In User mode, the feature may only be set by
    /// the user in Control Center. If you wish to take Center Stage control away
    /// from the user and exclusively enable / disable it programmatically, set this property
    /// to 'av::CaptureCenterStageControlMode::App'. When under exclusive app control,
    /// Center Stage user control is disallowed (for instance, the toggle is grayed out
    /// in Control Center). If you wish to take control of Center Stage, but also cooperate
    /// with the user by listening for and appropriately reacting to their changes to the
    /// 'center_stage_enabled' property, set this property to
    /// 'av::CaptureCenterStageControlMode::Cooperative'. Note that in this mode,
    /// the onus is on you, the app developer, to honor user intent and conform your
    /// 'av::CaptureSession' configuration to make Center Stage active (see the 'av::CaptureDevice'
    /// instance property 'center_stage_active'). In cooperative mode, the 'center_stage_enabled'
    /// property may change at any time (such as when the user enables / disables the feature
    /// in Control Center).
    #[objc::cls_msg_send(centerStageControlMode)]
    pub fn center_stage_control_mode() -> CenterStageControlMode;

    #[objc::cls_msg_send(setCenterStageControlMode:)]
    pub fn set_center_stage_control_mode(val: CenterStageControlMode);

    /// Indicates whether Center Stage is currently active on a particular av::CaptureDevice.
    ///
    /// This readonly property returns 'true' when Center Stage is currently active on
    /// the receiver. When active, the camera automatically adjusts to keep people optimally
    /// framed within the field of view. The field of view may pan, tighten or widen as needed.
    /// Certain restrictions come into play when Center Stage is active:
    ///
    /// - The device's 'min_available_video_zoom_factor' and 'max_available_video_zoom_factor'
    ///   become restricted (see av::CaptureDeviceFormat's 'video_min_zoom_factor_for_center_stage'
    ///   and 'video_max_zoom_factor_for_center_stage').
    /// - The device's 'active_video_min_frame_duration' and 'active_video_max_frame_duration'
    ///   are limited (see av::CaptureDeviceFormat's 'video_frame_rate_range_for_center_stage').
    ///
    /// Center Stage may be enabled via user control or application control, depending on the
    /// current 'av::CaptureDevice::center_stage_control_mode()'.
    /// When 'av::CaptureDevice::is_center_stage_enabled()' is 'true', a particular
    /// 'av::CaptureDevice' instance may return 'true' for this property, depending whether it
    /// supports the feature in its current configuration. Some device features are mutually
    /// exclusive to Center Stage:
    ///
    /// - If depth data delivery is enabled on any output, such as 'av::CaptureDepthDataOutput',
    ///   or 'av::CapturePhotoOutput.depth_data_delivery_enabled', Center Stage is deactivated.
    /// - If 'geometric_distortion_correction_supported' is 'true',
    ///   'geometric_distortion_correction_enabled' must also be 'true', or Center Stage
    ///   is deactivated.
    ///
    /// This property is key-value observable.
    #[objc::msg_send(isCenterStageActive)]
    pub fn is_center_stage_active(&self) -> bool;

    #[objc::msg_send(centerStageRectOfInterest)]
    pub fn center_stage_rect_of_interest(&self) -> cg::Rect;

    /// Specifies the effective region within the output pixel buffer that will be used
    /// to perform Center Stage framing.
    ///
    /// Applications that wish to apply additional processing (such as cropping) on top of
    /// Center Stage's output can use this property to guide Center Stage's framing.
    ///
    /// The rectangle's origin is top left and is relative to the coordinate space of the
    /// output pixel buffer. The default value of this property is the value
    /// 'cg::Rect::new(0.0, 0.0, 1.0, 1.0)', where {0.0,0.0} represents the top left of the
    /// picture area, and {1.0,1.0} represents the bottom right on an unrotated picture.
    /// This rectangle of interest is applied prior to rotation, mirroring or scaling.
    ///
    /// Pixels outside of this rectangle of interest will be blackened out.
    ///
    /// Setting this property has no impact on objects specified in the metadata output.
    ///
    /// 'set_center_stage_rect_of_interest_throws': throws an 'ns::ExceptionName::generic()'
    /// if called without first obtaining exclusive access to the receiver using
    /// 'lock_for_configuration' 'set_center_stage_rect_of_interest_throws'
    /// throws an 'ns::ExceptionName::invalid_argument()' if none of the av::CaptureDeviceFormats
    /// supported by the receiver support CenterStage.
    /// 'set_center_stage_rect_of_interest_throws' throws an 'ns::ExceptionName::invalid_argument()'
    /// '::center_stage_enabled' is 'false' on the av::CaptureDevice class.
    /// 'set_center_stage_rect_of_interest_throws' throws an 'ns::ExceptionName::invalid_argument()'
    /// if the provided rect of interest goes outside the normalized (0-1) coordinate space.
    #[objc::msg_send(setCenterStageRectOfInterest:)]
    pub unsafe fn set_center_stage_rect_of_interest_throws(&mut self, val: cg::Rect);
}

#[doc(alias = "AVCaptureSystemUserInterface")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum SystemUi {
    VideoEffects = 1,
    MicModes = 2,
}

/// AVCaptureSystemUserInterface
impl Device {
    #[objc::cls_msg_send(showSystemUserInterface:)]
    pub fn show_system_ui(system_ui: SystemUi);
}

#[doc(alias = "AVCaptureColorSpace")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ColorSpace {
    /// The sRGB color space (<https://www.w3.org/Graphics/Color/srgb>)
    Srgb = 0,
    /// The P3 D65 wide color space which uses Illuminant D65 as the white point.
    P3D65 = 1,
    /// The BT2020 wide color space which uses Illuminant D65 as the white point
    /// and Hybrid Log-Gamma as the transfer function.
    HlgBt2020 = 2,
    /// The Apple Log Color space, which uses BT2020 as the color primaries,
    /// and an Apple defined Log curve as a transfer function. When this is set as the active color
    /// space on an 'av::CaptureDevice', any 'av::CapturePhotoOutput' or 'av::CaptureStillImageOutput'
    /// connected to the same 'av::CaptureDevice' will have its video connection disabled.
    AppleLog = 3,
}

/// AVCaptureDeviceColorSpaceSupport
impl Device {
    /// Indicates the receiver's current active color space.
    #[objc::msg_send(activeColorSpace)]
    pub fn active_color_space(&self) -> ColorSpace;

    /// By default, an 'av::CaptureDevice' attached to an 'av::CaptureSession' is automatically
    /// configured for wide color by the 'av::CaptureSession'
    /// (see AVCaptureSession automaticallyConfiguresCaptureDeviceForWideColor).
    /// You may also set the active_ColorSpace manually. To prevent the AVCaptureSession from
    /// undoing your work, remember to set AVCaptureSession's automaticallyConfiguresCaptureDeviceForWideColor
    /// property to NO. Changing the receiver's activeColorSpace while the session is running
    /// requires a disruptive reconfiguration of the capture render pipeline. Movie captures
    /// in progress will be ended immediately; unfulfilled photo requests will be aborted;
    /// video preview will temporarily freeze. -setActiveColorSpace: throws an NSGenericException
    /// if called without first obtaining exclusive access to the receiver
    /// using -lockForConfiguration:.
    #[objc::msg_send(setActiveColorSpace:)]
    pub unsafe fn set_active_color_space_throws(&mut self, val: ColorSpace);
}

pub struct ConfigLockGuard<'a> {
    device: &'a mut Device,
}

impl<'a> ConfigLockGuard<'a> {
    pub fn set_active_format(&mut self, val: &Format) {
        unsafe { self.device.set_active_format(val) }
    }

    pub fn set_active_video_min_frame_duration(&mut self, val: cm::Time) {
        unsafe { self.device.set_active_video_min_frame_duration(val) }
    }

    pub fn set_active_video_max_frame_duration(&mut self, val: cm::Time) {
        unsafe { self.device.set_active_video_max_frame_duration(val) }
    }

    pub fn set_center_stage_rect_of_interest<'ar>(
        &mut self,
        val: cg::Rect,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.device.set_center_stage_rect_of_interest_throws(val) })
    }

    pub fn set_active_color_space<'ar>(&mut self, val: ColorSpace) {
        unsafe { self.device.set_active_color_space_throws(val) }
    }

    #[inline]
    pub unsafe fn set_focus_mode_throws(&mut self, mode: FocusMode) {
        self.device.set_focus_mode_throws(mode)
    }

    pub fn set_focus_mode<'ar>(&mut self, mode: FocusMode) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_focus_mode_throws(mode) })
    }

    #[inline]
    pub unsafe fn set_focus_point_of_interest_throws(&mut self, val: cg::Point) {
        self.device.set_focus_point_of_interest_throws(val)
    }

    /// The value of this property is a 'cg::Point' that determines the receiver's focus point of interest,
    /// if it has one. A value of (0,0) indicates that the camera should focus on the top left corner of the image,
    /// while a value of (1,1) indicates that it should focus on the bottom right.
    /// The default value is (0.5,0.5).
    ///
    /// Clients can observe automatic changes to the receiver's 'focusPointOfInterest' by key value observing this property.
    /// Note that setting focusPointOfInterest alone does not initiate a focus operation. After setting
    /// 'set_focus_point_of_interest', call 'set_focus_mode()' to apply the new point of interest.
    pub fn set_focus_point_of_interest<'ar>(
        &mut self,
        val: cg::Point,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_focus_point_of_interest_throws(val) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_auto_focus_range_restriction_throws(
        &mut self,
        val: AutoFocusRangeRestriction,
    ) {
        self.device.set_auto_focus_range_restriction_throws(val)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_auto_focus_range_restriction<'ar>(
        &mut self,
        val: AutoFocusRangeRestriction,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_auto_focus_range_restriction_throws(val) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_smooth_auto_focus_enabled_throws(&mut self, val: bool) {
        self.device.set_smooth_auto_focus_enabled_throws(val)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_smooth_auto_focus_enabled<'ar>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_smooth_auto_focus_enabled_throws(val) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_automatically_adjusts_face_driven_auto_focus_enabled_throws(
        &mut self,
        val: bool,
    ) {
        self.device
            .set_automatically_adjusts_face_driven_auto_focus_enabled_throws(val)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_automatically_adjusts_face_driven_auto_focus_enabled<'ar>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.set_automatically_adjusts_face_driven_auto_focus_enabled_throws(val)
        })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_face_driven_auto_focus_enabled_throws(&mut self, val: bool) {
        self.device.set_face_driven_auto_focus_enabled_throws(val)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_face_driven_auto_focus_enabled<'ar>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_face_driven_auto_focus_enabled_throws(val) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_focus_mode_locked_with_lens_position_no_ch_throws(&mut self, val: f32) {
        self.device
            .set_focus_mode_locked_with_lens_position_ch_throws(val, std::ptr::null_mut())
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_focus_mode_locked_with_lens_position_no_ch<'ar>(
        &mut self,
        val: f32,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_focus_mode_locked_with_lens_position_no_ch_throws(val) })
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_focus_mode_locked_with_lens_position_with_ch_throws<F>(
        &mut self,
        val: f32,
        block: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(cm::Time),
    {
        self.device
            .set_focus_mode_locked_with_lens_position_ch_throws(val, block.as_mut_ptr())
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_focus_mode_locked_with_lens_position_with_ch<'ar, F>(
        &mut self,
        val: f32,
        block: &'static mut blocks::Block<F>,
    ) -> Result<(), &'ar ns::Exception>
    where
        F: FnOnce(cm::Time),
    {
        ns::try_catch(|| unsafe {
            self.device
                .set_focus_mode_locked_with_lens_position_ch_throws(val, block.as_mut_ptr())
        })
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async unsafe fn set_focus_mode_locked_with_lens_position_throws(
        &mut self,
        val: f32,
    ) -> cm::Time {
        let (future, block) = blocks::comp1();
        self.set_focus_mode_locked_with_lens_position_with_ch_throws(val, block.escape());
        future.await
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async fn set_focus_mode_locked_with_lens_position(
        &mut self,
        val: f32,
    ) -> Result<cm::Time, arc::R<ns::Exception>> {
        let (future, block) = blocks::comp1();
        let res = ns::try_catch(move || unsafe {
            self.set_focus_mode_locked_with_lens_position_with_ch_throws(val, block.escape())
        });
        if let Err(err) = res {
            return Err(err.retained());
        }
        Ok(future.await)
    }
}

#[doc(alias = "AVCaptureDevicePosition")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(isize)]
pub enum Position {
    Unspecified = 0,
    Back = 1,
    Front = 2,
}

/// AVCaptureDevicePosition
impl Device {
    #[objc::msg_send(position)]
    pub fn position(&self) -> Position;
}

impl<'a> Drop for ConfigLockGuard<'a> {
    fn drop(&mut self) {
        unsafe { self.device.unlock_for_config() }
    }
}

impl<'a> Deref for ConfigLockGuard<'a> {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        self.device
    }
}

impl<'a> DerefMut for ConfigLockGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.device
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_DEVICE: &'static objc::Class<Device>;
}

#[doc(alias = "AVCaptureFocusMode")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum FocusMode {
    /// Indicates that the focus should be locked at the lens' current position
    Locked = 0,

    /// Indicates that the device should autofocus once and then change the focus mode
    /// to 'av::CaptureFocusMode::Locked'.
    AutoFocus = 1,

    /// Indicates that the device should automatically focus when needed.
    ContinuousAutoFocus = 2,
}

#[doc(alias = "AVCaptureAutoFocusSystem")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum AutoFocusSystem {
    /// Indicates that autofocus is not available.
    None = 0,

    /// Indicates that autofocus is achieved by contrast detection.
    /// Contrast detection performs a focus scan to find the optimal position.
    ContrastDetection = 1,

    /// Indicates that autofocus is achieved by phase detection.
    /// Phase detection has the ability to achieve focus in many cases without a focus scan.
    /// Phase detection autofocus is typically less visually intrusive than contrast
    // detection autofocus.
    PhaseDetection = 2,
}

#[doc(alias = "AVCaptureAutoFocusRangeRestriction")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum AutoFocusRangeRestriction {
    /// Indicates that the autofocus system should not restrict the focus range.
    #[doc(alias = "AVCaptureAutoFocusRangeRestrictionNone")]
    None = 0,

    /// Indicates that the autofocus system should restrict the focus range
    /// for subject matter that is near to the camera.
    #[doc(alias = "AVCaptureAutoFocusRangeRestrictionNear")]
    Near = 1,

    /// Indicates that the autofocus system should restrict the focus range
    /// for subject matter that is far from the camera.
    #[doc(alias = "AVCaptureAutoFocusRangeRestrictionFar")]
    Far = 2,
}

/// AVCaptureDeviceFocus
impl Device {
    #[objc::msg_send(isFocusModeSupported:)]
    pub fn is_focus_mode_supported(&self, mode: FocusMode) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isLockingFocusWithCustomLensPositionSupported)]
    pub fn is_locking_focus_with_custom_lens_position_supported(&self) -> bool;

    #[objc::msg_send(focusMode)]
    pub fn focus_mode(&self) -> FocusMode;

    #[objc::msg_send(setFocusMode:)]
    unsafe fn set_focus_mode_throws(&mut self, mode: FocusMode);

    #[objc::msg_send(isFocusPointOfInterestSupported)]
    pub fn is_focus_point_of_interest_supported(&self) -> bool;

    #[objc::msg_send(focusPointOfInterest)]
    pub fn focus_point_of_intereset(&self) -> cg::Point;

    #[objc::msg_send(setFocusPointOfInterest:)]
    unsafe fn set_focus_point_of_interest_throws(&mut self, val: cg::Point);

    /// The value of this property is a bool indicating whether the receiver's camera focus
    /// is being automatically adjusted by means of a focus scan, because its focus mode is 'av::CaptureFocusModeAutoFocus'
    /// or 'av::CaptureFocusModeContinuousAutoFocus'. Clients can observe the value of this property to determine whether
    /// the camera's focus is stable.
    #[objc::msg_send(isAdjustingFocus)]
    pub fn is_adjusting_focus(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isAutoFocusRangeRestrictionSupported)]
    pub fn is_auto_focus_range_restriction_supported(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(autoFocusRangeRestriction)]
    pub fn auto_focus_range_restriction(&self) -> AutoFocusRangeRestriction;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setAutoFocusRangeRestriction:)]
    unsafe fn set_auto_focus_range_restriction_throws(&mut self, val: AutoFocusRangeRestriction);

    /// Indicates whether the receiver supports smooth autofocus.
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isSmoothAutoFocusSupported)]
    pub fn is_smooth_auto_focus_supported(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isSmoothAutoFocusEnabled)]
    pub fn is_smooth_auto_focus_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setSmoothAutoFocusEnabled:)]
    unsafe fn set_smooth_auto_focus_enabled_throws(&mut self, val: bool);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(automaticallyAdjustsFaceDrivenAutoFocusEnabled)]
    pub fn automatically_adjusts_face_driven_auto_focus_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setAutomaticallyAdjustsFaceDrivenAutoFocusEnabled:)]
    unsafe fn set_automatically_adjusts_face_driven_auto_focus_enabled_throws(&mut self, val: bool);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isFaceDrivenAutoFocusEnabled)]
    pub fn is_face_driven_auto_focus_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setFaceDrivenAutoFocusEnabled:)]
    unsafe fn set_face_driven_auto_focus_enabled_throws(&mut self, val: bool);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(lensPosition)]
    pub fn lens_position(&self) -> f32;

    // #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setFocusModeLockedWithLensPosition:completionHandler:)]
    unsafe fn set_focus_mode_locked_with_lens_position_ch_throws(
        &mut self,
        val: f32,
        ch: *mut c_void,
    );
    /// A property indicating the minimum focus distance.
    ///
    /// The minimum focus distance is given in millimeters, -1 if unknown.
    /// For virtual cameras, the value reported is the smallest minimum focus distance of the auto-focus-capable
    /// cameras that it sources.
    #[objc::msg_send(minimumFocusDistance)]
    pub fn minimum_focus_distance(&self) -> ns::Integer;
}

#[doc(alias = "AVCaptureExposureMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ExposureMode {
    /// Indicates that the exposure should be locked at its current value.
    Locked = 0,
    /// Indicates that the device should automatically adjust exposure once and then change the exposure mode to
    /// av::CaptureExposureMode::Locked
    AutoExpose = 1,
    /// Indicates that the device should automatically adjust exposure when needed.
    ContinuousAutoExposure = 2,
    /// Indicates that the device should only adjust exposure according to user provided ISO, exposureDuration values.
    Custom = 3,
}

/// AVCaptureExposureMode
impl Device {
    /// Returns whether the receiver supports the given exposure mode.
    #[objc::msg_send(isExposureModeSupported:)]
    pub fn is_exposure_mode_supported(&self, val: ExposureMode) -> bool;

    #[objc::msg_send(exposureMode)]
    pub fn exposure_mode(&self) -> ExposureMode;

    #[objc::msg_send(setExposureMode:)]
    unsafe fn set_exposure_mode_throws(&mut self, val: ExposureMode);

    #[objc::msg_send(isExposurePointOfInterestSupported)]
    pub fn is_exposure_point_of_interest_supported(&self) -> bool;

    #[objc::msg_send(exposurePointOfInterest)]
    pub fn exposure_point_of_interest(&self) -> cg::Point;

    #[objc::msg_send(setExposurePointOfInterest:)]
    unsafe fn set_exposure_point_of_interest_throws(&mut self, val: cg::Point);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(automaticallyAdjustsFaceDrivenAutoExposureEnabled)]
    pub fn automatically_adjusts_face_driven_auto_exposure_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setAutomaticallyAdjustsFaceDrivenAutoExposureEnabled:)]
    unsafe fn set_automatically_adjusts_face_driven_auto_exposure_enabled_throws(
        &mut self,
        val: bool,
    );

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isFaceDrivenAutoExposureEnabled)]
    pub fn is_face_driven_auto_exposure_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setFaceDrivenAutoExposureEnabled:)]
    unsafe fn set_face_driven_auto_exposure_enabled_throws(&mut self, val: bool);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(activeMaxExposureDuration)]
    pub fn active_max_exposure_duration(&self) -> cm::Time;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setActiveMaxExposureDuration:)]
    unsafe fn set_active_max_exposure_duration_throws(&mut self, val: cm::Time);

    #[objc::msg_send(isAdjustingExposure)]
    pub fn is_adjusting_exposure(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(lensAperture)]
    pub fn lens_aperture(&self) -> f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(exposureDuration)]
    pub fn exposure_duration(&self) -> cm::Time;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(ISO)]
    pub fn iso(&self) -> cm::Time;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setExposureModeCustomWithDuration:ISO:completionHandler:)]
    pub fn set_exposure_mode_custom_with_duration_and_iso_throws(
        &mut self,
        duration: cm::Time,
        iso: f32,
        handler: *mut c_void,
    );

    /// Indicates the metered exposure level's offset from the target exposure value, in EV units.
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(exposureTargetOffset)]
    pub fn exposure_target_offset(&self) -> f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(exposureTargetBias)]
    pub fn exposure_target_bias(&self) -> f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(minExposureTargetBias)]
    pub fn min_exposure_target_bias(&self) -> f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(maxExposureTargetBias)]
    pub fn max_exposure_target_bias(&self) -> f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setExposureTargetBias:completionHandler:)]
    unsafe fn set_exposure_target_bias_throws(&mut self, bias: f32, handler: *mut c_void);
}

/// AVCaptureExposureMode
impl<'a> ConfigLockGuard<'a> {
    pub unsafe fn set_exposure_mode_throws(&mut self, val: ExposureMode) {
        self.device.set_exposure_mode_throws(val)
    }

    pub fn set_exposure_mode<'ar>(&mut self, val: ExposureMode) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_exposure_mode_throws(val) })
    }

    pub unsafe fn set_exposure_point_of_interest_throws(&mut self, val: cg::Point) {
        self.device.set_exposure_point_of_interest_throws(val)
    }

    pub fn set_exposure_point_of_interest<'ar>(
        &mut self,
        val: cg::Point,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_exposure_point_of_interest_throws(val) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_automatically_adjusts_face_driven_auto_exposure_enabled_throws(
        &mut self,
        val: bool,
    ) {
        self.device
            .set_automatically_adjusts_face_driven_auto_exposure_enabled_throws(val)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_automatically_adjusts_face_driven_auto_exposure_enabled<'ar>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.set_automatically_adjusts_face_driven_auto_exposure_enabled_throws(val)
        })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_face_driven_auto_exposure_enabled_throws(&mut self, val: bool) {
        self.device
            .set_face_driven_auto_exposure_enabled_throws(val)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_face_driven_auto_exposure_enabled<'ar>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_face_driven_auto_exposure_enabled_throws(val) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_active_max_exposure_duration_throws(&mut self, val: cm::Time) {
        self.device.set_active_max_exposure_duration_throws(val)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_active_max_exposure_duration<'ar>(
        &mut self,
        val: cm::Time,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_active_max_exposure_duration_throws(val) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_exposure_mode_custom_with_duration_and_iso_no_ch_throws(
        &mut self,
        duration: cm::Time,
        iso: f32,
    ) {
        self.device
            .set_exposure_mode_custom_with_duration_and_iso_throws(
                duration,
                iso,
                std::ptr::null_mut(),
            )
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_exposure_mode_custom_with_duration_and_iso_no_ch<'ar>(
        &mut self,
        duration: cm::Time,
        iso: f32,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.set_exposure_mode_custom_with_duration_and_iso_no_ch_throws(duration, iso)
        })
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_exposure_mode_custom_with_duration_and_iso_with_ch_throws<F>(
        &mut self,
        duration: cm::Time,
        iso: f32,
        block: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(cm::Time),
    {
        self.device
            .set_exposure_mode_custom_with_duration_and_iso_throws(
                duration,
                iso,
                block.as_mut_ptr(),
            )
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_exposure_mode_custom_with_duration_and_iso_with_ch<'ar, F>(
        &mut self,
        duration: cm::Time,
        iso: f32,
        block: &'static mut blocks::Block<F>,
    ) -> Result<(), &'ar ns::Exception>
    where
        F: FnOnce(cm::Time),
    {
        ns::try_catch(|| unsafe {
            self.device
                .set_exposure_mode_custom_with_duration_and_iso_throws(
                    duration,
                    iso,
                    block.as_mut_ptr(),
                )
        })
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async unsafe fn set_exposure_mode_custom_with_duration_and_iso_throws(
        &mut self,
        duration: cm::Time,
        iso: f32,
    ) -> cm::Time {
        let (future, block) = blocks::comp1();
        self.set_exposure_mode_custom_with_duration_and_iso_with_ch_throws(
            duration,
            iso,
            block.escape(),
        );
        future.await
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async fn set_exposure_mode_custom_with_duration_and_iso(
        &mut self,
        duration: cm::Time,
        iso: f32,
    ) -> Result<cm::Time, arc::R<ns::Exception>> {
        let (future, block) = blocks::comp1();
        let res = ns::try_catch(move || unsafe {
            self.set_exposure_mode_custom_with_duration_and_iso_with_ch_throws(
                duration,
                iso,
                block.escape(),
            )
        });
        if let Err(err) = res {
            return Err(err.retained());
        }
        Ok(future.await)
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_exposure_target_bias_with_ch_throws<F>(
        &mut self,
        bias: f32,
        block: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(cm::Time),
    {
        self.device
            .set_exposure_target_bias_throws(bias, block.as_mut_ptr())
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_exposure_target_bias_no_ch_throws(&mut self, bias: f32) {
        self.device
            .set_exposure_target_bias_throws(bias, std::ptr::null_mut())
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_exposure_target_bias_with_ch<'ar, F>(
        &mut self,
        bias: f32,
        block: &'static mut blocks::Block<F>,
    ) -> Result<(), &'ar ns::Exception>
    where
        F: FnOnce(cm::Time),
    {
        ns::try_catch(|| unsafe {
            self.device
                .set_exposure_target_bias_throws(bias, block.as_mut_ptr())
        })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_exposure_target_bias_no_ch<'ar>(
        &mut self,
        bias: f32,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.device
                .set_exposure_target_bias_throws(bias, std::ptr::null_mut())
        })
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async unsafe fn set_exposure_target_bias_throws(&mut self, bias: f32) -> cm::Time {
        let (future, block) = blocks::comp1();
        self.set_exposure_target_bias_with_ch_throws(bias, block.escape());
        future.await
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async fn set_exposure_target_bias(
        &mut self,
        bias: f32,
    ) -> Result<cm::Time, arc::R<ns::Exception>> {
        let (future, block) = blocks::comp1();
        let res = ns::try_catch(move || unsafe {
            self.set_exposure_target_bias_with_ch_throws(bias, block.escape())
        });
        if let Err(err) = res {
            return Err(err.retained());
        }
        Ok(future.await)
    }
}

/// AVCaptureDeviceToneMapping
impl Device {
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isGlobalToneMappingEnabled)]
    pub fn is_global_tone_mapping_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setGlobalToneMappingEnabled:)]
    pub fn set_global_tone_mapping_enabled_throws(&mut self, val: bool);
}

/// AVCaptureDeviceToneMapping
impl<'a> ConfigLockGuard<'a> {
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_global_tone_mapping_enabled_throws(&mut self, val: bool) {
        self.device.set_global_tone_mapping_enabled_throws(val)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_global_tone_mapping_enabled<'ar>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_global_tone_mapping_enabled_throws(val) })
    }
}

/// AVCaptureWhiteBalanceMode
#[doc(alias = "AVCaptureWhiteBalanceMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum WbMode {
    /// Indicates that the white balance should be locked at its current value.
    Locked = 0,
    /// Indicates that the device should automatically adjust white balance
    /// once and then change the white balance mode to 'Locked'
    Auto = 1,
    /// Indicates that the device should automatically adjust white balance when needed.
    ContinuousAuto = 2,
}

#[doc(alias = "AVCaptureWhiteBalanceGains")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct WbGains {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[doc(alias = "AVCaptureWhiteBalanceChromaticityValues")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct WbChromaticityValues {
    pub x: f32,
    pub y: f32,
}

#[doc(alias = "AVCaptureWhiteBalanceTemperatureAndTintValues")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct WbTempTintValues {
    pub temp: f32,
    pub tint: f32,
}

/// AVCaptureDeviceWhiteBalance
impl Device {
    #[objc::msg_send(isWhiteBalanceModeSupported:)]
    pub fn is_wb_mode_supported(&self, mode: WbMode) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isLockingWhiteBalanceWithCustomDeviceGainsSupported)]
    pub fn is_locking_wb_with_custom_device_gains_supported(&self) -> bool;

    #[objc::msg_send(whiteBalanceMode)]
    pub fn wb_mode(&self) -> WbMode;

    #[objc::msg_send(setWhiteBalanceMode:)]
    unsafe fn set_wb_mode_throws(&mut self, val: WbMode);

    #[objc::msg_send(isAdjustingWhiteBalance)]
    pub fn is_adjusting_wb(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(deviceWhiteBalanceGains)]
    pub fn device_wb_gains(&self) -> WbGains;

    /// Indicates the current device-specific Gray World RGB white balance gain values in use.
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(grayWorldDeviceWhiteBalanceGains)]
    pub fn gray_world_device_wb_gains(&self) -> WbGains;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(maxWhiteBalanceGain)]
    pub fn max_wb_gain(&self) -> f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setWhiteBalanceModeLockedWithDeviceWhiteBalanceGains:completionHandler:)]
    unsafe fn set_wb_mode_locked_with_device_wb_gains_throws(
        &mut self,
        gains: WbGains,
        block: *mut c_void,
    );

    /// Converts device-independent chromaticity values to device-specific white balance RGB gain values.
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(deviceWhiteBalanceGainsForChromaticityValues:)]
    pub fn device_wb_gains_for_chromaticity_values(&self, values: WbChromaticityValues) -> WbGains;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(temperatureAndTintValuesForDeviceWhiteBalanceGains:)]
    pub fn temp_tint_values_for_device_wb_gains(&self, gains: WbGains) -> WbTempTintValues;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(deviceWhiteBalanceGainsForTemperatureAndTintValues:)]
    pub fn device_wb_gains_for_temp_tint_values(&self, values: WbTempTintValues) -> WbGains;
}

/// AVCaptureDeviceWhiteBalance
impl<'a> ConfigLockGuard<'a> {
    #[inline]
    pub unsafe fn set_wb_mode_throws(&mut self, val: WbMode) {
        self.device.set_wb_mode_throws(val)
    }

    pub fn set_wb_mode<'ar>(&mut self, val: WbMode) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_wb_mode_throws(val) })
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_wb_mode_locked_with_device_wb_gains_with_ch_throws<F>(
        &mut self,
        gains: WbGains,
        block: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(cm::Time),
    {
        self.device
            .set_wb_mode_locked_with_device_wb_gains_throws(gains, block.as_mut_ptr())
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_wb_mode_locked_with_device_wb_gains_no_ch_throws(&mut self, gains: WbGains) {
        self.device
            .set_wb_mode_locked_with_device_wb_gains_throws(gains, std::ptr::null_mut())
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_wb_mode_locked_with_device_wb_gains_with_ch<'ar, F>(
        &mut self,
        gains: WbGains,
        block: &'static mut blocks::Block<F>,
    ) -> Result<(), &'ar ns::Exception>
    where
        F: FnOnce(cm::Time),
    {
        ns::try_catch(|| unsafe {
            self.set_wb_mode_locked_with_device_wb_gains_with_ch_throws(gains, block)
        })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_wb_mode_locked_with_device_wb_gains_no_ch<'ar>(
        &mut self,
        gains: WbGains,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.set_wb_mode_locked_with_device_wb_gains_no_ch_throws(gains)
        })
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async unsafe fn set_wb_mode_locked_with_device_wb_gains_throws(
        &mut self,
        gains: WbGains,
    ) -> cm::Time {
        let (future, block) = blocks::comp1();
        self.set_wb_mode_locked_with_device_wb_gains_with_ch_throws(gains, block.escape());
        future.await
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async fn set_wb_mode_locked_with_device_wb_gains(
        &mut self,
        gains: WbGains,
    ) -> Result<cm::Time, arc::R<ns::Exception>> {
        let (future, block) = blocks::comp1();
        let res = ns::try_catch(move || unsafe {
            self.set_wb_mode_locked_with_device_wb_gains_with_ch_throws(gains, block.escape())
        });
        if let Err(err) = res {
            return Err(err.retained());
        }
        Ok(future.await)
    }
}

/// AVCaptureDeviceSubjectAreaChangeMonitoring
#[cfg(any(target_os = "tvos", target_os = "ios"))]
impl Device {
    #[objc::msg_send(isSubjectAreaChangeMonitoringEnabled)]
    pub fn is_subject_area_change_monitoring_enabled(&self) -> bool;

    #[objc::msg_send(setSubjectAreaChangeMonitoringEnabled:)]
    unsafe fn set_subject_area_change_monitoring_enabled_throws(&mut self, val: bool);
}

/// AVCaptureDeviceSubjectAreaChangeMonitoring
#[cfg(any(target_os = "tvos", target_os = "ios"))]
impl<'a> ConfigLockGuard<'a> {
    pub fn set_subject_area_change_monitoring_enabled(&mut self, val: bool) {
        unsafe {
            self.device
                .set_subject_area_change_monitoring_enabled_throws(val)
        }
    }
}

/// AVCaptureDeviceLowLightBoost
#[cfg(any(target_os = "tvos", target_os = "ios"))]
impl Device {
    #[objc::msg_send(isLowLightBoostSupported)]
    pub fn is_low_light_boost_supported(&self) -> bool;

    #[objc::msg_send(isLowLightBoostEnabled)]
    pub fn is_low_light_boost_enabled(&self) -> bool;

    #[objc::msg_send(setLowLightBoostEnabled:)]
    unsafe fn set_low_light_boost_enabled_throws(&mut self, val: bool);

    #[objc::msg_send(automaticallyEnablesLowLightBoostWhenAvailable)]
    pub fn automatically_enables_low_light_boost_when_available(&self) -> bool;

    #[objc::msg_send(setAutomaticallyEnablesLowLightBoostWhenAvailable:)]
    unsafe fn set_automatically_enables_low_light_boost_when_available_throws(&mut self, val: bool);
}

/// AVCaptureDeviceLowLightBoost
#[cfg(any(target_os = "tvos", target_os = "ios"))]
impl<'a> ConfigLockGuard<'a> {
    pub unsafe fn set_low_light_boost_enabled_throws(&mut self, val: bool) {
        self.device.set_low_light_boost_enabled_throws(val)
    }

    pub fn set_low_light_boost_enabled<'ar>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_low_light_boost_enabled_throws(val) })
    }

    pub unsafe fn set_automatically_enables_low_light_boost_when_available_throws(
        &mut self,
        val: bool,
    ) {
        self.device
            .set_automatically_enables_low_light_boost_when_available_throws(val)
    }

    pub fn set_automatically_enables_low_light_boost_when_available<'ar>(
        &mut self,
        val: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.set_automatically_enables_low_light_boost_when_available_throws(val)
        })
    }
}

/// AVCaptureDeviceVideoZoom
impl Device {
    #[objc::msg_send(videoZoomFactor)]
    pub fn video_zoom_factor(&self) -> cg::Float;

    #[objc::msg_send(setVideoZoomFactor:)]
    unsafe fn set_video_zoom_factor_throws(&mut self, val: cg::Float);

    #[objc::msg_send(isRampingVideoZoom)]
    pub fn is_ramping_video_zoom(&self) -> bool;

    #[objc::msg_send(rampToVideoZoomFactor:withRate:)]
    pub unsafe fn ramp_to_video_zoom_factor_throws(&mut self, factor: cg::Float, rate: f32);

    #[objc::msg_send(cancelVideoZoomRamp)]
    pub unsafe fn cancel_video_zoom_ramp_throws(&mut self);

    #[objc::msg_send(displayVideoZoomFactorMultiplier)]
    pub fn display_video_zoom_factor_multiplier(&self) -> cg::Float;
}

/// AVCaptureDeviceVideoZoom
impl<'a> ConfigLockGuard<'a> {
    pub fn set_video_zoom_factor(&mut self, val: cg::Float) {
        unsafe { self.device.set_video_zoom_factor_throws(val) }
    }

    pub fn ramp_to_video_zoom_factor(&mut self, factor: cg::Float, rate: f32) {
        unsafe { self.device.ramp_to_video_zoom_factor_throws(factor, rate) }
    }

    pub fn cancel_video_zoom_ramp(&mut self) {
        unsafe { self.device.cancel_video_zoom_ramp_throws() }
    }
}

define_obj_type!(Type(ns::String));

/// ```
/// use cidre::av;
///
/// let device_type = av::CaptureDeviceType::external();
/// let device_type = av::CaptureDeviceType::built_in_microphone();
/// let device_type = av::CaptureDeviceType::built_in_wide_angle_camera();
/// let device_type = av::CaptureDeviceType::built_in_telephoto_camera();
/// let device_type = av::CaptureDeviceType::built_in_ultra_wide_camera();
/// let device_type = av::CaptureDeviceType::built_in_dual_camera();
/// let device_type = av::CaptureDeviceType::built_in_dual_wide_camera();
/// let device_type = av::CaptureDeviceType::built_in_tripple_camera();
/// let device_type = av::CaptureDeviceType::built_in_true_depth_camera();
/// let device_type = av::CaptureDeviceType::built_in_lidar_depth_camera();
/// ```
impl Type {
    /// An external device type. On iPad, external devices are those that conform
    /// to the UVC (USB Video Class) specification.
    ///
    /// Starting in Mac Catalyst 17.0, apps may opt in for using
    /// 'av::CaptureDeviceType::external()' by adding the following key
    /// to their Info.plist:
    /// ```xml
    ///  <key>NSCameraUseExternalDeviceType</key>
    ///  <true/>
    /// ```
    /// Otherwise, external cameras on Mac Catalyst report that their device type is
    /// 'av::CaptureDeviceType::built_in_wide_angle_camera()'.
    #[doc(alias = "AVCaptureDeviceTypeExternal")]
    pub fn external() -> &'static Self {
        unsafe { AVCaptureDeviceTypeExternal }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInMicrophone")]
    pub fn built_in_microphone() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInMicrophone }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInWideAngleCamera")]
    pub fn built_in_wide_angle_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInWideAngleCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInTelephotoCamera")]
    pub fn built_in_telephoto_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTelephotoCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInUltraWideCamera")]
    pub fn built_in_ultra_wide_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInUltraWideCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInDualCamera")]
    pub fn built_in_dual_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInDualCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInDualWideCamera")]
    pub fn built_in_dual_wide_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInDualWideCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInTripleCamera")]
    pub fn built_in_tripple_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTripleCamera }
    }

    /// A device that consists of two cameras, one YUV and one Infrared.
    /// The infrared camera provides high quality depth information that is synchronized
    /// and perspective corrected to frames produced by the YUV camera. While the resolution
    /// of the depth data and YUV frames may differ, their field of view and aspect ratio
    /// always match. Note that devices of this type may only be discovered using an
    /// `av::CaptureDevice::default_device_with_device_type_media_type_position`.
    #[doc(alias = "AVCaptureDeviceTypeBuiltInTrueDepthCamera")]
    pub fn built_in_true_depth_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTrueDepthCamera }
    }
    /// A device that consists of two cameras, one YUV and one LiDAR.
    /// The LiDAR camera provides high quality, high accuracy depth information by measuring
    /// the round trip of an artificial light signal emitted by a laser. The depth
    /// is synchronized and perspective corrected to frames produced by the paired YUV camera.
    /// While the resolution of the depth data and YUV frames may differ, their field of view
    /// and aspect ratio always match. Note that devices of this type may only be discovered
    /// using an av::CaptureDeviceDiscoverySession or
    /// `av::CaptureDevice::default_device_with_device_type_media_type_position`.
    #[doc(alias = "AVCaptureDeviceTypeBuiltInLiDARDepthCamera")]
    pub fn built_in_lidar_depth_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInLiDARDepthCamera }
    }

    /// A continuity camera device. These devices are suitable for general purpose use.
    /// Note that devices of this type may only be discovered using an
    /// av::CaptureDeviceDiscoverySession or
    ///`av::CaptureDevice::default_device_with_device_type_media_type_position`.
    ///
    /// Starting in macOS 14.0 and Mac Catalyst 17.0, apps may opt in for using
    /// 'av::CaptureDeviceType::continuity_camera()' by adding the following key
    /// to their Info.plist:
    /// ```xml
    /// <key>NSCameraUseContinuityCameraDeviceType</key>
    /// <true/>
    /// ```
    #[doc(alias = "AVCaptureDeviceTypeContinuityCamera")]
    pub fn continuity_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeContinuityCamera }
    }

    /// A distortion corrected cut out from an ultra wide camera, made to approximate
    /// an overhead camera pointing at a desk.
    /// Supports multicam operation.
    #[doc(alias = "AVCaptureDeviceTypeDeskViewCamera")]
    #[cfg(target_os = "macos")]
    pub fn desk_view_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeDeskViewCamera }
    }
}

/// AVCaptureDeviceType
impl Device {
    #[objc::msg_send(deviceType)]
    pub fn device_type(&self) -> &Type;
}

/// AVCaptureDevicePreferredCamera
impl Device {
    /// Settable property that specifies a user preferred camera.
    #[objc::cls_msg_send(userPreferredCamera)]
    fn user_preferred_camera<'a>() -> Option<&'a Self>;

    #[objc::cls_msg_send(setUserPreferredCamera:)]
    fn set_user_preferred_camera(val: Option<&Self>);

    /// Specifies the best camera to use as determined by the system.
    #[objc::cls_msg_send(systemPreferredCamera)]
    fn system_preferred_camera<'a>() -> Option<&'a Self>;
}

#[doc(alias = "AVAuthorizationStatus")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AuthorizationStatus {
    /// Indicates that the user has not yet made a choice regarding whether the client can access the hardware.
    NotDetermined = 0,
    /// The client is not authorized to access the hardware for the media type.
    /// The user cannot change the client's status, possibly due to active restrictions
    /// such as parental controls being in place.
    Restricted = 1,
    /// The user explicitly denied access to the hardware supporting a media type for the client.
    Denied = 2,
    /// The client is authorized to access the hardware supporting a media type.
    Authorized = 3,
}

/// AVCaptureDeviceAuthorization
impl Device {
    #[objc::cls_msg_send(authorizationStatusForMediaType:)]
    pub unsafe fn authorization_status_for_media_type_throws(
        media_type: &av::MediaType,
    ) -> AuthorizationStatus;

    pub fn authorization_status_for_media_type<'ar>(
        media_type: &av::MediaType,
    ) -> Result<AuthorizationStatus, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { Self::authorization_status_for_media_type_throws(media_type) })
    }

    #[cfg(feature = "blocks")]
    #[objc::cls_msg_send(requestAccessForMediaType:completionHandler:)]
    unsafe fn _request_access_for_media_type_ch_throws(
        media_type: &av::MediaType,
        block: *mut c_void,
    );

    #[cfg(feature = "blocks")]
    pub unsafe fn request_access_for_media_type_ch_throws<F>(
        media_type: &av::MediaType,
        block: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(bool),
    {
        unsafe { Self::_request_access_for_media_type_ch_throws(media_type, block.as_mut_ptr()) }
    }

    #[cfg(feature = "blocks")]
    pub fn request_access_for_media_type_ch<'ar, F>(
        media_type: &av::MediaType,
        block: &'static mut blocks::Block<F>,
    ) -> Result<(), &'ar ns::Exception>
    where
        F: FnOnce(bool),
    {
        ns::try_catch(|| unsafe {
            Self::request_access_for_media_type_ch_throws(media_type, block)
        })
    }

    #[cfg(feature = "async")]
    pub async fn request_access_for_media_type<'ar>(
        media_type: &av::MediaType,
    ) -> Result<bool, arc::R<ns::Exception>> {
        let (future, block) = blocks::comp1();
        match Self::request_access_for_media_type_ch(media_type, block.escape()) {
            Ok(_) => Ok(future.await),
            Err(e) => Err(e.retained()),
        }
    }
}

define_obj_type!(FrameRateRange(ns::Id));

impl FrameRateRange {
    #[objc::msg_send(minFrameRate)]
    pub fn min_frame_rate(&self) -> f64;

    #[objc::msg_send(maxFrameRate)]
    pub fn max_frame_rate(&self) -> f64;

    #[objc::msg_send(maxFrameDuration)]
    pub fn max_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(minFrameDuration)]
    pub fn min_frame_duration(&self) -> cm::Time;
}

define_obj_type!(Format(ns::Id));
impl Format {
    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> &av::MediaType;

    #[objc::msg_send(formatDescription)]
    pub fn format_desc(&self) -> &cm::FormatDesc;

    #[objc::msg_send(videoSupportedFrameRateRanges)]
    pub fn video_supported_frame_rate_ranges(&self) -> &ns::Array<FrameRateRange>;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoFieldOfView)]
    pub fn video_fov(&self) -> f32;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isVideoBinned)]
    pub fn is_video_binned(&self) -> bool;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isVideoStabilizationModeSupported:)]
    pub fn is_video_stabilization_mode_supported(&self, mode: VideoStabilizationMode) -> bool;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoMaxZoomFactor)]
    pub fn video_max_zoom_factor(&self) -> cg::Float;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoZoomFactorUpscaleThreshold)]
    pub fn video_zoom_factor_upscale_threshold(&self) -> cg::Float;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(minExposureDuration)]
    pub fn min_exposure_duration(&self) -> cm::Time;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(maxExposureDuration)]
    pub fn max_exposure_duration(&self) -> cm::Time;

    /// A 'f32' indicating the minimum supported exposure ISO value.
    ///
    /// This read-only property indicates the minimum supported exposure ISO value.
    #[objc::msg_send(minISO)]
    pub fn min_iso(&self) -> f32;

    /// An 'f32' indicating the maximum supported exposure ISO value.
    ///
    /// This read-only property indicates the maximum supported exposure ISO value.
    #[objc::msg_send(maxISO)]
    pub fn max_iso(&self) -> f32;

    #[objc::msg_send(autoFocusSystem)]
    pub fn auto_focus_system(&self) -> AutoFocusSystem;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isMultiCamSupported)]
    pub fn is_mutli_cam_supported(&self) -> bool;

    #[objc::msg_send(geometricDistortionCorrectedVideoFieldOfView)]
    pub fn geometric_distortion_corrected_video_field_of_view(&self) -> f32;
}

/// Center Stage
impl Format {
    #[objc::msg_send(isCenterStageSupported)]
    pub fn is_center_stage_supported(&self) -> bool;

    #[objc::msg_send(videoMinZoomFactorForCenterStage)]
    pub fn video_min_zoom_factor_for_center_stage(&self) -> cg::Float;

    #[objc::msg_send(videoFrameRateRangeForCenterStage)]
    pub fn video_frame_rate_range_for_center_stage(&self) -> Option<&FrameRateRange>;
}

/// Portrait Effect
impl Format {
    /// Indicates whether the format supports the Portrait Effect feature.
    #[objc::msg_send(isPortraitEffectSupported)]
    pub fn is_portrait_effect_supported(&self) -> bool;
}

pub mod notifications {
    use crate::ns;

    /// Posted when a device becomes available on the system.
    pub fn was_connected() -> &'static ns::NotificationName {
        unsafe { AVCaptureDeviceWasConnectedNotification }
    }

    /// Posted when a device becomes unavailable on the system.
    pub fn was_disconnected() -> &'static ns::NotificationName {
        unsafe { AVCaptureDeviceWasDisconnectedNotification }
    }

    /// Posted when the instance of av::CaptureDevice has detected a substantial
    /// change to the video subject area.
    #[cfg(not(target_os = "macos"))]
    pub fn subject_area_did_change() -> &'static ns::NotificationName {
        unsafe { AVCaptureDeviceSubjectAreaDidChangeNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureDeviceWasConnectedNotification: &'static ns::NotificationName;
        static AVCaptureDeviceWasDisconnectedNotification: &'static ns::NotificationName;
        #[cfg(not(target_os = "macos"))]
        static AVCaptureDeviceSubjectAreaDidChangeNotification: &'static ns::NotificationName;
    }
}

define_obj_type!(CaptureAudioChannel(ns::Id));

define_obj_type!(DiscoverySession(ns::Id));
impl DiscoverySession {
    define_cls!(AV_CAPTURE_DEVICE_DISCOVERY_SESSION);

    #[objc::cls_msg_send(discoverySessionWithDeviceTypes:mediaType:position:)]
    pub fn with_device_types_media_and_position_ar(
        device_types: &ns::Array<Type>,
        media_type: Option<&av::MediaType>,
        position: Position,
    ) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_device_types_media_and_position(
        device_types: &ns::Array<Type>,
        media_type: Option<&av::MediaType>,
        position: Position,
    ) -> arc::R<Self>;

    /// The list of devices that comply to the search criteria specified
    /// on the discovery session.
    ///
    /// The returned array contains only devices that are available at the time the method
    /// is called. Applications can key-value observe this property to be notified when
    /// the list of available devices has changed. For apps linked against iOS 10,
    /// the devices returned are unsorted. For apps linked against iOS 11 or later,
    /// the devices are sorted by 'av::CaptureDeviceType', matching the order specified
    /// in the deviceTypes parameter of 'av::CaptureDeviceDiscoverySession::with_device_types_media_position`.
    /// If a position of 'av::CaptureDevicePosition::unspecified' is specified,
    /// the results are further ordered by position in the 'av::CaptureDevicePosition' enum.
    /// Starting in Mac Catalyst 14.0, clients can key value observe the value of this
    /// property to be notified when the devices change.
    #[objc::msg_send(devices)]
    pub fn devices(&self) -> &ns::Array<Device>;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(supportedMultiCamDeviceSets)]
    pub fn supported_multi_cam_device_sets(&self) -> &ns::Array<ns::Set<Device>>;
}

impl ns::KVObserverRegistration for DiscoverySession {}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_DEVICE_DISCOVERY_SESSION: &'static objc::Class<DiscoverySession>;
}

#[doc(alias = "AVCaptureVideoStabilizationMode")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum VideoStabilizationMode {
    /// Indicates that video should not be stabilized.
    Off = 0,

    /// Indicates that video should be stabilized using the standard
    /// video stabilization algorithm introduced with iOS 5.0. Standard video
    /// stabilization has a reduced field of view. Enabling video stabilization
    /// may introduce additional latency into the video capture pipeline.
    Standard = 1,

    /// Indicates that video should be stabilized using the cinematic stabilization
    /// algorithm for more dramatic results. Cinematic video stabilization has a reduced
    /// field of view compared to standard video stabilization. Enabling cinematic
    /// video stabilization introduces much more latency into the video capture pipeline
    /// than standard video stabilization and consumes significantly more system memory.
    /// Use narrow or identical min and max frame durations in conjunction with this mode.
    Cinematic = 2,

    /// Indicates that the video should be stabilized using the extended cinematic
    /// stabilization algorithm. Enabling extended cinematic stabilization introduces
    /// longer latency into the video capture pipeline compared to the
    /// 'av::CaptureVideoStabilizationMode::Cinematic' and consumes more memory, but yields
    /// improved stability. It is recommended to use identical or similar min and max frame
    /// durations in conjunction with this mode.
    CinematicExtended = 3,

    /// Indicates that video should be stabilized using the preview optimized stabilization
    /// algorithm. Preview stabilization is a low latency and low power algorithm
    /// which is supported only on connections which either have an associated preview layer
    /// or have a preview-sized VideoDataOutput.
    PreviewOptimized = 4,

    /// Indicates that the most appropriate video stabilization mode for the device and
    /// format should be chosen.
    Auto = -1,
}

#[cfg(any(target_os = "tvos", target_os = "ios"))]
#[doc(alias = "AVCaptureLensPositionCurrent")]
pub fn lens_position_current() -> f32 {
    unsafe { AVCaptureLensPositionCurrent }
}

#[cfg(any(target_os = "tvos", target_os = "ios"))]
#[doc(alias = "AVCaptureISOCurrent")]
pub fn iso_current() -> f32 {
    unsafe { AVCaptureISOCurrent }
}

#[cfg(any(target_os = "tvos", target_os = "ios"))]
#[doc(alias = "AVCaptureExposureDurationCurrent")]
pub fn exposure_duration_current() -> cm::Time {
    unsafe { AVCaptureExposureDurationCurrent }
}

#[cfg(any(target_os = "tvos", target_os = "ios"))]
#[doc(alias = "AVCaptureExposureTargetBiasCurrent")]
pub fn exposure_target_bias_current() -> f32 {
    unsafe { AVCaptureExposureTargetBiasCurrent }
}

#[cfg(any(target_os = "tvos", target_os = "ios"))]
#[doc(alias = "AVCaptureWhiteBalanceGainsCurrent")]
pub fn wb_gains_current() -> WbGains {
    unsafe { AVCaptureWhiteBalanceGainsCurrent }
}

#[doc(alias = "AVCaptureMaxAvailableTorchLevel")]
pub fn torch_max_level() -> f32 {
    unsafe { AVCaptureMaxAvailableTorchLevel }
}

#[cfg(test)]
mod tests {
    use crate::{
        av::{
            self, capture,
            capture::device::{self, Device},
        },
        cm::io,
        ns,
    };

    #[test]
    fn basics() {
        let device_type = device::Type::built_in_wide_angle_camera();
        let media_type = av::MediaType::video();
        let position = device::Position::Front;
        let mut device =
            Device::with_device_type_media_and_position(device_type, Some(media_type), position)
                .expect("device");
        //device.unique_id().show();
        assert!(!device.has_torch());
        assert!(device.formats().len() > 0);
        assert!(device.supports_preset(av::CaptureSessionPreset::photo()));
        let mut _lock = device.config_lock().expect("locked");
    }

    #[test]
    fn session() {
        io::Object::SYSTEM
            .allow_screen_capture_devices(true)
            .unwrap();
        io::Object::SYSTEM
            .allow_wireless_screen_capture_devices(true)
            .unwrap();

        io::Object::SYSTEM.show();

        let list = ns::Array::from_slice(&[av::CaptureDeviceType::external()]);
        let session = capture::DiscoverySession::with_device_types_media_and_position(
            list.as_ref(),
            Some(av::MediaType::muxed()),
            capture::DevicePosition::Unspecified,
        );

        let devices = session.devices();
        devices.as_type_ref().show();
    }

    #[tokio::test]
    async fn access() {
        let res = av::CaptureDevice::request_access_for_media_type(av::MediaType::video()).await;
        assert!(matches!(res, Ok(true)));
        let res = av::CaptureDevice::request_access_for_media_type(av::MediaType::audio()).await;
        assert!(matches!(res, Ok(true)));
        let res = av::CaptureDevice::request_access_for_media_type(av::MediaType::text()).await;
        assert!(res.is_err());

        let status = av::CaptureDevice::authorization_status_for_media_type(av::MediaType::video())
            .expect("Failed on valid media type");
        assert_eq!(status, av::AuthorizationStatus::Authorized);

        let res = av::CaptureDevice::authorization_status_for_media_type(av::MediaType::text());
        assert!(res.is_err());
    }
}
