mod beacon_region;
pub use beacon_region::Beacon;
pub use beacon_region::BeaconRegion;

mod condition;
pub use condition::Condition;

mod beacon_identity_condition;
pub use beacon_identity_condition::BeaconIdentityCondition;
pub use beacon_identity_condition::BeaconMajorValue;
pub use beacon_identity_condition::BeaconMinorValue;

mod beacon_identity_constraint;
pub use beacon_identity_constraint::BeaconIdentityConstraint;

mod body;
pub use body::AnyBody;
pub use body::BodyIdentifiable;

mod region;
// #[cfg(any(target_os = "ios", target_os = "macos"))]
pub use region::Proximity;
#[cfg(not(target_os = "visionos"))]
pub use region::Region;
// #[cfg(any(target_os = "ios", target_os = "macos"))]

pub use region::RegionState;

mod location_manager;
pub use location_manager::AccuracyAuthorization;
pub use location_manager::ActivityType;
pub use location_manager::AuthorizationStatus;
#[cfg(feature = "blocks")]
pub use location_manager::HistoricalLocationsHandler;
pub use location_manager::LocationManager;
#[cfg(feature = "blocks")]
pub use location_manager::LocationPushHandler;
#[cfg(feature = "blocks")]
pub use location_manager::TemporaryFullAccuracyHandler;

mod location_manager_delegate;
pub use location_manager_delegate::AnyDelegate as AnyLocationManagerDelegate;
pub use location_manager_delegate::Delegate as LocationManagerDelegate;
pub use location_manager_delegate::DelegateImpl as LocationManagerDelegateImpl;

mod location_manager_visit_extensions;

mod visit;
pub use visit::Visit;

mod location;
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
pub use location::accuracy as location_accuracy;

mod heading;
pub use heading::Heading;

mod location_updater;
pub use location_updater::LiveUpdateCfg;
pub use location_updater::LocationUpdater;
pub use location_updater::Update as LocationUpdate;
#[cfg(feature = "blocks")]
pub use location_updater::UpdateHandler as LocationUpdateHandler;

#[link(name = "CoreLocation", kind = "framework")]
unsafe extern "C" {}

#[link(name = "cl", kind = "static")]
unsafe extern "C" {}
