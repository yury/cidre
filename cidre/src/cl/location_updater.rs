use crate::{arc, cl, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;
#[cfg(feature = "dispatch")]
use crate::dispatch;

#[doc(alias = "CLLiveUpdateConfiguration")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum LiveUpdateCfg {
    #[doc(alias = "CLLiveUpdateConfigurationDefault")]
    Default = 0,
    #[doc(alias = "CLLiveUpdateConfigurationAutomotiveNavigation")]
    AutomotiveNavigation,
    #[doc(alias = "CLLiveUpdateConfigurationOtherNavigation")]
    OtherNavigation,
    #[doc(alias = "CLLiveUpdateConfigurationFitness")]
    Fitness,
    #[doc(alias = "CLLiveUpdateConfigurationAirborne")]
    Airborne,
    #[doc(alias = "CLLiveUpdateConfigurationMaritime")]
    #[cfg(any(
        all(target_os = "macos", feature = "macos_27_0"),
        all(target_os = "ios", feature = "ios_27_0"),
        all(target_os = "watchos", feature = "watchos_27_0"),
        all(target_os = "tvos", feature = "tvos_27_0"),
        all(target_os = "visionos", feature = "visionos_27_0")
    ))]
    Maritime,
}

define_obj_type!(
    #[doc(alias = "CLUpdate")]
    pub Update(ns::Id)
);

impl Update {
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    crate::define_cls!(CL_UPDATE);

    #[objc::msg_send(authorizationDenied)]
    #[objc::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0)]
    pub fn authorization_denied(&self) -> bool;

    #[objc::msg_send(authorizationDeniedGlobally)]
    #[objc::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0)]
    pub fn authorization_denied_globally(&self) -> bool;

    #[objc::msg_send(authorizationRestricted)]
    #[objc::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0)]
    pub fn authorization_restricted(&self) -> bool;

    #[deprecated(note = "use stationary()")]
    #[objc::msg_send(isStationary)]
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0)]
    pub fn is_stationary(&self) -> bool;

    #[objc::msg_send(stationary)]
    #[objc::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0)]
    pub fn stationary(&self) -> bool;

    #[objc::msg_send(insufficientlyInUse)]
    #[objc::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0)]
    pub fn insufficiently_in_use(&self) -> bool;

    #[objc::msg_send(locationUnavailable)]
    #[objc::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0)]
    pub fn location_unavailable(&self) -> bool;

    #[objc::msg_send(accuracyLimited)]
    #[objc::available(macos = 15.0, ios = 18.0, tvos = 18.0, watchos = 11.0)]
    pub fn accuracy_limited(&self) -> bool;

    #[objc::msg_send(serviceSessionRequired)]
    #[objc::available(macos = 15.0, ios = 18.0)]
    pub fn service_session_required(&self) -> bool;

    #[objc::msg_send(authorizationRequestInProgress)]
    #[objc::available(macos = 15.0, ios = 18.0)]
    pub fn authorization_request_in_progress(&self) -> bool;

    #[objc::msg_send(location)]
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn location(&self) -> Option<arc::R<cl::Location>>;
}

#[cfg(feature = "blocks")]
pub type UpdateHandler = blocks::SendBlock<fn(update: Option<&Update>)>;

define_obj_type!(
    #[doc(alias = "CLLocationUpdater")]
    pub LocationUpdater(ns::Id)
);

impl LocationUpdater {
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    crate::define_cls!(CL_LOCATION_UPDATER);

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    #[objc::msg_send(liveUpdaterWithQueue:handler:)]
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn with_queue_handler_block(
        queue: &dispatch::Queue,
        handler: &mut UpdateHandler,
    ) -> Option<arc::R<Self>>;

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn with_queue(
        queue: &dispatch::Queue,
        handler: impl FnMut(Option<&Update>) + Send + 'static,
    ) -> Option<arc::R<Self>> {
        let mut handler = UpdateHandler::new1(handler);
        Self::with_queue_handler_block(queue, &mut handler)
    }

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    #[objc::msg_send(liveUpdaterWithConfiguration:queue:handler:)]
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn with_cfg_queue_handler_block(
        cfg: LiveUpdateCfg,
        queue: &dispatch::Queue,
        handler: &mut UpdateHandler,
    ) -> Option<arc::R<Self>>;

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn with_cfg_queue(
        cfg: LiveUpdateCfg,
        queue: &dispatch::Queue,
        handler: impl FnMut(Option<&Update>) + Send + 'static,
    ) -> Option<arc::R<Self>> {
        let mut handler = UpdateHandler::new1(handler);
        Self::with_cfg_queue_handler_block(cfg, queue, &mut handler)
    }

    #[objc::msg_send(resume)]
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn resume(&mut self);

    #[objc::msg_send(pause)]
    #[objc::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn pause(&mut self);

    #[objc::msg_send(invalidate)]
    #[objc::available(macos = 14.0, ios = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn invalidate(&mut self);
}

unsafe extern "C" {
    static CL_UPDATE: &'static objc::Class<Update>;
    static CL_LOCATION_UPDATER: &'static objc::Class<LocationUpdater>;
}
