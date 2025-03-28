#[cfg(feature = "ca")]
use crate::ca;
use crate::{arc, cg, define_obj_type, ns, objc};

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
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_VIEW: &'static objc::Class<View>;
}
