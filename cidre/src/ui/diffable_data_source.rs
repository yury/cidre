use std::marker::PhantomData;

use crate::{api, arc, define_cls, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

#[cfg(feature = "blocks")]
pub type CollectionViewDiffableDataSrcCellProvider<I> = blocks::EscBlock<
    fn(&ui::CollectionView, &ns::IndexPath, &I) -> Option<arc::R<ui::CollectionViewCell>>,
>;

#[cfg(feature = "blocks")]
pub type CollectionViewDiffableDataSrcSupplementaryViewProvider = blocks::EscBlock<
    fn(
        &ui::CollectionView,
        &ns::String,
        &ns::IndexPath,
    ) -> Option<arc::R<ui::CollectionReusableView>>,
>;

#[doc(alias = "NSDiffableDataSourceSectionTransaction")]
#[repr(transparent)]
pub struct DiffableDataSrcSectionTransaction<S, I>(ns::Id, PhantomData<S>, PhantomData<I>);

impl<S: objc::Obj, I: objc::Obj> objc::Obj for DiffableDataSrcSectionTransaction<S, I> {}

impl<S: objc::Obj, I: objc::Obj> DiffableDataSrcSectionTransaction<S, I> {
    #[objc::msg_send(sectionIdentifier)]
    pub fn section_id(&self) -> arc::R<S>;

    #[objc::msg_send(initialSnapshot)]
    pub fn initial_snapshot(&self) -> arc::R<ns::DiffableDataSrcSectionSnapshot<I>>;

    #[objc::msg_send(finalSnapshot)]
    pub fn final_snapshot(&self) -> arc::R<ns::DiffableDataSrcSectionSnapshot<I>>;

    #[objc::msg_send(difference)]
    pub fn difference(&self) -> arc::R<ns::OrderedCollectionDiff<I>>;
}

#[doc(alias = "NSDiffableDataSourceTransaction")]
#[repr(transparent)]
pub struct DiffableDataSrcTransaction<S, I>(ns::Id, PhantomData<S>, PhantomData<I>);

impl<S: objc::Obj, I: objc::Obj> objc::Obj for DiffableDataSrcTransaction<S, I> {}

impl<S: objc::Obj, I: objc::Obj> DiffableDataSrcTransaction<S, I> {
    #[objc::msg_send(initialSnapshot)]
    pub fn initial_snapshot(&self) -> arc::R<ns::DiffableDataSrcSnapshot<S, I>>;

    #[objc::msg_send(finalSnapshot)]
    pub fn final_snapshot(&self) -> arc::R<ns::DiffableDataSrcSnapshot<S, I>>;

    #[objc::msg_send(difference)]
    pub fn difference(&self) -> arc::R<ns::OrderedCollectionDiff<I>>;

    #[objc::msg_send(sectionTransactions)]
    pub fn section_transactions(
        &self,
    ) -> arc::R<ns::Array<DiffableDataSrcSectionTransaction<S, I>>>;
}

#[doc(alias = "UICollectionViewDiffableDataSourceReorderingHandlers")]
#[repr(transparent)]
pub struct CollectionViewDiffableDataSrcReorderingHandlers<S, I>(
    ns::Id,
    PhantomData<S>,
    PhantomData<I>,
);

impl<S: objc::Obj, I: objc::Obj> objc::Obj
    for CollectionViewDiffableDataSrcReorderingHandlers<S, I>
{
}

impl<S: objc::Obj, I: objc::Obj> arc::A<CollectionViewDiffableDataSrcReorderingHandlers<S, I>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<CollectionViewDiffableDataSrcReorderingHandlers<S, I>>;
}

