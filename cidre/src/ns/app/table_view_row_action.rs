use crate::{arc, blocks, define_obj_type, ns, objc};

#[doc(alias = "NSTableViewRowActionStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TableViewRowActionStyle {
    Regular = 0,
    /// Red; the row is expected to go.
    Destructive = 1,
}

/// Called with the action and the row it was shown on.
pub type TableViewRowActionHandler = blocks::EscBlock<fn(&TableViewRowAction, ns::Integer)>;

define_obj_type!(
    /// A button a swipe across a table's row shows.
    #[doc(alias = "NSTableViewRowAction")]
    pub TableViewRowAction(ns::Id),
    sym NSTableViewRowAction
);

impl TableViewRowAction {
    #[objc::msg_send(rowActionWithStyle:title:handler:)]
    pub fn with_style_title_handler(
        style: TableViewRowActionStyle,
        title: &ns::String,
        handler: &mut TableViewRowActionHandler,
    ) -> arc::R<Self>;

    #[objc::msg_send(style)]
    pub fn style(&self) -> TableViewRowActionStyle;

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);

    #[objc::msg_send(backgroundColor)]
    pub fn bg_color(&self) -> arc::R<ns::Color>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_bg_color(&mut self, val: Option<&ns::Color>);

    /// Shown instead of the title.
    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ns::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ns::Image>);
}
