use std::ffi::c_void;

use crate::define_cf_type;

use crate::cf;

define_cf_type!(NotificationName(cf::String));

pub type NotificationCallback = extern "C" fn(
    center: &NotificationCenter,
    observer: *mut c_void,
    name: &NotificationName,
    object: *const c_void,
    user_info: Option<&cf::Dictionary>,
);

define_cf_type!(NotificationCenter(cf::Type));

impl NotificationCenter {
    /// ```
    /// use cidre::cf;
    ///
    /// let nc = cf::NotificationCenter::local_center();
    /// nc.show();
    /// ```
    pub fn local_center<'a>() -> &'a NotificationCenter {
        unsafe { CFNotificationCenterGetLocalCenter() }
    }

    pub fn add_observer(
        &mut self,
        observer: *const c_void,
        callback: NotificationCallback,
        name: &NotificationName,
        object: *const c_void,
        suspension_behavior: NotificationSuspensionBehavior,
    ) {
        unsafe {
            CFNotificationCenterAddObserver(
                self,
                observer,
                callback,
                name,
                object,
                suspension_behavior,
            )
        }
    }

    pub fn remove_observer(
        &mut self,
        observer: *const c_void,
        name: &NotificationName,
        object: *const c_void,
    ) {
        unsafe { CFNotificationCenterRemoveObserver(self, observer, name, object) }
    }

    pub fn remove_every_observer(&mut self, observer: *const c_void) {
        unsafe { CFNotificationCenterRemoveEveryObserver(self, observer) }
    }

    pub fn post_notification(
        &self,
        name: &NotificationName,
        object: *const c_void,
        user_info: Option<&cf::Dictionary>,
        deliver_immediately: bool,
    ) {
        unsafe {
            CFNotificationCenterPostNotification(self, name, object, user_info, deliver_immediately)
        }
    }

    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFNotificationCenterGetTypeID() }
    }
}

#[repr(isize)]
pub enum NotificationSuspensionBehavior {
    Drop = 1,
    Coalesce = 2,
    Hold = 3,
    DeliverImmediately = 4,
}

extern "C" {
    fn CFNotificationCenterGetTypeID() -> cf::TypeId;
    fn CFNotificationCenterGetLocalCenter<'a>() -> &'a NotificationCenter;
    fn CFNotificationCenterAddObserver(
        center: &mut NotificationCenter,
        observer: *const c_void,
        callback: NotificationCallback,
        name: &NotificationName,
        object: *const c_void,
        suspension_behavior: NotificationSuspensionBehavior,
    );
    fn CFNotificationCenterRemoveObserver(
        center: &mut NotificationCenter,
        observer: *const c_void,
        name: &NotificationName,
        object: *const c_void,
    );
    fn CFNotificationCenterRemoveEveryObserver(
        center: &mut NotificationCenter,
        observer: *const c_void,
    );

    fn CFNotificationCenterPostNotification(
        center: &NotificationCenter,
        name: &NotificationName,
        object: *const c_void,
        user_info: Option<&cf::Dictionary>,
        deliver_immediately: bool,
    );
}
