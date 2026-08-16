use crate::{cl, define_obj_type, ns, objc};

#[doc(alias = "CLLocationManagerDelegate")]
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
        heading: &cl::Heading,
    );

    #[objc::optional]
    #[objc::msg_send(locationManagerShouldDisplayHeadingCalibration:)]
    fn location_manager_should_display_heading_calibration(
        &mut self,
        manager: &cl::LocationManager,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(locationManager:didDetermineState:forRegion:)]
    fn location_manager_did_determine_state_for_region(
        &mut self,
        manager: &cl::LocationManager,
        state: cl::RegionState,
        region: &cl::Region,
    );

    #[objc::optional]
    #[objc::msg_send(locationManager:didRangeBeacons:satisfyingConstraint:)]
    fn location_manager_did_range_beacons_satisfying_constraint(
        &mut self,
        manager: &cl::LocationManager,
        beacons: &ns::Array<cl::Beacon>,
        constraint: &cl::BeaconIdentityConstraint,
    );

    #[objc::optional]
    #[objc::msg_send(locationManager:didFailRangingBeaconsForConstraint:error:)]
    fn location_manager_did_fail_ranging_beacons(
        &mut self,
        manager: &cl::LocationManager,
        constraint: &cl::BeaconIdentityConstraint,
        error: &ns::Error,
    );

    #[objc::optional]
    #[objc::msg_send(locationManager:didEnterRegion:)]
    fn location_manager_did_enter_region(
        &mut self,
        manager: &cl::LocationManager,
        region: &cl::Region,
    );

    #[objc::optional]
    #[objc::msg_send(locationManager:didExitRegion:)]
    fn location_manager_did_exit_region(
        &mut self,
        manager: &cl::LocationManager,
        region: &cl::Region,
    );

    #[objc::optional]
    #[objc::msg_send(locationManager:didFailWithError:)]
    fn location_manager_did_fail(&mut self, manager: &cl::LocationManager, error: &ns::Error);

    #[objc::optional]
    #[objc::msg_send(locationManager:monitoringDidFailForRegion:withError:)]
    fn location_manager_monitoring_did_fail_for_region(
        &mut self,
        manager: &cl::LocationManager,
        region: Option<&cl::Region>,
        error: &ns::Error,
    );

    #[objc::optional]
    #[objc::msg_send(locationManagerDidChangeAuthorization:)]
    fn location_manager_did_change_authorization(&mut self, manager: &cl::LocationManager);

    #[objc::optional]
    #[objc::msg_send(locationManager:didStartMonitoringForRegion:)]
    fn location_manager_did_start_monitoring_for_region(
        &mut self,
        manager: &cl::LocationManager,
        region: &cl::Region,
    );

    #[objc::optional]
    #[objc::msg_send(locationManagerDidPauseLocationUpdates:)]
    fn location_manager_did_pause_location_updates(&mut self, manager: &cl::LocationManager);

    #[objc::optional]
    #[objc::msg_send(locationManagerDidResumeLocationUpdates:)]
    fn location_manager_did_resume_location_updates(&mut self, manager: &cl::LocationManager);

    #[objc::optional]
    #[objc::msg_send(locationManager:didFinishDeferredUpdatesWithError:)]
    fn location_manager_did_finish_deferred_updates(
        &mut self,
        manager: &cl::LocationManager,
        error: Option<&ns::Error>,
    );

    #[objc::optional]
    #[objc::msg_send(locationManager:didVisit:)]
    fn location_manager_did_visit(&mut self, manager: &cl::LocationManager, visit: &cl::Visit);
}

define_obj_type!(pub AnyDelegate(ns::Id));
impl Delegate for AnyDelegate {}
