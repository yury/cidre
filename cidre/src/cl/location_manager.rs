use crate::{arc, cl, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "CLAuthorizationStatus")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum AuthorizationStatus {
    #[doc(alias = "kCLAuthorizationStatusNotDetermined")]
    NotDetermined = 0,
    #[doc(alias = "kCLAuthorizationStatusRestricted")]
    Restricted = 1,
    #[doc(alias = "kCLAuthorizationStatusDenied")]
    Denied = 2,
    #[doc(alias = "kCLAuthorizationStatusAuthorizedAlways")]
    #[cfg(not(target_os = "visionos"))]
    AuthorizedAlways = 3,
    #[doc(alias = "kCLAuthorizationStatusAuthorizedWhenInUse")]
    #[cfg(not(target_os = "macos"))]
    AuthorizedWhenInUse = 4,
}

#[cfg(target_os = "macos")]
impl AuthorizationStatus {
    #[doc(alias = "kCLAuthorizationStatusAuthorized")]
    pub const AUTHORIZED: Self = Self::AuthorizedAlways;
}

#[doc(alias = "CLAccuracyAuthorization")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AccuracyAuthorization {
    #[doc(alias = "CLAccuracyAuthorizationFullAccuracy")]
    FullAccuracy,
    #[doc(alias = "CLAccuracyAuthorizationReducedAccuracy")]
    ReducedAccuracy,
}

#[doc(alias = "CLActivityType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ActivityType {
    #[doc(alias = "CLActivityTypeOther")]
    Other = 1,
    #[doc(alias = "CLActivityTypeAutomotiveNavigation")]
    AutomotiveNavigation,
    #[doc(alias = "CLActivityTypeFitness")]
    Fitness,
    #[doc(alias = "CLActivityTypeOtherNavigation")]
    OtherNavigation,
    #[doc(alias = "CLActivityTypeAirborne")]
    Airborne,
    #[doc(alias = "CLActivityTypeMaritime")]
    #[cfg(any(
        all(target_os = "macos", feature = "macos_27_0"),
        all(target_os = "ios", feature = "ios_27_0"),
        all(target_os = "watchos", feature = "watchos_27_0"),
        all(target_os = "tvos", feature = "tvos_27_0"),
        all(target_os = "visionos", feature = "visionos_27_0")
    ))]
    Maritime,
}

#[cfg(feature = "blocks")]
pub type TemporaryFullAccuracyHandler = blocks::ErrCh;

#[cfg(feature = "blocks")]
pub type LocationPushHandler =
    blocks::EscBlock<fn(token: Option<&ns::Data>, error: Option<&ns::Error>)>;

#[cfg(feature = "blocks")]
pub type HistoricalLocationsHandler =
    blocks::EscBlock<fn(locations: &ns::Array<cl::Location>, error: Option<&ns::Error>)>;

define_obj_type!(
    #[doc(alias = "CLLocationManager")]
    pub LocationManager(ns::Id),
    CL_LOCATION_MANAGER
);

impl LocationManager {
    #[objc::msg_send(locationServicesEnabled)]
    #[objc::available(macos = 10.7, ios = 4.0)]
    pub fn location_services_enabled() -> bool;

    #[objc::msg_send(headingAvailable)]
    #[objc::available(macos = 10.7, ios = 4.0, watchos = 2.0)]
    pub fn heading_available() -> bool;

    #[objc::msg_send(significantLocationChangeMonitoringAvailable)]
    #[objc::available(macos = 10.7, ios = 4.0)]
    pub fn significant_location_change_monitoring_available() -> bool;

    #[objc::msg_send(isMonitoringAvailableForClass:)]
    #[objc::available(macos = 10.10, ios = 7.0)]
    pub fn is_monitoring_available_for_class<T: objc::Obj>(cls: &objc::Class<T>) -> bool;

    #[objc::msg_send(isRangingAvailable)]
    #[objc::available(macos = 10.15, ios = 7.0)]
    pub fn is_ranging_available() -> bool;

    #[objc::msg_send(authorizationStatus)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0, watchos = 7.0)]
    pub fn authorization_status(&self) -> cl::AuthorizationStatus;

    #[objc::msg_send(accuracyAuthorization)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0, watchos = 7.0)]
    pub fn accuracy_authorization(&self) -> cl::AccuracyAuthorization;

