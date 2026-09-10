use crate::{arc, cg, define_obj_type, ns, objc};

/// How strongly a constraint is enforced; 1000 is required.
#[doc(alias = "NSLayoutPriority")]
#[doc(alias = "UILayoutPriority")]
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
#[repr(transparent)]
pub struct LayoutPriority(pub f32);

impl LayoutPriority {
    #[doc(alias = "NSLayoutPriorityRequired")]
    pub const REQUIRED: Self = Self(1000.0);

    #[doc(alias = "NSLayoutPriorityDefaultHigh")]
    pub const DEFAULT_HIGH: Self = Self(750.0);

    #[doc(alias = "NSLayoutPriorityDefaultLow")]
    pub const DEFAULT_LOW: Self = Self(250.0);

    #[doc(alias = "NSLayoutPriorityFittingSizeCompression")]
    #[doc(alias = "UILayoutPriorityFittingSizeLevel")]
    pub const FITTING_SIZE_COMPRESSION: Self = Self(50.0);
}

#[doc(alias = "NSLayoutAttribute")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum LayoutAttr {
    Left = 1,
    Right,
    Top,
    Bottom,
    Leading,
    Trailing,
    Width,
    Height,
    CenterX,
    CenterY,
    LastBaseline,
    FirstBaseline,
    LeftMargin,
    RightMargin,
    TopMargin,
    BottomMargin,
    LeadingMargin,
    TrailingMargin,
    CenterXWithinMargins,
    CenterYWithinMargins,
    NotAnAttribute = 0,
}

#[doc(alias = "NSLayoutRelation")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum LayoutRelation {
    LessThanOrEqual = -1,
    Equal = 0,
    GreaterThanOrEqual = 1,
}

define_obj_type!(
    #[doc(alias = "NSLayoutConstraint")]
    pub LayoutConstraint(ns::Id)
);

/// `first.attr1 <relation> second.attr2 * multiplier + constant`
impl LayoutConstraint {
    crate::define_cls!(NS_LAYOUT_CONSTRAINT);

    #[objc::msg_send(constraintWithItem:attribute:relatedBy:toItem:attribute:multiplier:constant:)]
    pub fn with_item(
        first: &ns::Id,
        first_attr: LayoutAttr,
        relation: LayoutRelation,
        second: Option<&ns::Id>,
        second_attr: LayoutAttr,
        multiplier: cg::Float,
        constant: cg::Float,
    ) -> arc::R<Self>;

    /// Activates every constraint, adding each to the closest common ancestor of
    /// the views it involves.
    #[objc::msg_send(activateConstraints:)]
    pub fn activate(constraints: &ns::Array<Self>);

    #[objc::msg_send(deactivateConstraints:)]
    pub fn deactivate(constraints: &ns::Array<Self>);

    /// Whether the constraint takes part in layout. Setting it adds the constraint
    /// to, or removes it from, the closest common ancestor of the views involved.
    #[objc::msg_send(isActive)]
    pub fn is_active(&self) -> bool;

    #[objc::msg_send(setActive:)]
    pub fn set_active(&mut self, val: bool);

    #[objc::msg_send(priority)]
    pub fn priority(&self) -> LayoutPriority;

    /// Can only change between required and non-required before the constraint is active.
    #[objc::msg_send(setPriority:)]
    pub fn set_priority(&mut self, val: LayoutPriority);

    #[objc::msg_send(shouldBeArchived)]
    pub fn should_be_archived(&self) -> bool;

    #[objc::msg_send(setShouldBeArchived:)]
    pub fn set_should_be_archived(&mut self, val: bool);

    #[objc::msg_send(firstItem)]
    pub fn first_item(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(secondItem)]
    pub fn second_item(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(firstAttribute)]
    pub fn first_attr(&self) -> LayoutAttr;

    #[objc::msg_send(secondAttribute)]
    pub fn second_attr(&self) -> LayoutAttr;

    #[objc::msg_send(relation)]
    pub fn relation(&self) -> LayoutRelation;

    #[objc::msg_send(multiplier)]
    pub fn multiplier(&self) -> cg::Float;

    /// The only part of a constraint that may change after it is active. Animatable.
    #[objc::msg_send(constant)]
    pub fn constant(&self) -> cg::Float;

    #[objc::msg_send(setConstant:)]
    pub fn set_constant(&mut self, val: cg::Float);

    /// Shown in debugging output for unsatisfiable constraints.
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setIdentifier:)]
    pub fn set_id(&mut self, val: Option<&ns::String>);
}

unsafe extern "C" {
    static NS_LAYOUT_CONSTRAINT: &'static objc::Class<LayoutConstraint>;
}
