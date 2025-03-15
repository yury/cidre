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
