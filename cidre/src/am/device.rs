use std::{ffi::c_void, intrinsics::transmute, ops::Deref};
pub mod base;
pub mod development;
pub mod discovery;
pub mod error;
pub mod installation;
pub mod log;

pub use base::{Device, Error, Notification};
pub use discovery::{Action, IfaceConnectionType, QueryBuilder, Speed};

use crate::{arc, cf, os};

use self::base::ServiceConnection;

impl Device {
    pub fn connection_id(&self) -> u32 {
        unsafe { AMDeviceGetConnectionID(self) }
    }

    pub fn id(&self) -> arc::R<cf::String> {
        unsafe { AMDeviceCopyDeviceIdentifier(self) }
    }

    /// Connect to the mobile device.
    ///
    /// If you are already connected, this function will attempt to
    /// verify that the connection is still open.
    ///
    /// Returns Error::WRONG_DROID if the device is in the restore OS.
    pub fn connected(&self) -> Result<Connected, Error> {
        unsafe {
            AMDeviceConnect(self).result()?;
            Ok(Connected(self))
        }
    }

    /// Checks to see whether or not we are paired with the device.
    ///
    /// Only checks locally. May return true even if not paired. This may happen if the
    /// device has recently been restored. Use AMDeviceValidatePair() to confer with device
    /// about pair-ed-ness. Can be done without having a session.
    ///
    /// FOR ALMOST ALL USAGES, am::Device::validate_pairing() IS BETTER!
    ///
    pub fn is_paired(&self) -> bool {
        unsafe { AMDeviceIsPaired(self) == 1 }
    }

    /// Create a pairing relationship with an iOS device
    /// This is equivalent to calling AMDevicePairWithOptions with options set to NULL
    pub fn pair(&self) -> Result<(), Error> {
        unsafe { AMDevicePair(self).result() }
    }

    /// Validate the pairing with the device.
    ///
    /// Checks to see if the host and device are paired. Prefer this to am::Device::is_paired().
    /// On success, the device will also be notified that it is attached to a Trusted Host.
    pub fn validate_pairing(&self) -> Result<(), Error> {
        unsafe { AMDeviceValidatePairing(self).result() }
    }

