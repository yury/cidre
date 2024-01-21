mod region;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use region::Proximity;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use region::RegionState;

mod location_manager;
pub use location_manager::AccuracyAuthorization;
pub use location_manager::ActivityType;
pub use location_manager::AuthorizationStatus;
pub use location_manager::LocationManager;

mod location;
pub use location::accuracy as location_accuracy;
pub use location::Accuracy as LocationAccuracy;
pub use location::Coordinate2d as LocationCoordinate2d;
pub use location::Direction as LocationDirection;
pub use location::DirectionAccuracy as LocationDirectionAccuracy;
pub use location::Distance as LocationDistance;
pub use location::Speed as LocationSpeed;
pub use location::SpeedAccuracy as LocationSpeedAccuracy;
