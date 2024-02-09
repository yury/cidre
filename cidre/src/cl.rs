mod beacon_region;
pub use beacon_region::Beacon;
pub use beacon_region::BeaconRegion;

mod region;
// #[cfg(any(target_os = "ios", target_os = "macos"))]
pub use region::Proximity;
pub use region::Region;
// #[cfg(any(target_os = "ios", target_os = "macos"))]
pub use region::RegionState;

mod location_manager;
pub use location_manager::AccuracyAuthorization;
pub use location_manager::ActivityType;
pub use location_manager::AuthorizationStatus;
pub use location_manager::Delegate as LocationManagerDelegate;
pub use location_manager::DelegateImpl as LocationManagerDelegateImpl;
pub use location_manager::LocationManager;

mod location;
pub use location::accuracy as location_accuracy;
pub use location::Accuracy as LocationAccuracy;
pub use location::Coordinate2d as LocationCoordinate2d;
pub use location::Degrees as LocationDegrees;
pub use location::Direction as LocationDirection;
pub use location::DirectionAccuracy as LocationDirectionAccuracy;
pub use location::Distance as LocationDistance;
pub use location::Floor;
pub use location::Location;
pub use location::Speed as LocationSpeed;
pub use location::SpeedAccuracy as LocationSpeedAccuracy;
pub use location::SrcInfo as LocationSrcInfo;

mod heading;
pub use heading::Heading;
