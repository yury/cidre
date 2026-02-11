use crate::{arc, define_obj_type, define_opts, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "NSCollectionViewDropOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CollectionViewDropOp {
    DropOn = 0,
    DropBefore = 1,
}

#[doc(alias = "NSCollectionViewItemHighlightState")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CollectionViewItemHighlightState {
    None = 0,
    ForSelection = 1,
    ForDeselection = 2,
    AsDropTarget = 3,
}

define_opts!(
    #[doc(alias = "NSCollectionViewScrollPosition")]
    pub CollectionViewScrollPos(usize)
);

impl CollectionViewScrollPos {
    pub const NONE: Self = Self(0);
    pub const TOP: Self = Self(1 << 0);
    pub const CENTERED_VERTICALLY: Self = Self(1 << 1);
    pub const BOTTOM: Self = Self(1 << 2);
    pub const LEFT: Self = Self(1 << 3);
    pub const CENTERED_HORIZONTALLY: Self = Self(1 << 4);
    pub const RIGHT: Self = Self(1 << 5);
    pub const LEADING_EDGE: Self = Self(1 << 6);
    pub const TRAILING_EDGE: Self = Self(1 << 7);
    pub const NEAREST_VERTICAL_EDGE: Self = Self(1 << 8);
    pub const NEAREST_HORIZONTAL_EDGE: Self = Self(1 << 9);
}

#[doc(alias = "NSCollectionViewSupplementaryElementKind")]
pub type CollectionViewElementKind = ns::String;

/// NSCollectionViewAdditions
impl ns::IndexPath {
    #[objc::msg_send(indexPathForItem:inSection:)]
    pub fn with_item(item: ns::Integer, section: ns::Integer) -> arc::R<Self>;

    #[objc::msg_send(item)]
    pub fn item(&self) -> ns::Integer;

    #[objc::msg_send(section)]
    pub fn section(&self) -> ns::Integer;
}

/// NSCollectionViewAdditions
impl ns::Set<ns::IndexPath> {
    #[objc::msg_send(setWithCollectionViewIndexPath:)]
    pub fn with_collection_view_index_path(index_path: &ns::IndexPath) -> arc::R<Self>;

    #[objc::msg_send(setWithCollectionViewIndexPaths:)]
    pub fn with_collection_view_index_paths(index_paths: &ns::Array<ns::IndexPath>)
    -> arc::R<Self>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(enumerateIndexPathsWithOptions:usingBlock:)]
    pub fn enumerate_index_paths_with_opts(
        &self,
        opts: ns::EnumerationOpts,
        block: &mut blocks::NoEscBlock<fn(&ns::IndexPath, &mut bool)>,
    );
}

#[objc::protocol(NSCollectionViewElement)]
pub trait CollectionViewElement: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(prepareForReuse)]
    fn prepare_for_reuse(&mut self);

    #[objc::optional]
    #[objc::msg_send(applyLayoutAttributes:)]
    fn apply_layout_attributes(&mut self, layout_attributes: &ns::CollectionViewLayoutAttrs);

    #[objc::optional]
    #[objc::msg_send(willTransitionFromLayout:toLayout:)]
    fn will_transition_from_layout(
        &mut self,
        old_layout: &ns::CollectionViewLayout,
        new_layout: &ns::CollectionViewLayout,
    );

    #[objc::optional]
    #[objc::msg_send(didTransitionFromLayout:toLayout:)]
    fn did_transition_from_layout(
        &mut self,
        old_layout: &ns::CollectionViewLayout,
        new_layout: &ns::CollectionViewLayout,
    );

    #[objc::optional]
    #[objc::msg_send(preferredLayoutAttributesFittingAttributes:)]
    fn preferred_layout_attributes_fitting_attributes(
        &mut self,
        layout_attributes: &ns::CollectionViewLayoutAttrs,
    ) -> arc::R<ns::CollectionViewLayoutAttrs>;
}

