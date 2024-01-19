mod accelerometer;
pub use accelerometer::Acceleration;
pub use accelerometer::AccelerometerData;

#[cfg(any(target_os = "ios", target_os = "watchos"))]
mod altimeter;
#[cfg(any(target_os = "ios", target_os = "watchos"))]
pub use altimeter::Altimeter;

#[cfg(any(target_os = "ios", target_os = "watchos"))]
mod altitude;
#[cfg(any(target_os = "ios", target_os = "watchos"))]
pub use altitude::AltitudeData;

#[cfg(any(target_os = "ios", target_os = "watchos"))]
mod absolute_altitude;
#[cfg(any(target_os = "ios", target_os = "watchos"))]
pub use absolute_altitude::AbsAltitudeData;

#[cfg(target_os = "ios")]
mod attitude;
#[cfg(target_os = "ios")]
pub use attitude::Attitude;
#[cfg(target_os = "ios")]
pub use attitude::AttitudeRefFrame;
#[cfg(target_os = "ios")]
pub use attitude::Quaternion;
#[cfg(target_os = "ios")]
pub use attitude::RotationMatrix;

mod authorization;
#[cfg(any(target_os = "ios", target_os = "watchos"))]
pub use authorization::AuthorizationStatus;

mod pedometer;
pub use pedometer::Pedometer;
pub use pedometer::PedometerData;
pub use pedometer::PedometerEvent;
pub use pedometer::PedometerEventType;

mod log_item;
pub use log_item::LogItem;
