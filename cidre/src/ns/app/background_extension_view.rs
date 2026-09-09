use crate::{api, arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSBackgroundExtensionView")]
    pub BackgroundExtensionView(ns::View)
);

/// Extends the visible edges of its content view to fill its own bounds, so
/// content appears to continue under sidebars, inspectors and the title bar.
impl BackgroundExtensionView {
    #[api::available(macos = 26.0)]
    crate::define_cls!(NS_BACKGROUND_EXTENSION_VIEW);

    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<BackgroundExtensionView>;

    #[api::available(macos = 26.0)]
    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    /// The view whose edges are extended.
    #[objc::msg_send(contentView)]
    #[objc::available(macos = 26.0)]
    pub fn content_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setContentView:)]
    #[objc::available(macos = 26.0)]
    pub fn set_content_view(&mut self, val: Option<&ns::View>);

    /// Whether the content view is positioned automatically, within the safe area.
    /// Default is true.
    #[objc::msg_send(automaticallyPlacesContentView)]
    #[objc::available(macos = 26.0)]
    pub fn automatically_places_content_view(&self) -> bool;

    #[objc::msg_send(setAutomaticallyPlacesContentView:)]
    #[objc::available(macos = 26.0)]
    pub fn set_automatically_places_content_view(&mut self, val: bool);
}

unsafe extern "C" {
    static NS_BACKGROUND_EXTENSION_VIEW: &'static objc::Class<BackgroundExtensionView>;
}
