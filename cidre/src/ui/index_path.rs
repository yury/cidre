use crate::{arc, ns, objc};

/// UIKitAdditions
///
/// This category provides convenience methods to make it easier to use an ns::IndexPath to represent a section and row/item,
/// for use with ui::TableView and ui::CollectionView.
impl ns::IndexPath {
    #[objc::msg_send(section)]
    pub fn section(&self) -> ns::Integer;

    #[objc::msg_send(row)]
    pub fn row(&self) -> ns::Integer;

    #[objc::msg_send(item)]
    pub fn item(&self) -> ns::Integer;

    #[objc::msg_send(indexPathForRow:inSection:)]
    pub fn with_row(row: ns::Integer, section: ns::Integer) -> arc::R<Self>;

    #[objc::msg_send(indexPathForItem:inSection:)]
    pub fn with_item(item: ns::Integer, section: ns::Integer) -> arc::R<Self>;
}
