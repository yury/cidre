use std::{ffi::c_void, ptr::NonNull, intrinsics::transmute, ops::Deref};

use crate::{
    cf::{self, Retained},
    define_cf_type, os,
};

define_cf_type!(Device(cf::Type));

#[repr(C)]
pub struct Notification(NonNull<c_void>);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct NotificationCallbackInfo {
    dev: *const Device,
    msg: MessageType,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum MessageType {
    Connected = 1,
    Disconneced = 2,
    Unsubscribed = 3,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum InterfaceType {
    Unknown = 0,
    DirectUSB = 1,
    InderectWiFi = 2,
    Proxy = 3,
}

pub type NotificationCallback = extern "C" fn(info: &NotificationCallbackInfo, arg: *mut c_void);
pub type MountCallback = extern "C" fn(info: &cf::Dictionary, ctx: *mut c_void);

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

    pub fn subscribe_with_options(
        callback: NotificationCallback,
        ctx: *mut c_void,
        options: &cf::Dictionary,
    ) -> Result<&'static Self, os::Status> {
        unsafe {
            let mut notification = None;
            AMDeviceNotificationSubscribeWithOptions(
                callback,
                0,
                0,
                ctx,
                &mut notification,
                options,
            )
            .to_result(notification)
        }
    }

    pub fn unsubscribe(&self) -> Result<(), os::Status> {
        unsafe { AMDeviceNotificationUnsubscribe(&self).result() }
    }
}

impl Device {
    pub fn connection_id(&self) -> u32 {
        unsafe { AMDeviceGetConnectionID(self) }
    }

    pub fn identifier<'a>(&self) -> Retained<'a, cf::String> {
        unsafe { AMDeviceCopyDeviceIdentifier(self) }
    }


    pub fn connected(&self) -> Result<Connected, os::Status> {
        unsafe { 
            AMDeviceConnect(&self).result()?;
            Ok(Connected(self))
        }
    }

    pub fn is_paired(&self) -> bool {
        unsafe { AMDeviceIsPaired(self).is_ok() }
    }

    pub fn pair(&self) -> Result<(), os::Status> {
        unsafe { AMDevicePair(self).result() }
    }

    pub fn validate_pairing(&self) -> Result<(), os::Status> {
        unsafe { AMDeviceValidatePairing(self).result() }
    }

    pub fn secure_install_application(&self, url: &cf::URL, options: &cf::Dictionary) -> Result<(), os::Status> {
        unsafe {
            AMDeviceSecureInstallApplication(0, self, url, options, std::ptr::null(), std::ptr::null()).result()
        }
    }

    pub fn secure_transfer_path(&self, url: &cf::URL, options: &cf::Dictionary) -> Result<(), os::Status> {
        unsafe {
            AMDeviceSecureTransferPath(0, self, url, options, std::ptr::null(), std::ptr::null()).result()
        }
    }

    pub fn interface_type(&self) -> InterfaceType {
        unsafe { AMDeviceGetInterfaceType(self) }
    }


    pub fn list<'a>() -> Retained<'a, cf::ArrayOf<Device>> {
        unsafe { AMDCreateDeviceList() }
    }
}

pub struct Connected<'a>(&'a Device);

impl<'a> Connected<'a> {


