use crate::{arc, define_obj_type, ns, objc};

/// When a popover closes on its own.
#[doc(alias = "NSPopoverBehavior")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum PopoverBehavior {
    /// The app closes it; the default.
    AppDefined = 0,
    /// Closes when the user interacts with something outside it.
    Transient = 1,
    /// Closes when the user interacts with the window that holds it.
    SemiTransient = 2,
}

define_obj_type!(
    #[doc(alias = "NSPopover")]
    pub Popover(ns::Responder),
    sym NSPopover
);

impl Popover {
    #[objc::msg_send(behavior)]
    pub fn behavior(&self) -> PopoverBehavior;

    #[objc::msg_send(setBehavior:)]
    pub fn set_behavior(&mut self, val: PopoverBehavior);

    /// Whether showing and closing animate. Defaults to `true`.
    #[objc::msg_send(animates)]
    pub fn animates(&self) -> bool;

    #[objc::msg_send(setAnimates:)]
    pub fn set_animates(&mut self, val: bool);

    #[objc::msg_send(contentViewController)]
    pub fn content_vc(&self) -> Option<arc::R<ns::ViewController>>;

    #[objc::msg_send(setContentViewController:)]
    pub fn set_content_vc(&mut self, val: Option<&ns::ViewController>);

    /// The content size; zero takes the content view controller's preferred size.
    #[objc::msg_send(contentSize)]
    pub fn content_size(&self) -> ns::Size;

    #[objc::msg_send(setContentSize:)]
    pub fn set_content_size(&mut self, val: ns::Size);

    #[objc::msg_send(isShown)]
    pub fn is_shown(&self) -> bool;

    #[objc::msg_send(isDetached)]
    pub fn is_detached(&self) -> bool;

    #[objc::msg_send(positioningRect)]
    pub fn positioning_rect(&self) -> ns::Rect;

    #[objc::msg_send(setPositioningRect:)]
    pub fn set_positioning_rect(&mut self, val: ns::Rect);

    /// Shows the popover anchored to `rect` of `view`, its arrow on `edge`.
    #[objc::msg_send(showRelativeToRect:ofView:preferredEdge:)]
    pub fn show_relative_to_rect(&mut self, rect: ns::Rect, view: &ns::View, edge: ns::RectEdge);

    /// Shows the popover anchored to a toolbar item.
    #[objc::msg_send(showRelativeToToolbarItem:)]
    #[objc::available(macos = 14.0)]
    pub fn show_relative_to_toolbar_item(&mut self, item: &ns::ToolbarItem);

    /// Closes it as if the user did, which a delegate may refuse.
    #[objc::msg_send(performClose:)]
    pub fn perform_close(&mut self, sender: Option<&ns::Id>);

    /// Closes it, whatever a delegate says.
    #[objc::msg_send(close)]
    pub fn close(&mut self);
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut popover = ns::Popover::new();
        assert_eq!(popover.behavior(), ns::PopoverBehavior::AppDefined);
        popover.set_behavior(ns::PopoverBehavior::Transient);
        assert_eq!(popover.behavior(), ns::PopoverBehavior::Transient);
        popover.set_animates(false);
        assert!(!popover.animates());
        assert!(!popover.is_shown());
        let vc = ns::ViewController::new();
        popover.set_content_vc(Some(&vc));
        assert!(popover.content_vc().is_some());
        popover.set_content_size(ns::Size::new(320.0, 200.0));
        assert_eq!(popover.content_size(), ns::Size::new(320.0, 200.0));
    }
}
