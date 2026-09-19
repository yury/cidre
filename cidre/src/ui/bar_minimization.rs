use crate::{arc, define_cls, define_obj_type, ns, objc};

#[doc(alias = "UIBarMinimizationBehavior")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum BarMinimizationBehavior {
    /// The system determines the minimization behavior.
    Automatic = 0,
    /// Bar minimization is disabled.
    Never = 1,
    /// Minimize when the user scrolls down.
    OnScrollDown = 2,
    /// Minimize when the user scrolls up.
    OnScrollUp = 3,
}

#[doc(alias = "UIBarMinimizationSafeAreaAdjustment")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum BarMinimizationSafeAreaAdjustment {
    /// The system determines the safe area adjustment.
    Automatic = 0,
    /// The safe area adjusts as bars minimize, allowing content to reflow.
    Enabled = 1,
    /// The safe area remains unchanged as bars minimize.
    Disabled = 2,
}

#[doc(alias = "UIBarMinimizationRestorationBehavior")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum BarMinimizationRestorationBehavior {
    /// The bar restores when the user reverses scroll direction.
    Automatic = 0,
    /// The bar restores only when the observed scroll view's content reaches the scroll edge.
    AtScrollEdge = 1,
}

define_obj_type!(
    /// How a navigation bar minimizes in response to scrolling; set on a
    /// navigation item through [`crate::ui::NavItem::set_nav_bar_minimization`].
    #[doc(alias = "UIBarMinimization")]
    pub BarMinimization(ns::Id)
);

impl arc::A<BarMinimization> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<BarMinimization>;
}

impl BarMinimization {
    define_cls!(UI_BAR_MINIMIZATION);

    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(minimizationBehavior)]
    pub fn minimization_behavior(&self) -> BarMinimizationBehavior;

    #[objc::msg_send(setMinimizationBehavior:)]
    pub fn set_minimization_behavior(&mut self, val: BarMinimizationBehavior);

    #[objc::msg_send(safeAreaAdjustment)]
    pub fn safe_area_adjustment(&self) -> BarMinimizationSafeAreaAdjustment;

    #[objc::msg_send(setSafeAreaAdjustment:)]
    pub fn set_safe_area_adjustment(&mut self, val: BarMinimizationSafeAreaAdjustment);

    #[objc::msg_send(restorationBehavior)]
    pub fn restoration_behavior(&self) -> BarMinimizationRestorationBehavior;

    #[objc::msg_send(setRestorationBehavior:)]
    pub fn set_restoration_behavior(&mut self, val: BarMinimizationRestorationBehavior);
}

unsafe extern "C" {
    static UI_BAR_MINIMIZATION: &'static objc::Class<BarMinimization>;
}
