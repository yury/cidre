use crate::{arc, define_obj_type, ns, objc};

/// NSCollectionViewAdditions
impl ns::IndexPath {
    #[objc::msg_send(indexPathForItem:section:)]
    pub fn with_item(item: ns::Integer, section: ns::Integer) -> arc::R<Self>;

    #[objc::msg_send(item)]
    pub fn item(&self) -> ns::Integer;

    #[objc::msg_send(section)]
    pub fn section(&self) -> ns::Integer;
}

define_obj_type!(
    pub CollectionView(ns::View)
);

#[objc::protocol(NSCollectionViewDataSource)]
pub trait CollectionViewDataSrc {
    #[objc::msg_send(collectionView:numberOfItemsInSection:)]
    fn collection_view_items_n_in_section(
        &self,
        collection_view: &ns::CollectionView,
        section: ns::Integer,
    ) -> ns::Integer;

    #[objc::msg_send(collectionView:itemForRepresentedObjectAtIndexPath:)]
    fn collection_view_item_for_represented_obj_at(
        &self,
        collection_view: &ns::CollectionView,
        index_path: ns::IndexPath,
    ) -> arc::R<ns::View>;

    #[objc::optional]
    #[objc::msg_send(numberOfSectionsInCollectionView:)]
    fn collection_view_sections_n(&self, collection_view: &ns::CollectionView) -> ns::Integer;
}

define_obj_type!(
    pub AnyCollectionViewDataSrc(ns::Id)
);

impl CollectionViewDataSrc for AnyCollectionViewDataSrc {}
