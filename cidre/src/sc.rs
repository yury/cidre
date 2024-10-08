pub mod content_sharing_picker;
pub use content_sharing_picker::Cfg as ContentSharingPickerCfg;
pub use content_sharing_picker::Mode as ContentSharingPickerMode;
pub use content_sharing_picker::Observer as ContentSharingPickerObserver;
pub use content_sharing_picker::ObserverImpl as ContentSharingPickerObserverImpl;
pub use content_sharing_picker::Picker as ContentSharingPicker;

pub mod stream;

pub use stream::CaptureResolution;
pub use stream::Cfg as StreamCfg;
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

mod screenshot_manager;
pub use screenshot_manager::ScreenshotManager;

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

pub mod error;
