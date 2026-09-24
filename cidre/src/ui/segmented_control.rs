use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UISegmentedControl")]
    pub SegmentedControl(ui::Control),
    UI_SEGMENTED_CONTROL
);

impl SegmentedControl {
    /// No segment is selected.
    #[doc(alias = "UISegmentedControlNoSegment")]
    pub const NO_SEGMENT: isize = -1;

    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<SegmentedControl>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    /// `items` are `ns::String` titles or `ui::Image`s.
    #[objc::init(initWithItems:)]
    pub fn init_with_items(self, items: Option<&ns::Array<ns::Id>>) -> arc::R<SegmentedControl>;

    pub fn with_items(items: Option<&ns::Array<ns::Id>>) -> arc::R<Self> {
        Self::alloc().init_with_items(items)
    }

    /// One segment per action, in order: the segment shows the action's image, or its title
    /// if it has none, and performs the action when it is selected.
    #[objc::init(initWithFrame:actions:)]
    #[objc::available(ios = 14.0)]
    pub fn init_with_frame_actions(
        self,
        frame: cg::Rect,
        actions: &ns::Array<ui::Action>,
    ) -> arc::R<SegmentedControl>;

    #[objc::available(ios = 14.0)]
    pub fn with_frame_actions(frame: cg::Rect, actions: &ns::Array<ui::Action>) -> arc::R<Self> {
        Self::alloc().init_with_frame_actions(frame, actions)
    }

    #[objc::msg_send(insertSegmentWithAction:atIndex:animated:)]
    #[objc::available(ios = 14.0)]
    pub fn insert_segment_with_action(&mut self, action: &ui::Action, index: usize, animated: bool);

    #[objc::msg_send(setAction:forSegmentAtIndex:)]
    #[objc::available(ios = 14.0)]
    pub fn set_action_for_segment(&mut self, action: &ui::Action, index: usize);

    #[objc::msg_send(actionForSegmentAtIndex:)]
    #[objc::available(ios = 14.0)]
    pub fn action_for_segment(&self, index: usize) -> Option<arc::R<ui::Action>>;

    #[objc::msg_send(insertSegmentWithTitle:atIndex:animated:)]
    pub fn insert_segment_with_title(
        &mut self,
        title: Option<&ns::String>,
        index: usize,
        animated: bool,
    );

    #[objc::msg_send(insertSegmentWithImage:atIndex:animated:)]
    pub fn insert_segment_with_image(
        &mut self,
        image: Option<&ui::Image>,
        index: usize,
        animated: bool,
    );

    #[objc::msg_send(removeSegmentAtIndex:animated:)]
    pub fn remove_segment(&mut self, index: usize, animated: bool);

    #[objc::msg_send(removeAllSegments)]
    pub fn remove_all_segments(&mut self);

    #[objc::msg_send(numberOfSegments)]
    pub fn number_of_segments(&self) -> usize;

    #[objc::msg_send(setTitle:forSegmentAtIndex:)]
    pub fn set_title_for_segment(&mut self, title: Option<&ns::String>, index: usize);

    #[objc::msg_send(titleForSegmentAtIndex:)]
    pub fn title_for_segment(&self, index: usize) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setImage:forSegmentAtIndex:)]
    pub fn set_image_for_segment(&mut self, image: Option<&ui::Image>, index: usize);

    #[objc::msg_send(imageForSegmentAtIndex:)]
    pub fn image_for_segment(&self, index: usize) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(setWidth:forSegmentAtIndex:)]
    pub fn set_width_for_segment(&mut self, width: cg::Float, index: usize);

    #[objc::msg_send(widthForSegmentAtIndex:)]
    pub fn width_for_segment(&self, index: usize) -> cg::Float;

    #[objc::msg_send(setEnabled:forSegmentAtIndex:)]
    pub fn set_enabled_for_segment(&mut self, enabled: bool, index: usize);

    #[objc::msg_send(isEnabledForSegmentAtIndex:)]
    pub fn is_enabled_for_segment(&self, index: usize) -> bool;

    /// The selected segment, or [`Self::NO_SEGMENT`].
    #[objc::msg_send(selectedSegmentIndex)]
    pub fn selected_segment_index(&self) -> isize;

    #[objc::msg_send(setSelectedSegmentIndex:)]
    pub fn set_selected_segment_index(&mut self, val: isize);

    #[objc::msg_send(isMomentary)]
    pub fn is_momentary(&self) -> bool;

    #[objc::msg_send(setMomentary:)]
    pub fn set_momentary(&mut self, val: bool);

    #[objc::msg_send(apportionsSegmentWidthsByContent)]
    pub fn apportions_segment_widths_by_content(&self) -> bool;

    #[objc::msg_send(setApportionsSegmentWidthsByContent:)]
    pub fn set_apportions_segment_widths_by_content(&mut self, val: bool);

    #[objc::msg_send(selectedSegmentTintColor)]
    pub fn selected_segment_tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setSelectedSegmentTintColor:)]
    pub fn set_selected_segment_tint_color(&mut self, val: Option<&ui::Color>);
}

unsafe extern "C" {
    static UI_SEGMENTED_CONTROL: &'static objc::Class<SegmentedControl>;
}
