use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSSplitViewController")]
    pub SplitViewController(ns::ViewController)
);

impl SplitViewController {
    #[objc::msg_send(splitView)]
    pub fn split_view(&self) -> arc::R<ns::SplitView>;

    #[objc::msg_send(setSplitView:)]
    pub fn set_split_view(&mut self, val: &ns::SplitView);
}

/// NSSplitViewControllerToggleSidebarAction
impl SplitViewController {
    #[objc::msg_send(toggleSidebar:)]
    #[objc::available(macos = 10.11)]
    pub fn toggle_sidebar(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(toggleInspector:)]
    #[objc::available(macos = 14.0)]
    pub fn toggle_inspector(&mut self, sender: Option<&ns::Id>);
}
