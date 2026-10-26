use crate::{arc, define_obj_type, ns, objc};

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

impl Text {
    #[objc::msg_send(string)]
    pub fn string(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setString:)]
    pub fn set_string(&mut self, val: &ns::String);

    /// The selection, the caret where it is empty, in UTF-16 units.
    #[objc::msg_send(selectedRange)]
    pub fn selected_range(&self) -> ns::Range;

    #[objc::msg_send(setSelectedRange:)]
    pub fn set_selected_range(&mut self, val: ns::Range);

    #[objc::msg_send(font)]
    pub fn font(&self) -> Option<arc::R<ns::Font>>;

    #[objc::msg_send(setFont:)]
    pub fn set_font(&mut self, val: Option<&ns::Font>);

    #[objc::msg_send(isRichText)]
    pub fn is_rich_text(&self) -> bool;

    #[objc::msg_send(setRichText:)]
    pub fn set_rich_text(&mut self, val: bool);
}
