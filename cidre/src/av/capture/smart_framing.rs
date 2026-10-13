use crate::{api, arc, define_obj_type, ns, objc};

define_obj_type!(
    /// An aspect ratio a device's dynamic aspect ratio can be set to.
    #[doc(alias = "AVCaptureAspectRatio")]
    pub AspectRatio(ns::String)
);

impl AspectRatio {
    #[doc(alias = "AVCaptureAspectRatio1x1")]
    #[api::available(ios = 26.0)]
    pub fn _1x1() -> &'static Self {
        unsafe { AVCaptureAspectRatio1x1 }
    }

    #[doc(alias = "AVCaptureAspectRatio16x9")]
    #[api::available(ios = 26.0)]
    pub fn _16x9() -> &'static Self {
        unsafe { AVCaptureAspectRatio16x9 }
    }

    #[doc(alias = "AVCaptureAspectRatio9x16")]
    #[api::available(ios = 26.0)]
    pub fn _9x16() -> &'static Self {
        unsafe { AVCaptureAspectRatio9x16 }
    }

    #[doc(alias = "AVCaptureAspectRatio4x3")]
    #[api::available(ios = 26.0)]
    pub fn _4x3() -> &'static Self {
        unsafe { AVCaptureAspectRatio4x3 }
    }

    #[doc(alias = "AVCaptureAspectRatio3x4")]
    #[api::available(ios = 26.0)]
    pub fn _3x4() -> &'static Self {
        unsafe { AVCaptureAspectRatio3x4 }
    }
}

#[api::weak]
unsafe extern "C" {
    #[api::available(ios = 26.0)]
    static AVCaptureAspectRatio1x1: &'static AspectRatio;
    #[api::available(ios = 26.0)]
    static AVCaptureAspectRatio16x9: &'static AspectRatio;
    #[api::available(ios = 26.0)]
    static AVCaptureAspectRatio9x16: &'static AspectRatio;
    #[api::available(ios = 26.0)]
    static AVCaptureAspectRatio4x3: &'static AspectRatio;
    #[api::available(ios = 26.0)]
    static AVCaptureAspectRatio3x4: &'static AspectRatio;
}

define_obj_type!(
    /// Watches a device's scene and recommends a framing (an aspect ratio and a zoom factor)
    /// for objects of interest entering and leaving it. Answer a recommendation with the
    /// device's `set_dynamic_aspect_ratio` (on its config lock guard) and its video zoom factor,
    /// in whichever order animates best.
    #[doc(alias = "AVCaptureSmartFramingMonitor")]
    pub SmartFramingMonitor(ns::Id)
);

impl SmartFramingMonitor {
    /// The framings the monitor can recommend in the device's current configuration.
    /// Key-value observable.
    #[objc::msg_send(supportedFramings)]
    pub fn supported_framings(&self) -> arc::R<ns::Array<Framing>>;

    /// The framings the monitor may recommend, of the supported ones; none by default. Set
    /// before the session runs, changeable while it runs.
    #[objc::msg_send(enabledFramings)]
    pub fn enabled_framings(&self) -> arc::R<ns::Array<Framing>>;

    #[objc::msg_send(setEnabledFramings:)]
    pub fn set_enabled_framings(&mut self, val: &ns::Array<Framing>);

    /// The latest recommendation, one of the enabled framings; `None` while not monitoring.
    /// Key-value observable.
    #[objc::msg_send(recommendedFraming)]
    pub fn recommended_framing(&self) -> Option<arc::R<Framing>>;

    #[objc::msg_send(startMonitoringWithError:)]
    pub unsafe fn start_monitoring_err<'ear>(
        &mut self,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Starts watching the scene and recommending framings.
    pub fn start_monitoring<'ear>(&mut self) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe { self.start_monitoring_err(err) })
    }

    #[objc::msg_send(stopMonitoring)]
    pub fn stop_monitoring(&mut self);

    #[objc::msg_send(isMonitoring)]
    pub fn is_monitoring(&self) -> bool;
}

define_obj_type!(
    /// A framing: an aspect ratio and a zoom factor.
    #[doc(alias = "AVCaptureFraming")]
    pub Framing(ns::Id)
);

impl Framing {
    #[objc::msg_send(aspectRatio)]
    pub fn aspect_ratio(&self) -> arc::R<AspectRatio>;

    #[objc::msg_send(zoomFactor)]
    pub fn zoom_factor(&self) -> f32;
}
