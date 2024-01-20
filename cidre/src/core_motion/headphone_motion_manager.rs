use crate::{arc, blocks, core_motion as cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CMHeadphoneMotionManager")]
    pub HeadphoneMotionManager(ns::Id),
    CM_HEADPHONE_MOTION_MANAGER
);

impl HeadphoneMotionManager {
    #[objc::cls_msg_send(authorizationStatus)]
    pub fn authorization_status() -> cm::AuthorizationStatus;

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<&AnyDelegate>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(isDeviceMotionAvailable)]
    pub fn is_device_motion_available(&self) -> bool;

    #[objc::msg_send(isDeviceMotionActive)]
    pub fn is_device_motion_active(&self) -> bool;

    #[objc::msg_send(deviceMotion)]
    pub fn device_motion(&self) -> Option<&cm::DeviceMotion>;

    #[objc::msg_send(startDeviceMotionUpdates)]
    pub fn start_device_motion_updates(&mut self);

    #[objc::msg_send(startDeviceMotionUpdatesToQueue:withHandler:)]
    pub unsafe fn _start_device_motion_updates_to_queue(
        &mut self,
        queue: &ns::OpQueue,
        handler: *mut std::ffi::c_void,
    );

    #[inline]
    pub fn start_device_motion_updates_to_queue_block<'a, F>(
        &mut self,
        queue: &ns::OpQueue,
        handler: &mut blocks::Block<F>,
    ) where
        F: FnMut(Option<&'a cm::DeviceMotion>, Option<&'a ns::Error>),
    {
        unsafe { self._start_device_motion_updates_to_queue(queue, handler.as_mut_ptr()) }
    }

    #[inline]
    pub fn start_device_motion_updates_to_queue<F>(&mut self, queue: &ns::OpQueue, handler: F)
    where
        F: FnMut(Option<&cm::DeviceMotion>, Option<&ns::Error>) + 'static,
    {
        let mut handler = blocks::mut2(handler);
        self.start_device_motion_updates_to_queue_block(queue, handler.escape())
    }

    #[objc::msg_send(stopDeviceMotionUpdates)]
    pub fn stop_device_motion_updates(&mut self);
}

#[objc::obj_trait]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(headphoneMotionManagerDidConnect:)]
    fn headphone_motion_manager_did_connect(&mut self, manager: &cm::HeadphoneMotionManager);

    #[objc::optional]
    #[objc::msg_send(headphoneMotionManagerDidDisconnect:)]
    fn headphone_motion_manager_did_disconnect(&mut self, manager: &cm::HeadphoneMotionManager);
}

define_obj_type!(
    pub AnyDelegate(ns::Id)
);

impl Delegate for AnyDelegate {}

#[link(name = "core_motion", kind = "static")]
extern "C" {
    static CM_HEADPHONE_MOTION_MANAGER: &'static objc::Class<HeadphoneMotionManager>;
}
