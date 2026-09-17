use crate::{arc, cg, define_obj_type, ns, objc};

/// How far a scroll view lets its content be pulled past its edge.
#[doc(alias = "NSScrollElasticity")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
#[non_exhaustive]
pub enum ScrollElasticity {
    /// Elastic along the axis when the content is scrollable along it.
    Automatic = 0,
    None = 1,
    Allowed = 2,
}

define_obj_type!(
    #[doc(alias = "NSScrollView")]
    pub ScrollView(ns::View),
    NS_SCROLL_VIEW
);

impl ScrollView {
    #[objc::msg_send(setDocumentView:)]
    pub fn set_document_view(&mut self, view: Option<&ns::View>);

    /// The clip view, whose bounds are the scroll position.
    #[objc::msg_send(contentView)]
    pub fn content_view(&self) -> arc::R<ns::View>;

    /// The part of the document on show, in the document's coordinates.
    #[objc::msg_send(documentVisibleRect)]
    pub fn document_visible_rect(&self) -> ns::Rect;

    #[objc::msg_send(setHasHorizontalScroller:)]
    pub fn set_has_horizontal_scroller(&mut self, val: bool);

    #[objc::msg_send(documentView)]
    pub fn document_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setHasVerticalScroller:)]
    pub fn set_has_vertical_scroller(&mut self, val: bool);

    #[objc::msg_send(setAutohidesScrollers:)]
    pub fn set_autohides_scrollers(&mut self, val: bool);

    #[objc::msg_send(setDrawsBackground:)]
    pub fn set_draws_bg(&mut self, val: bool);

    #[objc::msg_send(contentInsets)]
    pub fn content_insets(&self) -> ns::EdgeInsets;

    #[objc::msg_send(setContentInsets:)]
    pub fn set_content_insets(&mut self, val: ns::EdgeInsets);

    #[objc::msg_send(scrollerInsets)]
    pub fn scroller_insets(&self) -> ns::EdgeInsets;

    #[objc::msg_send(setScrollerInsets:)]
    pub fn set_scroller_insets(&mut self, val: ns::EdgeInsets);

    /// Whether the insets follow the window's title bar and toolbar. `true` by default.
    #[objc::msg_send(automaticallyAdjustsContentInsets)]
    pub fn automatically_adjusts_content_insets(&self) -> bool;

    #[objc::msg_send(setAutomaticallyAdjustsContentInsets:)]
    pub fn set_automatically_adjusts_content_insets(&mut self, val: bool);

    #[objc::msg_send(contentSize)]
    pub fn content_size(&self) -> cg::Size;

    #[objc::msg_send(horizontalScrollElasticity)]
    pub fn horizontal_scroll_elasticity(&self) -> ns::ScrollElasticity;

    #[objc::msg_send(setHorizontalScrollElasticity:)]
    pub fn set_horizontal_scroll_elasticity(&mut self, val: ns::ScrollElasticity);

    #[objc::msg_send(verticalScrollElasticity)]
    pub fn vertical_scroll_elasticity(&self) -> ns::ScrollElasticity;

    #[objc::msg_send(setVerticalScrollElasticity:)]
    pub fn set_vertical_scroll_elasticity(&mut self, val: ns::ScrollElasticity);
}

unsafe extern "C" {
    static NS_SCROLL_VIEW: &'static objc::Class<ScrollView>;
}