impl<S: objc::Obj + 'static, I: objc::Obj + 'static>
    CollectionViewDiffableDataSrcReorderingHandlers<S, I>
{
    define_cls!(UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_REORDERING_HANDLERS);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(canReorderItemHandler)]
    pub fn can_reorder_item_handler(&self) -> Option<arc::R<blocks::EscBlock<fn(&I) -> bool>>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setCanReorderItemHandler:)]
    pub fn set_can_reorder_item_handler_block(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&I) -> bool>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_can_reorder_item_handler(&mut self, handler: impl FnMut(&I) -> bool + 'static) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_can_reorder_item_handler_block(Some(&mut handler));
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(willReorderHandler)]
    pub fn will_reorder_handler(
        &self,
    ) -> Option<arc::R<blocks::EscBlock<fn(&DiffableDataSrcTransaction<S, I>)>>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setWillReorderHandler:)]
    pub fn set_will_reorder_handler_block(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&DiffableDataSrcTransaction<S, I>)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_will_reorder_handler(
        &mut self,
        handler: impl FnMut(&DiffableDataSrcTransaction<S, I>) + 'static,
    ) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_will_reorder_handler_block(Some(&mut handler));
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(didReorderHandler)]
    pub fn did_reorder_handler(
        &self,
    ) -> Option<arc::R<blocks::EscBlock<fn(&DiffableDataSrcTransaction<S, I>)>>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setDidReorderHandler:)]
    pub fn set_did_reorder_handler_block(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&DiffableDataSrcTransaction<S, I>)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_did_reorder_handler(
        &mut self,
        handler: impl FnMut(&DiffableDataSrcTransaction<S, I>) + 'static,
    ) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_did_reorder_handler_block(Some(&mut handler));
    }
}

#[doc(alias = "UICollectionViewDiffableDataSourceSectionSnapshotHandlers")]
#[repr(transparent)]
pub struct CollectionViewDiffableDataSrcSectionSnapshotHandlers<I>(ns::Id, PhantomData<I>);

impl<I: objc::Obj> objc::Obj for CollectionViewDiffableDataSrcSectionSnapshotHandlers<I> {}

impl<I: objc::Obj> arc::A<CollectionViewDiffableDataSrcSectionSnapshotHandlers<I>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<CollectionViewDiffableDataSrcSectionSnapshotHandlers<I>>;
}

impl<I: objc::Obj + 'static> CollectionViewDiffableDataSrcSectionSnapshotHandlers<I> {
    define_cls!(UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT_HANDLERS);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(shouldExpandItemHandler)]
    pub fn should_expand_item_handler(&self) -> Option<arc::R<blocks::EscBlock<fn(&I) -> bool>>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setShouldExpandItemHandler:)]
    pub fn set_should_expand_item_handler_block(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&I) -> bool>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_should_expand_item_handler(&mut self, handler: impl FnMut(&I) -> bool + 'static) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_should_expand_item_handler_block(Some(&mut handler));
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(willExpandItemHandler)]
    pub fn will_expand_item_handler(&self) -> Option<arc::R<blocks::EscBlock<fn(&I)>>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setWillExpandItemHandler:)]
    pub fn set_will_expand_item_handler_block(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&I)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_will_expand_item_handler(&mut self, handler: impl FnMut(&I) + 'static) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_will_expand_item_handler_block(Some(&mut handler));
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(shouldCollapseItemHandler)]
    pub fn should_collapse_item_handler(&self) -> Option<arc::R<blocks::EscBlock<fn(&I) -> bool>>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setShouldCollapseItemHandler:)]
    pub fn set_should_collapse_item_handler_block(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&I) -> bool>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_should_collapse_item_handler(&mut self, handler: impl FnMut(&I) -> bool + 'static) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_should_collapse_item_handler_block(Some(&mut handler));
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(willCollapseItemHandler)]
    pub fn will_collapse_item_handler(&self) -> Option<arc::R<blocks::EscBlock<fn(&I)>>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setWillCollapseItemHandler:)]
    pub fn set_will_collapse_item_handler_block(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&I)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_will_collapse_item_handler(&mut self, handler: impl FnMut(&I) + 'static) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_will_collapse_item_handler_block(Some(&mut handler));
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(snapshotForExpandingParentItemHandler)]
    pub fn snapshot_for_expanding_parent_item_handler(
        &self,
    ) -> Option<
        arc::R<
            blocks::EscBlock<
                fn(
                    &I,
                    &ns::DiffableDataSrcSectionSnapshot<I>,
                ) -> arc::R<ns::DiffableDataSrcSectionSnapshot<I>>,
            >,
        >,
    >;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setSnapshotForExpandingParentItemHandler:)]
    pub fn set_snapshot_for_expanding_parent_item_handler_block(
        &mut self,
        handler: Option<
            &mut blocks::EscBlock<
                fn(
                    &I,
                    &ns::DiffableDataSrcSectionSnapshot<I>,
                ) -> arc::R<ns::DiffableDataSrcSectionSnapshot<I>>,
            >,
        >,
    );

    #[cfg(feature = "blocks")]
    pub fn set_snapshot_for_expanding_parent_item_handler(
        &mut self,
        handler: impl FnMut(
            &I,
            &ns::DiffableDataSrcSectionSnapshot<I>,
        ) -> arc::R<ns::DiffableDataSrcSectionSnapshot<I>>
        + 'static,
    ) {
        let mut handler = blocks::EscBlock::new2(handler);
        self.set_snapshot_for_expanding_parent_item_handler_block(Some(&mut handler));
    }
}

