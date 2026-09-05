use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSScrollView")]
    pub ScrollView(ns::View),
    NS_SCROLL_VIEW
);

impl ScrollView {
    #[objc::msg_send(setDocumentView:)]
    pub fn set_document_view(&mut self, view: Option<&ns::View>);

    #[objc::msg_send(documentView)]
    pub fn document_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setHasVerticalScroller:)]
    pub fn set_has_vertical_scroller(&mut self, val: bool);

    #[objc::msg_send(setAutohidesScrollers:)]
    pub fn set_autohides_scrollers(&mut self, val: bool);

    #[objc::msg_send(setDrawsBackground:)]
    pub fn set_draws_bg(&mut self, val: bool);

    #[objc::msg_send(contentSize)]
    pub fn content_size(&self) -> cg::Size;
}

unsafe extern "C" {
    static NS_SCROLL_VIEW: &'static objc::Class<ScrollView>;
}
