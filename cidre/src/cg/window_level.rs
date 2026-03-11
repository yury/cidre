#[doc(alias = "CGWindowLevelKey")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct WindowLevelKey(pub i32);

impl WindowLevelKey {
    #[doc(alias = "kCGBaseWindowLevelKey")]
    pub const BASE: Self = Self(0);

    #[doc(alias = "kCGMinimumWindowLevelKey")]
    pub const MINIMUM: Self = Self(1);

    #[doc(alias = "kCGDesktopWindowLevelKey")]
    pub const DESKTOP: Self = Self(2);

    #[doc(alias = "kCGBackstopMenuLevelKey")]
    pub const BACKSTOP_MENU: Self = Self(3);

    #[doc(alias = "kCGNormalWindowLevelKey")]
    pub const NORMAL: Self = Self(4);

    #[doc(alias = "kCGFloatingWindowLevelKey")]
    pub const FLOATING: Self = Self(5);

    #[doc(alias = "kCGTornOffMenuWindowLevelKey")]
    pub const TORN_OFF_MENU: Self = Self(6);

    #[doc(alias = "kCGDockWindowLevelKey")]
    pub const DOCK: Self = Self(7);

    #[doc(alias = "kCGMainMenuWindowLevelKey")]
    pub const MAIN_MENU: Self = Self(8);

    #[doc(alias = "kCGStatusWindowLevelKey")]
    pub const STATUS: Self = Self(9);

    #[doc(alias = "kCGModalPanelWindowLevelKey")]
    pub const MODAL_PANEL: Self = Self(10);

    #[doc(alias = "kCGPopUpMenuWindowLevelKey")]
    pub const POP_UP_MENU: Self = Self(11);

    #[doc(alias = "kCGDraggingWindowLevelKey")]
    pub const DRAGGING: Self = Self(12);

    #[doc(alias = "kCGScreenSaverWindowLevelKey")]
    pub const SCREEN_SAVER: Self = Self(13);

    #[doc(alias = "kCGMaximumWindowLevelKey")]
    pub const MAXIMUM: Self = Self(14);

    #[doc(alias = "kCGOverlayWindowLevelKey")]
    pub const OVERLAY: Self = Self(15);

    #[doc(alias = "kCGHelpWindowLevelKey")]
    pub const HELP: Self = Self(16);

    #[doc(alias = "kCGUtilityWindowLevelKey")]
    pub const UTILITY: Self = Self(17);

    #[doc(alias = "kCGDesktopIconWindowLevelKey")]
    pub const DESKTOP_ICON: Self = Self(18);

    #[doc(alias = "kCGCursorWindowLevelKey")]
    pub const CURSOR: Self = Self(19);

    #[doc(alias = "kCGAssistiveTechHighWindowLevelKey")]
    pub const ASSISTIVE_TECH_HIGH: Self = Self(20);

    #[doc(alias = "kCGNumberOfWindowLevelKeys")]
    pub const NUMBER_OF_KEYS: Self = Self(21);
}

#[cfg(target_os = "macos")]
impl WindowLevelKey {
    #[doc(alias = "CGWindowLevelForKey")]
    #[inline]
    pub fn window_level(self) -> WindowLevel {
        unsafe { CGWindowLevelForKey(self) }
    }
}

#[doc(alias = "CGWindowLevel")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[cfg(target_os = "macos")]
unsafe extern "C-unwind" {
    fn CGWindowLevelForKey(key: WindowLevelKey) -> WindowLevel;
}
