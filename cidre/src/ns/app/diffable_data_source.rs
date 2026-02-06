use std::marker::PhantomData;

use crate::{arc, define_cls, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[cfg(feature = "blocks")]
pub type CollectionViewDiffableDataSrcItemProvider<I> = blocks::EscBlock<
    fn(&ns::CollectionView, &ns::IndexPath, &I) -> Option<arc::R<ns::CollectionViewItem>>,
>;

#[cfg(feature = "blocks")]
pub type CollectionViewDiffableDataSrcSupplementaryViewProvider = blocks::EscBlock<
    fn(
        &ns::CollectionView,
        &ns::CollectionViewElementKind,
        &ns::IndexPath,
    ) -> Option<arc::R<ns::View>>,
>;

#[doc(alias = "NSCollectionViewDiffableDataSource")]
#[repr(transparent)]
pub struct CollectionViewDiffableDataSrc<S, I>(ns::Id, PhantomData<S>, PhantomData<I>);

impl<S: objc::Obj, I: objc::Obj> objc::Obj for CollectionViewDiffableDataSrc<S, I> {}

impl<S: objc::Obj, I: objc::Obj> arc::A<CollectionViewDiffableDataSrc<S, I>> {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(initWithCollectionView:itemProvider:)]
    pub fn init_with_collection_view_item_provider(
        self,
        collection_view: &ns::CollectionView,
        item_provider: &mut CollectionViewDiffableDataSrcItemProvider<I>,
    ) -> arc::R<CollectionViewDiffableDataSrc<S, I>>;
}

impl<S: objc::Obj, I: objc::Obj> CollectionViewDiffableDataSrc<S, I> {
    define_cls!(NS_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE);

    #[cfg(feature = "blocks")]
    pub fn with_collection_view_item_provider(
        collection_view: &ns::CollectionView,
        item_provider: impl FnMut(
            &ns::CollectionView,
            &ns::IndexPath,
            &I,
        ) -> Option<arc::R<ns::CollectionViewItem>>
        + 'static,
    ) -> arc::R<Self> {
        let mut item_provider = CollectionViewDiffableDataSrcItemProvider::new3(item_provider);
        Self::alloc().init_with_collection_view_item_provider(collection_view, &mut item_provider)
    }

    #[objc::msg_send(snapshot)]
    pub fn snapshot(&self) -> arc::R<ns::DiffableDataSrcSnapshot<S, I>>;

    #[objc::msg_send(applySnapshot:animatingDifferences:)]
    pub fn apply_snapshot_animating_differences(
        &mut self,
        snapshot: &ns::DiffableDataSrcSnapshot<S, I>,
        animating_differences: bool,
    );

    #[objc::msg_send(itemIdentifierForIndexPath:)]
    pub fn item_id_for_index_path(&self, index_path: &ns::IndexPath) -> Option<arc::R<I>>;

    #[objc::msg_send(indexPathForItemIdentifier:)]
    pub fn index_path_for_item_id(&self, item_id: &I) -> Option<arc::R<ns::IndexPath>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(supplementaryViewProvider)]
    pub fn supplementary_view_provider(
        &self,
    ) -> Option<arc::R<CollectionViewDiffableDataSrcSupplementaryViewProvider>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setSupplementaryViewProvider:)]
    pub fn set_supplementary_view_provider_block(
        &mut self,
        provider: Option<&mut CollectionViewDiffableDataSrcSupplementaryViewProvider>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_supplementary_view_provider(
        &mut self,
        provider: impl FnMut(
            &ns::CollectionView,
            &ns::CollectionViewElementKind,
            &ns::IndexPath,
        ) -> Option<arc::R<ns::View>>
        + 'static,
    ) {
        let mut provider = CollectionViewDiffableDataSrcSupplementaryViewProvider::new3(provider);
        self.set_supplementary_view_provider_block(Some(&mut provider));
    }
}

impl<S: objc::Obj, I: objc::Obj> ns::CollectionViewDataSrc for CollectionViewDiffableDataSrc<S, I> {}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE:
        &'static objc::Class<CollectionViewDiffableDataSrc<ns::Id, ns::Id>>;
}
