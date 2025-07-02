use crate::{api, arc, define_obj_type, ns, objc};

/// Visual styles for the toolbar item.
#[doc(alias = "NSToolbarItemStyle")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(isize)]
pub enum ToolbarItemStyle {
    Plain,
    Prominent,
}

#[doc(alias = "NSToolbarItemVisibilityPriority")]
pub struct ToolbarItemVisibilityPriority(pub isize);

impl ToolbarItemVisibilityPriority {
    pub const STANDARD: Self = Self(0);
    pub const LOW: Self = Self(-1000);
    pub const HIGH: Self = Self(1000);
    pub const USER: Self = Self(2000);
}

define_obj_type! {
    #[doc(alias = "NSToolbarItem")]
    pub ToolbarItem(ns::Id),
    NS_TOOLBAR_ITEM
}

impl arc::A<ToolbarItem> {
    #[objc::msg_send(initWithItemIdentifier:)]
    pub fn init_with_item_id(self, id: &ns::ToolbarItemId) -> arc::R<ToolbarItem>;
}

impl ToolbarItem {
    pub fn with_id(id: &ns::ToolbarItemId) -> arc::R<Self> {
        Self::alloc().init_with_item_id(id)
    }
    #[objc::msg_send(toolbar)]
    pub fn toolbar(&self) -> Option<arc::R<ns::Toolbar>>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: &ns::String);

    #[objc::msg_send(paletteLabel)]
    pub fn palette_label(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setPaletteLabel:)]
    pub fn set_palette_label(&self, val: &ns::String);

    #[objc::msg_send(possibleLabels)]
    pub fn possible_labels(&self) -> arc::R<ns::Set<ns::String>>;

    #[objc::msg_send(setPossibleLabels:)]
    pub fn set_possible_labels(&mut self, val: &ns::Set<ns::String>);

    #[objc::msg_send(toolTip)]
    pub fn tool_tip(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setToolTip:)]
    pub fn set_tool_tip(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(tag)]
    pub fn tag(&self) -> isize;

    #[objc::msg_send(setTag:)]
    pub fn set_tag(&mut self, val: isize);

    #[objc::msg_send(target)]
    pub fn target(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setTarget:)]
    pub fn set_target(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(action)]
    pub fn action(&self) -> Option<&'static objc::Sel>;

    #[objc::msg_send(setAction:)]
    pub fn set_action(&mut self, val: Option<&objc::Sel>);

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ns::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ns::Image>);

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(isBordered)]
    pub fn is_bordered(&self) -> bool;

    #[objc::msg_send(setBordered:)]
    pub fn set_bordered(&mut self, val: bool);

    #[objc::msg_send(backgroundTintColor)]
    #[objc::available(macos = 26.0)]
    pub fn bg_tint_color(&self) -> Option<arc::R<ns::Color>>;

    #[objc::msg_send(setBackgroundTintColor:)]
    #[objc::available(macos = 26.0)]
    pub fn set_bg_tint_color(&mut self, val: Option<&ns::Color>);

    #[objc::msg_send(style)]
    #[objc::available(macos = 26.0)]
    pub fn style(&self) -> ns::ToolbarItemStyle;

    #[objc::msg_send(setStyle:)]
    #[objc::available(macos = 26.0)]
    pub fn set_style(&mut self, val: ns::ToolbarItemStyle);

    #[objc::msg_send(isNavigational)]
    #[objc::available(macos = 11.0)]
    pub fn is_navigational(&self) -> bool;

    #[objc::msg_send(setNavigational:)]
    #[objc::available(macos = 11.0)]
    pub fn set_navigational(&mut self, val: bool);

    #[objc::msg_send(view)]
    pub fn view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setView:)]
    pub fn set_view(&mut self, val: Option<&ns::View>);

    #[objc::msg_send(isVisible)]
    #[objc::available(macos = 12.0)]
    pub fn is_visible(&self) -> bool;

    #[objc::msg_send(isHidden)]
    #[objc::available(macos = 15.0)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    #[objc::available(macos = 15.0)]
    pub fn set_hidden(&mut self, val: bool);

    #[objc::msg_send(visibilityPriority)]
    pub fn visibility_priority(&self) -> ns::ToolbarItemVisibilityPriority;

    #[objc::msg_send(setVisibilityPriority:)]
    pub fn set_visibility_priority(&mut self, val: ns::ToolbarItemVisibilityPriority);

    #[objc::msg_send(validate)]
    pub fn validate(&mut self);

    #[objc::msg_send(autovalidates)]
    pub fn autovalidates(&self) -> bool;

    #[objc::msg_send(setAutovalidates:)]
    pub fn set_autovalidates(&mut self, val: bool);
}

