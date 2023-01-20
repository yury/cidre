use crate::{arc, av, cf, define_obj_type, ns, objc};

define_obj_type!(Input(ns::Id));
define_obj_type!(DeviceInput(Input));
define_obj_type!(Port(ns::Id));

impl Input {
    #[objc::msg_send2(ports)]
    pub fn ports(&self) -> &ns::Array<Port>;
}

impl DeviceInput {
    pub fn with_device<'a>(device: &av::CaptureDevice) -> Result<arc::R<Self>, &'a cf::Error> {
        let mut error = None;
        unsafe {
            let res = AVCaptureDeviceInput_deviceInputWithDevice_error(device, &mut error);
            match error {
                Some(e) => Err(e),
                None => Ok(res.unwrap_unchecked()),
            }
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVCaptureDeviceInput_deviceInputWithDevice_error<'a>(
        device: &av::CaptureDevice,
        error: &mut Option<&'a cf::Error>,
    ) -> Option<arc::R<DeviceInput>>;
}

impl Port {
    #[objc::msg_send2(input)]
    pub fn input(&self) -> &Input;

    #[objc::msg_send2(isEnabled)]
    pub fn enabled(&self) -> bool;

    #[objc::msg_send2(setEnabled:)]
    pub fn set_enabled(&mut self, value: bool);
}

pub mod port_notifications {
    use crate::cf;

    pub fn format_description_did_change() -> &'static cf::NotificationName {
        unsafe { AVCaptureInputPortFormatDescriptionDidChangeNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureInputPortFormatDescriptionDidChangeNotification:
            &'static cf::NotificationName;
    }
}
