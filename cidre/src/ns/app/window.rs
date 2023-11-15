use crate::define_options;

define_options!(StyleMask(usize));

impl StyleMask {
    const BORDERLESS: Self = Self(0);
    const TITLED: Self = Self(1 << 0);
    const CLOSABLE: Self = Self(1 << 1);
    const MINIATURIZABLE: Self = Self(1 << 2);
    const RESIZABLE: Self = Self(1 << 3);

    /// Specifies a window whose titlebar and toolbar have a unified look - that is,
    /// a continuous background. Under the titlebar and toolbar a horizontal separator
    /// line will appear.
    const UNIFIED_TITLE_AND_TOOLBAR: Self = Self(1 << 12);

    /// When present, the window will appear full screen. This mask is automatically toggled
    /// when \c -toggleFullScreen: is called.
    const FULL_SCREEN: Self = Self(1 << 14);

    /// If set, the \c contentView will consume the full size of the window; it can be combined
    /// with other window style masks, but is only respected for windows with a titlebar.
    /// Utilizing this mask opts-in to layer-backing. Utilize the \c contentLayoutRect or
    /// auto-layout \c contentLayoutGuide to layout views underneath the titlebar/toolbar area.
    const FULL_SIZE_CONTENT_VIEW: Self = Self(1 << 15);

    /// Only applicable for \c NSPanel (or a subclass thereof).
    const UTILITY_WINDOW: Self = Self(1 << 4);

    /// Only applicable for \c NSPanel (or a subclass thereof).
    const DOC_MODAL_WINDOW: Self = Self(1 << 6);

    /// Specifies that a panel that does not activate the owning application.
    /// Only applicable for \c NSPanel (or a subclass thereof).
    const NONACTIVATING_PANEL: Self = Self(1 << 7);

    /// Specifies a heads up display panel. Only applicable for \c NSPanel (or a subclass thereof).
    const HUD_WINDOW: Self = Self(1 << 13);
}
