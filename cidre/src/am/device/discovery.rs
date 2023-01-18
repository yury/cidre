use std::{ffi::c_void, intrinsics::transmute};

use crate::{arc, cf};

use super::base::{Device, Error, Notification};

///
/// The interface connection type.  Pass ONE and ONLY ONE of these to AMDeviceNotificationSubscribe(WithOptions).  Not a bitfield (unfortunately).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[repr(i32)]
pub enum InterfaceConnectionType {
    Invalid = -1,
    Any = 0,
    /// e.g. USB, Firewire, Bluetooth pairing
    Direct = 1,
    /// e.g. Ethernet, 802.11g or other network
    Inderect = 2,
    /// This can be returned from AMDeviceGetInterfaceType, but should not be passed to AMDeviceNotificationSubscribe(WithOptions).
    /// e.g. Connection to this device is proxied through a paired companion device
    Proxied = 3,
}

/// Various interface connection speeds
/// in kilobits per second.
#[repr(transparent)]
pub struct Speed(pub i32);

impl Speed {
    pub const ANY: Self = Self(0);
    pub const USB_LOW_SPEED: Self = Self(3 * 512);
    pub const USB_FULL_SPEED: Self = Self(12 * 1024);
    pub const USB_HIGH_SPEED: Self = Self(480 * 1024);
    pub const FIREWIRE_400: Self = Self(400 * 1024);
    pub const FIREWIRE_800: Self = Self(800 * 1024);
    pub const _10_BASE_T: Self = Self(10 * 1024);
    pub const _100_BASE_T: Self = Self(100 * 1024);
    pub const GIGABIT: Self = Self(1024 * 1024);
    pub const _80211B: Self = Self(11 * 1024);
    pub const _80211G: Self = Self(54 * 1024);
    pub const _80211N: Self = Self(540 * 1024);
    pub const BLUETOOTH1: Self = Self(1024);
    pub const BLUETOOTH2: Self = Self(21 * 1024);
}

