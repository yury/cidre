use crate::{arc, cg, define_obj_type, ns, objc, ui};

#[doc(alias = "UILayoutConstraintAxis")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum LayoutConstraintAxis {
    Horizontal = 0,
    Vertical = 1,
}

#[doc(alias = "UIStackViewDistribution")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum StackViewDistribution {
    Fill = 0,
    FillEqually = 1,
    FillProportionally = 2,
    EqualSpacing = 3,
    EqualCentering = 4,
}

#[doc(alias = "UIStackViewAlignment")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum StackViewAlignment {
    Fill = 0,
    /// Leading for a vertical stack, top for a horizontal one.
    Leading = 1,
    FirstBaseline = 2,
    Center = 3,
    /// Trailing for a vertical stack, bottom for a horizontal one.
    Trailing = 4,
    LastBaseline = 5,
}

define_obj_type!(
    #[doc(alias = "UIStackView")]
    pub StackView(ui::View),
    UI_STACK_VIEW
);

impl StackView {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<StackView>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::init(initWithArrangedSubviews:)]
    pub fn init_with_arranged_subviews(self, views: &ns::Array<ui::View>) -> arc::R<StackView>;

    pub fn with_arranged_subviews(views: &ns::Array<ui::View>) -> arc::R<Self> {
        Self::alloc().init_with_arranged_subviews(views)
    }

    #[objc::msg_send(arrangedSubviews)]
    pub fn arranged_subviews(&self) -> arc::R<ns::Array<ui::View>>;

    #[objc::msg_send(addArrangedSubview:)]
    pub fn add_arranged_subview(&mut self, view: &ui::View);

    #[objc::msg_send(removeArrangedSubview:)]
    pub fn remove_arranged_subview(&mut self, view: &ui::View);

    #[objc::msg_send(insertArrangedSubview:atIndex:)]
    pub fn insert_arranged_subview_at(&mut self, view: &ui::View, index: usize);

    #[objc::msg_send(axis)]
    pub fn axis(&self) -> LayoutConstraintAxis;

    #[objc::msg_send(setAxis:)]
    pub fn set_axis(&mut self, val: LayoutConstraintAxis);

    #[objc::msg_send(distribution)]
    pub fn distribution(&self) -> StackViewDistribution;

    #[objc::msg_send(setDistribution:)]
    pub fn set_distribution(&mut self, val: StackViewDistribution);

    #[objc::msg_send(alignment)]
    pub fn alignment(&self) -> StackViewAlignment;

    #[objc::msg_send(setAlignment:)]
    pub fn set_alignment(&mut self, val: StackViewAlignment);

    #[objc::msg_send(spacing)]
    pub fn spacing(&self) -> cg::Float;

    #[objc::msg_send(setSpacing:)]
    pub fn set_spacing(&mut self, val: cg::Float);

    #[objc::msg_send(setCustomSpacing:afterView:)]
    pub fn set_custom_spacing_after_view(&mut self, spacing: cg::Float, view: &ui::View);

    #[objc::msg_send(customSpacingAfterView:)]
    pub fn custom_spacing_after_view(&self, view: &ui::View) -> cg::Float;

    #[objc::msg_send(isBaselineRelativeArrangement)]
    pub fn is_baseline_relative_arrangement(&self) -> bool;

    #[objc::msg_send(setBaselineRelativeArrangement:)]
    pub fn set_baseline_relative_arrangement(&mut self, val: bool);

    #[objc::msg_send(isLayoutMarginsRelativeArrangement)]
    pub fn is_layout_margins_relative_arrangement(&self) -> bool;

    #[objc::msg_send(setLayoutMarginsRelativeArrangement:)]
    pub fn set_layout_margins_relative_arrangement(&mut self, val: bool);
}

unsafe extern "C" {
    static UI_STACK_VIEW: &'static objc::Class<StackView>;
}
