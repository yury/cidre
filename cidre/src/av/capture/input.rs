use crate::{arc, av, define_cls, define_obj_type, ns, objc};

define_obj_type!(Input(ns::Id));
define_obj_type!(DeviceInput(Input));
define_obj_type!(Port(ns::Id));

impl Input {
    #[objc::msg_send(ports)]
    pub fn ports(&self) -> &ns::Array<Port>;
}

impl arc::A<DeviceInput> {
    #[objc::msg_send(initWithDevice:error:)]
    pub fn init_with_device_err(
        self,
        device: &av::CaptureDevice,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<DeviceInput>>;
}

impl DeviceInput {
    define_cls!(AV_CAPTURE_DEVICE_INPUT);

    pub fn with_device<'a>(device: &av::CaptureDevice) -> Result<arc::R<Self>, &'a ns::Error> {
        let mut error = None;
        unsafe {
            let res = Self::alloc().init_with_device_err(device, &mut error);
            match error {
                Some(e) => Err(e),
                None => Ok(res.unwrap_unchecked()),
            }
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_DEVICE_INPUT: &'static objc::Class<DeviceInput>;
}

impl Port {
    #[objc::msg_send(input)]
    pub fn input(&self) -> &Input;

    #[objc::msg_send(isEnabled)]
    pub fn enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, value: bool);
}

pub mod port_notifications {
    use crate::cf;

    #[doc(alias = "AVCaptureInputPortFormatDescriptionDidChangeNotification")]
    pub fn format_desc_did_change() -> &'static cf::NotificationName {
        unsafe { AVCaptureInputPortFormatDescriptionDidChangeNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureInputPortFormatDescriptionDidChangeNotification:
            &'static cf::NotificationName;
    }
}