/*
 * Device Action
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[repr(i32)]
pub enum Action {
    /// A device has attached. The device reference belongs to the
    /// client. It must be explicitly released, or else it will leak.
    Attached = 1,
    /// A device has detached. The device object delivered will be
    /// the same as the one delivered in the Attached notification. This
    /// device reference does not need to be released.
    Detached = 2,

    /// This notification is delivered in response to
    ///
    ///   1. A call to am::DeviceNotificationUnsubscribe().
    ///   2. An error occurred on one of the underlying notification systems
    ///     (i.e. usbmuxd or mDNSResponder crashed or stopped responding).
    ///     Unsubcribing and resubscribing may recover the notification system.
    NotificationStopped = 3,

    Paired = 4,
}

#[repr(C)]
pub struct NotificationInfo {
    pub device: *const Device, // test for strong
    pub action: Action,
    pub notification: &'static Notification,
}

pub enum SafeInfo<'a> {
    Attached(arc::R<Device>),
    Detached(&'a Device),
    NotificationStopped,
    Paired(&'a Device),
}

impl NotificationInfo {
    pub fn safe<'a>(&self) -> SafeInfo<'a> {
        match self.action {
            Action::Attached => SafeInfo::Attached(unsafe { transmute(self.device) }),
            Action::Detached => SafeInfo::Detached(unsafe { &*self.device }),
            Action::NotificationStopped => SafeInfo::NotificationStopped,
            Action::Paired => SafeInfo::Paired(unsafe { &*self.device }),
        }
    }
}

///
/// Notification callback. Ownership of the notification info struct *info* DOES NOT
/// pass to the callback function.
///
/// The Action field of the 'info' parameter describes the notification being sent.
///
/// - kAMDeviceAttached: A device has attached. The device reference belongs to the
///   client. It must be explicitly released, or else it will leak.
/// - kAMDeviceDetached: A device has detached. The device object delivered will be
///   the same as the one delivered in the kAMDeviceAttached notification. This
///   device reference does not need to be released.
/// - kAMDeviceNotificationStopped: This notification is delivered in response to
///   1. A call to AMDeviceNotificationUnsubscribe().
///   2. An error occurred on one of the underlying notification systems
///     (i.e. usbmuxd or mDNSResponder crashed or stopped responding).
///     Unsubcribing and resubscribing may recover the notification system.
///
pub type NotificationCallback<T> = extern "C" fn(info: &NotificationInfo, context: *mut T);

impl Device {
    /// Get a list of currently attached devices.
    /// An array of AMDeviceRefs on success or NULL on failure.
    ///
    /// Synchronously queries for and returns a list of the currently connected devices as
    /// an array of AMDeviceRefs. Devices may be disconnected at any time at which will
    /// cause the corresponding AMDeviceRef to become invalid. If no devices are attached,
    /// returned array will have zero length. Ownership follows the Create Rule.
    ///
    /// AMDCopyArrayOfDevicesMatchingQuery() is similar.
    ///
    /// To deal with devices dynamically coming and going, use AMDeviceNotificationSubscribe() instead.
    ///
    pub fn list() -> Option<arc::R<cf::ArrayOf<Device>>> {
        unsafe { AMDCreateDeviceList() }
    }

    /// use am::DeviceQueryBuilder
    pub unsafe fn copy_array_of_devices_matching_query(
        note: Option<&Notification>,
        query: &cf::Dictionary,
        out_array: *mut Option<arc::R<cf::ArrayOf<Device>>>,
    ) -> Error {
        AMDCopyArrayOfDevicesMatchingQuery(note, query, out_array)
    }
}

pub struct QueryBuilder {
    query: arc::Retained<cf::DictionaryMut>,
}

impl QueryBuilder {
    pub fn new_match_all() -> Self {
        Self::new(&matching::mode::all_value())
    }
    pub fn new_match_any() -> Self {
        Self::new(&matching::mode::any_value())
    }
    pub fn new_match_wildcard() -> Self {
        Self::new(&matching::mode::wildcard_value())
    }

    pub fn new(matching_mode: &cf::String) -> Self {
        let mut query = cf::DictionaryMut::with_capacity(3);

        query.insert(&matching::mode::key(), matching_mode);

        Self { query }
    }

    pub fn udids(&mut self, udids: &[&str]) -> &mut Self {
        let mut array = cf::ArrayMut::with_capacity(udids.len() as _);
        for u in udids {
            let s = cf::String::from_str(u);
            array.append(&s);
        }
        self.query.insert(&matching::criteria::udid_key(), &array);
        self
    }

    pub fn connection(&mut self, connection_type: InterfaceConnectionType) -> &mut Self {
        let value = match connection_type {
            InterfaceConnectionType::Invalid | InterfaceConnectionType::Any => {
                self.query
                    .remove(&matching::criteria::connection_type_key());
                return self;
            }
            InterfaceConnectionType::Direct => matching::criteria::usb_value(),
            InterfaceConnectionType::Inderect => matching::criteria::network_value(),
            InterfaceConnectionType::Proxied => matching::criteria::paired_device_value(),
        };

        self.query
            .insert(&matching::criteria::connection_type_key(), &value);
        self
    }

    pub fn matching_list<'a>(
        &self,
        note: Option<&Notification>,
    ) -> Result<arc::R<cf::ArrayOf<Device>>, Error> {
        let mut out_array = None;
        unsafe {
            let query = self.query.copy();
            Device::copy_array_of_devices_matching_query(note, &query, &mut out_array)
                .to_result(out_array)
        }
    }
}

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    fn AMDCreateDeviceList() -> Option<arc::R<cf::ArrayOf<Device>>>;
    fn AMDCopyArrayOfDevicesMatchingQuery<'a>(
        note: Option<&Notification>,
        query: &cf::Dictionary,
        out_array: *mut Option<arc::R<cf::ArrayOf<Device>>>,
    ) -> Error;
}

impl Notification {
    pub unsafe fn subscribe<T>(
        callback: NotificationCallback<T>,
        minimum_interface_speed: Speed,
        connection_type: InterfaceConnectionType,
        context: *mut T,
        ref_out: &mut Option<arc::R<Notification>>,
    ) -> Error {
        AMDeviceNotificationSubscribe(
            transmute(callback),
            minimum_interface_speed,
            connection_type,
            transmute(context),
            ref_out,
        )
    }

    pub fn with<'a, T>(
        callback: NotificationCallback<T>,
        minimum_interface_speed: Speed,
        connection_type: InterfaceConnectionType,
        context: *mut T,
    ) -> Result<SubscriptionGuard, Error> {
        let mut notification = None;
        unsafe {
            let res = Self::subscribe(
                callback,
                minimum_interface_speed,
                connection_type,
                context,
                &mut notification,
            );
            if res.is_ok() {
                Ok(SubscriptionGuard(notification))
            } else {
                Err(res)
            }
        }
    }

    pub unsafe fn unsubscribe(&self) -> Error {
        AMDeviceNotificationUnsubscribe(self)
    }
}

pub struct SubscriptionGuard(Option<arc::R<Notification>>);

impl SubscriptionGuard {
    pub fn note(&self) -> Option<&Notification> {
        self.0.as_deref()
    }
}

impl Drop for SubscriptionGuard {
    fn drop(&mut self) {
        // AMDeviceNotificationUnsubscribe decrease ref count.
        // So we need to do trick here
        if let Some(r) = self.0.take() {
            let res = unsafe { r.unsubscribe() };
            if res.is_err() {
                #[cfg(debug_assertions)]
                eprintln!("error: {res:?}");
                // put it back
                self.0 = Some(r)
            } else {
                std::mem::forget(r)
            }
        }
    }
}

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    fn AMDeviceNotificationSubscribe(
        callback: NotificationCallback<c_void>,
        minimum_interface_speed: Speed,
        connection_type: InterfaceConnectionType,
        context: *mut c_void,
        ref_out: &mut Option<arc::R<Notification>>,
    ) -> Error;

    fn AMDeviceNotificationUnsubscribe(notification: &Notification) -> Error;

}

pub mod matching {
    pub mod mode {
        use crate::{arc, cf};

        /// This key determines how the matching works. (Required)
        #[inline]
        pub fn key() -> arc::R<cf::String> {
            "MatchingMode".into()
        }

        /// If a device matches ANY of the criteria it will be part of the returned array.
        #[inline]
        pub fn any_value() -> arc::R<cf::String> {
            "MatchAny".into()
        }

        /// Only if a device matches ALL of the criteria will it be part of the returned array.
        #[inline]
        pub fn all_value() -> arc::R<cf::String> {
            "MatchAll".into()
        }

        /// Ignore all criteria, just return all devices.
        #[inline]
        pub fn wildcard_value() -> arc::R<cf::String> {
            "MatchWildcard".into()
        }
    }

    pub mod criteria {
        use crate::{arc, cf};

        /// Value is an array of CFStrings of device UDIDs, as returned
        /// by AMDeviceCopyDeviceIdentifier(). Case IN-sensitive.
        #[inline]
        pub fn udid_key() -> arc::R<cf::String> {
            "MatchUDID".into()
        }

        /// Value must be either kAMDCriteriaUSBKey or kAMDCriteriaNetworkKey.
        #[inline]
        pub fn connection_type_key() -> arc::R<cf::String> {
            "MatchConnectionType".into()
        }

        #[inline]
        pub fn usb_value() -> arc::R<cf::String> {
            "MatchConnectionTypeUSB".into()
        }

        #[inline]
        pub fn network_value() -> arc::R<cf::String> {
            "MatchConnectionTypeNetwork".into()
        }

        #[inline]
        pub fn paired_device_value() -> arc::R<cf::String> {
            "MatchConnectionTypePairedDevice".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use crate::{
        am::{
            self,
            device::discovery::{matching, NotificationInfo, SafeInfo},
        },
        cf,
    };

    #[test]
    fn notification_drop() {
        extern "C" fn callback(info: &NotificationInfo, _context: *mut c_void) {
            match info.safe() {
                SafeInfo::Attached(device) => {
                    println!("attached");
                    println!("{:?}", device.connected().unwrap().name().to_string());
                    device.show()
                }
                SafeInfo::Detached(device) => {
                    println!("detached");
                    device.show()
                }
                SafeInfo::NotificationStopped => {
                    println!("stopped")
                }
                SafeInfo::Paired(device) => {
                    println!("paired");
                    device.show()
                }
            }
        }

        let _subscription = am::DeviceNotification::with(
            callback,
            am::DeviceSpeed::ANY,
            am::DeviceInterfaceConnectionType::Inderect,
            std::ptr::null_mut(),
        )
        .unwrap();

        cf::RunLoop::run_in_mode(cf::RunLoopMode::default(), 0.5, false);
    }

    #[test]
    fn filters() {
        let list = am::DeviceQueryBuilder::new(&matching::mode::wildcard_value())
            // .connection(am::DeviceInterfaceConnectionType::Direct)
            .matching_list(None)
            .unwrap();

        list.show();
    }
}
