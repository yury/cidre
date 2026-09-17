use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIScrollView")]
    pub ScrollView(ui::View),
    UI_SCROLL_VIEW
);

impl ScrollView {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<ScrollView>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(setShowsHorizontalScrollIndicator:)]
    pub fn set_shows_horizontal_scroll_indicator(&mut self, val: bool);

    #[objc::msg_send(setShowsVerticalScrollIndicator:)]
    pub fn set_shows_vertical_scroll_indicator(&mut self, val: bool);

    #[objc::msg_send(contentSize)]
    pub fn content_size(&self) -> cg::Size;

    #[objc::msg_send(setContentSize:)]
    pub fn set_content_size(&mut self, val: cg::Size);

    #[objc::msg_send(contentInset)]
    pub fn content_inset(&self) -> ui::EdgeInsets;

    #[objc::msg_send(setContentInset:)]
    pub fn set_content_inset(&mut self, val: ui::EdgeInsets);

    #[objc::msg_send(setContentInsetAdjustmentBehavior:)]
    #[objc::available(ios = 11.0)]
    pub fn set_content_inset_adjustment_behavior(&mut self, val: ContentInsetAdjustmentBehavior);

    #[objc::msg_send(contentOffset)]
    pub fn content_offset(&self) -> cg::Point;

    #[objc::msg_send(setContentOffset:)]
    pub fn set_content_offset(&mut self, val: cg::Point);

    #[objc::msg_send(setContentOffset:animated:)]
    pub fn set_content_offset_animated(&mut self, val: cg::Point, animated: bool);

    #[objc::msg_send(bounces)]
    pub fn bounces(&self) -> bool;

    #[objc::msg_send(setBounces:)]
    pub fn set_bounces(&mut self, val: bool);

    #[objc::msg_send(minimumZoomScale)]
    pub fn min_zoom_scale(&self) -> cg::Float;

    #[objc::msg_send(setMinimumZoomScale:)]
    pub fn set_min_zoom_scale(&mut self, val: cg::Float);

    #[objc::msg_send(maximumZoomScale)]
    pub fn max_zoom_scale(&self) -> cg::Float;

    #[objc::msg_send(setMaximumZoomScale:)]
    pub fn set_max_zoom_scale(&mut self, val: cg::Float);

    #[objc::msg_send(zoomScale)]
    pub fn zoom_scale(&self) -> cg::Float;

    #[objc::msg_send(setZoomScale:)]
    pub fn set_zoom_scale(&mut self, val: cg::Float);

    #[objc::msg_send(setZoomScale:animated:)]
    pub fn set_zoom_scale_animated(&mut self, val: cg::Float, animated: bool);

    #[objc::msg_send(zoomToRect:animated:)]
    pub fn zoom_to_rect_animated(&mut self, rect: cg::Rect, animated: bool);

    #[objc::msg_send(bouncesZoom)]
    pub fn bounces_zoom(&self) -> bool;

    #[objc::msg_send(setBouncesZoom:)]
    pub fn set_bounces_zoom(&mut self, val: bool);

    #[objc::msg_send(isZooming)]
    pub fn is_zooming(&self) -> bool;

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyScrollViewDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: ScrollViewDelegate>(&mut self, val: Option<&D>);
}

#[objc::protocol(UIScrollViewDelegate)]
pub trait ScrollViewDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(scrollViewDidScroll:)]
    fn scroll_view_did_scroll(&mut self, scroll_view: &mut ui::ScrollView);

    #[objc::optional]
    #[objc::msg_send(scrollViewWillBeginDragging:)]
    fn scroll_view_will_begin_dragging(&mut self, scroll_view: &mut ui::ScrollView);

    #[objc::optional]
    #[objc::msg_send(scrollViewDidEndDragging:willDecelerate:)]
    fn scroll_view_did_end_dragging_will_decelerate(
        &mut self,
        scroll_view: &mut ui::ScrollView,
        decelerate: bool,
    );

    #[objc::optional]
    #[objc::msg_send(scrollViewDidEndDecelerating:)]
    fn scroll_view_did_end_decelerating(&mut self, scroll_view: &mut ui::ScrollView);

    #[objc::optional]
    #[objc::msg_send(scrollViewDidZoom:)]
    fn scroll_view_did_zoom(&mut self, scroll_view: &mut ui::ScrollView);

    /// The subview that scales when the user pinches; `None` for no zooming.
    #[objc::optional]
    #[objc::msg_send(viewForZoomingInScrollView:)]
    fn view_for_zooming_in_scroll_view(
        &mut self,
        scroll_view: &mut ui::ScrollView,
    ) -> Option<arc::R<ui::View>>;

    #[objc::optional]
    #[objc::msg_send(scrollViewWillBeginZooming:withView:)]
    fn scroll_view_will_begin_zooming_with_view(
        &mut self,
        scroll_view: &mut ui::ScrollView,
        view: Option<&ui::View>,
    );

    #[objc::optional]
    #[objc::msg_send(scrollViewDidEndZooming:withView:atScale:)]
    fn scroll_view_did_end_zooming_with_view_at_scale(
        &mut self,
        scroll_view: &mut ui::ScrollView,
        view: Option<&ui::View>,
        scale: cg::Float,
    );
}

define_obj_type!(
    pub AnyScrollViewDelegate(ns::Id)
);

impl ScrollViewDelegate for AnyScrollViewDelegate {}

#[doc(alias = "UIScrollViewContentInsetAdjustmentBehavior")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum ContentInsetAdjustmentBehavior {
    Automatic,
    ScrollableAxes,
    Never,
    Always,
}

unsafe extern "C" {
    static UI_SCROLL_VIEW: &'static objc::Class<ScrollView>;
}