#[doc(alias = "UICollectionViewDiffableDataSource")]
#[repr(transparent)]
pub struct CollectionViewDiffableDataSrc<S, I>(ns::Id, PhantomData<S>, PhantomData<I>);

impl<S: objc::Obj, I: objc::Obj> objc::Obj for CollectionViewDiffableDataSrc<S, I> {}

impl<S: objc::Obj, I: objc::Obj> arc::A<CollectionViewDiffableDataSrc<S, I>> {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(initWithCollectionView:cellProvider:)]
    pub fn init_with_collection_view_cell_provider(
        self,
        collection_view: &ui::CollectionView,
        cell_provider: &mut CollectionViewDiffableDataSrcCellProvider<I>,
    ) -> arc::R<CollectionViewDiffableDataSrc<S, I>>;
}

impl<S: objc::Obj, I: objc::Obj> CollectionViewDiffableDataSrc<S, I> {
    define_cls!(UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE);

    #[cfg(feature = "blocks")]
    pub fn with_collection_view_cell_provider(
        collection_view: &ui::CollectionView,
        cell_provider: impl FnMut(
            &ui::CollectionView,
            &ns::IndexPath,
            &I,
        ) -> Option<arc::R<ui::CollectionViewCell>>
        + 'static,
    ) -> arc::R<Self> {
        let mut cell_provider = CollectionViewDiffableDataSrcCellProvider::new3(cell_provider);
        Self::alloc().init_with_collection_view_cell_provider(collection_view, &mut cell_provider)
    }

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
            &ui::CollectionView,
            &ns::String,
            &ns::IndexPath,
        ) -> Option<arc::R<ui::CollectionReusableView>>
        + 'static,
    ) {
        let mut provider = CollectionViewDiffableDataSrcSupplementaryViewProvider::new3(provider);
        self.set_supplementary_view_provider_block(Some(&mut provider));
    }

    #[objc::msg_send(snapshot)]
    pub fn snapshot(&self) -> arc::R<ns::DiffableDataSrcSnapshot<S, I>>;

    #[objc::msg_send(applySnapshot:animatingDifferences:)]
    pub fn apply_snapshot_animating_differences(
        &mut self,
        snapshot: &ns::DiffableDataSrcSnapshot<S, I>,
        animating_differences: bool,
    );

    #[cfg(feature = "blocks")]
    #[objc::msg_send(applySnapshot:animatingDifferences:completion:)]
    pub fn apply_snapshot_animating_differences_completion(
        &mut self,
        snapshot: &ns::DiffableDataSrcSnapshot<S, I>,
        animating_differences: bool,
        completion: Option<&mut blocks::CompletionBlock>,
    );

    #[cfg(feature = "blocks")]
    pub fn apply_snapshot_animating_differences_with_completion(
        &mut self,
        snapshot: &ns::DiffableDataSrcSnapshot<S, I>,
        animating_differences: bool,
        completion: impl FnMut() + 'static,
    ) {
        let mut completion = blocks::CompletionBlock::new0(completion);
        self.apply_snapshot_animating_differences_completion(
            snapshot,
            animating_differences,
            Some(&mut completion),
        );
    }

    #[api::available(ios = 15.0, tvos = 15.0)]
    #[objc::msg_send(applySnapshotUsingReloadData:)]
    pub fn apply_snapshot_using_reload_data(
        &mut self,
        snapshot: &ns::DiffableDataSrcSnapshot<S, I>,
    );

    #[cfg(feature = "blocks")]
    #[api::available(ios = 15.0, tvos = 15.0)]
    #[objc::msg_send(applySnapshotUsingReloadData:completion:)]
    pub fn apply_snapshot_using_reload_data_completion(
        &mut self,
        snapshot: &ns::DiffableDataSrcSnapshot<S, I>,
        completion: Option<&mut blocks::CompletionBlock>,
    );

    #[cfg(feature = "blocks")]
    #[api::available(ios = 15.0, tvos = 15.0)]
    pub fn apply_snapshot_using_reload_data_with_completion(
        &mut self,
        snapshot: &ns::DiffableDataSrcSnapshot<S, I>,
        completion: impl FnMut() + 'static,
    ) {
        let mut completion = blocks::CompletionBlock::new0(completion);
        self.apply_snapshot_using_reload_data_completion(snapshot, Some(&mut completion));
    }

    #[api::available(ios = 15.0, tvos = 15.0)]
    #[objc::msg_send(sectionIdentifierForIndex:)]
    pub fn section_id_for_index(&self, index: ns::Integer) -> Option<arc::R<S>>;

    #[api::available(ios = 15.0, tvos = 15.0)]
    #[objc::msg_send(indexForSectionIdentifier:)]
    pub fn index_for_section_id(&self, section_id: &S) -> ns::Integer;

    #[objc::msg_send(itemIdentifierForIndexPath:)]
    pub fn item_id_for_index_path(&self, index_path: &ns::IndexPath) -> Option<arc::R<I>>;

    #[objc::msg_send(indexPathForItemIdentifier:)]
    pub fn index_path_for_item_id(&self, item_id: &I) -> Option<arc::R<ns::IndexPath>>;

    #[api::available(ios = 14.0, tvos = 14.0)]
    #[objc::msg_send(reorderingHandlers)]
    pub fn reordering_handlers(
        &self,
    ) -> arc::R<CollectionViewDiffableDataSrcReorderingHandlers<S, I>>;

    #[api::available(ios = 14.0, tvos = 14.0)]
    #[objc::msg_send(setReorderingHandlers:)]
    pub fn set_reordering_handlers(
        &mut self,
        handlers: &CollectionViewDiffableDataSrcReorderingHandlers<S, I>,
    );

    #[api::available(ios = 14.0, tvos = 14.0)]
    #[objc::msg_send(applySnapshot:toSection:animatingDifferences:)]
    pub fn apply_section_snapshot_animating_differences(
        &mut self,
        snapshot: &ns::DiffableDataSrcSectionSnapshot<I>,
        section_id: &S,
        animating_differences: bool,
    );

    #[cfg(feature = "blocks")]
    #[api::available(ios = 14.0, tvos = 14.0)]
    #[objc::msg_send(applySnapshot:toSection:animatingDifferences:completion:)]
    pub fn apply_section_snapshot_animating_differences_completion(
        &mut self,
        snapshot: &ns::DiffableDataSrcSectionSnapshot<I>,
        section_id: &S,
        animating_differences: bool,
        completion: Option<&mut blocks::CompletionBlock>,
    );

    #[cfg(feature = "blocks")]
    #[api::available(ios = 14.0, tvos = 14.0)]
    pub fn apply_section_snapshot_animating_differences_with_completion(
        &mut self,
        snapshot: &ns::DiffableDataSrcSectionSnapshot<I>,
        section_id: &S,
        animating_differences: bool,
        completion: impl FnMut() + 'static,
    ) {
        let mut completion = blocks::CompletionBlock::new0(completion);
        self.apply_section_snapshot_animating_differences_completion(
            snapshot,
            section_id,
            animating_differences,
            Some(&mut completion),
        );
    }

    #[api::available(ios = 14.0, tvos = 14.0)]
    #[objc::msg_send(snapshotForSection:)]
    pub fn snapshot_for_section(
        &self,
        section_id: &S,
    ) -> arc::R<ns::DiffableDataSrcSectionSnapshot<I>>;

    #[api::available(ios = 14.0, tvos = 14.0)]
    #[objc::msg_send(sectionSnapshotHandlers)]
    pub fn section_snapshot_handlers(
        &self,
    ) -> arc::R<CollectionViewDiffableDataSrcSectionSnapshotHandlers<I>>;

    #[api::available(ios = 14.0, tvos = 14.0)]
    #[objc::msg_send(setSectionSnapshotHandlers:)]
    pub fn set_section_snapshot_handlers(
        &mut self,
        handlers: &CollectionViewDiffableDataSrcSectionSnapshotHandlers<I>,
    );
}

impl<S: objc::Obj, I: objc::Obj> ui::CollectionViewDataSrc for CollectionViewDiffableDataSrc<S, I> {}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE:
        &'static objc::Class<CollectionViewDiffableDataSrc<ns::Id, ns::Id>>;
    static UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_REORDERING_HANDLERS:
        &'static objc::Class<CollectionViewDiffableDataSrcReorderingHandlers<ns::Id, ns::Id>>;
    static UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT_HANDLERS:
        &'static objc::Class<CollectionViewDiffableDataSrcSectionSnapshotHandlers<ns::Id>>;
}
