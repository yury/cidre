use crate::{arc, ns, objc};

/// NSCollectionViewAdditions
impl ns::IndexPath {
    #[objc::msg_send(indexPathForItem:section:)]
    pub fn with_item(item: ns::Integer, section: ns::Integer) -> arc::R<Self>;

    #[objc::msg_send(item)]
    pub fn item(&self) -> ns::Integer;

    #[objc::msg_send(section)]
    pub fn section(&self) -> ns::Integer;
}
