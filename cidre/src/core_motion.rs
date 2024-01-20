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

mod attitude;
pub use attitude::Attitude;
pub use attitude::AttitudeRefFrame;
pub use attitude::Quaternion;
pub use attitude::RotationMatrix;

mod authorization;
pub use authorization::AuthorizationStatus;

mod device_motion;
pub use device_motion::CalibratedMagneticField;
pub use device_motion::DeviceMotion;
pub use device_motion::MagneticFieldCalibrationAccuracy;
pub use device_motion::SensorLocation as DeviceMotionSensorLocation;

mod rotation_rate_data;
pub use rotation_rate_data::RecordedRotationRateData;
pub use rotation_rate_data::RotationRateData;

mod gyro;
pub use gyro::GyroData;
pub use gyro::RotationRate;

mod headphone_motion_manager;
pub use headphone_motion_manager::Delegate as HeadphoneMotionManagerDelegate;
pub use headphone_motion_manager::DelegateImpl as HeadphoneMotionManagerDelegateImpl;
pub use headphone_motion_manager::HeadphoneMotionManager;

mod magnetometer;
pub use magnetometer::MagneticField;
pub use magnetometer::MagnetometerData;

mod pedometer;
pub use pedometer::Pedometer;
pub use pedometer::PedometerData;
pub use pedometer::PedometerEvent;
pub use pedometer::PedometerEventType;

mod log_item;
pub use log_item::LogItem;