#[objc::protocol(NSCollectionViewSectionHeaderView)]
pub trait CollectionViewSectionHeaderView: CollectionViewElement {
    #[objc::optional]
    #[objc::msg_send(sectionCollapseButton)]
    fn section_collapse_button(&self) -> Option<arc::R<ns::Button>>;

    #[objc::optional]
    #[objc::msg_send(setSectionCollapseButton:)]
    fn set_section_collapse_button(&mut self, val: Option<&ns::Button>);
}

define_obj_type!(
    #[doc(alias = "NSCollectionView")]
    pub CollectionView(ns::View),
    NS_COLLECTION_VIEW
);

impl CollectionView {
    #[objc::msg_send(dataSource)]
    pub fn data_src(&self) -> Option<arc::R<AnyCollectionViewDataSrc>>;

    #[objc::msg_send(setDataSource:)]
    pub fn set_data_src<D: CollectionViewDataSrc>(&mut self, val: Option<&D>);

    #[objc::msg_send(prefetchDataSource)]
    pub fn prefetch_data_src(&self) -> Option<arc::R<AnyCollectionViewPrefetching>>;

    #[objc::msg_send(setPrefetchDataSource:)]
    pub fn set_prefetch_data_src<D: CollectionViewPrefetching>(&mut self, val: Option<&D>);

    #[objc::msg_send(content)]
    pub fn content(&self) -> arc::R<ns::Array<ns::Id>>;

    #[objc::msg_send(setContent:)]
    pub fn set_content(&mut self, val: &ns::Array<ns::Id>);