    #[objc::msg_send(isAuthorizedForWidgetUpdates)]
    #[objc::available(macos = 11.0, ios = 14.0)]
    pub fn is_authorized_for_widget_updates(&self) -> bool;

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<cl::AnyLocationManagerDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: cl::LocationManagerDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(activityType)]
    #[objc::available(macos = 10.15, ios = 6.0, watchos = 4.0)]
    pub fn activity_type(&self) -> cl::ActivityType;

    #[objc::msg_send(setActivityType:)]
    #[objc::available(macos = 10.15, ios = 6.0, watchos = 4.0)]
    pub fn set_activity_type(&mut self, val: cl::ActivityType);

    #[objc::msg_send(distanceFilter)]
    pub fn distance_filter(&self) -> cl::LocationDistance;

    #[objc::msg_send(setDistanceFilter:)]
    pub fn set_distance_filter(&mut self, val: cl::LocationDistance);

    #[objc::msg_send(desiredAccuracy)]
    pub fn desired_accuracy(&self) -> cl::LocationAccuracy;

    #[objc::msg_send(setDesiredAccuracy:)]
    pub fn set_desired_accuracy(&mut self, val: cl::LocationAccuracy);

    #[objc::msg_send(pausesLocationUpdatesAutomatically)]
    #[objc::available(macos = 10.15, ios = 6.0)]
    pub fn pauses_location_updates_automatically(&self) -> bool;

    #[objc::msg_send(setPausesLocationUpdatesAutomatically:)]
    #[objc::available(macos = 10.15, ios = 6.0)]
    pub fn set_pauses_location_updates_automatically(&mut self, val: bool);

    #[objc::msg_send(allowsBackgroundLocationUpdates)]
    #[objc::available(macos = 10.15, ios = 9.0, watchos = 4.0)]
    pub fn allows_background_location_updates(&self) -> bool;

    #[objc::msg_send(setAllowsBackgroundLocationUpdates:)]
    #[objc::available(macos = 10.15, ios = 9.0, watchos = 4.0)]
    pub fn set_allows_background_location_updates(&mut self, val: bool);

    #[cfg(target_os = "ios")]
    #[objc::msg_send(showsBackgroundLocationIndicator)]
    #[objc::available(ios = 11.0)]
    pub fn shows_background_location_indicator(&self) -> bool;

    #[cfg(target_os = "ios")]
    #[objc::msg_send(setShowsBackgroundLocationIndicator:)]
    #[objc::available(ios = 11.0)]
    pub fn set_shows_background_location_indicator(&mut self, val: bool);

    #[objc::msg_send(location)]
    pub fn location(&self) -> Option<arc::R<cl::Location>>;

    #[objc::msg_send(headingFilter)]
    #[objc::available(macos = 10.15, ios = 3.0, watchos = 2.0)]
    pub fn heading_filter(&self) -> cl::LocationDegrees;

    #[objc::msg_send(setHeadingFilter:)]
    #[objc::available(macos = 10.15, ios = 3.0, watchos = 2.0)]
    pub fn set_heading_filter(&mut self, val: cl::LocationDegrees);

    #[objc::msg_send(headingBody)]
    #[objc::available(macos = 27.0, ios = 27.0, watchos = 27.0)]
    pub fn heading_body(&self) -> Option<arc::R<cl::AnyBody>>;

    #[objc::msg_send(setHeadingBody:)]
    #[objc::available(macos = 27.0, ios = 27.0, watchos = 27.0)]
    pub fn set_heading_body<B: cl::BodyIdentifiable>(&mut self, val: Option<&B>);

    #[objc::msg_send(heading)]
    #[objc::available(macos = 10.15, ios = 4.0, watchos = 2.0)]
    pub fn heading(&self) -> Option<arc::R<cl::Heading>>;

    #[objc::msg_send(maximumRegionMonitoringDistance)]
    #[objc::available(macos = 10.8, ios = 4.0)]
    pub fn max_region_monitoring_distance(&self) -> cl::LocationDistance;

    #[objc::msg_send(monitoredRegions)]
    #[objc::available(macos = 10.8, ios = 4.0)]
    pub fn monitored_regions(&self) -> arc::R<ns::Set<cl::Region>>;

    #[objc::msg_send(rangedBeaconConstraints)]
    #[objc::available(macos = 10.15, ios = 13.0)]
    pub fn ranged_beacon_constraints(&self) -> arc::R<ns::Set<cl::BeaconIdentityConstraint>>;

    #[objc::msg_send(requestWhenInUseAuthorization)]
    #[objc::available(macos = 10.15, ios = 8.0)]
    pub fn request_when_in_use_authorization(&mut self);

    #[objc::msg_send(requestAlwaysAuthorization)]
    #[objc::available(macos = 10.15, ios = 8.0)]
    pub fn request_always_authorization(&mut self);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(requestTemporaryFullAccuracyAuthorizationWithPurposeKey:completion:)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0, watchos = 7.0)]
    pub fn request_temporary_full_accuracy_ch_block(
        &mut self,
        purpose_key: &ns::String,
        completion: Option<&mut TemporaryFullAccuracyHandler>,
    );

    #[cfg(feature = "blocks")]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0, watchos = 7.0)]
    pub fn request_temporary_full_accuracy(
        &mut self,
        purpose_key: &ns::String,
        completion: impl FnMut(Option<&ns::Error>) + 'static,
    ) {
        let mut completion = TemporaryFullAccuracyHandler::new1(completion);
        self.request_temporary_full_accuracy_ch_block(purpose_key, Some(&mut completion));
    }

    #[objc::msg_send(requestTemporaryFullAccuracyAuthorizationWithPurposeKey:)]
    #[objc::available(macos = 11.0, ios = 14.0, tvos = 14.0, watchos = 7.0)]
    pub fn request_temporary_full_accuracy_without_handler(&mut self, purpose_key: &ns::String);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(startUpdatingLocation)]
    #[objc::available(macos = 10.6, ios = 2.0, watchos = 3.0)]
    pub fn start_updating_location(&mut self);

    #[objc::msg_send(stopUpdatingLocation)]
    pub fn stop_updating_location(&mut self);

    #[objc::msg_send(requestLocation)]
    #[objc::available(macos = 10.14, ios = 9.0)]
    pub fn request_location(&mut self);

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "watchos"))]
    #[objc::msg_send(startUpdatingHeading)]
    #[objc::available(macos = 10.15, ios = 3.0, watchos = 2.0)]
    pub fn start_updating_heading(&mut self);

    #[cfg(any(target_os = "ios", target_os = "watchos"))]
    #[objc::msg_send(stopUpdatingHeading)]
    #[objc::available(ios = 3.0, watchos = 2.0)]
    pub fn stop_updating_heading(&mut self);

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "watchos"))]
    #[objc::msg_send(dismissHeadingCalibrationDisplay)]
    #[objc::available(macos = 10.15, ios = 3.0, watchos = 2.0)]
    pub fn dismiss_heading_calibration_display(&mut self);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[objc::msg_send(startMonitoringSignificantLocationChanges)]
    #[objc::available(macos = 10.7, ios = 4.0)]
    pub fn start_monitoring_significant_location_changes(&mut self);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[objc::msg_send(stopMonitoringSignificantLocationChanges)]
    #[objc::available(macos = 10.7, ios = 4.0)]
    pub fn stop_monitoring_significant_location_changes(&mut self);

    #[cfg(all(feature = "blocks", target_os = "ios", not(target_abi = "macabi")))]
    #[objc::msg_send(startMonitoringLocationPushesWithCompletion:)]
    #[objc::available(ios = 15.0)]
    pub fn start_monitoring_location_pushes_ch_block(
        &mut self,
        completion: Option<&mut LocationPushHandler>,
    );

    #[cfg(all(feature = "blocks", target_os = "ios", not(target_abi = "macabi")))]
    #[objc::available(ios = 15.0)]
    pub fn start_monitoring_location_pushes(
        &mut self,
        completion: impl FnMut(Option<&ns::Data>, Option<&ns::Error>) + 'static,
    ) {
        let mut completion = LocationPushHandler::new2(completion);
        self.start_monitoring_location_pushes_ch_block(Some(&mut completion));
    }

    #[cfg(all(target_os = "ios", not(target_abi = "macabi")))]
    #[objc::msg_send(stopMonitoringLocationPushes)]
    #[objc::available(ios = 15.0)]
    pub fn stop_monitoring_location_pushes(&mut self);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[objc::msg_send(startRangingBeaconsSatisfyingConstraint:)]
    #[objc::available(macos = 10.15, ios = 13.0)]
    pub fn start_ranging_beacons(&mut self, constraint: &cl::BeaconIdentityConstraint);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[objc::msg_send(stopRangingBeaconsSatisfyingConstraint:)]
    #[objc::available(macos = 10.15, ios = 13.0)]
    pub fn stop_ranging_beacons(&mut self, constraint: &cl::BeaconIdentityConstraint);

    #[cfg(all(feature = "blocks", target_os = "watchos"))]
    #[objc::msg_send(requestHistoricalLocationsWithPurposeKey:sampleCount:completionHandler:)]
    #[objc::available(watchos = 9.0)]
    pub fn request_historical_locations_ch_block(
        &mut self,
        purpose_key: &ns::String,
        sample_count: isize,
        completion: &mut HistoricalLocationsHandler,
    );

    #[cfg(all(feature = "blocks", target_os = "watchos"))]
    #[objc::available(watchos = 9.0)]
    pub fn request_historical_locations(
        &mut self,
        purpose_key: &ns::String,
        sample_count: isize,
        completion: impl FnMut(&ns::Array<cl::Location>, Option<&ns::Error>) + 'static,
    ) {
        let mut completion = HistoricalLocationsHandler::new2(completion);
        unsafe {
            self.request_historical_locations_ch_block(purpose_key, sample_count, &mut completion);
        }
    }
}

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
        assert!(!manager.is_authorized_for_widget_updates());
    }
}
