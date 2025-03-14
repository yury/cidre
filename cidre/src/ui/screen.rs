use crate::{arc, cf, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIScreen")]
    pub Screen(ns::Id),
    UI_SCREEN
);

impl Screen {
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    #[objc::msg_send(scale)]
    pub fn scale(&self) -> cg::Float;

    #[objc::msg_send(availableModes)]
    pub fn available_modes(&self) -> arc::R<ns::Array<ui::ScreenMode>>;

    #[objc::msg_send(preferredMode)]
    pub fn preferred_mode(&self) -> Option<arc::R<ui::ScreenMode>>;

    #[objc::msg_send(currentMode)]
    pub fn current_mode(&self) -> Option<arc::R<ui::ScreenMode>>;

    #[objc::msg_send(setCurrentMode:)]
    pub fn set_current_mode(&mut self, val: Option<&ui::ScreenMode>);

    #[objc::msg_send(overscanCompensationInsets)]
    pub fn overscan_compensation_insets(&self) -> ui::EdgeInsets;

    #[objc::msg_send(mirroredScreen)]
    pub fn mirrored_screen(&self) -> Option<arc::R<Self>>;

    #[objc::msg_send(brightness)]
    pub fn brightness(&self) -> cg::Float;

    #[objc::msg_send(setBrightness:)]
    pub fn set_brightness(&self, val: cg::Float);

    #[objc::msg_send(wantsSoftwareDimming)]
    pub fn wants_software_dimming(&self) -> bool;

    #[objc::msg_send(setWantsSoftwareDimming:)]
    pub fn set_wants_software_dimming(&self, val: bool);

    /// Native bounds of the physical screen in pixels
    #[objc::msg_send(nativeBounds)]
    pub fn native_bounds(&self) -> cg::Rect;

    /// Native scale factor of the physical screen
    #[objc::msg_send(nativeScale)]
    pub fn native_scale(&self) -> cg::Float;

    #[objc::msg_send(maximumFramesPerSecond)]
    pub fn max_fps(&self) -> ns::Integer;

    /// The latency of the display hardware associated with this screen.
    ///
    /// Can be used along with CoreAudio devices' kAudioDeviceLatencyProperty to
    /// achieve A/V sync when writing custom video playback software.
    /// Will be `0` if display latency has not been calibrated by the user.
    #[objc::msg_send(calibratedLatency)]
    #[objc::available(ios = 13.0)]
    pub fn calibrated_latency(&self) -> cf::TimeInterval;

    #[objc::msg_send(currentEDRHeadroom)]
    #[objc::available(ios = 16.0)]
    pub fn current_edr_headroom(&self) -> cg::Float;

    #[objc::msg_send(potentialEDRHeadroom)]
    #[objc::available(ios = 16.0)]
    pub fn potential_edr_headroom(&self) -> cg::Float;
}

/// UISnapshotting
impl Screen {
    #[objc::msg_send(snapshotViewAfterScreenUpdates:)]
    fn snapshot_view_after_screen_updates(&self, after_updates: bool) -> arc::R<ui::View>;
}

unsafe extern "C" {
    static UI_SCREEN: &'static objc::Class<Screen>;
}
