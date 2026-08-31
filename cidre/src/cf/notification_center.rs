use std::ffi::c_void;

use crate::{cf, define_cf_type};

#[cfg(feature = "ns")]
use crate::ns;

define_cf_type!(
    #[doc(alias = "CFNotificationName")]
    NotificationName(cf::String)
);

impl NotificationName {
    pub fn with_raw(string: &cf::String) -> &Self {
        unsafe { std::mem::transmute(string) }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::NotificationName {
        unsafe { std::mem::transmute(self) }
    }
}

pub type NotificationCallback<T = c_void> = unsafe extern "C" fn(
    center: &NotificationCenter,
    observer: *mut T,
    name: &NotificationName,
    object: *const c_void,
    user_info: Option<&cf::Dictionary>,
);

define_cf_type!(
    #[doc(alias = "CFNotificationCenterRef")]
    NotificationCenter(cf::Type)
);

impl NotificationCenter {
    #[doc(alias = "CFNotificationCenterGetTypeID")]
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

    /// # Safety
    ///
    /// `observer` must remain valid for every operation performed by `callback`
    /// until this registration is removed. The caller must also ensure that the
    /// callback's thread-safety and reentrancy requirements are upheld.
    #[doc(alias = "CFNotificationCenterAddObserver")]
    #[inline]
    pub unsafe fn add_observer<T>(
        &mut self,
        observer: *mut T,
        callback: NotificationCallback<T>,
        name: Option<&NotificationName>,
        object: *const c_void,
        suspension_behavior: NotificationSuspensionBehavior,
    ) {
        unsafe {
            CFNotificationCenterAddObserver(
                self,
                observer.cast(),
                std::mem::transmute::<NotificationCallback<T>, NotificationCallback>(callback),
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
        name: Option<&NotificationName>,
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
    pub fn post(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "CFNotificationSuspensionBehavior")]
#[repr(isize)]
pub enum NotificationSuspensionBehavior {
    #[doc(alias = "kCFNotificationSuspensionBehaviorDrop")]
    Drop = 1,
    #[doc(alias = "kCFNotificationSuspensionBehaviorCoalesce")]
    Coalesce = 2,
    #[doc(alias = "kCFNotificationSuspensionBehaviorHold")]
    Hold = 3,
    #[doc(alias = "kCFNotificationSuspensionBehaviorDeliverImmediately")]
    DeliverImmediately = 4,
}

unsafe extern "C-unwind" {
    fn CFNotificationCenterGetTypeID() -> cf::TypeId;
    fn CFNotificationCenterGetLocalCenter<'a>() -> &'a mut NotificationCenter;
    fn CFNotificationCenterAddObserver(
        center: &mut NotificationCenter,
        observer: *const c_void,
        callback: NotificationCallback,
        name: Option<&NotificationName>,
        object: *const c_void,
        suspension_behavior: NotificationSuspensionBehavior,
    );
    fn CFNotificationCenterRemoveObserver(
        center: &mut NotificationCenter,
        observer: *const c_void,
        name: Option<&NotificationName>,
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

#[cfg(test)]
mod tests {
    use std::{ffi::c_void, sync::atomic};

    use crate::cf;

    unsafe extern "C" fn count_notification(
        _center: &cf::NotificationCenter,
        observer: *mut atomic::AtomicUsize,
        _name: &cf::NotificationName,
        _object: *const c_void,
        _user_info: Option<&cf::Dictionary>,
    ) {
        unsafe { &*observer }.fetch_add(1, atomic::Ordering::Relaxed);
    }

    #[test]
    fn typed_observer_receives_notification() {
        let center = cf::NotificationCenter::local();
        let name = cf::NotificationName::with_raw(cf::str!(c"cidre.test.notification"));
        let count = atomic::AtomicUsize::new(0);
        let observer = std::ptr::from_ref(&count).cast_mut();

        unsafe {
            center.add_observer(
                observer,
                count_notification,
                Some(name),
                std::ptr::null(),
                cf::notification_center::NotificationSuspensionBehavior::DeliverImmediately,
            );
        }
        center.post(name, std::ptr::null(), None, true);
        center.remove_observer(observer.cast(), Some(name), std::ptr::null());
        center.post(name, std::ptr::null(), None, true);

        assert_eq!(count.load(atomic::Ordering::Relaxed), 1);
    }
}
