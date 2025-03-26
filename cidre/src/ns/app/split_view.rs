use crate::{arc, define_obj_type, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[doc(alias = "NSSplitViewDividerStyle")]
pub enum SplitViewDividerStyle {
    Thick = 1,
    Thin = 2,
    PaneSplitter = 3,
}

define_obj_type!(
    #[doc(alias = "NSSplitView")]
    pub SplitView(ns::View)
);

impl SplitView {
    /// Whether the long axes of a split view's dividers are oriented up-and-down (true) or left-and-right (false)
    #[objc::msg_send(isVertical)]
    pub fn is_vertical(&self) -> bool;

    #[objc::msg_send(setVertical:)]
    pub fn set_vertical(&mut self, val: bool);

    #[objc::msg_send(dividerStyle)]
    pub fn divider_style(&self) -> SplitViewDividerStyle;

    #[objc::msg_send(setDividerStyle:)]
    pub fn set_divider_style(&mut self, val: SplitViewDividerStyle);

    #[objc::msg_send(autosaveName)]
    pub fn autosave_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAutosaveName:)]
    pub fn set_autosave_name(&mut self, val: Option<&ns::String>);
}
