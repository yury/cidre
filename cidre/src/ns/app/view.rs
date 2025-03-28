#[cfg(feature = "ca")]
use crate::ca;
use crate::{arc, cg, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSAutoresizingMaskOptions")]
    pub AutoresizingMaskOpts(usize)
);
impl AutoresizingMaskOpts {
    #[doc(alias = "NSViewNotSizable")]
    pub const NON_SIZEABLE: Self = Self(0);

    #[doc(alias = "NSViewMinXMargin")]
    pub const MIN_X_MARGIN: Self = Self(1);

    #[doc(alias = "NSViewWidthSizable")]
    pub const WIDTH_SIZABLE: Self = Self(2);

    #[doc(alias = "NSViewMaxXMargin")]
    pub const MAX_X_MARGIN: Self = Self(4);

    #[doc(alias = "NSViewMinYMargin")]
    pub const MIN_Y_MARGIN: Self = Self(8);

    #[doc(alias = "NSViewHeightSizable")]
    pub const HEIGHT_SIZABLE: Self = Self(16);

    #[doc(alias = "NSViewMaxYMargin")]
    pub const MAX_Y_MARGIN: Self = Self(32);
}

define_obj_type!(
    #[doc(alias = "NSView")]
    pub View(ns::Responder),
    NS_VIEW
);

impl arc::A<View> {
    #[objc::msg_send(initWithFrame:)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<View>;
}

