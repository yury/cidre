use std::ffi::c_void;

use crate::{cf, define_cf_type};

#[cfg(feature = "ns")]
use crate::ns;

define_cf_type!(NotificationName(cf::String));

impl NotificationName {
    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::NotificationName {
        unsafe { std::mem::transmute(self) }
    }
}

pub type NotificationCallback = extern "C" fn(
    center: &NotificationCenter,
    observer: *mut c_void,
    name: &NotificationName,
    object: *const c_void,
    user_info: Option<&cf::Dictionary>,
);

define_cf_type!(NotificationCenter(cf::Type));

impl NotificationCenter {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFNotificationCenterGetTypeID() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let nc = cf::NotificationCenter::local();
    /// nc.show();
    /// ```
    #[doc(alias = "CFNotificationCenterGetLocalCenter")]
    #[inline]
    pub fn local<'a>() -> &'a mut NotificationCenter {
        unsafe { CFNotificationCenterGetLocalCenter() }
    }

    #[doc(alias = "CFNotificationCenterAddObserver")]
    #[inline]
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

    #[doc(alias = "CFNotificationCenterRemoveObserver")]
    #[inline]
    pub fn remove_observer(
        &mut self,
        observer: *const c_void,
        name: &NotificationName,
        object: *const c_void,
    ) {
        unsafe { CFNotificationCenterRemoveObserver(self, observer, name, object) }
    }

    #[doc(alias = "CFNotificationCenterRemoveEveryObserver")]
    #[inline]
    pub fn remove_every_observer(&mut self, observer: *const c_void) {
        unsafe { CFNotificationCenterRemoveEveryObserver(self, observer) }
    }

    #[doc(alias = "CFNotificationCenterPostNotification")]
    #[inline]
    pub fn post_notification(
        &mut self,
        name: &NotificationName,
        object: *const c_void,
        user_info: Option<&cf::Dictionary>,
        deliver_immediately: bool,
    ) {
        unsafe {
            CFNotificationCenterPostNotification(self, name, object, user_info, deliver_immediately)
        }
    }
}

#[repr(isize)]
pub enum NotificationSuspensionBehavior {
    Drop = 1,
    Coalesce = 2,
    Hold = 3,
    DeliverImmediately = 4,
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFNotificationCenterGetTypeID() -> cf::TypeId;
    fn CFNotificationCenterGetLocalCenter<'a>() -> &'a mut NotificationCenter;
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
        center: &mut NotificationCenter,
        name: &NotificationName,
        object: *const c_void,
        user_info: Option<&cf::Dictionary>,
        deliver_immediately: bool,
    );
}