    /* Reads various device settings. One of domain or cfstring arguments should be NULL.
     *
     * ActivationPublicKey
     * ActivationState
     * ActivationStateAcknowledged
     * ActivityURL
     * BasebandBootloaderVersion
     * BasebandSerialNumber
     * BasebandStatus
     * BasebandVersion
     * BluetoothAddress
     * BuildVersion
     * CPUArchitecture
     * DeviceCertificate
     * DeviceClass
     * DeviceColor
     * DeviceName
     * DevicePublicKey
     * DieID
     * FirmwareVersion
     * HardwareModel
     * HardwarePlatform
     * HostAttached
     * IMLockdownEverRegisteredKey
     * IntegratedCircuitCardIdentity
     * InternationalMobileEquipmentIdentity
     * InternationalMobileSubscriberIdentity
     * iTunesHasConnected
     * MLBSerialNumber
     * MobileSubscriberCountryCode
     * MobileSubscriberNetworkCode
     * ModelNumber
     * PartitionType
     * PasswordProtected
     * PhoneNumber
     * ProductionSOC
     * ProductType
     * ProductVersion
     * ProtocolVersion
     * ProximitySensorCalibration
     * RegionInfo
     * SBLockdownEverRegisteredKey
     * SerialNumber
     * SIMStatus
     * SoftwareBehavior
     * SoftwareBundleVersion
     * SupportedDeviceFamilies
     * TelephonyCapability
     * TimeIntervalSince1970
     * TimeZone
     * TimeZoneOffsetFromUTC
     * TrustedHostAttached
     * UniqueChipID
     * UniqueDeviceID
     * UseActivityURL
     * UseRaptorCerts
     * Uses24HourClock
     * WeDelivered
     * WiFiAddress
     * // Updated by DiAifU 14.10.2010 for iOS5 and iTunes 5.0
     *
     * Possible values for domain:
     * com.apple.mobile.battery
     */
    pub unsafe fn copy_value<'b>(
        &self,
        domain: Option<&cf::String>,
        key: Option<&cf::String>,
    ) -> Option<Retained<'b, cf::Type>> {
        AMDeviceCopyValue(self.0, domain, key)
    }

    pub fn domain_value<'b>(&self, domain: &cf::String) -> Option<Retained<'b, cf::Type>> {
        unsafe { self.copy_value(Some(domain), None) }
    }

    pub fn value<'b>(&self, key: &cf::String) -> Option<Retained<'b, cf::Type>> {
        unsafe { self.copy_value(None, Some(key)) }
    }

    #[inline]
    pub fn name<'b>(&self) -> Retained<'b, cf::String> {
        let key = cf::String::from_str_no_copy("DeviceName");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn cpu_arch<'b>(&self) -> Retained<'b, cf::String> {
        let key = cf::String::from_str_no_copy("CPUArchitecture");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn hardware_model<'b>(&self) -> Retained<'b, cf::String> {
        let key = cf::String::from_str_no_copy("HardwareModel");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn product_name<'b>(&self) -> Retained<'b, cf::String> {
        let key = cf::String::from_str_no_copy("HardwareModel");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn product_type<'b>(&self) -> Retained<'b, cf::String> {
        let key = cf::String::from_str_no_copy("ProductType");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn product_version<'b>(&self) -> Retained<'b, cf::String> {
        let key = cf::String::from_str_no_copy("ProductVersion");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    pub fn start_session(&self) -> Result<(), os::Status> {
        unsafe { AMDeviceStartSession(self.0).result() }
    }
}

impl<'a> Drop for Connected<'a> {
    fn drop(&mut self) {
        unsafe { AMDeviceDisconnect(&self.0) };
    }
}

impl<'a> Deref for Connected<'a> {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

struct Session<'a>(Connected<'a>);

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        _ = unsafe { AMDeviceStopSession(&self.0) };
    }
}

impl<'a> Deref for Session<'a> {
    type Target = Connected<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    fn AMDCreateDeviceList<'a>() -> Retained<'a, cf::ArrayOf<Device>>;
    fn AMDeviceNotificationSubscribe(
        callback: NotificationCallback,
        unused0: u32,
        unused1: u32,
        context: *const c_void,
        notification: &mut Option<&Notification>,
    ) -> os::Status;

    fn AMDeviceNotificationSubscribeWithOptions(
        callback: NotificationCallback,
        unused0: u32,
        unused1: u32,
        context: *const c_void,
        notification: &mut Option<&Notification>,
        options: &cf::Dictionary,
    ) -> os::Status;

    fn AMDeviceNotificationUnsubscribe(notification: &Notification) -> os::Status;
    fn AMDeviceGetConnectionID(device: &Device) -> u32;
    fn AMDeviceCopyDeviceIdentifier<'a>(device: &Device) -> Retained<'a, cf::String>;
    fn AMDeviceCopyValue<'a>(
        device: &Device,
        domain: Option<&cf::String>,
        key: Option<&cf::String>,
    ) -> Option<Retained<'a, cf::Type>>;
    fn AMDeviceConnect(device: &Device) -> os::Status;
    fn AMDeviceDisconnect(device: &Device) -> os::Status;
    fn AMDeviceIsPaired(device: &Device) -> os::Status;
    fn AMDevicePair(device: &Device) -> os::Status;
    fn AMDeviceValidatePairing(device: &Device) -> os::Status;
    fn AMDeviceStartSession(device: &Device) -> os::Status;
    fn AMDeviceStopSession(device: &Device) -> os::Status;
    fn AMDeviceSecureInstallApplication(
        zero: i32,
        device: &Device,
        url: &cf::URL,
        options: &cf::Dictionary,
        callback: *const c_void,
        cbarg: *const c_void,
    ) -> os::Status;

    fn AMDeviceSecureTransferPath(
        zero: i32,
        device: &Device,
        url: &cf::URL,
        options: &cf::Dictionary,
        callback: *const c_void,
        cbarg: *const c_void,
    ) -> os::Status;

    fn AMDeviceGetInterfaceType(device: &Device) -> InterfaceType;
}

#[cfg(test)]
mod tests {
    use std::{ffi::c_void, intrinsics::transmute};

    use crate::{am, cf};

    use super::{Notification, NotificationCallbackInfo};

    extern "C" fn notification_callback(info: &NotificationCallbackInfo, arg: *mut c_void) {
        let dev = info.dev;
        let msg = info.msg;
        let dev: &am::Device = unsafe { transmute(dev) };
        let connected_dev = dev.connected().unwrap();
        println!(
            "msg: {:?} {:?}, {:?}",
            msg,
            dev.connection_id(),
            dev.identifier().to_string()
        );

        unsafe {
            let v = connected_dev.copy_value(None, None).unwrap();
            v.show();
            // assert!(v.is_none());
        }

        let key = cf::String::from_str("DeviceName");
        let v = connected_dev.value(&key).unwrap();
        v.show();
        dev.show();
    }

    #[test]
    pub fn list() {
        let list = am::Device::list();
        assert!(list.len() > 0);
        println!("interface type {:?}", list[0].interface_type());
    }

    #[test]
    pub fn notification_sub() {
        // let notification = Notification::subscribe(notification_callback, std::ptr::null_mut())
            // .expect("notification");

        // cf::RunLoop::run();

        // notification.unsubscribe().expect("unsub")
    }
}
