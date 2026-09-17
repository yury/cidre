use crate::{api, arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIOverlayArrangementViewProperties")]
    pub OverlayArrangementViewProps(ns::Id)
);

/// How a child is placed in an overlay arrangement.
impl OverlayArrangementViewProps {
    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    crate::define_cls!(UI_OVERLAY_ARRANGEMENT_VIEW_PROPERTIES);

    #[objc::init(init)]
    pub fn init(self) -> arc::R<OverlayArrangementViewProps>;

    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    /// The edge the child is attached to.
    #[objc::msg_send(edge)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn edge(&self) -> ui::DirectionalRectEdge;

    #[objc::msg_send(setEdge:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_edge(&mut self, val: ui::DirectionalRectEdge);
}

define_obj_type!(
    #[doc(alias = "UIOverlayArrangement")]
    pub OverlayArrangement(ui::Arrangement)
);

/// One child over the other, attached to an edge.
impl OverlayArrangement {
    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    crate::define_cls!(UI_OVERLAY_ARRANGEMENT);

    #[objc::msg_send(overlayArrangement)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn new() -> arc::R<Self>;

    #[objc::msg_send(axes)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn axes(&self) -> ui::Axis;

    #[objc::msg_send(setAxes:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_axes(&mut self, val: ui::Axis);

    /// The properties of a placement that has none of its own.
    #[objc::msg_send(defaultViewProperties)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn default_view_props(&self) -> arc::R<OverlayArrangementViewProps>;

    #[objc::msg_send(setViewProperties:forPlacement:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_view_props_for_placement(
        &mut self,
        props: &OverlayArrangementViewProps,
        placement: ui::ArrangementViewPlacement,
    );
}

unsafe extern "C" {
    static UI_OVERLAY_ARRANGEMENT_VIEW_PROPERTIES:
        &'static objc::Class<OverlayArrangementViewProps>;
    static UI_OVERLAY_ARRANGEMENT: &'static objc::Class<OverlayArrangement>;
}
