#[doc(alias = "CGWindowLevel")]
#[repr(transparent)]
pub struct WindowLevel(pub i32);

#[doc(alias = "kCGNumReservedWindowLevels")]
pub const NUM_RESERVED_WINDOW_LEVELS: i32 = 16;

#[doc(alias = "kCGNumReservedBaseWindowLevels")]
pub const NUM_RESERVED_BASE_WINDOW_LEVELS: i32 = 5;

impl WindowLevel {
    #[doc(alias = "kCGBaseWindowLevel")]
    pub const BASE: Self = Self(i32::MIN);

    #[doc(alias = "kCGMinimumWindowLevel")]
    pub const MINIMUM: Self = Self(Self::BASE.0 + NUM_RESERVED_BASE_WINDOW_LEVELS);

    #[doc(alias = "kCGMaximumWindowLevel")]
    pub const MAXIMUM: Self = Self(i32::MAX - NUM_RESERVED_WINDOW_LEVELS);

    #[doc(alias = "kCGDesktopWindowLevel")]
    pub const DESKTOP: Self = Self(Self::MINIMUM.0 + 20);

    #[doc(alias = "kCGDesktopIconWindowLevel")]
    pub const DESKTOP_ICON: Self = Self(Self::DESKTOP.0 + 20);

    #[doc(alias = "kCGBackstopMenuLevel")]
    pub const BACKSTOP_MENU: Self = Self(-20);

    #[doc(alias = "kCGNormalWindowLevel")]
    pub const NORMAL: Self = Self(0);

    #[doc(alias = "kCGFloatingWindowLevel")]
    pub const FLOATING: Self = Self(3);

    #[doc(alias = "kCGTornOffMenuWindowLevel")]
    pub const TORN_OFF_MENU: Self = Self(3);

    #[doc(alias = "kCGModalPanelWindowLevel")]
    pub const MODAL_PANEL: Self = Self(8);

    #[doc(alias = "kCGUtilityWindowLevel")]
    pub const UTILITY: Self = Self(19);

    #[doc(alias = "kCGDockWindowLevel")]
    pub const DOCK: Self = Self(20);

    #[doc(alias = "kCGMainMenuWindowLevel")]
    pub const MAIN_MENU: Self = Self(24);

    #[doc(alias = "kCGStatusWindowLevel")]
    pub const STATUS: Self = Self(25);

    #[doc(alias = "kCGPopUpMenuWindowLevel")]
    pub const POP_UP_MENU: Self = Self(101);

    #[doc(alias = "kCGOverlayWindowLevel")]
    pub const OVERLAY: Self = Self(102);

    #[doc(alias = "kCGHelpWindowLevel")]
    pub const HELP: Self = Self(200);

    #[doc(alias = "kCGDraggingWindowLevel")]
    pub const DRAGGING: Self = Self(500);

    #[doc(alias = "kCGScreenSaverWindowLevel")]
    pub const SCREEN_SAVER: Self = Self(1000);

    #[doc(alias = "kCGAssistiveTechHighWindowLevel")]
    pub const ASSISTIVE_TECH_HIGH: Self = Self(1500);

    #[doc(alias = "kCGCursorWindowLevel")]
    pub const CURSOR: Self = Self(Self::MAXIMUM.0 - 1);
}
