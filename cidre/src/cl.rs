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