impl View {
    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(window)]
    pub fn window(&self) -> Option<arc::R<ns::Window>>;

    #[objc::msg_send(superview)]
    pub fn superview(&self) -> Option<arc::R<Self>>;

    #[objc::msg_send(subviews)]
    pub fn subviews(&self) -> arc::R<ns::Array<Self>>;

    #[objc::msg_send(isDescendantOf:)]
    pub fn is_descendant_of(&self, view: &ns::View) -> bool;

    #[objc::msg_send(ancestorSharedWithView:)]
    pub fn ancestor_shared_with_view(&self, view: &ns::View) -> Option<arc::R<Self>>;

    #[objc::msg_send(opaqueAncestor)]
    pub fn opaque_ancestor(&self) -> Option<arc::R<Self>>;

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    pub fn set_hidden(&mut self, val: bool);

    #[objc::msg_send(isHiddenOrHasHiddenAncestor)]
    pub fn is_hidden_or_has_hidden_ancestor(&self) -> bool;

    #[objc::msg_send(addSubview:)]
    pub fn add_subview(&mut self, subview: &ns::View);

    #[objc::msg_send(removeFromSuperview)]
    pub fn remove_from_superview(&mut self);

    #[objc::msg_send(replaceSubview:with:)]
    pub fn replace_subview_with(&mut self, old: &ns::View, new: &ns::View);

    #[objc::msg_send(removeFromSuperviewWithoutNeedingDisplay)]
    pub fn remove_from_superview_without_needing_display(&mut self);

    #[objc::msg_send(viewDidChangeBackingProperties)]
    pub fn view_did_change_backing_properties(&mut self);

    #[objc::msg_send(postsFrameChangedNotifications)]
    pub fn posts_frame_changed_notifications(&self) -> bool;

    #[objc::msg_send(setPostsFrameChangedNotifications:)]
    pub fn set_posts_frame_changed_notifications(&mut self, val: bool);

    #[cfg(feature = "ca")]
    #[objc::msg_send(layer)]
    pub fn layer(&self) -> Option<arc::R<ca::Layer>>;

    #[objc::msg_send(wantsLayer)]
    pub fn wants_layer(&self) -> bool;

    #[objc::msg_send(setWantsLayer:)]
    pub fn set_wants_layer(&mut self, val: bool);

    #[objc::msg_send(wantsUpdateLayer)]
    pub fn wants_update_layer(&self) -> bool;

    #[objc::msg_send(needsLayout)]
    pub fn needs_layout(&self) -> bool;

    #[objc::msg_send(setNeedsLayout:)]
    pub fn set_needs_layout(&mut self, val: bool);

    #[objc::msg_send(alphaValue)]
    pub fn alpha_value(&self) -> cg::Float;

    #[objc::msg_send(setAlphaValue:)]
    pub fn set_alpha_value(&mut self, val: cg::Float);

    #[objc::msg_send(frame)]
    pub fn frame(&self) -> ns::Rect;

    #[objc::msg_send(setFrame:)]
    pub fn set_frame(&mut self, frame: ns::Rect);

    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> ns::Rect;

    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, bounds: ns::Rect);

    #[objc::msg_send(setFrameOrigin:)]
    pub fn set_frame_origin(&mut self, origin: cg::Point);

    #[objc::msg_send(setFrameSize:)]
    pub fn set_frame_size(&mut self, size: cg::Size);

    #[objc::msg_send(setBoundsOrigin:)]
    pub fn set_bounds_origin(&mut self, origin: cg::Point);

    #[objc::msg_send(setBoundsSize:)]
    pub fn set_bounds_size(&mut self, size: cg::Size);

    #[objc::msg_send(convertRectToBacking:)]
    pub fn convert_rect_to_backing(&self, rect: ns::Rect) -> ns::Rect;

    #[objc::msg_send(convertRectFromBacking:)]
    pub fn convert_rect_from_backing(&self, rect: ns::Rect) -> ns::Rect;

    #[objc::msg_send(convertPointToBacking:)]
    pub fn convert_point_to_backing(&self, point: cg::Point) -> cg::Point;

    #[objc::msg_send(convertPointFromBacking:)]
    pub fn convert_point_from_backing(&self, point: cg::Point) -> cg::Point;

    #[objc::msg_send(convertSizeToBacking:)]
    pub fn convert_size_to_backing(&self, size: cg::Size) -> cg::Size;

    #[objc::msg_send(convertSizeFromBacking:)]
    pub fn convert_size_from_backing(&self, size: cg::Size) -> cg::Size;

    #[objc::msg_send(isOpaque)]
    pub fn is_opaque(&self) -> bool;

    #[objc::msg_send(convertPoint:fromView:)]
    pub fn convert_point_from_view(
        &self,
        point: cg::Point,
        from_view: Option<&ns::View>,
    ) -> cg::Point;

    #[objc::msg_send(convertRect:fromView:)]
    pub fn convert_rect_from_view(&self, rect: ns::Rect, from_view: Option<&ns::View>) -> ns::Rect;

    #[objc::msg_send(convertPoint:toView:)]
    pub fn convert_point_to_view(&self, point: cg::Point, to_view: Option<&ns::View>) -> cg::Point;

    #[objc::msg_send(convertRect:toView:)]
    pub fn convert_rect_to_view(&self, rect: ns::Rect, to_view: Option<&ns::View>) -> ns::Rect;

    #[objc::msg_send(autoresizingMask)]
    pub fn autoresizing_mask(&self) -> ns::AutoresizingMaskOpts;

    #[objc::msg_send(setAutoresizingMask:)]
    pub fn set_autoresizing_mask(&mut self, mask: ns::AutoresizingMaskOpts);

    #[objc::msg_send(autoresizesSubviews)]
    pub fn autoresizes_subviews(&self) -> bool;

    #[objc::msg_send(setAutoresizesSubviews:)]
    pub fn set_autoresizes_subviews(&mut self, val: bool);

    #[objc::msg_send(needsDisplay)]
    pub fn needs_display(&self) -> bool;

    #[objc::msg_send(setNeedsDisplay:)]
    pub fn set_needs_display(&mut self, val: bool);

    #[objc::msg_send(display)]
    pub fn display(&mut self);

    #[objc::msg_send(displayIfNeeded)]
    pub fn display_if_needed(&mut self);
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_VIEW: &'static objc::Class<View>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut view = ns::View::new();
        assert!(view.layer().is_none());
        view.set_wants_layer(true);
        assert!(view.layer().is_some());
        assert_eq!(view.is_hidden(), false);
        view.set_hidden(true);
        assert_eq!(view.is_hidden(), true);
        let frame = ns::Rect::new(0.0, 0.0, 100.0, 100.0);
        view.set_frame(frame);
        assert_eq!(view.frame(), frame);
        assert_eq!(view.needs_display(), false);
    }
}
