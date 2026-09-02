use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSMenuItem")]
    pub MenuItem(ns::Id),
    NS_MENU_ITEM
);

impl arc::A<MenuItem> {
    #[objc::msg_send(initWithTitle:action:keyEquivalent:)]
    pub fn init_with_title_action_key_equivalent(
        self,
        title: &ns::String,
        action: Option<&objc::Sel>,
        key_equivalent: &ns::String,
    ) -> arc::R<MenuItem>;
}

impl MenuItem {
    /// `action` with no target is sent up the responder chain (e.g. `terminate:`, `hide:`,
    /// `performMiniaturize:`, `performClose:`).
    pub fn with_title_action_key_equivalent(
        title: &ns::String,
        action: Option<&objc::Sel>,
        key_equivalent: &ns::String,
    ) -> arc::R<Self> {
        Self::alloc().init_with_title_action_key_equivalent(title, action, key_equivalent)
    }

    #[objc::msg_send(separatorItem)]
    pub fn separator_item() -> arc::R<Self>;

    #[objc::msg_send(isSeparatorItem)]
    pub fn is_separator_item(&self) -> bool;

    #[objc::msg_send(menu)]
    pub fn menu(&self) -> Option<arc::R<ns::Menu>>;

    #[objc::msg_send(submenu)]
    pub fn submenu(&self) -> Option<arc::R<ns::Menu>>;

    #[objc::msg_send(setSubmenu:)]
    pub fn set_submenu(&mut self, val: Option<&ns::Menu>);

    #[objc::msg_send(hasSubmenu)]
    pub fn has_submenu(&self) -> bool;

    #[objc::msg_send(parentItem)]
    pub fn parent_item(&self) -> Option<arc::R<Self>>;

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);

    #[objc::msg_send(keyEquivalent)]
    pub fn key_equivalent(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setKeyEquivalent:)]
    pub fn set_key_equivalent(&mut self, val: &ns::String);

    #[objc::msg_send(keyEquivalentModifierMask)]
    pub fn key_equivalent_modifier_mask(&self) -> ns::EventModifierFlags;

    #[objc::msg_send(setKeyEquivalentModifierMask:)]
    pub fn set_key_equivalent_modifier_mask(&mut self, val: ns::EventModifierFlags);

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ns::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ns::Image>);

    #[objc::msg_send(state)]
    pub fn state(&self) -> ns::ControlStateValue;

    #[objc::msg_send(setState:)]
    pub fn set_state(&mut self, val: ns::ControlStateValue);

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    pub fn set_hidden(&mut self, val: bool);

    #[objc::msg_send(target)]
    pub fn target(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setTarget:)]
    pub fn set_target(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(action)]
    pub fn action(&self) -> Option<&objc::Sel>;

    #[objc::msg_send(setAction:)]
    pub fn set_action(&mut self, val: Option<&objc::Sel>);

    #[objc::msg_send(tag)]
    pub fn tag(&self) -> ns::Integer;

    #[objc::msg_send(setTag:)]
    pub fn set_tag(&mut self, val: ns::Integer);

    #[objc::msg_send(toolTip)]
    pub fn tool_tip(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setToolTip:)]
    pub fn set_tool_tip(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(isHighlighted)]
    pub fn is_highlighted(&self) -> bool;
}

unsafe extern "C" {
    static NS_MENU_ITEM: &'static objc::Class<MenuItem>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut item = ns::MenuItem::with_title_action_key_equivalent(
            ns::str!(c"Quit"),
            None,
            ns::str!(c"q"),
        );
        assert_eq!(item.title().as_ref(), "Quit");
        assert!(item.is_enabled());
        assert!(!item.is_separator_item());
        assert!(item.menu().is_none());
        item.set_tag(7);
        assert_eq!(item.tag(), 7);
        item.set_state(ns::ControlStateValue::ON);
        assert_eq!(item.state(), ns::ControlStateValue::ON);
        item.set_enabled(false);
        assert!(!item.is_enabled());
    }
}
