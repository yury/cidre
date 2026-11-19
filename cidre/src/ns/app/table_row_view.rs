use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSTableRowView")]
    pub TableRowView(ns::View),
    sym NSTableRowView
);

impl TableRowView {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<TableRowView>;

    /// Whether selection uses the emphasized selection color.
    #[objc::msg_send(isEmphasized)]
    pub fn is_emphasized(&self) -> bool;

    /// Changes the emphasis of the selection highlight.
    #[objc::msg_send(setEmphasized:)]
    pub fn set_emphasized(&mut self, val: bool);

    #[objc::msg_send(isSelected)]
    pub fn is_selected(&self) -> bool;

    #[objc::msg_send(setSelected:)]
    pub fn set_selected(&mut self, val: bool);

    /// The number of columns represented by views in this row.
    #[objc::msg_send(numberOfColumns)]
    pub fn columns_n(&self) -> ns::Integer;

    /// The cell view at the given column, if present.
    #[objc::msg_send(viewAtColumn:)]
    pub fn view_at_column(&self, column: ns::Integer) -> Option<arc::R<ns::Id>>;
}
