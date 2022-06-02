use std::{ffi::c_void, path::PathBuf, intrinsics::transmute};

use crate::cf;

use super::{Device, Error, Connected, Session};

pub type MountCallback<T> =
    extern "C" fn(status: cf::Retained<'static, cf::Dictionary>, context: *mut T);

#[link(name = "MobileDevice", kind = "framework")]
extern "C" {
    fn AMDeviceMountImage(
        device: &Device,
        image_path: &cf::String,
        options: &cf::Dictionary,
        callback: MountCallback<c_void>,
        context: *mut c_void,
    ) -> Error;
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
        callback: MountCallback<T>,
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

    pub fn mound_disk(
        &self,
        image: &cf::String,
        options: &cf::Dictionary,
    ) -> Result<(), Error> {
        unsafe {
            extern "C" fn callback(info: cf::Retained<'static, cf::Dictionary>, _ctx: *mut c_void) {
                println!("!!!!!!");
                info.show();
            }
            AMDeviceMountImage(
                self,
                image,
                options,
                callback as _,
                std::ptr::null_mut(),
            )
            .result()
        }
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