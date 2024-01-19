mod authorization;

#[cfg(any(target_os = "ios", target_os = "watchos"))]
pub use authorization::AuthorizationStatus;

mod pedometer;
pub use pedometer::Pedometer;
pub use pedometer::PedometerData;
pub use pedometer::PedometerEvent;
pub use pedometer::PedometerEventType;