    #[objc::msg_send(reloadData)]
    pub fn reload_data(&mut self);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyCollectionViewDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: CollectionViewDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(backgroundView)]
    pub fn bg_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setBackgroundView:)]
    pub fn set_bg_view(&mut self, val: Option<&ns::View>);

    #[objc::msg_send(backgroundViewScrollsWithContent)]
    pub fn bg_view_scrolls_with_content(&self) -> bool;

    #[objc::msg_send(setBackgroundViewScrollsWithContent:)]
    pub fn set_bg_view_scrolls_with_content(&mut self, val: bool);

    #[objc::msg_send(collectionViewLayout)]
    pub fn collection_view_layout(&self) -> Option<arc::R<ns::CollectionViewLayout>>;

    #[objc::msg_send(setCollectionViewLayout:)]
    pub fn set_collection_view_layout(&mut self, val: Option<&ns::CollectionViewLayout>);

    #[objc::msg_send(layoutAttributesForItemAtIndexPath:)]
    pub fn layout_attributes_for_item_at_index_path(
        &self,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ns::CollectionViewLayoutAttrs>>;

    #[objc::msg_send(layoutAttributesForSupplementaryElementOfKind:atIndexPath:)]
    pub fn layout_attributes_for_supplementary_element_of_kind_at_index_path(
        &self,
        kind: &ns::CollectionViewElementKind,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ns::CollectionViewLayoutAttrs>>;

    #[objc::msg_send(frameForItemAtIndex:)]
    pub fn frame_for_item_at_index(&self, index: usize) -> ns::Rect;

    #[objc::msg_send(frameForItemAtIndex:withNumberOfItems:)]
    pub fn frame_for_item_at_index_with_number_of_items(
        &self,
        index: usize,
        number_of_items: usize,
    ) -> ns::Rect;

    #[objc::msg_send(backgroundColors)]
    pub fn bg_colors(&self) -> arc::R<ns::Array<ns::Color>>;

    #[objc::msg_send(setBackgroundColors:)]
    pub fn set_bg_colors(&mut self, val: Option<&ns::Array<ns::Color>>);

    #[objc::msg_send(numberOfSections)]
    pub fn sections_n(&self) -> ns::Integer;

    #[objc::msg_send(numberOfItemsInSection:)]
    pub fn items_n_in_section(&self, section: ns::Integer) -> ns::Integer;

    #[objc::msg_send(isFirstResponder)]
    pub fn is_first_responder(&self) -> bool;

    #[objc::msg_send(isSelectable)]
    pub fn is_selectable(&self) -> bool;

    #[objc::msg_send(setSelectable:)]
    pub fn set_selectable(&mut self, val: bool);

    #[objc::msg_send(allowsEmptySelection)]
    pub fn allows_empty_selection(&self) -> bool;

    #[objc::msg_send(setAllowsEmptySelection:)]
    pub fn set_allows_empty_selection(&mut self, val: bool);

    #[objc::msg_send(allowsMultipleSelection)]
    pub fn allows_multiple_selection(&self) -> bool;

    #[objc::msg_send(setAllowsMultipleSelection:)]
    pub fn set_allows_multiple_selection(&mut self, val: bool);

    #[objc::msg_send(selectionIndexes)]
    pub fn selection_indexes(&self) -> arc::R<ns::IndexSet>;

    #[objc::msg_send(setSelectionIndexes:)]
    pub fn set_selection_indexes(&mut self, val: &ns::IndexSet);

    #[objc::msg_send(selectionIndexPaths)]
    pub fn selection_index_paths(&self) -> arc::R<ns::Set<ns::IndexPath>>;

    #[objc::msg_send(setSelectionIndexPaths:)]
    pub fn set_selection_index_paths(&mut self, val: &ns::Set<ns::IndexPath>);

    #[objc::msg_send(selectItemsAtIndexPaths:scrollPosition:)]
    pub fn select_items_at_index_paths_scroll_position(
        &mut self,
        index_paths: &ns::Set<ns::IndexPath>,
        scroll_position: CollectionViewScrollPos,
    );

    #[objc::msg_send(deselectItemsAtIndexPaths:)]
    pub fn deselect_items_at_index_paths(&mut self, index_paths: &ns::Set<ns::IndexPath>);

    #[objc::msg_send(selectAll:)]
    pub fn select_all(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(deselectAll:)]
    pub fn deselect_all(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(registerClass:forItemWithIdentifier:)]
    pub fn register_class_for_item_with_id(
        &mut self,
        item_class: Option<&objc::Class<ns::Id>>,
        identifier: &ns::UiItemId,
    );

    #[objc::msg_send(registerNib:forItemWithIdentifier:)]
    pub fn register_nib_for_item_with_id(
        &mut self,
        nib: Option<&ns::Id>,
        identifier: &ns::UiItemId,
    );

    #[objc::msg_send(registerClass:forSupplementaryViewOfKind:withIdentifier:)]
    pub fn register_class_for_supplementary_view_of_kind_with_id(
        &mut self,
        view_class: Option<&objc::Class<ns::Id>>,
        kind: &ns::CollectionViewElementKind,
        identifier: &ns::UiItemId,
    );

    #[objc::msg_send(registerNib:forSupplementaryViewOfKind:withIdentifier:)]
    pub fn register_nib_for_supplementary_view_of_kind_with_id(
        &mut self,
        nib: Option<&ns::Id>,
        kind: &ns::CollectionViewElementKind,
        identifier: &ns::UiItemId,
    );

    #[objc::msg_send(makeItemWithIdentifier:forIndexPath:)]
    pub fn make_item_with_id_for_index_path(
        &self,
        identifier: &ns::UiItemId,
        index_path: &ns::IndexPath,
    ) -> arc::R<ns::CollectionViewItem>;

    #[objc::msg_send(makeSupplementaryViewOfKind:withIdentifier:forIndexPath:)]
    pub fn make_supplementary_view_of_kind_with_id_for_index_path(
        &self,
        element_kind: &ns::CollectionViewElementKind,
        identifier: &ns::UiItemId,
        index_path: &ns::IndexPath,
    ) -> arc::R<ns::View>;

    #[objc::msg_send(itemAtIndex:)]
    pub fn item_at_index(&self, index: usize) -> Option<arc::R<ns::CollectionViewItem>>;

    #[objc::msg_send(itemAtIndexPath:)]
    pub fn item_at_index_path(
        &self,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ns::CollectionViewItem>>;

    #[objc::msg_send(visibleItems)]
    pub fn visible_items(&self) -> arc::R<ns::Array<ns::CollectionViewItem>>;

    #[objc::msg_send(indexPathsForVisibleItems)]
    pub fn index_paths_for_visible_items(&self) -> arc::R<ns::Set<ns::IndexPath>>;

    #[objc::msg_send(indexPathForItem:)]
    pub fn index_path_for_item(
        &self,
        item: &ns::CollectionViewItem,
    ) -> Option<arc::R<ns::IndexPath>>;

    #[objc::msg_send(indexPathForItemAtPoint:)]
    pub fn index_path_for_item_at_point(&self, point: ns::Point) -> Option<arc::R<ns::IndexPath>>;

    #[objc::msg_send(supplementaryViewForElementKind:atIndexPath:)]
    pub fn supplementary_view_for_element_kind_at_index_path(
        &self,
        element_kind: &ns::CollectionViewElementKind,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(visibleSupplementaryViewsOfKind:)]
    pub fn visible_supplementary_views_of_kind(
        &self,
        element_kind: &ns::CollectionViewElementKind,
    ) -> arc::R<ns::Array<ns::View>>;

    #[objc::msg_send(indexPathsForVisibleSupplementaryElementsOfKind:)]
    pub fn index_paths_for_visible_supplementary_elements_of_kind(
        &self,
        element_kind: &ns::CollectionViewElementKind,
    ) -> arc::R<ns::Set<ns::IndexPath>>;

    #[objc::msg_send(insertSections:)]
    pub fn insert_sections(&mut self, sections: &ns::IndexSet);

    #[objc::msg_send(deleteSections:)]
    pub fn delete_sections(&mut self, sections: &ns::IndexSet);

    #[objc::msg_send(reloadSections:)]
    pub fn reload_sections(&mut self, sections: &ns::IndexSet);

    #[objc::msg_send(moveSection:toSection:)]
    pub fn move_section_to_section(&mut self, section: ns::Integer, new_section: ns::Integer);

    #[objc::msg_send(insertItemsAtIndexPaths:)]
    pub fn insert_items_at_index_paths(&mut self, index_paths: &ns::Set<ns::IndexPath>);

    #[objc::msg_send(deleteItemsAtIndexPaths:)]
    pub fn delete_items_at_index_paths(&mut self, index_paths: &ns::Set<ns::IndexPath>);

    #[objc::msg_send(reloadItemsAtIndexPaths:)]
    pub fn reload_items_at_index_paths(&mut self, index_paths: &ns::Set<ns::IndexPath>);

    #[objc::msg_send(moveItemAtIndexPath:toIndexPath:)]
    pub fn move_item_at_index_path_to_index_path(
        &mut self,
        index_path: &ns::IndexPath,
        new_index_path: &ns::IndexPath,
    );

    #[cfg(feature = "blocks")]
    #[objc::msg_send(performBatchUpdates:completionHandler:)]
    pub fn perform_batch_updates_ch(
        &mut self,
        updates: Option<&mut blocks::NoEscBlock<fn()>>,
        completion: Option<&mut blocks::EscBlock<fn(bool)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn perform_batch_updates(&mut self, mut updates: impl FnMut()) {
        let mut updates = unsafe { blocks::NoEscBlock::stack0(&mut updates) };
        self.perform_batch_updates_ch(Some(&mut updates), None);
    }

    #[cfg(feature = "blocks")]
    pub fn perform_batch_updates_with_completion(
        &mut self,
        mut updates: impl FnMut(),
        completion: impl FnMut(bool) + 'static,
    ) {
        let mut updates = unsafe { blocks::NoEscBlock::stack0(&mut updates) };
        let mut completion = blocks::EscBlock::new1(completion);
        self.perform_batch_updates_ch(Some(&mut updates), Some(&mut completion));
    }

    #[objc::msg_send(toggleSectionCollapse:)]
    pub fn toggle_section_collapse(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(scrollToItemsAtIndexPaths:scrollPosition:)]
    pub fn scroll_to_items_at_index_paths_scroll_position(
        &mut self,
        index_paths: &ns::Set<ns::IndexPath>,
        scroll_position: CollectionViewScrollPos,
    );

    #[objc::msg_send(setDraggingSourceOperationMask:forLocal:)]
    pub fn set_dragging_source_operation_mask_for_local(
        &mut self,
        drag_operation_mask: ns::UInteger,
        local_destination: bool,
    );

    #[objc::msg_send(draggingImageForItemsAtIndexPaths:withEvent:offset:)]
    pub fn dragging_image_for_items_at_index_paths_with_event_offset(
        &self,
        index_paths: &ns::Set<ns::IndexPath>,
        event: &ns::Event,
        drag_image_offset: *mut ns::Point,
    ) -> arc::R<ns::Image>;

    #[objc::msg_send(draggingImageForItemsAtIndexes:withEvent:offset:)]
    pub fn dragging_image_for_items_at_indexes_with_event_offset(
        &self,
        indexes: &ns::IndexSet,
        event: &ns::Event,
        drag_image_offset: *mut ns::Point,
    ) -> arc::R<ns::Image>;
}

impl ns::AnimatablePropContainer for CollectionView {}

#[objc::protocol(NSCollectionViewDataSource)]
pub trait CollectionViewDataSrc: objc::Obj {
    #[objc::msg_send(collectionView:numberOfItemsInSection:)]
    fn collection_view_items_n_in_section(
        &mut self,
        collection_view: &mut ns::CollectionView,
        section: ns::Integer,
    ) -> ns::Integer;

    #[objc::msg_send(collectionView:itemForRepresentedObjectAtIndexPath:)]
    fn collection_view_item_for_represented_obj_at(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_path: &ns::IndexPath,
    ) -> arc::R<ns::CollectionViewItem>;

    #[objc::optional]
    #[objc::msg_send(numberOfSectionsInCollectionView:)]
    fn collection_view_sections_n(
        &mut self,
        collection_view: &mut ns::CollectionView,
    ) -> ns::Integer;

    #[objc::optional]
    #[objc::msg_send(collectionView:viewForSupplementaryElementOfKind:atIndexPath:)]
    fn collection_view_view_for_supplementary_element_of_kind_at_index_path(
        &mut self,
        collection_view: &mut ns::CollectionView,
        kind: &ns::CollectionViewElementKind,
        index_path: &ns::IndexPath,
    ) -> arc::R<ns::View>;
}

define_obj_type!(
    pub AnyCollectionViewDataSrc(ns::Id)
);

impl CollectionViewDataSrc for AnyCollectionViewDataSrc {}

#[objc::protocol(NSCollectionViewPrefetching)]
pub trait CollectionViewPrefetching: objc::Obj {
    #[objc::msg_send(collectionView:prefetchItemsAtIndexPaths:)]
    fn collection_view_prefetch_items_at_index_paths(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Array<ns::IndexPath>,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:cancelPrefetchingForItemsAtIndexPaths:)]
    fn collection_view_cancel_prefetching_for_items_at_index_paths(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Array<ns::IndexPath>,
    );
}

define_obj_type!(
    pub AnyCollectionViewPrefetching(ns::Id)
);

impl CollectionViewPrefetching for AnyCollectionViewPrefetching {}

#[objc::protocol(NSCollectionViewDelegate)]
pub trait CollectionViewDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(collectionView:canDragItemsAtIndexPaths:withEvent:)]
    fn collection_view_can_drag_items_at_index_paths_with_event(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
        event: &ns::Event,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:canDragItemsAtIndexes:withEvent:)]
    fn collection_view_can_drag_items_at_indexes_with_event(
        &mut self,
        collection_view: &mut ns::CollectionView,
        indexes: &ns::IndexSet,
        event: &ns::Event,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:writeItemsAtIndexPaths:toPasteboard:)]
    fn collection_view_write_items_at_index_paths_to_pasteboard(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
        pasteboard: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:writeItemsAtIndexes:toPasteboard:)]
    fn collection_view_write_items_at_indexes_to_pasteboard(
        &mut self,
        collection_view: &mut ns::CollectionView,
        indexes: &ns::IndexSet,
        pasteboard: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:namesOfPromisedFilesDroppedAtDestination:forDraggedItemsAtIndexPaths:)]
    fn collection_view_names_of_promised_files_dropped_at_destination_for_dragged_items_at_index_paths(
        &mut self,
        collection_view: &mut ns::CollectionView,
        drop_url: &ns::Url,
        index_paths: &ns::Set<ns::IndexPath>,
    ) -> arc::R<ns::Array<ns::String>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:namesOfPromisedFilesDroppedAtDestination:forDraggedItemsAtIndexes:)]
    fn collection_view_names_of_promised_files_dropped_at_destination_for_dragged_items_at_indexes(
        &mut self,
        collection_view: &mut ns::CollectionView,
        drop_url: &ns::Url,
        indexes: &ns::IndexSet,
    ) -> arc::R<ns::Array<ns::String>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:draggingImageForItemsAtIndexPaths:withEvent:offset:)]
    fn collection_view_dragging_image_for_items_at_index_paths_with_event_offset(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
        event: &ns::Event,
        drag_image_offset: *mut ns::Point,
    ) -> arc::R<ns::Image>;

    #[objc::optional]
    #[objc::msg_send(collectionView:draggingImageForItemsAtIndexes:withEvent:offset:)]
    fn collection_view_dragging_image_for_items_at_indexes_with_event_offset(
        &mut self,
        collection_view: &mut ns::CollectionView,
        indexes: &ns::IndexSet,
        event: &ns::Event,
        drag_image_offset: *mut ns::Point,
    ) -> arc::R<ns::Image>;

    #[objc::optional]
    #[objc::msg_send(collectionView:validateDrop:proposedIndexPath:dropOperation:)]
    fn collection_view_validate_drop_proposed_index_path_drop_operation(
        &mut self,
        collection_view: &mut ns::CollectionView,
        dragging_info: &ns::Id,
        proposed_drop_index_path: *mut *mut ns::IndexPath,
        proposed_drop_operation: *mut CollectionViewDropOp,
    ) -> ns::UInteger;

    #[objc::optional]
    #[objc::msg_send(collectionView:validateDrop:proposedIndex:dropOperation:)]
    fn collection_view_validate_drop_proposed_index_drop_operation(
        &mut self,
        collection_view: &mut ns::CollectionView,
        dragging_info: &ns::Id,
        proposed_drop_index: *mut ns::Integer,
        proposed_drop_operation: *mut CollectionViewDropOp,
    ) -> ns::UInteger;

    #[objc::optional]
    #[objc::msg_send(collectionView:acceptDrop:indexPath:dropOperation:)]
    fn collection_view_accept_drop_index_path_drop_operation(
        &mut self,
        collection_view: &mut ns::CollectionView,
        dragging_info: &ns::Id,
        index_path: &ns::IndexPath,
        drop_operation: CollectionViewDropOp,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:acceptDrop:index:dropOperation:)]
    fn collection_view_accept_drop_index_drop_operation(
        &mut self,
        collection_view: &mut ns::CollectionView,
        dragging_info: &ns::Id,
        index: ns::Integer,
        drop_operation: CollectionViewDropOp,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:pasteboardWriterForItemAtIndexPath:)]
    fn collection_view_pasteboard_writer_for_item_at_index_path(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ns::Id>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:pasteboardWriterForItemAtIndex:)]
    fn collection_view_pasteboard_writer_for_item_at_index(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index: usize,
    ) -> Option<arc::R<ns::Id>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:draggingSession:willBeginAtPoint:forItemsAtIndexPaths:)]
    fn collection_view_dragging_session_will_begin_at_point_for_items_at_index_paths(
        &mut self,
        collection_view: &mut ns::CollectionView,
        session: &ns::Id,
        screen_point: ns::Point,
        index_paths: &ns::Set<ns::IndexPath>,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:draggingSession:willBeginAtPoint:forItemsAtIndexes:)]
    fn collection_view_dragging_session_will_begin_at_point_for_items_at_indexes(
        &mut self,
        collection_view: &mut ns::CollectionView,
        session: &ns::Id,
        screen_point: ns::Point,
        indexes: &ns::IndexSet,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:draggingSession:endedAtPoint:dragOperation:)]
    fn collection_view_dragging_session_ended_at_point_drag_operation(
        &mut self,
        collection_view: &mut ns::CollectionView,
        session: &ns::Id,
        screen_point: ns::Point,
        operation: ns::UInteger,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:updateDraggingItemsForDrag:)]
    fn collection_view_update_dragging_items_for_drag(
        &mut self,
        collection_view: &mut ns::CollectionView,
        dragging_info: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldChangeItemsAtIndexPaths:toHighlightState:)]
    fn collection_view_should_change_items_at_index_paths_to_highlight_state(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
        highlight_state: CollectionViewItemHighlightState,
    ) -> arc::R<ns::Set<ns::IndexPath>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:didChangeItemsAtIndexPaths:toHighlightState:)]
    fn collection_view_did_change_items_at_index_paths_to_highlight_state(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
        highlight_state: CollectionViewItemHighlightState,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldSelectItemsAtIndexPaths:)]
    fn collection_view_should_select_items_at_index_paths(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
    ) -> arc::R<ns::Set<ns::IndexPath>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldDeselectItemsAtIndexPaths:)]
    fn collection_view_should_deselect_items_at_index_paths(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
    ) -> arc::R<ns::Set<ns::IndexPath>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:didSelectItemsAtIndexPaths:)]
    fn collection_view_did_select_items_at_index_paths(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:didDeselectItemsAtIndexPaths:)]
    fn collection_view_did_deselect_items_at_index_paths(
        &mut self,
        collection_view: &mut ns::CollectionView,
        index_paths: &ns::Set<ns::IndexPath>,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:willDisplayItem:forRepresentedObjectAtIndexPath:)]
    fn collection_view_will_display_item_for_represented_object_at_index_path(
        &mut self,
        collection_view: &mut ns::CollectionView,
        item: &mut ns::CollectionViewItem,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:willDisplaySupplementaryView:forElementKind:atIndexPath:)]
    fn collection_view_will_display_supplementary_view_for_element_kind_at_index_path(
        &mut self,
        collection_view: &mut ns::CollectionView,
        view: &mut ns::View,
        element_kind: &ns::CollectionViewElementKind,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:didEndDisplayingItem:forRepresentedObjectAtIndexPath:)]
    fn collection_view_did_end_displaying_item_for_represented_object_at_index_path(
        &mut self,
        collection_view: &mut ns::CollectionView,
        item: &mut ns::CollectionViewItem,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:didEndDisplayingSupplementaryView:forElementOfKind:atIndexPath:)]
    fn collection_view_did_end_displaying_supplementary_view_for_element_of_kind_at_index_path(
        &mut self,
        collection_view: &mut ns::CollectionView,
        view: &mut ns::View,
        element_kind: &ns::CollectionViewElementKind,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:transitionLayoutForOldLayout:newLayout:)]
    fn collection_view_transition_layout_for_old_layout_new_layout(
        &mut self,
        collection_view: &mut ns::CollectionView,
        from_layout: &ns::CollectionViewLayout,
        to_layout: &ns::CollectionViewLayout,
    ) -> arc::R<ns::CollectionViewTransitionLayout>;
}

define_obj_type!(
    pub AnyCollectionViewDelegate(ns::Id)
);

impl CollectionViewDelegate for AnyCollectionViewDelegate {}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_COLLECTION_VIEW: &'static objc::Class<CollectionView>;
}
