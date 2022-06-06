use std::{ffi::c_void, intrinsics::transmute, path::PathBuf};

use crate::cf;

use super::{Connected, Device, Error, Session};

pub type MountCallback<T> = extern "C" fn(status: &cf::Dictionary, context: *mut T);

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    fn AMDeviceMountImage(
        device: &Device,
        image_path: &cf::String,
        options: &cf::Dictionary,
        callback: *const MountCallback<c_void>,
        context: *mut c_void,
    ) -> Error;

    fn AMDeviceRelayFile(device: &Device, source: &cf::String, options: &cf::Dictionary) -> Error;
}

pub fn xcode_dev_path() -> PathBuf {
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

impl<'a> Connected<'a> {
    pub fn device_support_path(&self) -> Option<PathBuf> {
        let version = self.product_version().to_string();
        platform_support_path("iPhoneOS.platform", &version)
    }
}

impl<'a> Session<'a> {
    pub unsafe fn mound_disk_with_callback<T>(
        &self,
        image: &cf::String,
        options: &cf::Dictionary,
        callback: *const MountCallback<T>,
        ctx: *mut T,
    ) -> Error {
        AMDeviceMountImage(self, image, options, transmute(callback), transmute(ctx))
    }

    pub fn mount_developer_image(&self) -> Result<(), Error> {
        let ds_path = self.device_support_path();
        if ds_path.is_none() {
            return Err(Error::NOT_FOUND);
        }
        let ds_path = ds_path.unwrap();
        let image_path = ds_path.join("DeveloperDiskImage.dmg");
        let sig_image_path = ds_path.join("DeveloperDiskImage.dmg.signature");
        let sig = std::fs::read(sig_image_path).expect("sig file read");
        let sig = cf::Retained::from(&sig[..]);

        let image_sig_key = cf::String::from_str("ImageSignature");
        let options = cf::Dictionary::with_keys_values(
            &[&image_type::key(), &image_sig_key],
            &[&image_type::developer(), &sig],
        )
        .expect("options for mount created");

        let path = image_path.to_str().unwrap();
        let ref cf_image_path = cf::String::from_str_no_copy(&path);
        self.mound_disk(&cf_image_path, &options)
    }

    pub fn mound_disk(&self, image: &cf::String, options: &cf::Dictionary) -> Result<(), Error> {
        extern "C" fn mount_cb(status: &cf::Dictionary, context: *mut c_void) {
            status.show();
            println!("satus rc: {:?}", status.retain_count())
        }
        unsafe {
            self.mound_disk_with_callback::<c_void>(
                image,
                options,
                mount_cb as _,
                std::ptr::null_mut(),
            )
            .result()
        }
    }

    /// Copy a file from the relay service.
    /// Options must contain:
    ///
    /// kAMDRelayTypeKey: one of
    ///     kAMDRelayTypeFileDescriptor
    ///     kAMDRelayTypeData
    ///
    //// kAMDRelayLocationKey:
    ////     for kAMDRelayTypeFileDescriptor, this must be a CFNumber containing a file descriptor
    ////     on Windows, this must be a *file descriptor* that can be passed
    ////     to the 'write' call; this is not the same as a Windows HANDLE.
    ////
    ////     for kAMDRelayTypeData, this must be a CFMutableData; the relayed stream will be appended
    ////     to that CFMutableData
    pub fn relay_file(&self, source: &cf::String, options: &cf::Dictionary) -> Error {
        unsafe { AMDeviceRelayFile(self, source, options) }
    }
}

pub fn platform_support_path(platform: &str, os_version: &str) -> Option<PathBuf> {
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

pub mod image_type {
    use crate::cf;

    #[inline]
    pub fn key() -> cf::Retained<'static, cf::String> {
        "ImageType".into()
    }

    #[inline]
    pub fn developer() -> cf::Retained<'static, cf::String> {
        "Developer".into()
    }

    #[inline]
    pub fn debug() -> cf::Retained<'static, cf::String> {
        "Debug".into()
    }

    #[inline]
    pub fn factory() -> cf::Retained<'static, cf::String> {
        "Factory".into()
    }
}

pub mod signature {
    use crate::cf;

    #[inline]
    pub fn key() -> cf::Retained<'static, cf::String> {
        "ImageSignature".into()
    }
}

pub mod relay_type {

    use crate::cf;

    #[inline]
    pub fn key<'a>() -> cf::Retained<'a, cf::String> {
        "RelayType".into()
    }

    #[inline]
    pub fn file_descriptor<'a>() -> cf::Retained<'a, cf::String> {
        "RelayTypeFileDescriptor".into()
    }

    #[inline]
    pub fn data<'a>() -> cf::Retained<'a, cf::String> {
        "RelayTypeData".into()
    }
}

pub mod location {
    use crate::cf;

    #[inline]
    pub fn key<'a>() -> cf::Retained<'a, cf::String> {
        "RelayLocation".into()
    }
}
