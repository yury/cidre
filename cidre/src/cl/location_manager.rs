use crate::{arc, cl, define_obj_type, ns, objc};

#[doc(alias = "CLAuthorizationStatus")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum AuthorizationStatus {
    NotDetermined = 0,
    Restricted,
    Denied,
    AuthorizedAlways,
    #[cfg(not(target_os = "macos"))]
    AuthorizedWhenInUse,
}

#[doc(alias = "CLAccuracyAuthorization")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AccuracyAuthorization {
    FullAccuracy,
    ReducedAccuracy,
}

#[doc(alias = "CLActivityType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ActivityType {
    /// positioning in activities that are not covered by one of
    /// the other activity types.  Includes activities without a
    /// specific user intention; for example, positioning while
    /// the user sits on a bench interacting with the device
    Other = 1,
    /// positioning in an automobile following a road network
    AutomotiveNavigation,
    /// positioning in dedicated fitness sessions, e.g. walking
    /// workouts, running workouts, cycling workouts, etc.
    Fitnes,
    /// positioning for transportation that does not or may not
    /// adhere to roads such as cycling, scooters, trains, boats
    /// and off-road vehicles; also for positioning indoors and
    /// outdoors that isn’t tied to a dedicated fitness session,
    /// e.g. walking
    OtherNavigation,
    /// positioning for activities in the air, e.g. flying in an
    /// airplane or helicopter, paragliding, flying on a drone,
    /// skydiving, etc.  Also includes runway taxiing
    Airborne,
}

define_obj_type!(
    #[doc(alias = "CLLocationManager")]
    pub LocationManager(ns::Id),
    CL_LOCATION_MANAGER
);

impl LocationManager {
    #[objc::msg_send(locationServicesEnabled)]
    pub fn location_services_enabled() -> bool;

    #[objc::msg_send(significantLocationChangeMonitoringAvailable)]
    pub fn significant_location_change_monitoring_available() -> bool;

    /// true if the device supports the heading service, otherwise false.
    #[objc::msg_send(headingAvailable)]
    pub fn heading_available() -> bool;

    #[objc::msg_send(isMonitoringAvailableForClass:)]
    pub fn is_monitoring_available_for_class<T: objc::Obj>(cls: &objc::Class<T>) -> bool;

    #[objc::msg_send(isRangingAvailable)]
    #[api::available(macos = 10.15, ios = 7.0)]
    pub fn is_ranging_available() -> bool;

    #[objc::msg_send(authorizationStatus)]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0, watchos = 7.0)]
    pub fn authorization_status(&self) -> cl::AuthorizationStatus;

    #[objc::msg_send(accuracyAuthorization)]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0, watchos = 7.0)]
    pub fn accuracy_authorization(&self) -> cl::AccuracyAuthorization;

    #[objc::msg_send(isAuthorizedForWidgetUpdates)]
    #[api::available(macos = 11.0, ios = 14.0)]
    pub fn is_authorized_for_widget_updates(&self) -> bool;
}

#[objc::protocol(CLLocationManagerDelegate)]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(locationManager:didUpdateLocations:)]
    fn location_manager_did_update_locations(
        &mut self,
        manager: &cl::LocationManager,
        locations: &ns::Array<cl::Location>,
    );

    #[objc::optional]
    #[objc::msg_send(locationManager:didUpdateHeading:)]
    fn location_manager_did_update_heading(
        &mut self,
        manager: &cl::LocationManager,
        heading: &ns::Array<cl::Heading>,
    );

    #[objc::optional]
    #[objc::msg_send(locationManagerShouldDisplayHeadingCalibration)]
    fn location_manager_should_display_heading_calibration(&mut self) -> bool;

    #[objc::optional]
    #[objc::msg_send(locationManager:didDetermineState:forRegion:)]
    fn location_manager_did_determine_state_for_region(
        &mut self,
        manager: &cl::LocationManager,
        state: cl::RegionState,
        region: &cl::Region,
    );
}

#[link(name = "cl", kind = "static")]
unsafe extern "C" {
    static CL_LOCATION_MANAGER: &'static objc::Class<LocationManager>;
}

#[cfg(test)]
mod tests {
    use crate::cl;

    #[test]
    fn basics() {
        let manager = cl::LocationManager::new();
        assert_eq!(
            manager.authorization_status(),
            cl::AuthorizationStatus::NotDetermined
        );
        assert_eq!(manager.is_authorized_for_widget_updates(), false);
    }
}
