use crate::{api, arc, av, cg, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    /// A control that adjusts the video zoom factor of a capture device
    /// within the system-recommended range.
    #[doc(alias = "AVCaptureSystemZoomSlider")]
    pub SysZoomSlider(av::CaptureControl)
);

impl arc::A<SysZoomSlider> {
    #[objc::msg_send(initWithDevice:)]
    pub unsafe fn init_with_device_throws(
        self,
        device: &av::CaptureDevice,
    ) -> arc::R<SysZoomSlider>;

    pub fn init_with_device<'ear>(
        self,
        device: &av::CaptureDevice,
    ) -> Result<arc::R<SysZoomSlider>, &'ear ns::Exception> {
        ns::try_catch(|| unsafe { self.init_with_device_throws(device) })
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(initWithDevice:action:)]
    pub unsafe fn init_with_device_action_throws(
        self,
        device: &av::CaptureDevice,
        action: &mut blocks::EscBlock<fn(video_zoom_factor: cg::Float)>,
    ) -> arc::R<SysZoomSlider>;

    #[cfg(feature = "blocks")]
    pub fn init_with_device_action<'ear>(
        self,
        device: &av::CaptureDevice,
        action: &mut blocks::EscBlock<fn(video_zoom_factor: cg::Float)>,
    ) -> Result<arc::R<SysZoomSlider>, &'ear ns::Exception> {
        ns::try_catch(|| unsafe { self.init_with_device_action_throws(device, action) })
    }
}

impl SysZoomSlider {
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    crate::define_cls!(AV_CAPTURE_SYSTEM_ZOOM_SLIDER);

    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn with_device<'ear>(
        device: &av::CaptureDevice,
    ) -> Result<arc::R<Self>, &'ear ns::Exception> {
        Self::alloc().init_with_device(device)
    }

    #[cfg(feature = "blocks")]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn with_device_action<'ear>(
        device: &av::CaptureDevice,
        action: &mut blocks::EscBlock<fn(video_zoom_factor: cg::Float)>,
    ) -> Result<arc::R<Self>, &'ear ns::Exception> {
        Self::alloc().init_with_device_action(device, action)
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_SYSTEM_ZOOM_SLIDER: &'static objc::Class<SysZoomSlider>;
}
