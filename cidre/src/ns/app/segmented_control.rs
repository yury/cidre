use crate::{arc, cg, define_obj_type, ns, objc};

#[doc(alias = "NSSegmentSwitchTracking")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SegmentSwitchTracking {
    /// Only one segment may be selected.
    SelectOne = 0,
    /// Any segment can be selected.
    SelectAny = 1,
    /// Only selected while tracking.
    Momentary = 2,
    /// Accelerator behavior, only selected while tracking.
    MomentaryAccelerator = 3,
}

#[doc(alias = "NSSegmentStyle")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum SegmentStyle {
    Automatic = 0,
    Rounded = 1,
    RoundRect = 3,
    TexturedSquare = 4,
    SmallSquare = 6,
    Separated = 8,
    TexturedRounded = 2,
    Capsule = 5,
}

#[doc(alias = "NSSegmentDistribution")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum SegmentDistribution {
    /// Dynamically sized segments, the default.
    Fit = 0,
    /// Segments fill the control, each sized to its content.
    Fill = 1,
    /// Segments fill the control, all of the same width.
    FillEqually = 2,
    /// Segments fill the control, sized in proportion to their content.
    FillProportionally = 3,
}

define_obj_type!(
    #[doc(alias = "NSSegmentedControl")]
    pub SegmentedControl(ns::Control),
    NS_SEGMENTED_CONTROL
);

impl SegmentedControl {
    #[objc::msg_send(segmentedControlWithLabels:trackingMode:target:action:)]
    pub fn with_labels(
        labels: &ns::Array<ns::String>,
        tracking_mode: SegmentSwitchTracking,
        target: Option<&ns::Id>,
        action: *const objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(segmentedControlWithImages:trackingMode:target:action:)]
    pub fn with_images(
        images: &ns::Array<ns::Image>,
        tracking_mode: SegmentSwitchTracking,
        target: Option<&ns::Id>,
        action: *const objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(segmentCount)]
    pub fn segment_count(&self) -> isize;

    #[objc::msg_send(setSegmentCount:)]
    pub fn set_segment_count(&mut self, val: isize);

    /// The selected segment, or -1.
    #[objc::msg_send(selectedSegment)]
    pub fn selected_segment(&self) -> isize;

    #[objc::msg_send(setSelectedSegment:)]
    pub fn set_selected_segment(&mut self, val: isize);

    #[objc::msg_send(selectSegmentWithTag:)]
    pub fn select_segment_with_tag(&mut self, tag: isize) -> bool;

    #[objc::msg_send(setSelected:forSegment:)]
    pub fn set_selected_for_segment(&mut self, selected: bool, segment: isize);

    #[objc::msg_send(isSelectedForSegment:)]
    pub fn is_selected_for_segment(&self, segment: isize) -> bool;

    #[objc::msg_send(setWidth:forSegment:)]
    pub fn set_width_for_segment(&mut self, width: cg::Float, segment: isize);

    #[objc::msg_send(widthForSegment:)]
    pub fn width_for_segment(&self, segment: isize) -> cg::Float;

    #[objc::msg_send(setImage:forSegment:)]
    pub fn set_image_for_segment(&mut self, image: Option<&ns::Image>, segment: isize);

    #[objc::msg_send(imageForSegment:)]
    pub fn image_for_segment(&self, segment: isize) -> Option<arc::R<ns::Image>>;

    #[objc::msg_send(setLabel:forSegment:)]
    pub fn set_label_for_segment(&mut self, label: &ns::String, segment: isize);

    #[objc::msg_send(labelForSegment:)]
    pub fn label_for_segment(&self, segment: isize) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setToolTip:forSegment:)]
    pub fn set_tool_tip_for_segment(&mut self, tool_tip: Option<&ns::String>, segment: isize);

    #[objc::msg_send(toolTipForSegment:)]
    pub fn tool_tip_for_segment(&self, segment: isize) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTag:forSegment:)]
    pub fn set_tag_for_segment(&mut self, tag: isize, segment: isize);

    #[objc::msg_send(tagForSegment:)]
    pub fn tag_for_segment(&self, segment: isize) -> isize;

    #[objc::msg_send(setEnabled:forSegment:)]
    pub fn set_enabled_for_segment(&mut self, enabled: bool, segment: isize);

    #[objc::msg_send(isEnabledForSegment:)]
    pub fn is_enabled_for_segment(&self, segment: isize) -> bool;

    #[objc::msg_send(segmentStyle)]
    pub fn segment_style(&self) -> SegmentStyle;

    #[objc::msg_send(setSegmentStyle:)]
    pub fn set_segment_style(&mut self, val: SegmentStyle);

    #[objc::msg_send(trackingMode)]
    pub fn tracking_mode(&self) -> SegmentSwitchTracking;

    #[objc::msg_send(setTrackingMode:)]
    pub fn set_tracking_mode(&mut self, val: SegmentSwitchTracking);

    #[objc::msg_send(segmentDistribution)]
    pub fn segment_distribution(&self) -> SegmentDistribution;

    #[objc::msg_send(setSegmentDistribution:)]
    pub fn set_segment_distribution(&mut self, val: SegmentDistribution);

    #[objc::msg_send(selectedSegmentBezelColor)]
    pub fn selected_segment_bezel_color(&self) -> Option<arc::R<ns::Color>>;

    #[objc::msg_send(setSelectedSegmentBezelColor:)]
    pub fn set_selected_segment_bezel_color(&mut self, val: Option<&ns::Color>);
}

unsafe extern "C" {
    static NS_SEGMENTED_CONTROL: &'static objc::Class<SegmentedControl>;
}
