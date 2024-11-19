use crate::{api, arc, av, blocks, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVExternalStorageDevice")]
    pub Device(ns::Id)
);

impl Device {
    #[api::available(macos = 14.0, maccatalyst = 17.0, ios = 17.0, tvos = 17.0)]
    crate::define_cls!(AV_EXTERNAL_STORAGE_DEVICE);

    /// Display name of the external storage device.
    ///
    /// This property can be used for displaying the name of an
    /// external storage device in a user interface. Will return None
    /// if we fail to extract information from external storage device.
    #[objc::msg_send(displayName)]
    pub fn display_name(&self) -> Option<arc::R<ns::String>>;

    /// Current free size in bytes.
    ///
    /// This property represents the free size available on the external storage device.
    /// Will return -1 if we fail to extract information from external storage device.
    #[objc::msg_send(freeSize)]
    pub fn free_size(&self) -> isize;

    /// Total storage size in bytes.
    ///
    /// This property represents the total storage size available on the external storage device.
    /// Will return -1 if we fail to extract information from external storage device.
    #[objc::msg_send(totalSize)]
    pub fn total_size(&self) -> isize;

    /// Indicates whether the external storage device is connected and available to the system.
    #[objc::msg_send(isConnected)]
    pub fn is_connected(&self) -> bool;

    /// A unique identifier for external storage device.
    #[objc::msg_send(uuid)]
    pub fn uuid(&self) -> Option<arc::R<ns::Uuid>>;

    /// Indicates whether the external storage device is not recommended for capture use.
    ///
    /// This property is used to let the client know if the external storage device is not suitable for camera capture.
    #[objc::msg_send(isNotRecommendedForCaptureUse)]
    pub fn is_not_recommended_for_capture_use(&self) -> bool;

    #[objc::msg_send(nextAvailableURLsWithPathExtensions:error:)]
    pub fn next_available_urls_err<'ear>(
        &self,
        extensions: &ns::Array<ns::String>,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Array<ns::Url>>>;

    pub fn next_available_urls<'ear>(
        &self,
        extensions: &ns::Array<ns::String>,
    ) -> ns::Result<'ear, arc::R<ns::Array<ns::Url>>> {
        ns::if_none(|err| self.next_available_urls_err(extensions, err))
    }
}

/// AVExternalStorageDeviceAuthorization
impl Device {
    #[objc::msg_send(authorizationStatus)]
    pub fn authorization_status() -> av::AuthorizationStatus;

    #[objc::msg_send(requestAccessWithCompletionHandler:)]
    pub fn request_access_ch_block(block: &mut blocks::EscBlock<fn(granted: bool)>);

    pub fn request_access_ch(block: impl FnMut(bool) + 'static) {
        let mut block = blocks::EscBlock::new1(block);
        Self::request_access_ch_block(&mut block);
    }

    #[cfg(feature = "async")]
    pub async fn request_access() -> bool {
        let (fut, mut block) = blocks::comp1();
        Self::request_access_ch_block(block.as_esc_mut());
        fut.await
    }
}

define_obj_type!(
    #[doc(alias = "AVExternalStorageDeviceDiscoverySession")]
    pub DiscoverySession(ns::Id)
);

impl DiscoverySession {
    #[api::available(macos = 14.0, maccatalyst = 17.0, ios = 17.0, tvos = 17.0)]
    crate::define_cls!(AV_EXTERNAL_STORAGE_DEVICE_DISCOVERY_SESSION);

    #[objc::msg_send(sharedSession)]
    #[api::available(macos = 14.0, maccatalyst = 17.0, ios = 17.0, tvos = 17.0)]
    pub fn shared() -> Option<arc::R<Self>>;

    #[objc::msg_send(externalStorageDevices)]
    pub fn devices(&self) -> arc::R<ns::Array<Device>>;

    #[objc::msg_send(isSupported)]
    #[api::available(macos = 14.0, maccatalyst = 17.0, ios = 17.0, tvos = 17.0)]
    pub fn is_supported() -> bool;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_EXTERNAL_STORAGE_DEVICE: &'static objc::Class<Device>;
    static AV_EXTERNAL_STORAGE_DEVICE_DISCOVERY_SESSION: &'static objc::Class<DiscoverySession>;
}

#[cfg(test)]
mod tests {
    use crate::av::external_storage_device::*;

    #[test]
    fn basics() {
        assert!(!DiscoverySession::is_supported());
        assert!(DiscoverySession::shared().is_none());

        assert_eq!(
            Device::authorization_status(),
            av::AuthorizationStatus::NotDetermined
        );
    }
}
