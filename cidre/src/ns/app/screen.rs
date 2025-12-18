use crate::{arc, ca, cg, define_obj_type, ns, objc};

define_obj_type!(
    pub Screen(ns::Id),
    NS_SCREEN
);

impl Screen {
    /// All screens; first one is "zero" screen
    #[objc::msg_send(screens)]
    pub fn screens() -> arc::R<ns::Array<Self>>;

    /// Screen with key window
    #[objc::msg_send(mainScreen)]
    pub fn main() -> Option<arc::R<Self>>;

    #[objc::msg_send(deepestScreen)]
    pub fn deepest() -> Option<arc::R<Self>>;

    #[objc::msg_send(screensHaveSeparateSpaces)]
    pub fn screens_have_separate_spaces() -> bool;

    #[objc::msg_send(depth)]
    pub fn depth(&self) -> ns::WindowDepth;

    #[objc::msg_send(frame)]
    pub fn frame(&self) -> ns::Rect;

    #[objc::msg_send(visibleFrame)]
    pub fn visible_frame(&self) -> ns::Rect;

    #[objc::msg_send(colorSpace)]
    pub fn color_space(&self) -> Option<arc::R<ns::ColorSpace>>;

    #[objc::msg_send(localizedName)]
    pub fn localized_name(&self) -> arc::R<ns::String>;

    /// Indicates the obscured distance from each edge of the screen
    #[objc::msg_send(safeAreaInsets)]
    #[objc::available(macos = 12.0)]
    pub fn safe_area_insets(&self) -> ns::EdgeInsets;

    #[objc::msg_send(auxiliaryTopLeftArea)]
    #[objc::available(macos = 12.0)]
    pub fn auxiliary_top_left_area(&self) -> ns::Rect;

    #[objc::msg_send(auxiliaryTopRightArea)]
    #[objc::available(macos = 12.0)]
    pub fn auxiliary_top_right_area(&self) -> ns::Rect;

    #[objc::msg_send(CGDirectDisplayID)]
    #[objc::available(macos = 26.0)]
    pub fn cg_direct_display_id(&self) -> cg::DirectDisplayId;
}

/// Variable Rate Refresh
impl Screen {
    #[objc::msg_send(maximumFramesPerSecond)]
    #[objc::available(macos = 12.0)]
    fn max_fps(&self) -> ns::Integer;

    #[objc::msg_send(minimumRefreshInterval)]
    #[objc::available(macos = 12.0)]
    fn min_refresh_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(maximumRefreshInterval)]
    #[objc::available(macos = 12.0)]
    fn max_refresh_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(displayUpdateGranularity)]
    #[objc::available(macos = 12.0)]
    fn display_update_granularity(&self) -> ns::TimeInterval;

    /// The time at which the last framebuffer update occurred on the display, in seconds since startup that the system has been awake.
    #[objc::msg_send(lastDisplayUpdateTimestamp)]
    #[objc::available(macos = 12.0)]
    fn last_display_update_ts(&self) -> ns::TimeInterval;
}

/// NSDisplayLink
impl Screen {
    #[objc::msg_send(displayLinkWithTarget:selector:)]
    #[objc::available(macos = 14.0)]
    pub fn display_link_with_target_selector(
        &self,
        target: &ns::Id,
        selector: &objc::Sel,
    ) -> arc::R<ca::DisplayLink>;

    #[objc::available(macos = 14.0)]
    #[allow(unused_unsafe)]
    pub fn display_link_with_target<D: ca::DisplayLinkTargetImpl>(
        &self,
        target: &D,
    ) -> arc::R<ca::DisplayLink> {
        unsafe {
            self.display_link_with_target_selector(target.as_id_ref(), D::sel_on_display_link())
        }
    }
}

unsafe extern "C" {
    static NS_SCREEN: &'static objc::Class<Screen>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let screen = ns::Screen::main().unwrap();
        println!(
            "{}: {:?} {:?} {:?} {:?}",
            screen.localized_name(),
            screen.frame(),
            screen.safe_area_insets(),
            screen.visible_frame(),
            screen.color_space()
        );
    }
}
