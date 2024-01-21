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
    Other = 1,
    AutomotiveNavigation,
    Fitnes,
    OtherNavigation,
    Airborne,
}

define_obj_type!(
    #[doc(alias = "CLLocationManager")]
    pub LocationManager(ns::Id),
    CL_LOCATION_MANAGER
);

impl LocationManager {
    #[objc::cls_msg_send(locationServicesEnabled)]
    pub fn location_services_enabled() -> bool;

    #[objc::cls_msg_send(significantLocationChangeMonitoringAvailable)]
    pub fn significant_location_change_monitoring_available() -> bool;

    #[objc::cls_msg_send(headingAvailable)]
    pub fn heading_available() -> bool;

    #[objc::cls_msg_send(isMonitoringAvailableForClass:)]
    pub fn is_monitoring_available_for_class<T: objc::Obj>(cls: &objc::Class<T>) -> bool;

    #[cfg(not(any(target_os = "watchos", target_os = "tvos")))]
    #[objc::cls_msg_send(isRangingAvailable)]
    pub fn is_ranging_available() -> bool;

    #[objc::msg_send(authorizationStatus)]
    pub fn authorization_status(&self) -> cl::AuthorizationStatus;

    #[objc::msg_send(accuracyAuthorization)]
    pub fn accuracy_authorization(&self) -> cl::AccuracyAuthorization;

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    #[objc::msg_send(isAuthorizedForWidgetUpdates)]
    pub fn is_authorized_for_widget_updates(&self) -> bool;
}

#[link(name = "cl", kind = "static")]
extern "C" {
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
