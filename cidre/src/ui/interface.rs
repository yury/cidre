use crate::{arc, objc, ui};

#[doc(alias = "UIBarStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum BarStyle {
    Default = 0,
    Black = 1,
    _BlackTranslucent = 2,
}

#[doc(alias = "UIUserInterfaceSizeClass")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum SizeClass {
    Unspecified = 0,
    Compact = 1,
    Regular = 2,
}

#[doc(alias = "UIUserInterfaceStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum Style {
    Unspecified = 0,
    Light,
    Dark,
}

#[doc(alias = "UIUserInterfaceLayoutDirection")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum LayoutDirection {
    LeftToRight,
    RightToLeft,
}

#[doc(alias = "UITraitEnvironmentLayoutDirection")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TraitEnvLayoutDirection {
    Unspecified = -1,
    LeftToRight = LayoutDirection::LeftToRight as isize,
    RightToLeft = LayoutDirection::RightToLeft as isize,
}

#[doc(alias = "UIDisplayGamut")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum DisplayGamut {
    Unspecified = -1,
    Srgb,
    P3,
}

impl ui::Color {
    #[objc::msg_send(systemRedColor)]
    pub fn sys_red() -> arc::R<Self>;

    #[objc::msg_send(systemGreenColor)]
    pub fn sys_green() -> arc::R<Self>;

    #[objc::msg_send(systemBlueColor)]
    pub fn sys_blue() -> arc::R<Self>;

    #[objc::msg_send(systemOrangeColor)]
    pub fn sys_orange() -> arc::R<Self>;

    #[objc::msg_send(systemYellowColor)]
    pub fn sys_yellow() -> arc::R<Self>;

    #[objc::msg_send(systemBrownColor)]
    pub fn sys_brown() -> arc::R<Self>;

    #[objc::msg_send(systemPinkColor)]
    pub fn sys_pink() -> arc::R<Self>;

    #[objc::msg_send(systemPurpleColor)]
    pub fn sys_purple() -> arc::R<Self>;

    #[objc::msg_send(systemGrayColor)]
    pub fn sys_gray() -> arc::R<Self>;

    #[objc::msg_send(systemTealColor)]
    pub fn sys_teal() -> arc::R<Self>;

    #[objc::msg_send(systemIndigoColor)]
    pub fn sys_indigo() -> arc::R<Self>;

    #[objc::msg_send(systemMintColor)]
    pub fn sys_mint() -> arc::R<Self>;

    #[objc::msg_send(systemCyanColor)]
    pub fn sys_cyan() -> arc::R<Self>;
}
