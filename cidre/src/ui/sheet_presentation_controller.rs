use crate::{arc, define_cls, define_obj_type, ns, objc, ui};

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
}

unsafe extern "C" {
    static UI_SHEET_PRESENTATION_CONTROLLER_DETENT: &'static objc::Class<Detent>;
}
