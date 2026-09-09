use crate::{api, arc, cg, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIBackgroundExtensionView")]
    pub BackgroundExtensionView(ui::View)
);

/// Extends the visible edges of its content view to fill its own bounds, so
/// content appears to continue under bars and the floating columns of a split
/// view controller.
impl BackgroundExtensionView {
    #[api::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    crate::define_cls!(UI_BACKGROUND_EXTENSION_VIEW);

    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<BackgroundExtensionView>;

    #[api::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    /// The view whose edges are extended.
    #[objc::msg_send(contentView)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn content_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setContentView:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn set_content_view(&mut self, val: Option<&ui::View>);

    /// Whether the content view is positioned automatically, within the safe area.
    /// Default is true.
    #[objc::msg_send(automaticallyPlacesContentView)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn automatically_places_content_view(&self) -> bool;

    #[objc::msg_send(setAutomaticallyPlacesContentView:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn set_automatically_places_content_view(&mut self, val: bool);
}

unsafe extern "C" {
    static UI_BACKGROUND_EXTENSION_VIEW: &'static objc::Class<BackgroundExtensionView>;
}
