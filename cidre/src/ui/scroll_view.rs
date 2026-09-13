use crate::{arc, cg, define_obj_type, objc, ui};

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

    #[objc::msg_send(setContentInsetAdjustmentBehavior:)]
    #[objc::available(ios = 11.0)]
    pub fn set_content_inset_adjustment_behavior(&mut self, val: ContentInsetAdjustmentBehavior);
}

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
