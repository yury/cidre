use crate::{arc, blocks, core_motion as cm, define_obj_type, ns, objc};

#[cfg(any(target_os = "ios", target_os = "watchos"))]
define_obj_type!(
    #[doc(alias = "CMAltimeter")]
    pub Altimeter(ns::Id),
    CM_ALTIMETER
);

#[cfg(any(target_os = "ios", target_os = "watchos"))]
impl Altimeter {
    #[objc::cls_msg_send(isRelativeAltitudeAvailable)]
    pub fn is_relative_altitude_available() -> bool;

    #[cfg(any(target_os = "ios", target_os = "watchos"))]
    #[objc::cls_msg_send(authorizationStatus)]
    pub fn authorization_status() -> cm::AuthorizationStatus;

    #[objc::msg_send(startRelativeAltitudeUpdatesToQueue:withHandler:)]
    pub fn start_relative_altitude_updates_with_handler(
        &mut self,
        queue: &ns::OpQueue,
        handler: &mut blocks::ResultCompletionHandler<cm::AltitudeData>,
    );

    #[inline]
    pub fn start_relative_altitude_updates(
        &mut self,
        queue: &ns::OpQueue,
        handler: impl FnMut(Option<&cm::AltitudeData>, Option<&ns::Error>) + 'static,
    ) {
        let mut handler = blocks::ResultCompletionHandler::new2(handler);
        self.start_relative_altitude_updates_with_handler(queue, &mut handler)
    }

    #[objc::msg_send(stopRelativeAltitudeUpdates)]
    pub fn stop_relative_altitude_updates(&mut self);

    #[objc::cls_msg_send(isAbsoluteAltitudeAvailable)]
    pub fn is_abs_altitude_available() -> bool;

    #[objc::msg_send(startAbsoluteAltitudeUpdatesToQueue:withHandler:)]
    pub fn start_abs_altitude_updates_to_queue_handler(
        &mut self,
        queue: &ns::OpQueue,
        handler: &mut blocks::ResultCompletionHandler<cm::AbsAltitudeData>,
    );

    #[inline]
    pub fn start_abs_altitude_updates_to_queue(
        &mut self,
        queue: &ns::OpQueue,
        handler: impl FnMut(Option<&cm::AbsAltitudeData>, Option<&ns::Error>) + 'static,
    ) {
        let mut handler = blocks::ResultCompletionHandler::new2(handler);
        self.start_abs_altitude_updates_to_queue_handler(queue, &mut handler)
    }

    #[objc::msg_send(stopAbsoluteAltitudeUpdates)]
    pub fn stop_abs_altitude_updates(&mut self);
}

#[link(name = "core_motion", kind = "static")]
extern "C" {
    static CM_ALTIMETER: &'static objc::Class<Altimeter>;
}
