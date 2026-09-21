use crate::{define_obj_type, ns};

/// Center is 1 and right is 2, as on iOS, everywhere but on Intel Macs, where
/// they are the other way round (`TARGET_ABI_USES_IOS_VALUES`).
#[doc(alias = "NSTextAlignment")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TextAlignment {
    Left = 0,
    #[cfg(not(target_arch = "x86_64"))]
    Center = 1,
    #[cfg(not(target_arch = "x86_64"))]
    Right = 2,
    #[cfg(target_arch = "x86_64")]
    Right = 1,
    #[cfg(target_arch = "x86_64")]
    Center = 2,
    Justified = 3,
    Natural = 4,
}

define_obj_type!(
    #[doc(alias = "NSText")]
    pub Text(ns::View)
);
