use crate::{arc, av, cf, define_obj_type, msg_send, ns};

define_obj_type!(Input(ns::Id));
define_obj_type!(DeviceInput(Input));
define_obj_type!(Port(ns::Id));

impl Input {
    pub fn ports(&self) -> &cf::ArrayOf<Port> {
        unsafe { rsel_ports(self) }
    }
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
    fn rsel_ports(input: &Input) -> &cf::ArrayOf<Port>;

    //csel_ab(, AVCaptureDeviceInput, deviceInputWithDevice, AVCaptureDevice *, error,  NSError * _Nullable * _Nullable, AVCaptureDeviceInput * _Nullable)

    fn AVCaptureDeviceInput_deviceInputWithDevice_error<'a>(
        device: &av::CaptureDevice,
        error: &mut Option<&'a cf::Error>,
    ) -> Option<arc::R<DeviceInput>>;
}

impl Port {
    pub fn input(&self) -> &Input {
        unsafe { rsel_input(self) }
    }

    pub fn enabled(&self) -> bool {
        msg_send!("common", self, sel_isEnabled)
    }

    pub fn set_enabled(&mut self, value: bool) {
        msg_send!("common", self, sel_isEnabled, value)
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_input(port: &Port) -> &Input;
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
