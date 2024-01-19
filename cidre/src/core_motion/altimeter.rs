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
    pub unsafe fn _start_relative_altitude_updates_with_handler(
        &mut self,
        queue: &ns::OpQueue,
        handler: *mut std::ffi::c_void,
    );

    #[inline]
    pub fn start_relative_altitude_updates_with_block<'a, F>(
        &mut self,
        queue: &ns::OpQueue,
        handler: &'static mut blocks::Block<F>,
    ) where
        F: FnMut(Option<&'a cm::AltitudeData>, Option<&'a ns::Error>),
    {
        unsafe { self._start_relative_altitude_updates_with_handler(queue, handler.as_mut_ptr()) }
    }

    #[inline]
    pub fn start_relative_altitude_updates<F>(&mut self, queue: &ns::OpQueue, handler: F)
    where
        F: FnMut(Option<&cm::AltitudeData>, Option<&ns::Error>) + 'static,
    {
        let mut handler = blocks::mut2(handler);
        self.start_relative_altitude_updates_with_block(queue, handler.escape())
    }

    #[objc::msg_send(stopRelativeAltitudeUpdates)]
    pub fn stop_relative_altitude_updates(&mut self);

    #[objc::cls_msg_send(isAbsoluteAltitudeAvailable)]
    pub fn is_abs_altitude_available() -> bool;

    #[objc::msg_send(startAbsoluteAltitudeUpdatesToQueue:withHandler:)]
    pub unsafe fn _start_abs_altitude_updates_to_queue_handler(
        &mut self,
        queue: &ns::OpQueue,
        handler: *mut std::ffi::c_void,
    );

    #[inline]
    pub fn start_abs_altitude_updates_to_queue_block<'a, F>(
        &mut self,
        queue: &ns::OpQueue,
        handler: &'static mut blocks::Block<F>,
    ) where
        F: FnMut(Option<&'a cm::AbsAltitudeData>, Option<&'a ns::Error>),
    {
        unsafe { self._start_abs_altitude_updates_to_queue_handler(queue, handler.as_mut_ptr()) }
    }

    #[inline]
    pub fn start_abs_altitude_updates_to_queue<F>(&mut self, queue: &ns::OpQueue, handler: F)
    where
        F: FnMut(Option<&cm::AbsAltitudeData>, Option<&ns::Error>) + 'static,
    {
        let mut handler = blocks::mut2(handler);
        self.start_abs_altitude_updates_to_queue_block(queue, handler.escape())
    }

    #[objc::msg_send(stopAbsoluteAltitudeUpdates)]
    pub fn stop_abs_altitude_updates(&mut self);
}

#[link(name = "core_motion", kind = "static")]
extern "C" {
    static CM_ALTIMETER: &'static objc::Class<Altimeter>;
}
