use std::{ffi::c_void, os::raw::c_char, ptr::NonNull};

use crate::{cf, define_cf_type, mach, os};

pub const IPHONE_PRODUCT_ID: u32 = 0x1290;

define_cf_type!(Device(cf::Type));

#[repr(C)]
pub struct Notification(NonNull<c_void>);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct NotificationCallbackInfo {
    dev: *const Device,
    msg: u32,
}

pub type NotificationCallback = extern "C" fn(info: &NotificationCallbackInfo, arg: *mut c_void);

impl Notification {
    pub fn subscribe(
        callback: NotificationCallback,
        ctx: *mut c_void,
    ) -> Result<&'static Self, os::Status> {
        unsafe {
            let mut notification = None;
            AMDeviceNotificationSubscribe(callback, 0, 0, ctx, &mut notification)
                .to_result(notification)
        }
    }

    pub fn unsubscribe(&self) -> Result<(), os::Status> {
        unsafe { AMDeviceNotificationUnsubscribe(&self).result() }
    }
}

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    fn AMDeviceNotificationSubscribe(
        callback: NotificationCallback,
        unused0: u32,
        unused1: u32,
        dn_unknown3: *const c_void,
        notification: &mut Option<&Notification>,
    ) -> os::Status;

    fn AMDeviceNotificationSubscribeWithOptions(
        callback: NotificationCallback,
        unused0: u32,
        unused1: u32,
        dn_unknown3: *const c_void,
        notification: &mut Option<&Notification>,
        options: &cf::Dictionary,
    ) -> os::Status;

    fn AMDeviceNotificationUnsubscribe(notification: &Notification) -> os::Status;
}

#[cfg(test)]
mod tests {
    use std::{ffi::c_void, intrinsics::transmute, thread::sleep, time::Duration};

    use crate::{am, cf};

    use super::{Notification, NotificationCallbackInfo};

    extern "C" fn notification_callback(info: &NotificationCallbackInfo, arg: *mut c_void) {
        let dev = info.dev;
        let msg = info.msg;
        println!("msg: {:?}", msg);
        let dev_ref: &am::Device = unsafe { transmute(dev) };
        dev_ref.show();
    }

    #[test]
    pub fn notification_sub() {
        let notification = Notification::subscribe(notification_callback, std::ptr::null_mut())
            .expect("notification");

        // cf::RunLoop::run();

        notification.unsubscribe().expect("unsub")
    }
}
