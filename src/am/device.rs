use std::{ffi::c_void, intrinsics::transmute, ops::Deref, path::PathBuf, ptr::NonNull};
pub mod base;
pub mod discovery;
pub mod error;

pub use base::{Device, Error, Notification};
pub use discovery::{Action, InterfaceConnectionType, Speed};

use crate::{
    cf::{self, Retained},
    define_cf_type, os,
};

// #[repr(C)]
// pub struct Notification(NonNull<c_void>);

// #[repr(C, packed)]
// #[derive(Copy, Clone, Debug)]
// pub struct NotificationCallbackInfo {
//     dev: *const Device,
//     msg: MessageType,
// }

// #[repr(u32)]
// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// #[non_exhaustive]
// pub enum MessageType {
//     Connected = 1,
//     Disconneced = 2,
//     Unsubscribed = 3,
// }

// #[repr(u32)]
// #[derive(Copy, Clone, Debug)]
// #[non_exhaustive]
// pub enum InterfaceType {
//     Unknown = 0,
//     DirectUSB = 1,
//     InderectWiFi = 2,
//     Proxy = 3,
// }

// pub type NotificationCallback<T> = extern "C" fn(info: &NotificationCallbackInfo, arg: *mut T);
pub type MounImageCallback<T> = extern "C" fn(info: &cf::Dictionary, ctx: *mut T);
//pub type MountCallback = extern "C" fn(info: &cf::Dictionary, ctx: *mut c_void);

// impl Notification {
//     pub fn subscribe<T>(
//         callback: NotificationCallback<T>,
//         ctx: *mut T,
//     ) -> Result<&'static Self, os::Status> {
//         unsafe {
//             let mut notification = None;
//             AMDeviceNotificationSubscribe(
//                 transmute(callback),
//                 0,
//                 0,
//                 transmute(ctx),
//                 &mut notification,
//             )
//             .to_result(notification)
//         }
//     }

//     pub fn subscribe_with_options<T>(
//         callback: NotificationCallback<T>,
//         ctx: *mut T,
//         options: &cf::Dictionary,
//     ) -> Result<&'static Self, os::Status> {
//         unsafe {
//             let mut notification = None;
//             AMDeviceNotificationSubscribeWithOptions(
//                 transmute(callback),
//                 0,
//                 0,
//                 transmute(ctx),
//                 &mut notification,
//                 options,
//             )
//             .to_result(notification)
//         }
//     }

//     pub fn unsubscribe(&self) -> Result<(), os::Status> {
//         unsafe { AMDeviceNotificationUnsubscribe(&self).result() }
//     }
// }

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

    pub fn secure_install_application(
        &self,
        url: &cf::URL,
        options: &cf::Dictionary,
    ) -> Result<(), os::Status> {
        unsafe {
            AMDeviceSecureInstallApplication(
                0,
                self,
                url,
                options,
                std::ptr::null(),
                std::ptr::null(),
            )
            .result()
        }
    }

    pub fn secure_transfer_path(
        &self,
        url: &cf::URL,
        options: &cf::Dictionary,
    ) -> Result<(), os::Status> {
        unsafe {
            AMDeviceSecureTransferPath(0, self, url, options, std::ptr::null(), std::ptr::null())
                .result()
        }
    }

    pub fn interface_type(&self) -> InterfaceConnectionType {
        unsafe { AMDeviceGetInterfaceType(self) }
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

    pub fn start_session(&self) -> Result<Session, os::Status> {
        unsafe {
            match AMDeviceStartSession(self.0).result() {
                Err(e) => Err(e),
                Ok(()) => Ok(Session(self)),
            }
        }
    }

    pub fn device_support_path(&self) -> Option<PathBuf> {
        let version = self.product_version().to_string();
        platform_support_path("iPhoneOS.platform", &version)
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

pub struct Session<'a>(&'a Connected<'a>);

impl<'a> Session<'a> {
    pub fn secure_start_service<'b>(
        &self,
        name: &cf::String,
    ) -> Result<Retained<'b, Service>, os::Status> {
        unsafe {
            let mut service = None;
            AMDeviceSecureStartService(self, name, std::ptr::null(), &mut service)
                .to_result(service)
        }
    }

    pub fn start_debug_server<'b>(&self) -> Result<Retained<'b, Service>, os::Status> {
        let name = cf::String::from_str_no_copy("com.apple.debugserver");
        self.secure_start_service(&name)
    }

    pub fn mound_disk(
        &self,
        image: &cf::String,
        options: &cf::Dictionary,
    ) -> Result<(), os::Status> {
        unsafe {
            extern "C" fn mount_callback(info: &cf::Dictionary, _ctx: *mut c_void) {
                println!("!!!!!!");
                info.show();
            }
            AMDeviceMountImage(
                self,
                image,
                options,
                mount_callback as _,
                std::ptr::null_mut(),
            )
            .result()
        }
    }

    pub unsafe fn mound_disk_with_callback<T>(
        &self,
        image: &cf::String,
        options: &cf::Dictionary,
        callback: MounImageCallback<T>,
        ctx: *mut T,
    ) -> os::Status {
        AMDeviceMountImage(self, image, options, transmute(callback), transmute(ctx))
    }

    pub fn mount_developer_image(&self) -> Result<(), os::Status> {
        let ds_path = self.device_support_path();
        if ds_path.is_none() {
            return Err(os::Status(-1));
        }
        let ds_path = ds_path.unwrap();
        let image_path = ds_path.join("DeveloperDiskImage.dmg");
        let sig_image_path = ds_path.join("DeveloperDiskImage.dmg.signature");
        let sig = std::fs::read(sig_image_path).expect("sig file read");
        let sig = Retained::from(&sig[..]);

        let image_type_key = cf::String::from_str("ImageType");
        let image_type_value = cf::String::from_str("Developer");
        let image_sig_key = cf::String::from_str("ImageSignature");
        let options = cf::Dictionary::with_keys_values(
            &[&image_type_key, &image_sig_key],
            &[&image_type_value, &sig],
        )
        .expect("options for mount created");

        image_type_key.show();
        options.show();
        sig.show();

        let path = image_path.to_str().unwrap();
        let ref cf_image_path = cf::String::from_str_no_copy(&path);
        self.mound_disk(&cf_image_path, &options)
    }
}

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

