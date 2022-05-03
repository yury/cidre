pub mod media_format;
pub use media_format::FileType;
pub use media_format::MediaType;
pub use media_format::VideoRange;

pub mod capture;
pub use capture::Connection as CaptureConnection;
pub use capture::DataDroppedReason as CaptureDataDroppedReason;
pub use capture::Device as CaptureDevice;
pub use capture::DeviceFormat as CaptureDeviceFormat;
pub use capture::DeviceType as CaptureDeviceType;
pub use capture::Input as CaptureInput;
pub use capture::InputPort as CaptureInputPort;
pub use capture::InterruptionReason as CaptureInterruptionReason;
pub use capture::Output as CaptureOutput;
pub use capture::Session as CaptureSession;
pub use capture::SessionPreset as CaptureSessionPreset;
pub use capture::VideoOrienation as CaptureVideoOrienation;

#[cfg(not(target_os = "macos"))]
pub use capture::SystemPressureFactors as CaptureSystemPressureFactors;
#[cfg(not(target_os = "macos"))]
pub use capture::SystemPressureLevel as CaptureSystemPressureLevel;
#[cfg(not(target_os = "macos"))]
pub use capture::SystemPressureState as CaptureSystemPressureState;