    pub fn secure_install_app(&self, url: &cf::Url, options: &cf::Dictionary) -> os::Result {
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

    #[inline]
    pub fn secure_transfer_path(&self, url: &cf::Url, options: &cf::Dictionary) -> os::Result {
        unsafe {
            AMDeviceSecureTransferPath(0, self, url, options, std::ptr::null(), std::ptr::null())
                .result()
        }
    }

    #[inline]
    pub fn iface_type(&self) -> IfaceConnectionType {
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
    pub unsafe fn copy_value(
        &self,
        domain: Option<&cf::String>,
        key: Option<&cf::String>,
    ) -> Option<arc::R<cf::Type>> {
        unsafe { AMDeviceCopyValue(self.0, domain, key) }
    }

    ///
    /// Copy a value from the device
    /// @param device The device to copy from
    /// @param domain The domain to query. May be NULL.
    /// @param key The key to query. May be NULL.
    /// @param error_out On return, an error code describing the result of the operation. May be NULL.
    /// @result A new plist value, or NULL.
    ///
    /// Copies a value from the lockdown property store. A key argument of NULL asks for the
    /// contents of the whole domain. A domain argument of NULL asks for the global domain.
    /// Some properties are unavailable outside of a session.
    ///
    /// Returns kAMDDeviceDisconnectedError is the device is no longer attached.
    /// Returns kAMDInvalidArgumentError if device is not a valid device ref, or if domain or key arguments are
    /// non-NULL and not string.
    ///
    pub unsafe fn copy_value_with_err(
        &self,
        domain: Option<&cf::String>,
        key: Option<&cf::String>,
        error_out: &mut Error,
    ) -> Option<arc::R<cf::Plist>> {
        unsafe { AMDeviceCopyValueWithError(self, domain, key, error_out) }
    }

    pub fn try_value(
        &self,
        domain: Option<&cf::String>,
        key: Option<&cf::String>,
    ) -> Result<arc::R<cf::Plist>, Error> {
        let mut error_out = Error::SUCCESS;
        unsafe {
            let value = self.copy_value_with_err(domain, key, &mut error_out);
            if error_out.is_err() {
                Err(error_out)
            } else {
                Ok(value.unwrap_unchecked())
            }
        }
    }

    pub fn domain_value(&self, domain: &cf::String) -> Option<arc::R<cf::Type>> {
        unsafe { self.copy_value(Some(domain), None) }
    }

    pub fn value(&self, key: &cf::String) -> Option<arc::R<cf::Type>> {
        unsafe { self.copy_value(None, Some(key)) }
    }

    #[inline]
    pub fn name(&self) -> arc::R<cf::String> {
        let key = cf::String::from_str("DeviceName");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn cpu_arch(&self) -> arc::R<cf::String> {
        let key = cf::String::from_str("CPUArchitecture");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn hardware_model(&self) -> arc::R<cf::String> {
        let key = cf::String::from_str("HardwareModel");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn product_name(&self) -> arc::R<cf::String> {
        let key = cf::String::from_str("HardwareModel");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn product_type(&self) -> arc::R<cf::String> {
        let key = cf::String::from_str("ProductType");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    #[inline]
    pub fn product_version(&self) -> arc::R<cf::String> {
        let key = cf::String::from_str("ProductVersion");
        let v = self.value(&key);
        unsafe { transmute(v) }
    }

    /// Start a session with the device.
    ///
    /// You must have paired with the device before you can start a session.
    /// A return value of kAMDInvalidPairRecordError is possible if the pair records have
    /// been damaged. In this case the pairing records will be discarded and the
    /// device connection will be shut down.
    ///
    /// To recover: AMDeviceConnect(), AMDeviceUnpair(), AMDevicePair() and try to start
    /// a session again.
    pub fn start_session(&self) -> Result<Session, Error> {
        self.validate_pairing()?;
        unsafe {
            match AMDeviceStartSession(self.0).result() {
                Err(e) => Err(e),
                Ok(()) => Ok(Session(self)),
            }
        }
    }
}

impl<'a> Drop for Connected<'a> {
    fn drop(&mut self) {
        eprintln!("disconnect");
        unsafe { AMDeviceDisconnect(self.0) };
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
    /// Securely start a service on the device specifying options.
    ///
    /// Starts a service on the device. Requires that a session with the device be active.
    /// Attempting to start a service without an active session will result in kAMDInvalidArgumentError.
    /// Fails with kAMDNoWifiSyncSupportError if the device side service does not support SSL.
    /// Fails with kAMDServiceProhibitedError if the service is not allowed to run. Some services are
    /// only allowed to run when a device has been activated.
    ///
    /// The device may request that a connection be encrypted. If so, this call will also perform
    /// the initial SSL handshake. See <MobileDevice/AMDServiceConnection.h> for more information
    /// on using the service connection.
    ///
    /// # Options
    ///
    /// kAMDServiceOptionTimeoutConnection - The service connection will have SO_SNDTIMEO and
    /// SO_RCVTIMEO set. It will be closed if either input or output operations fail to
    /// complete within 25 seconds (use carefully if you expect your service to perform
    /// long-lived operations on your behalf). Valid values are boolean true or false.
    /// This is off by default.
    ///
    /// kAMDServiceOptionUnlockEscrowBag - Attempt the passcode-unlock the device when starting
    /// a service.
    ///
    /// kAMDOptionCloseOnInvalidate - The returned service connection will take ownership of
    /// the underlying file descriptor and SSL context. Manually closing the fd or SSL_free()'ing
    /// the ssl context will result in an double free()s and close()s when the service connection
    /// object is invalidate or ultimately released. Valid values are boolean true or false.
    /// This is on by default.
    ///
    /// On success, the AMDServiceConnection returned in service_out must be CFRelease()'d by the caller.
    pub fn secure_start_service(
        &self,
        name: &cf::String,
    ) -> Result<arc::R<ServiceConnection>, Error> {
        unsafe {
            let mut service = None;
            AMDeviceSecureStartService(self, name, std::ptr::null_mut(), &mut service)
                .to_result(service)
        }
    }

    pub fn start_debug_server(&self) -> Result<arc::R<ServiceConnection>, Error> {
        let name = cf::str!(c"com.apple.debugserver.DVTSecureSocketProxy");
        self.secure_start_service(name)
    }

    pub fn battery_level(&self) -> Option<arc::R<cf::Number>> {
        let domain: arc::R<_> = "com.apple.mobile.battery".into();
        let key: arc::R<_> = "BatteryCurrentCapacity".into();
        unsafe { transmute(self.copy_value(Some(&domain), Some(&key))) }
    }
}

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        _ = unsafe { AMDeviceStopSession(self) };
    }
}

impl<'a> Deref for Session<'a> {
    type Target = Connected<'a>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[link(name = "MobileDevice", kind = "framework")]
unsafe extern "C" {
    fn AMDeviceGetConnectionID(device: &Device) -> u32;
    fn AMDeviceCopyDeviceIdentifier(device: &Device) -> arc::R<cf::String>;
    fn AMDeviceCopyValue(
        device: &Device,
        domain: Option<&cf::String>,
        key: Option<&cf::String>,
    ) -> Option<arc::R<cf::Type>>;
    fn AMDeviceConnect(device: &Device) -> Error;
    fn AMDeviceDisconnect(device: &Device) -> os::Status;
    fn AMDeviceIsPaired(device: &Device) -> u32;
    fn AMDevicePair(device: &Device) -> Error;
    fn AMDeviceValidatePairing(device: &Device) -> Error;
    fn AMDeviceStartSession(device: &Device) -> Error;
    fn AMDeviceStopSession(device: &Device) -> os::Status;
    fn AMDeviceSecureInstallApplication(
        zero: i32,
        device: &Device,
        url: &cf::Url,
        options: &cf::Dictionary,
        callback: *const c_void,
        cbarg: *const c_void,
    ) -> os::Status;

    fn AMDeviceSecureTransferPath(
        zero: i32,
        device: &Device,
        url: &cf::Url,
        options: &cf::Dictionary,
        callback: *const c_void,
        cbarg: *const c_void,
    ) -> os::Status;

    fn AMDeviceGetInterfaceType(device: &Device) -> IfaceConnectionType;

    fn AMDeviceSecureStartService(
        device: &Device,
        service_name: &cf::String,
        ssl: *mut c_void,
        service: *mut Option<arc::R<ServiceConnection>>,
    ) -> Error;

    fn AMDeviceCopyValueWithError(
        device: &Device,
        domain: Option<&cf::String>,
        key: Option<&cf::String>,
        error_out: &mut Error,
    ) -> Option<arc::R<cf::Plist>>;

    // fn AMDeviceCopyDeveloperModeStatus(device: &Device, err: &mut Error) -> bool;
    // fn AMDServiceConnectionGetSocket(service: &Service) -> os::Status;

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
}