define_cf_type!(Service(cf::Type));

fn xcode_dev_path() -> PathBuf {
    use std::process::Command;
    let command = Command::new("xcode-select")
        .arg("-print-path")
        .output()
        .expect("xcode-select prints path");
    String::from_utf8(command.stdout)
        .expect("valid utf-8 output from xcode-select command")
        .trim()
        .into()
}

fn platform_support_path(platform: &str, os_version: &str) -> Option<PathBuf> {
    let prefix = xcode_dev_path()
        .join("Platforms")
        .join(platform)
        .join("DeviceSupport");
    let version: String = os_version
        .splitn(3, '.')
        .take(2)
        .collect::<Vec<_>>()
        .join(".")
        .into();

    for directory in std::fs::read_dir(&prefix).expect("folder exists") {
        let directory = directory.expect("folder exists");
        let name = directory.file_name().into_string().expect("valid string");
        if name.starts_with(&version) {
            return Some(prefix.join(name));
        }
    }

    None
}

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    // fn AMDCreateDeviceList<'a>() -> Retained<'a, cf::ArrayOf<Device>>;
    // fn AMDeviceNotificationSubscribe(
    //     callback: NotificationCallback<c_void>,
    //     unused0: u32,
    //     unused1: u32,
    //     context: *const c_void,
    //     notification: &mut Option<&Notification>,
    // ) -> os::Status;

    // fn AMDeviceNotificationSubscribeWithOptions(
    //     callback: NotificationCallback<c_void>,
    //     unused0: u32,
    //     unused1: u32,
    //     context: *const c_void,
    //     notification: &mut Option<&Notification>,
    //     options: &cf::Dictionary,
    // ) -> os::Status;

    // fn AMDeviceNotificationUnsubscribe(notification: &Notification) -> os::Status;
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

    fn AMDeviceGetInterfaceType(device: &Device) -> InterfaceConnectionType;

    fn AMDeviceSecureStartService<'a>(
        device: &Device,
        service_name: &cf::String,
        unknwon: *const c_void,
        service: &Option<Retained<'a, Service>>,
    ) -> os::Status;
    fn AMDServiceConnectionGetSocket(service: &Service) -> os::Status;

    fn AMDeviceMountImage(
        device: &Device,
        image: &cf::String,
        options: &cf::Dictionary,
        callback: *const MounImageCallback<c_void>,
        ctx: *mut c_void,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {
    // use std::ffi::c_void;

    // use crate::{
    //     am::{self, device::MessageType},
    //     cf,
    // };

    // use super::{Notification, NotificationCallbackInfo};

    // extern "C" fn notification_callback(info: &NotificationCallbackInfo, _arg: *mut c_void) {
    //     let dev = info.dev;
    //     let msg = info.msg;
    //     println!("message: {:?}", msg);
    //     if msg != MessageType::Connected {
    //         return;
    //     }
    //     let dev: &am::Device = unsafe { &*dev };
    //     let connected_dev = dev.connected().unwrap();
    //     println!(
    //         "{:?}, {:?}",
    //         dev.connection_id(),
    //         dev.identifier().to_string()
    //     );

    //     unsafe {
    //         let v = connected_dev.copy_value(None, None).unwrap();
    //         v.show();
    //         // assert!(v.is_none());
    //     }

    //     let key = cf::String::from_str("DeviceName");
    //     let v = connected_dev.value(&key).unwrap();
    //     v.show();
    //     dev.show();
    // }

    // #[test]
    // pub fn list() {
    //     let list = am::Device::list();
    //     assert!(list.len() > 0);
    //     let device = &list[0];
    //     println!("interface type {:?}", device.interface_type());
    // }

    // #[test]
    // pub fn connected() {
    //     let list = am::Device::list();
    //     assert!(list.len() > 0);
    //     let device = &list[0];
    //     let connected_device = device.connected().expect("connected device");
    //     let device_support_path = connected_device.device_support_path();
    //     println!(
    //         "product version {:?} {:?}",
    //         connected_device.product_version().to_string(),
    //         device_support_path
    //     );

    //     if let Some(path) = device_support_path {
    //         let session = connected_device.start_session().unwrap();
    //         session.mount_developer_image().expect("mounted");
    //     }

    // }

    // #[test]
    // pub fn notification_sub() {
    //     let notification = Notification::subscribe(notification_callback, std::ptr::null_mut())
    //         .expect("notification");

    //     // cf::RunLoop::run();

    //     notification.unsubscribe().expect("unsub")
    // }
}
