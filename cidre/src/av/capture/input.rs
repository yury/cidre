use crate::{arc, av, cm, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVCaptureInput")]
    Input(ns::Id)
);
define_obj_type!(
    #[doc(alias = "AVCaptureDeviceInput")]
    DeviceInput(Input)
);
define_obj_type!(
    #[doc(alias = "AVCaptureInputPort")]
    Port(ns::Id)
);

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
    /// The input that owns the receiver.
    #[objc::msg_send(input)]
    pub fn input(&self) -> &Input;

    /// The media type of the data provided by the receiver.
    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> av::MediaType;

    /// The format of the data provided by the receiver.
    #[objc::msg_send(formatDescription)]
    pub fn format_desc(&self) -> Option<&cm::FormatDesc>;

    #[objc::msg_send(isEnabled)]
    pub fn enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    /// Provides access to the "native" clock used by the input port.
    #[objc::msg_send(clock)]
    pub fn clock(&self) -> Option<&cm::Clock>;

    /// The source device providing input through this port.
    #[objc::msg_send(sourceDeviceType)]
    pub fn src_device_type(&self) -> Option<&av::CaptureDeviceType>;

    /// Position of the source device providing input through this port.
    #[objc::msg_send(sourceDevicePosition)]
    pub fn src_device_position(&self) -> av::CaptureDevicePosition;
}

pub mod port_notifications {
    use crate::ns;

    #[doc(alias = "AVCaptureInputPortFormatDescriptionDidChangeNotification")]
    pub fn format_desc_did_change() -> &'static ns::NotificationName {
        unsafe { AVCaptureInputPortFormatDescriptionDidChangeNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureInputPortFormatDescriptionDidChangeNotification:
            &'static ns::NotificationName;
    }
}