#[objc::protocol(NSToolbarItemValidation)]
pub trait ToolbarItemValidation {
    #[objc::msg_send(validateToolbarItem:)]
    fn validate_toolbar_item(&mut self, item: &mut ns::ToolbarItem) -> bool;
}

impl ns::ToolbarItemId {
    pub fn space() -> &'static Self {
        unsafe { NSToolbarSpaceItemIdentifier }
    }

    pub fn flexible_space() -> &'static Self {
        unsafe { NSToolbarFlexibleSpaceItemIdentifier }
    }

    pub fn show_colors() -> &'static Self {
        unsafe { NSToolbarShowColorsItemIdentifier }
    }

    pub fn show_fonts() -> &'static Self {
        unsafe { NSToolbarShowFontsItemIdentifier }
    }

    pub fn print() -> &'static Self {
        unsafe { NSToolbarPrintItemIdentifier }
    }

    pub fn toggle_sidebar() -> &'static Self {
        unsafe { NSToolbarToggleSidebarItemIdentifier }
    }

    #[api::available(macos = 14.0)]
    pub fn toggle_inspector() -> &'static Self {
        unsafe { NSToolbarToggleInspectorItemIdentifier }
    }

    pub fn cloud_sharing() -> &'static Self {
        unsafe { NSToolbarCloudSharingItemIdentifier }
    }

    #[api::available(macos = 15.2)]
    pub fn writing_tools() -> &'static Self {
        unsafe { NSToolbarWritingToolsItemIdentifier }
    }

    #[api::available(macos = 11.0)]
    pub fn sidebar_tracking_separator() -> &'static Self {
        unsafe { NSToolbarSidebarTrackingSeparatorItemIdentifier }
    }

    #[api::available(macos = 14.0)]
    pub fn inspector_tracking_separator() -> &'static Self {
        unsafe { NSToolbarInspectorTrackingSeparatorItemIdentifier }
    }
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_TOOLBAR_ITEM: &'static objc::Class<ToolbarItem>;
}

#[link(name = "AppKit", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    static NSToolbarSpaceItemIdentifier: &'static ns::ToolbarItemId;
    static NSToolbarFlexibleSpaceItemIdentifier: &'static ns::ToolbarItemId;
    static NSToolbarShowColorsItemIdentifier: &'static ns::ToolbarItemId;
    static NSToolbarShowFontsItemIdentifier: &'static ns::ToolbarItemId;
    static NSToolbarPrintItemIdentifier: &'static ns::ToolbarItemId;
    static NSToolbarToggleSidebarItemIdentifier: &'static ns::ToolbarItemId;
    #[api::available(macos = 14.0)]
    static NSToolbarToggleInspectorItemIdentifier: &'static ns::ToolbarItemId;
    static NSToolbarCloudSharingItemIdentifier: &'static ns::ToolbarItemId;
    #[api::available(macos = 15.2)]
    static NSToolbarWritingToolsItemIdentifier: &'static ns::ToolbarItemId;
    #[api::available(macos = 11.0)]
    static NSToolbarSidebarTrackingSeparatorItemIdentifier: &'static ns::ToolbarItemId;
    #[api::available(macos = 14.0)]
    static NSToolbarInspectorTrackingSeparatorItemIdentifier: &'static ns::ToolbarItemId;
}
