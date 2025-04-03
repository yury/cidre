use std::{ffi::c_void, mem::transmute, path::PathBuf};

use crate::{arc, cf};

use super::{Connected, Device, Error, Session};

/// Image Mount Callback
/// Ownership of the status dictionary *status* passes to the callback function. The dict MUST BE
/// explicitly released or else it will leak.
pub type MountCallback<T> = extern "C" fn(status: arc::R<cf::Dictionary>, context: *mut T);

#[link(name = "MobileDevice", kind = "framework")]
unsafe extern "C" {
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
        unsafe { AMDeviceMountImage(self, image, options, transmute(callback), transmute(ctx)) }
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
        let sig: arc::R<cf::Data> = arc::R::from(&sig[..]);

        let image_sig_key = cf::String::from_str("ImageSignature");
        let options = cf::Dictionary::with_keys_values(
            &[&image_type::key(), &image_sig_key],
            &[&image_type::developer(), &sig],
        )
        .expect("options for mount created");

        let path = image_path.to_str().unwrap();
        let cf_image_path = &cf::String::from_str(path);
        self.mound_disk(cf_image_path, &options)
    }

    pub fn mound_disk(
        &self,
        image_path: &cf::String,
        options: &cf::Dictionary,
    ) -> Result<(), Error> {
        extern "C" fn mount_cb(status: arc::R<cf::Dictionary>, _context: *mut c_void) {
            status.show();
        }
        unsafe {
            self.mound_disk_with_callback::<c_void>(
                image_path,
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
    let version = os_version
        .splitn(3, '.')
        .take(2)
        .map(|s| u32::from_str_radix(s, 10).unwrap_or_default())
        .collect::<Vec<_>>();
    let major = version[0].to_string();
    let mut minor = version[1];

    let version = format!("{major}.{minor}");

    let mut matched_majors = Vec::new();

    for directory in std::fs::read_dir(&prefix).expect("folder exists") {
        let directory = directory.expect("folder exists");
        let name = directory.file_name().into_string().expect("valid string");
        if name.starts_with(&version) {
            return Some(prefix.join(name));
        }
        if name.starts_with(&major) {
            matched_majors.push(name);
        }
    }

    while minor > 0 {
        let version = format!("{major}.{minor}");
        if matched_majors.contains(&version) {
            return Some(prefix.join(version));
        }
        minor -= 1;
    }

    None
}

pub mod image_type {
    use crate::{arc, cf};

    #[inline]
    pub fn key() -> arc::R<cf::String> {
        "ImageType".into()
    }

    #[inline]
    pub fn developer() -> arc::R<cf::String> {
        "Developer".into()
    }

    #[inline]
    pub fn debug() -> arc::R<cf::String> {
        "Debug".into()
    }

    #[inline]
    pub fn factory() -> arc::R<cf::String> {
        "Factory".into()
    }
}

pub mod signature {
    use crate::{arc, cf};

    #[inline]
    pub fn key() -> arc::R<cf::String> {
        "ImageSignature".into()
    }
}

pub mod relay_type {

    use crate::{arc, cf};

    #[inline]
    pub fn key() -> arc::R<cf::String> {
        "RelayType".into()
    }

    #[inline]
    pub fn file_descriptor() -> arc::R<cf::String> {
        "RelayTypeFileDescriptor".into()
    }

    #[inline]
    pub fn data() -> arc::R<cf::String> {
        "RelayTypeData".into()
    }
}

pub mod location {
    use crate::{arc, cf};

    #[inline]
    pub fn key() -> arc::R<cf::String> {
        "RelayLocation".into()
    }
}
