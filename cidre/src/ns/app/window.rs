use crate::{arc, cg, define_obj_type, define_opts, ns, objc};

define_opts!(pub StyleMask(usize));

impl StyleMask {
    pub const BORDERLESS: Self = Self(0);
    pub const TITLED: Self = Self(1 << 0);
    pub const CLOSABLE: Self = Self(1 << 1);
    pub const MINIATURIZABLE: Self = Self(1 << 2);
    pub const RESIZABLE: Self = Self(1 << 3);

    /// Specifies a window whose titlebar and toolbar have a unified look - that is,
    /// a continuous background. Under the titlebar and toolbar a horizontal separator
    /// line will appear.
    pub const UNIFIED_TITLE_AND_TOOLBAR: Self = Self(1 << 12);

    /// When present, the window will appear full screen. This mask is automatically toggled
    /// when \c -toggleFullScreen: is called.
    pub const FULL_SCREEN: Self = Self(1 << 14);

    /// If set, the \c contentView will consume the full size of the window; it can be combined
    /// with other window style masks, but is only respected for windows with a titlebar.
    /// Utilizing this mask opts-in to layer-backing. Utilize the \c contentLayoutRect or
    /// auto-layout \c contentLayoutGuide to layout views underneath the titlebar/toolbar area.
    pub const FULL_SIZE_CONTENT_VIEW: Self = Self(1 << 15);

    /// Only applicable for \c NSPanel (or a subclass thereof).
    pub const UTILITY_WINDOW: Self = Self(1 << 4);

    /// Only applicable for \c NSPanel (or a subclass thereof).
    pub const DOC_MODAL_WINDOW: Self = Self(1 << 6);

    /// Specifies that a panel that does not activate the owning application.
    /// Only applicable for \c NSPanel (or a subclass thereof).
    pub const NONACTIVATING_PANEL: Self = Self(1 << 7);

    /// Specifies a heads up display panel. Only applicable for \c NSPanel (or a subclass thereof).
    pub const HUD_WINDOW: Self = Self(1 << 13);
}

define_opts!(pub CollectionBehavior(usize));

impl CollectionBehavior {
    pub const DEFAULT: Self = Self(0);
    pub const CAN_JOIN_ALL_SPACES: Self = Self(1 << 0);
    pub const MOVE_TO_ACTIVE_SPACE: Self = Self(1 << 1);
    pub const MANAGED: Self = Self(1 << 2);
    pub const TRANSIENT: Self = Self(1 << 3);
    pub const STATIONARY: Self = Self(1 << 4);
    pub const PARTICIPATES_IN_CYCLE: Self = Self(1 << 5);
    pub const IGNORES_CYCLE: Self = Self(1 << 6);
    pub const FULL_SCREEN_PRIMARY: Self = Self(1 << 7);
    pub const FULL_SCREEN_AUXILIARY: Self = Self(1 << 8);
    pub const FULL_SCREEN_NONE: Self = Self(1 << 9);
    pub const FULL_SCREEN_ALLOWS_TILING: Self = Self(1 << 11);
    pub const FULL_SCREEN_DISALLOWS_TILING: Self = Self(1 << 12);
    pub const PRIMARY: Self = Self(1 << 16);
    pub const AUXILIARY: Self = Self(1 << 17);
    pub const CAN_JOIN_ALL_APPLICATIONS: Self = Self(1 << 18);
}

#[doc(alias = "NSWindowTitleVisibility")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum TitleVisibility {
    Visible,
    Hidden,
}

#[doc(alias = "NSWindowToolbarStyle")]
#[repr(isize)]
pub enum ToolbarStyle {
    Automatic,
    Expanded,
    Preference,
    Unified,
    UnifiedCompact,
}

#[doc(alias = "NSWindowLevel")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct WindowLevel(pub isize);

impl WindowLevel {
    pub const fn with_cg(level: cg::WindowLevel) -> Self {
        Self(level.0 as _)
    }
    pub const NORMAL: Self = Self::with_cg(cg::WindowLevel::NORMAL);
    pub const FLOATING: Self = Self::with_cg(cg::WindowLevel::FLOATING);
    pub const SUBMENU: Self = Self::with_cg(cg::WindowLevel::TORN_OFF_MENU);
    pub const TORN_OFF_MENU: Self = Self::with_cg(cg::WindowLevel::TORN_OFF_MENU);
    pub const MAIN_MENU: Self = Self::with_cg(cg::WindowLevel::MAIN_MENU);
    pub const STATUS: Self = Self::with_cg(cg::WindowLevel::STATUS);
    pub const MODAL_PANEL: Self = Self::with_cg(cg::WindowLevel::MODAL_PANEL);
    pub const POP_UP_MENU: Self = Self::with_cg(cg::WindowLevel::POP_UP_MENU);
    pub const SCREEN_SAVER: Self = Self::with_cg(cg::WindowLevel::SCREEN_SAVER);
}

define_obj_type!(pub Window(ns::Id), NS_WINDOW);

impl Window {
    #[objc::msg_send(frame)]
    pub fn frame(&self) -> ns::Rect;

    #[objc::msg_send(display)]
    pub fn display(&mut self);

    #[objc::msg_send(displayIfNeeded)]
    pub fn display_if_needed(&mut self);

    #[objc::msg_send(addChildWindow:ordered:)]
    pub fn add_child_window(&mut self, window: &ns::Window, ordered: ns::WindowOrderingMode);

    #[objc::msg_send(removeChildWindow:)]
    pub fn remove_child_window(&mut self, window: &ns::Window);
}

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_WINDOW: &'static objc::Class<Window>;
}
