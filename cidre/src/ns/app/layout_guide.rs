use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSLayoutGuide")]
    pub LayoutGuide(ns::Id),
    NS_LAYOUT_GUIDE
);

/// A rectangle that takes part in Auto Layout without being a view.
impl LayoutGuide {
    /// The guide's rectangle in its owning view's coordinates, once laid out.
    #[objc::msg_send(frame)]
    pub fn frame(&self) -> ns::Rect;

    #[objc::msg_send(owningView)]
    pub fn owning_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setIdentifier:)]
    pub fn set_id(&mut self, val: &ns::String);

    #[objc::msg_send(hasAmbiguousLayout)]
    pub fn has_ambiguous_layout(&self) -> bool;
}

ns::impl_layout_anchors!(LayoutGuide);

unsafe extern "C" {
    static NS_LAYOUT_GUIDE: &'static objc::Class<LayoutGuide>;
}
