use crate::define_options;

define_options!(StyleMask(usize));

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

define_options!(CollectionBehavior(usize));

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
