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

/// Shades of gray. systemGray is the base gray color.
impl ui::Color {
    #[objc::msg_send(systemGray2Color)]
    pub fn sys_gray2() -> arc::R<Self>;
    #[objc::msg_send(systemGray3Color)]
    pub fn sys_gray3() -> arc::R<Self>;
    #[objc::msg_send(systemGray4Color)]
    pub fn sys_gray4() -> arc::R<Self>;
    #[objc::msg_send(systemGray5Color)]
    pub fn sys_gray5() -> arc::R<Self>;
    #[objc::msg_send(systemGray6Color)]
    pub fn sys_gray6() -> arc::R<Self>;
}

/// Tint color
impl ui::Color {
    #[objc::msg_send(tintColor)]
    pub fn tint() -> arc::R<Self>;
}

/// Foreground colors
///
/// Foreground colors for static text and related elements.
impl ui::Color {
    #[objc::msg_send(labelColor)]
    pub fn label() -> arc::R<Self>;

    #[objc::msg_send(secondaryLabelColor)]
    pub fn secondary_label() -> arc::R<Self>;

    #[objc::msg_send(tertiaryLabelColor)]
    pub fn tertiary_label() -> arc::R<Self>;

    #[objc::msg_send(quaternaryLabelColor)]
    pub fn quaternary_label() -> arc::R<Self>;

    #[objc::msg_send(linkColor)]
    pub fn link() -> arc::R<Self>;

    #[objc::msg_send(placeholderTextColor)]
    pub fn placeholder_text() -> arc::R<Self>;

    #[objc::msg_send(separatorColor)]
    pub fn separator() -> arc::R<Self>;

    #[objc::msg_send(opaqueSeparatorColor)]
    pub fn opaque_separator() -> arc::R<Self>;
}

/// Background colors
impl ui::Color {
    #[objc::msg_send(systemBackgroundColor)]
    pub fn sys_bg() -> arc::R<Self>;

    #[objc::msg_send(secondarySystemBackgroundColor)]
    pub fn sys_secondary_bg() -> arc::R<Self>;

    #[objc::msg_send(tertiarySystemBackgroundColor)]
    pub fn sys_tertiary_bg() -> arc::R<Self>;

    #[objc::msg_send(systemGroupedBackgroundColor)]
    pub fn sys_grouped_bg() -> arc::R<Self>;

    #[objc::msg_send(secondarySystemGroupedBackgroundColor)]
    pub fn sys_secondary_grouped_bg() -> arc::R<Self>;

    #[objc::msg_send(tertiarySystemGroupedBackgroundColor)]
    pub fn sys_tertiary_grouped_bg() -> arc::R<Self>;
}

/// Fill colors
///
/// Fill colors for UI elements.
/// These are meant to be used over the background colors, since their alpha component is less than 1.
impl ui::Color {
    #[objc::msg_send(systemFillColor)]
    pub fn sys_fill() -> arc::R<Self>;

    #[objc::msg_send(secondarySystemFillColor)]
    pub fn sys_secondary_fill() -> arc::R<Self>;

    #[objc::msg_send(tertiarySystemFillColor)]
    pub fn sys_tertiary_fill() -> arc::R<Self>;

    #[objc::msg_send(quaternarySystemFillColor)]
    pub fn sys_quaternary_fill() -> arc::R<Self>;
}

/// Overlay colors
impl ui::Color {
    /// for a dark background
    #[objc::msg_send(lightTextColor)]
    pub fn light_text() -> arc::R<Self>;

    /// For a light background
    #[objc::msg_send(darkTextColor)]
    pub fn dark_text() -> arc::R<Self>;
}
