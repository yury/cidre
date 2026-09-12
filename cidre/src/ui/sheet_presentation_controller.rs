use crate::{api, arc, define_cls, define_obj_type, ns, objc, ui};

#[doc(alias = "UISheetPresentationControllerPlacement")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum Placement {
    Automatic,
    Leading,
    Center,
    Trailing,
}

define_obj_type!(
    #[doc(alias = "UISheetPresentationControllerDetentIdentifier")]
    pub DetentId(ns::String)
);

impl DetentId {
    #[api::available(ios = 15.0)]
    pub fn medium() -> &'static Self {
        unsafe { UISheetPresentationControllerDetentIdentifierMedium }
    }

    #[api::available(ios = 15.0)]
    pub fn large() -> &'static Self {
        unsafe { UISheetPresentationControllerDetentIdentifierLarge }
    }
}

define_obj_type!(
    #[doc(alias = "UISheetPresentationControllerDetent")]
    pub Detent(ns::Id)
);

impl Detent {
    define_cls!(UI_SHEET_PRESENTATION_CONTROLLER_DETENT);

    #[objc::msg_send(mediumDetent)]
    #[objc::available(ios = 15.0)]
    pub fn medium() -> arc::R<Self>;

    #[objc::msg_send(largeDetent)]
    #[objc::available(ios = 15.0)]
    pub fn large() -> arc::R<Self>;

    #[objc::msg_send(identifier)]
    #[objc::available(ios = 16.0)]
    pub fn id(&self) -> arc::R<DetentId>;
}

define_obj_type!(
    #[doc(alias = "UISheetPresentationController")]
    pub SheetPresentationController(ui::PresentationController)
);

impl SheetPresentationController {
    #[objc::msg_send(setSourceView:)]
    #[objc::available(ios = 15.0)]
    pub fn set_src_view(&mut self, val: Option<&ui::View>);

    #[objc::msg_send(setPreferredPlacement:)]
    #[objc::available(ios = 27.0)]
    pub fn set_preferred_placement(&mut self, val: Placement);

    #[objc::msg_send(setPrefersEdgeAttachedInCompactHeight:)]
    #[objc::available(ios = 15.0)]
    pub fn set_prefers_edge_attached_in_compact_height(&mut self, val: bool);

    #[objc::msg_send(setWidthFollowsPreferredContentSizeWhenEdgeAttached:)]
    #[objc::available(ios = 15.0)]
    pub fn set_width_follows_preferred_content_size_when_edge_attached(&mut self, val: bool);

    #[objc::msg_send(setPrefersGrabberVisible:)]
    #[objc::available(ios = 15.0)]
    pub fn set_prefers_grabber_visible(&mut self, val: bool);

    #[objc::msg_send(setDetents:)]
    #[objc::available(ios = 15.0)]
    pub fn set_detents(&mut self, val: &ns::Array<Detent>);

    #[objc::msg_send(selectedDetentIdentifier)]
    #[objc::available(ios = 15.0)]
    pub fn selected_detent_id(&self) -> Option<arc::R<DetentId>>;

    #[objc::msg_send(setSelectedDetentIdentifier:)]
    #[objc::available(ios = 15.0)]
    pub fn set_selected_detent_id(&mut self, val: Option<&DetentId>);

    /// The largest detent at which the presenting view controller is not dimmed
    /// and still takes touches, as the map does under the sheet in Maps.
    #[objc::msg_send(largestUndimmedDetentIdentifier)]
    #[objc::available(ios = 15.0)]
    pub fn largest_undimmed_detent_id(&self) -> Option<arc::R<DetentId>>;

    #[objc::msg_send(setLargestUndimmedDetentIdentifier:)]
    #[objc::available(ios = 15.0)]
    pub fn set_largest_undimmed_detent_id(&mut self, val: Option<&DetentId>);
}

#[api::weak]
unsafe extern "C" {
    #[api::available(ios = 15.0)]
    static UISheetPresentationControllerDetentIdentifierMedium: &'static DetentId;
    #[api::available(ios = 15.0)]
    static UISheetPresentationControllerDetentIdentifierLarge: &'static DetentId;
}

unsafe extern "C" {
    static UI_SHEET_PRESENTATION_CONTROLLER_DETENT: &'static objc::Class<Detent>;
}
