pub mod content_sharing_picker;
pub use content_sharing_picker::Cfg as ContentSharingPickerCfg;
pub use content_sharing_picker::Mode as ContentSharingPickerMode;
pub use content_sharing_picker::Observer as ContentSharingPickerObserver;
pub use content_sharing_picker::ObserverImpl as ContentSharingPickerObserverImpl;
pub use content_sharing_picker::Picker as ContentSharingPicker;

pub mod stream;

pub use stream::CaptureDynamicRange;
pub use stream::CaptureResolution;
pub use stream::Cfg as StreamCfg;
pub use stream::CfgPreset as StreamCfgPreset;
pub use stream::ContentFilter;
pub use stream::Delegate as StreamDelegate;
pub use stream::DelegateImpl as StreamDelegateImpl;
pub use stream::FrameInfo;
pub use stream::FrameStatus;
pub use stream::Output as StreamOutput;
pub use stream::OutputImpl as StreamOutputImpl;
pub use stream::OutputType;
pub use stream::PresenterOverlayAlertSetting;
pub use stream::Stream;

#[cfg(any(target_os = "macos", target_abi = "macabi"))]
mod screenshot_manager;
#[cfg(any(target_os = "macos", target_abi = "macabi"))]
pub use screenshot_manager::DisplayIntent as ScreenshotDisplayIntent;
#[cfg(any(target_os = "macos", target_abi = "macabi"))]
pub use screenshot_manager::DynamicRange as ScreenshotDynamicRange;
#[cfg(any(target_os = "macos", target_abi = "macabi"))]
pub use screenshot_manager::ScreenshotCfg;
#[cfg(any(target_os = "macos", target_abi = "macabi"))]
pub use screenshot_manager::ScreenshotManager;
#[cfg(any(target_os = "macos", target_abi = "macabi"))]
pub use screenshot_manager::ScreenshotOutput;

pub mod shareable_content;
pub use shareable_content::Display;
pub use shareable_content::RunningApp;
pub use shareable_content::ShareableContent;
pub use shareable_content::Style as ShareableContentStyle;
pub use shareable_content::Window;

pub mod recording_output;
pub use recording_output::Delegate as RecordingOutputDelegate;
pub use recording_output::DelegateImpl as RecordingOutputDelegateImpl;
pub use recording_output::RecordingOutput;
pub use recording_output::RecordingOutputCfg;

pub mod clip_buffering_output;
pub use clip_buffering_output::ClipBufferingOutput;
pub use clip_buffering_output::Delegate as ClipBufferingOutputDelegate;
pub use clip_buffering_output::DelegateImpl as ClipBufferingOutputDelegateImpl;

pub mod recording_editor;
pub use recording_editor::Delegate as RecordingEditorDelegate;
pub use recording_editor::DelegateImpl as RecordingEditorDelegateImpl;
#[cfg(target_os = "tvos")]
pub use recording_editor::Mode as RecordingEditorMode;
pub use recording_editor::RecordingEditor;

#[cfg(all(target_os = "ios", not(target_abi = "macabi")))]
pub mod video_effect_output;
#[cfg(all(target_os = "ios", not(target_abi = "macabi")))]
pub use video_effect_output::VideoEffectOutput;

pub mod error;

#[cfg(any(
    target_os = "macos",
    all(target_os = "ios", not(target_env = "sim"), feature = "ios_27_0"),
    all(target_os = "tvos", not(target_env = "sim"), feature = "tvos_27_0"),
    all(
        target_os = "visionos",
        not(target_env = "sim"),
        feature = "visionos_27_0"
    )
))]
#[link(name = "ScreenCaptureKit", kind = "framework")]
unsafe extern "C" {}

#[link(name = "sc", kind = "static")]
unsafe extern "C" {}
