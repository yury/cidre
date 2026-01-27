use crate::{arc, cg, define_cls, define_obj_type, define_opts, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

define_opts!(
    #[doc(alias = "UICollectionViewScrollPosition")]
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
}

#[doc(alias = "UICollectionViewReorderingCadence")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CollectionViewReorderingCadence {
    Immediate,
    Fast,
    Slow,
}

#[doc(alias = "UICollectionViewSelfSizingInvalidation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CollectionViewSelfSizingInvalidation {
    Disabled,
    Enabled,
    EnabledIncludingConstraints,
}

#[doc(alias = "UICollectionViewDropIntent")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CollectionViewDropIntent {
    Unspecified,
    InsertAtDestinationIndexPath,
    InsertIntoDestinationIndexPath,
}

define_obj_type!(
    #[doc(alias = "UICollectionViewFocusUpdateContext")]
    pub CollectionViewFocusUpdateCtx(ns::Id)
);

define_obj_type!(
    #[doc(alias = "UICollectionViewDropProposal")]
    pub CollectionViewDropProposal(ui::DropProposal)
);

define_obj_type!(
    #[doc(alias = "UICollectionViewPlaceholder")]
    pub CollectionViewPlaceholder(ns::Id)
);

define_obj_type!(
    #[doc(alias = "UICollectionViewDropPlaceholder")]
    pub CollectionViewDropPlaceholder(ui::CollectionViewPlaceholder)
);

#[objc::protocol(UICollectionViewDataSource)]
pub trait CollectionViewDataSrc: objc::Obj {
    #[objc::msg_send(collectionView:numberOfItemsInSection:)]
    fn collection_view_items_n_in_section(
        &self,
        collection_view: &ui::CollectionView,
        section: ns::Integer,
    ) -> ns::Integer;

    #[objc::msg_send(collectionView:cellForItemAtIndexPath:)]
    fn collection_view_cell_for_item_at_index_path(
        &self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> arc::R<ui::CollectionViewCell>;

    #[objc::optional]
    #[objc::msg_send(numberOfSectionsInCollectionView:)]
    fn collection_view_sections_n(&self, collection_view: &ui::CollectionView) -> ns::Integer;

    #[objc::optional]
    #[objc::msg_send(collectionView:viewForSupplementaryElementOfKind:atIndexPath:)]
    fn collection_view_view_for_supplementary_element_of_kind_at_index_path(
        &self,
        collection_view: &ui::CollectionView,
        kind: &ns::String,
        index_path: &ns::IndexPath,
    ) -> arc::R<ui::CollectionReusableView>;

    #[objc::optional]
    #[objc::msg_send(collectionView:canMoveItemAtIndexPath:)]
    fn collection_view_can_move_item_at_index_path(
        &self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:moveItemAtIndexPath:toIndexPath:)]
    fn collection_view_move_item_at_index_path_to_index_path(
        &self,
        collection_view: &ui::CollectionView,
        source_index_path: &ns::IndexPath,
        destination_index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(indexTitlesForCollectionView:)]
    fn collection_view_index_titles(
        &self,
        collection_view: &ui::CollectionView,
    ) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:indexPathForIndexTitle:atIndex:)]
    fn collection_view_index_path_for_index_title_at_index(
        &self,
        collection_view: &ui::CollectionView,
        title: &ns::String,
        index: ns::Integer,
    ) -> arc::R<ns::IndexPath>;
}

define_obj_type!(
    pub AnyCollectionViewDataSrc(ns::Id)
);

impl CollectionViewDataSrc for AnyCollectionViewDataSrc {}

#[objc::protocol(UICollectionViewDataSourcePrefetching)]
pub trait CollectionViewDataSrcPrefetching: objc::Obj {
    #[objc::msg_send(collectionView:prefetchItemsAtIndexPaths:)]
    fn collection_view_prefetch_items_at_index_paths(
        &self,
        collection_view: &ui::CollectionView,
        index_paths: &ns::Array<ns::IndexPath>,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:cancelPrefetchingForItemsAtIndexPaths:)]
    fn collection_view_cancel_prefetching_for_items_at_index_paths(
        &self,
        collection_view: &ui::CollectionView,
        index_paths: &ns::Array<ns::IndexPath>,
    );
}

define_obj_type!(
    pub AnyCollectionViewDataSrcPrefetching(ns::Id)
);

impl CollectionViewDataSrcPrefetching for AnyCollectionViewDataSrcPrefetching {}

#[objc::protocol(UICollectionViewDelegate)]
pub trait CollectionViewDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(collectionView:shouldHighlightItemAtIndexPath:)]
    fn collection_view_should_highlight_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:didHighlightItemAtIndexPath:)]
    fn collection_view_did_highlight_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:didUnhighlightItemAtIndexPath:)]
    fn collection_view_did_unhighlight_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldSelectItemAtIndexPath:)]
    fn collection_view_should_select_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldDeselectItemAtIndexPath:)]
    fn collection_view_should_deselect_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:didSelectItemAtIndexPath:)]
    fn collection_view_did_select_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:didDeselectItemAtIndexPath:)]
    fn collection_view_did_deselect_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:canPerformPrimaryActionForItemAtIndexPath:)]
    fn collection_view_can_perform_primary_action_for_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:performPrimaryActionForItemAtIndexPath:)]
    fn collection_view_perform_primary_action_for_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:willDisplayCell:forItemAtIndexPath:)]
    fn collection_view_will_display_cell_for_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        cell: &ui::CollectionViewCell,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:willDisplaySupplementaryView:forElementKind:atIndexPath:)]
    fn collection_view_will_display_supplementary_view_for_element_kind_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        view: &ui::CollectionReusableView,
        element_kind: &ns::String,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:didEndDisplayingCell:forItemAtIndexPath:)]
    fn collection_view_did_end_displaying_cell_for_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        cell: &ui::CollectionViewCell,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:didEndDisplayingSupplementaryView:forElementOfKind:atIndexPath:)]
    fn collection_view_did_end_displaying_supplementary_view_for_element_kind_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        view: &ui::CollectionReusableView,
        element_kind: &ns::String,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldShowMenuForItemAtIndexPath:)]
    fn collection_view_should_show_menu_for_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:canPerformAction:forItemAtIndexPath:withSender:)]
    fn collection_view_can_perform_action_for_item_at_index_path_with_sender(
        &mut self,
        collection_view: &ui::CollectionView,
        action: &objc::Sel,
        index_path: &ns::IndexPath,
        sender: Option<&ns::Id>,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:performAction:forItemAtIndexPath:withSender:)]
    fn collection_view_perform_action_for_item_at_index_path_with_sender(
        &mut self,
        collection_view: &ui::CollectionView,
        action: &objc::Sel,
        index_path: &ns::IndexPath,
        sender: Option<&ns::Id>,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:transitionLayoutForOldLayout:newLayout:)]
    fn collection_view_transition_layout_for_old_layout_new_layout(
        &mut self,
        collection_view: &ui::CollectionView,
        from_layout: &ui::CollectionViewLayout,
        to_layout: &ui::CollectionViewLayout,
    ) -> arc::R<ui::CollectionViewTransitionLayout>;

    #[objc::optional]
    #[objc::msg_send(collectionView:canFocusItemAtIndexPath:)]
    fn collection_view_can_focus_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldUpdateFocusInContext:)]
    fn collection_view_should_update_focus_in_context(
        &mut self,
        collection_view: &ui::CollectionView,
        context: &ui::CollectionViewFocusUpdateCtx,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:didUpdateFocusInContext:withAnimationCoordinator:)]
    fn collection_view_did_update_focus_in_context_with_animation_coordinator(
        &mut self,
        collection_view: &ui::CollectionView,
        context: &ui::CollectionViewFocusUpdateCtx,
        coordinator: &ui::FocusAnimationCoordinator,
    );

    #[objc::optional]
    #[objc::msg_send(indexPathForPreferredFocusedViewInCollectionView:)]
    fn index_path_for_preferred_focused_view_in_collection_view(
        &mut self,
        collection_view: &ui::CollectionView,
    ) -> Option<arc::R<ns::IndexPath>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:selectionFollowsFocusForItemAtIndexPath:)]
    fn collection_view_selection_follows_focus_for_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:targetIndexPathForMoveOfItemFromOriginalIndexPath:atCurrentIndexPath:toProposedIndexPath:)]
    fn collection_view_target_index_path_for_move_of_item_from_original_index_path_at_current_index_path_to_proposed_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        original_index_path: &ns::IndexPath,
        current_index_path: &ns::IndexPath,
        proposed_index_path: &ns::IndexPath,
    ) -> arc::R<ns::IndexPath>;

    #[objc::optional]
    #[objc::msg_send(collectionView:targetIndexPathForMoveFromItemAtIndexPath:toProposedIndexPath:)]
    fn collection_view_target_index_path_for_move_from_item_at_index_path_to_proposed_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        current_index_path: &ns::IndexPath,
        proposed_index_path: &ns::IndexPath,
    ) -> arc::R<ns::IndexPath>;

    #[objc::optional]
    #[objc::msg_send(collectionView:targetContentOffsetForProposedContentOffset:)]
    fn collection_view_target_content_offset_for_proposed_content_offset(
        &mut self,
        collection_view: &ui::CollectionView,
        proposed_content_offset: cg::Point,
    ) -> cg::Point;

    #[objc::optional]
    #[objc::msg_send(collectionView:canEditItemAtIndexPath:)]
    fn collection_view_can_edit_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldSpringLoadItemAtIndexPath:withContext:)]
    fn collection_view_should_spring_load_item_at_index_path_with_context(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
        context: &ui::AnySpringLoadedInteractionContext,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:shouldBeginMultipleSelectionInteractionAtIndexPath:)]
    fn collection_view_should_begin_multiple_selection_interaction_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:didBeginMultipleSelectionInteractionAtIndexPath:)]
    fn collection_view_did_begin_multiple_selection_interaction_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    );

    #[objc::optional]
    #[objc::msg_send(collectionViewDidEndMultipleSelectionInteraction:)]
    fn collection_view_did_end_multiple_selection_interaction(
        &mut self,
        collection_view: &ui::CollectionView,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:contextMenuConfigurationForItemsAtIndexPaths:point:)]
    fn collection_view_context_menu_configuration_for_items_at_index_paths_point(
        &mut self,
        collection_view: &ui::CollectionView,
        index_paths: &ns::Array<ns::IndexPath>,
        point: cg::Point,
    ) -> Option<arc::R<ui::ContextMenuConfiguration>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:contextMenuConfiguration:highlightPreviewForItemAtIndexPath:)]
    fn collection_view_context_menu_configuration_highlight_preview_for_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        configuration: &ui::ContextMenuConfiguration,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ui::TargetedPreview>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:contextMenuConfiguration:dismissalPreviewForItemAtIndexPath:)]
    fn collection_view_context_menu_configuration_dismissal_preview_for_item_at_index_path(
        &mut self,
        collection_view: &ui::CollectionView,
        configuration: &ui::ContextMenuConfiguration,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ui::TargetedPreview>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:willPerformPreviewActionForMenuWithConfiguration:animator:)]
    fn collection_view_will_perform_preview_action_for_menu_with_configuration_animator(
        &mut self,
        collection_view: &ui::CollectionView,
        configuration: &ui::ContextMenuConfiguration,
        animator: &ui::AnyContextMenuInteractionCommitAnimating,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:willDisplayContextMenuWithConfiguration:animator:)]
    fn collection_view_will_display_context_menu_with_configuration_animator(
        &mut self,
        collection_view: &ui::CollectionView,
        configuration: &ui::ContextMenuConfiguration,
        animator: Option<&ui::AnyContextMenuInteractionAnimating>,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:willEndContextMenuInteractionWithConfiguration:animator:)]
    fn collection_view_will_end_context_menu_interaction_with_configuration_animator(
        &mut self,
        collection_view: &ui::CollectionView,
        configuration: &ui::ContextMenuConfiguration,
        animator: Option<&ui::AnyContextMenuInteractionAnimating>,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:sceneActivationConfigurationForItemAtIndexPath:point:)]
    fn collection_view_scene_activation_configuration_for_item_at_index_path_point(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
        point: cg::Point,
    ) -> Option<arc::R<ui::WindowSceneActivationConfiguration>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:contextMenuConfigurationForItemAtIndexPath:point:)]
    fn collection_view_context_menu_configuration_for_item_at_index_path_point(
        &mut self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
        point: cg::Point,
    ) -> Option<arc::R<ui::ContextMenuConfiguration>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:previewForHighlightingContextMenuWithConfiguration:)]
    fn collection_view_preview_for_highlighting_context_menu_with_configuration(
        &mut self,
        collection_view: &ui::CollectionView,
        configuration: &ui::ContextMenuConfiguration,
    ) -> Option<arc::R<ui::TargetedPreview>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:previewForDismissingContextMenuWithConfiguration:)]
    fn collection_view_preview_for_dismissing_context_menu_with_configuration(
        &mut self,
        collection_view: &ui::CollectionView,
        configuration: &ui::ContextMenuConfiguration,
    ) -> Option<arc::R<ui::TargetedPreview>>;
}

define_obj_type!(
    pub AnyCollectionViewDelegate(ns::Id)
);

impl CollectionViewDelegate for AnyCollectionViewDelegate {}

#[objc::protocol(UICollectionViewDragDelegate)]
pub trait CollectionViewDragDelegate: objc::Obj {
    #[objc::msg_send(collectionView:itemsForBeginningDragSession:atIndexPath:)]
    fn collection_view_items_for_beginning_drag_session_at_index_path(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDragSession,
        index_path: &ns::IndexPath,
    ) -> arc::R<ns::Array<ui::DragItem>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:itemsForAddingToDragSession:atIndexPath:point:)]
    fn collection_view_items_for_adding_to_drag_session_at_index_path_point(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDragSession,
        index_path: &ns::IndexPath,
        point: cg::Point,
    ) -> arc::R<ns::Array<ui::DragItem>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:dragPreviewParametersForItemAtIndexPath:)]
    fn collection_view_drag_preview_parameters_for_item_at_index_path(
        &self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ui::DragPreviewParameters>>;

    #[objc::optional]
    #[objc::msg_send(collectionView:dragSessionWillBegin:)]
    fn collection_view_drag_session_will_begin(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDragSession,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:dragSessionDidEnd:)]
    fn collection_view_drag_session_did_end(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDragSession,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:dragSessionAllowsMoveOperation:)]
    fn collection_view_drag_session_allows_move_operation(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDragSession,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:dragSessionIsRestrictedToDraggingApplication:)]
    fn collection_view_drag_session_is_restricted_to_dragging_application(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDragSession,
    ) -> bool;
}

define_obj_type!(
    pub AnyCollectionViewDragDelegate(ns::Id)
);

impl CollectionViewDragDelegate for AnyCollectionViewDragDelegate {}

#[objc::protocol(UICollectionViewDropDelegate)]
pub trait CollectionViewDropDelegate: objc::Obj {
    #[objc::msg_send(collectionView:performDropWithCoordinator:)]
    fn collection_view_perform_drop_with_coordinator(
        &self,
        collection_view: &ui::CollectionView,
        coordinator: &ui::AnyCollectionViewDropCoordinator,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:canHandleDropSession:)]
    fn collection_view_can_handle_drop_session(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDropSession,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(collectionView:dropSessionDidEnter:)]
    fn collection_view_drop_session_did_enter(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDropSession,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:dropSessionDidUpdate:withDestinationIndexPath:)]
    fn collection_view_drop_session_did_update_with_destination_index_path(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDropSession,
        destination_index_path: Option<&ns::IndexPath>,
    ) -> arc::R<ui::CollectionViewDropProposal>;

    #[objc::optional]
    #[objc::msg_send(collectionView:dropSessionDidExit:)]
    fn collection_view_drop_session_did_exit(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDropSession,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:dropSessionDidEnd:)]
    fn collection_view_drop_session_did_end(
        &self,
        collection_view: &ui::CollectionView,
        session: &ui::AnyDropSession,
    );

    #[objc::optional]
    #[objc::msg_send(collectionView:dropPreviewParametersForItemAtIndexPath:)]
    fn collection_view_drop_preview_parameters_for_item_at_index_path(
        &self,
        collection_view: &ui::CollectionView,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ui::DragPreviewParameters>>;
}

define_obj_type!(
    pub AnyCollectionViewDropDelegate(ns::Id)
);

impl CollectionViewDropDelegate for AnyCollectionViewDropDelegate {}

#[objc::protocol(UICollectionViewDropCoordinator)]
pub trait CollectionViewDropCoordinator: objc::Obj {
    #[objc::msg_send(items)]
    fn items(&self) -> arc::R<ns::Array<ui::AnyCollectionViewDropItem>>;

    #[objc::msg_send(destinationIndexPath)]
    fn destination_index_path(&self) -> Option<arc::R<ns::IndexPath>>;

    #[objc::msg_send(proposal)]
    fn proposal(&self) -> arc::R<ui::CollectionViewDropProposal>;

    #[objc::msg_send(session)]
    fn session(&self) -> arc::R<ui::AnyDropSession>;

    #[objc::msg_send(dropItem:toPlaceholder:)]
    fn drop_item_to_placeholder(
        &self,
        drag_item: &ui::DragItem,
        placeholder: &ui::CollectionViewDropPlaceholder,
    ) -> arc::R<ui::AnyCollectionViewDropPlaceholderCtx>;

    #[objc::msg_send(dropItem:toItemAtIndexPath:)]
    fn drop_item_to_item_at_index_path(
        &self,
        drag_item: &ui::DragItem,
        index_path: &ns::IndexPath,
    ) -> arc::R<ui::AnyDragAnimating>;

    #[objc::msg_send(dropItem:intoItemAtIndexPath:rect:)]
    fn drop_item_into_item_at_index_path_rect(
        &self,
        drag_item: &ui::DragItem,
        index_path: &ns::IndexPath,
        rect: cg::Rect,
    ) -> arc::R<ui::AnyDragAnimating>;

    #[objc::msg_send(dropItem:toTarget:)]
    fn drop_item_to_target(
        &self,
        drag_item: &ui::DragItem,
        target: &ui::DragPreviewTarget,
    ) -> arc::R<ui::AnyDragAnimating>;
}

define_obj_type!(
    pub AnyCollectionViewDropCoordinator(ns::Id)
);

impl CollectionViewDropCoordinator for AnyCollectionViewDropCoordinator {}

#[objc::protocol(UICollectionViewDropItem)]
pub trait CollectionViewDropItem: objc::Obj {
    #[objc::msg_send(dragItem)]
    fn drag_item(&self) -> arc::R<ui::DragItem>;

    #[objc::msg_send(sourceIndexPath)]
    fn source_index_path(&self) -> Option<arc::R<ns::IndexPath>>;

    #[objc::msg_send(previewSize)]
    fn preview_size(&self) -> cg::Size;
}

define_obj_type!(
    pub AnyCollectionViewDropItem(ns::Id)
);

impl CollectionViewDropItem for AnyCollectionViewDropItem {}

#[objc::protocol(UICollectionViewDropPlaceholderContext)]
pub trait CollectionViewDropPlaceholderCtx: ui::DragAnimating {
    #[objc::msg_send(dragItem)]
    fn drag_item(&self) -> arc::R<ui::DragItem>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(commitInsertionWithDataSourceUpdates:)]
    fn commit_insertion_with_data_source_updates(
        &self,
        data_source_updates: &blocks::EscBlock<fn(&ns::IndexPath)>,
    ) -> bool;

    #[objc::msg_send(deletePlaceholder)]
    fn delete_placeholder(&self) -> bool;

    #[objc::msg_send(setNeedsCellUpdate)]
    fn set_needs_cell_update(&self);
}

define_obj_type!(
    pub AnyCollectionViewDropPlaceholderCtx(ns::Id)
);

impl ui::DragAnimating for AnyCollectionViewDropPlaceholderCtx {}
impl CollectionViewDropPlaceholderCtx for AnyCollectionViewDropPlaceholderCtx {}

define_obj_type!(
    #[doc(alias = "UICollectionView")]
    pub CollectionView(ui::View)
);

impl arc::A<CollectionView> {
    #[objc::msg_send(initWithFrame:collectionViewLayout:)]
    pub fn init_with_frame_collection_view_layout(
        self,
        frame: cg::Rect,
        layout: &ui::CollectionViewLayout,
    ) -> arc::R<CollectionView>;

    #[objc::msg_send(initWithCoder:)]
    pub fn init_with_coder(self, coder: &ns::Coder) -> Option<arc::R<CollectionView>>;
}

impl CollectionView {
    define_cls!(UI_COLLECTION_VIEW);

    #[objc::msg_send(collectionViewLayout)]
    pub fn collection_view_layout(&self) -> arc::R<ui::CollectionViewLayout>;

    #[objc::msg_send(setCollectionViewLayout:)]
    pub fn set_collection_view_layout(&mut self, val: &ui::CollectionViewLayout);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyCollectionViewDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: CollectionViewDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(dataSource)]
    pub fn data_src(&self) -> Option<arc::R<AnyCollectionViewDataSrc>>;

    #[objc::msg_send(setDataSource:)]
    pub fn set_data_src<D: CollectionViewDataSrc>(&mut self, val: Option<&D>);

    #[objc::msg_send(prefetchDataSource)]
    pub fn prefetch_data_src(&self) -> Option<arc::R<AnyCollectionViewDataSrcPrefetching>>;

    #[objc::msg_send(setPrefetchDataSource:)]
    pub fn set_prefetch_data_src<D: CollectionViewDataSrcPrefetching>(&mut self, val: Option<&D>);

    #[objc::msg_send(isPrefetchingEnabled)]
    pub fn is_prefetching_enabled(&self) -> bool;

    #[objc::msg_send(setPrefetchingEnabled:)]
    pub fn set_prefetching_enabled(&mut self, val: bool);

    #[objc::msg_send(dragDelegate)]
    pub fn drag_delegate(&self) -> Option<arc::R<AnyCollectionViewDragDelegate>>;

    #[objc::msg_send(setDragDelegate:)]
    pub fn set_drag_delegate<D: CollectionViewDragDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(dropDelegate)]
    pub fn drop_delegate(&self) -> Option<arc::R<AnyCollectionViewDropDelegate>>;

    #[objc::msg_send(setDropDelegate:)]
    pub fn set_drop_delegate<D: CollectionViewDropDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(dragInteractionEnabled)]
    pub fn drag_interaction_enabled(&self) -> bool;

    #[objc::msg_send(setDragInteractionEnabled:)]
    pub fn set_drag_interaction_enabled(&mut self, val: bool);

    #[objc::msg_send(contextMenuInteraction)]
    pub fn context_menu_interaction(&self) -> Option<arc::R<ui::ContextMenuInteraction>>;

    #[objc::msg_send(reorderingCadence)]
    pub fn reordering_cadence(&self) -> CollectionViewReorderingCadence;

    #[objc::msg_send(setReorderingCadence:)]
    pub fn set_reordering_cadence(&mut self, val: CollectionViewReorderingCadence);

    #[objc::msg_send(selfSizingInvalidation)]
    pub fn self_sizing_invalidation(&self) -> CollectionViewSelfSizingInvalidation;

    #[objc::msg_send(setSelfSizingInvalidation:)]
    pub fn set_self_sizing_invalidation(&mut self, val: CollectionViewSelfSizingInvalidation);

    #[objc::msg_send(backgroundView)]
    pub fn background_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setBackgroundView:)]
    pub fn set_background_view(&mut self, val: Option<&ui::View>);

    #[objc::msg_send(registerClass:forCellWithReuseIdentifier:)]
    pub fn register_class_for_cell_with_reuse_id(
        &mut self,
        cell_class: Option<&objc::Class<ns::Id>>,
        reuse_id: &ns::String,
    );

    #[objc::msg_send(registerClass:forSupplementaryViewOfKind:withReuseIdentifier:)]
    pub fn register_class_for_supplementary_view_of_kind_with_reuse_id(
        &mut self,
        view_class: Option<&objc::Class<ns::Id>>,
        element_kind: &ns::String,
        reuse_id: &ns::String,
    );

    #[objc::msg_send(dequeueReusableCellWithReuseIdentifier:forIndexPath:)]
    pub fn dequeue_reusable_cell_with_reuse_id_for_index_path(
        &self,
        reuse_id: &ns::String,
        index_path: &ns::IndexPath,
    ) -> arc::R<ui::CollectionViewCell>;

    #[objc::msg_send(dequeueReusableSupplementaryViewOfKind:withReuseIdentifier:forIndexPath:)]
    pub fn dequeue_reusable_supplementary_view_of_kind_with_reuse_id_for_index_path(
        &self,
        element_kind: &ns::String,
        reuse_id: &ns::String,
        index_path: &ns::IndexPath,
    ) -> arc::R<ui::CollectionReusableView>;

    #[objc::msg_send(dequeueConfiguredReusableCellWithRegistration:forIndexPath:item:)]
    pub fn dequeue_configured_reusable_cell_with_registration_for_index_path_item(
        &self,
        registration: &ui::CollectionViewCellRegistration,
        index_path: &ns::IndexPath,
        item: Option<&ns::Id>,
    ) -> arc::R<ui::CollectionViewCell>;

    #[objc::msg_send(dequeueConfiguredReusableSupplementaryViewWithRegistration:forIndexPath:)]
    pub fn dequeue_configured_reusable_supplementary_view_with_registration_for_index_path(
        &self,
        registration: &ui::CollectionViewSupplementaryRegistration,
        index_path: &ns::IndexPath,
    ) -> arc::R<ui::CollectionReusableView>;

    #[objc::msg_send(allowsSelection)]
    pub fn allows_selection(&self) -> bool;

    #[objc::msg_send(setAllowsSelection:)]
    pub fn set_allows_selection(&mut self, val: bool);

    #[objc::msg_send(allowsMultipleSelection)]
    pub fn allows_multiple_selection(&self) -> bool;

    #[objc::msg_send(setAllowsMultipleSelection:)]
    pub fn set_allows_multiple_selection(&mut self, val: bool);

    #[objc::msg_send(indexPathsForSelectedItems)]
    pub fn index_paths_for_selected_items(&self) -> Option<arc::R<ns::Array<ns::IndexPath>>>;

    #[objc::msg_send(selectItemAtIndexPath:animated:scrollPosition:)]
    pub fn select_item_at_index_path_animated_scroll_position(
        &mut self,
        index_path: Option<&ns::IndexPath>,
        animated: bool,
        scroll_position: CollectionViewScrollPos,
    );

    #[objc::msg_send(deselectItemAtIndexPath:animated:)]
    pub fn deselect_item_at_index_path_animated(
        &mut self,
        index_path: &ns::IndexPath,
        animated: bool,
    );

    #[objc::msg_send(hasUncommittedUpdates)]
    pub fn has_uncommitted_updates(&self) -> bool;

    #[objc::msg_send(reloadData)]
    pub fn reload_data(&self);

    #[objc::msg_send(setCollectionViewLayout:animated:)]
    pub fn set_collection_view_layout_animated(
        &mut self,
        layout: &ui::CollectionViewLayout,
        animated: bool,
    );

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setCollectionViewLayout:animated:completion:)]
    pub fn set_collection_view_layout_animated_completion_block(
        &mut self,
        layout: &ui::CollectionViewLayout,
        animated: bool,
        completion: Option<&mut blocks::EscBlock<fn(bool)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_collection_view_layout_animated_completion(
        &mut self,
        layout: &ui::CollectionViewLayout,
        animated: bool,
        completion: impl FnMut(bool) + 'static,
    ) {
        let mut completion = blocks::EscBlock::new1(completion);
        self.set_collection_view_layout_animated_completion_block(
            layout,
            animated,
            Some(&mut completion),
        );
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(startInteractiveTransitionToCollectionViewLayout:completion:)]
    pub fn start_interactive_transition_to_collection_view_layout_completion_block(
        &mut self,
        layout: &ui::CollectionViewLayout,
        completion: Option<&mut blocks::EscBlock<fn(bool, bool)>>,
    ) -> arc::R<ui::CollectionViewTransitionLayout>;

    #[cfg(feature = "blocks")]
    pub fn start_interactive_transition_to_collection_view_layout_completion(
        &mut self,
        layout: &ui::CollectionViewLayout,
        completion: impl FnMut(bool, bool) + 'static,
    ) -> arc::R<ui::CollectionViewTransitionLayout> {
        let mut completion = blocks::EscBlock::new2(completion);
        self.start_interactive_transition_to_collection_view_layout_completion_block(
            layout,
            Some(&mut completion),
        )
    }

    #[objc::msg_send(finishInteractiveTransition)]
    pub fn finish_interactive_transition(&mut self);

    #[objc::msg_send(cancelInteractiveTransition)]
    pub fn cancel_interactive_transition(&mut self);

    #[objc::msg_send(numberOfSections)]
    pub fn sections_n(&self) -> ns::Integer;

    #[objc::msg_send(numberOfItemsInSection:)]
    pub fn items_n_in_section(&self, section: ns::Integer) -> ns::Integer;

    #[objc::msg_send(layoutAttributesForItemAtIndexPath:)]
    pub fn layout_attrs_for_item_at_index_path(
        &self,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ui::CollectionViewLayoutAttrs>>;

    #[objc::msg_send(layoutAttributesForSupplementaryElementOfKind:atIndexPath:)]
    pub fn layout_attrs_for_supplementary_element_of_kind_at_index_path(
        &self,
        kind: &ns::String,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ui::CollectionViewLayoutAttrs>>;

    #[objc::msg_send(indexPathForItemAtPoint:)]
    pub fn index_path_for_item_at_point(&self, point: cg::Point) -> Option<arc::R<ns::IndexPath>>;

    #[objc::msg_send(indexPathForCell:)]
    pub fn index_path_for_cell(
        &self,
        cell: &ui::CollectionViewCell,
    ) -> Option<arc::R<ns::IndexPath>>;

    #[objc::msg_send(indexPathForSupplementaryView:)]
    pub fn index_path_for_supplementary_view(
        &self,
        supplementary_view: &ui::CollectionReusableView,
    ) -> Option<arc::R<ns::IndexPath>>;

    #[objc::msg_send(cellForItemAtIndexPath:)]
    pub fn cell_for_item_at_index_path(
        &self,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ui::CollectionViewCell>>;

    #[objc::msg_send(visibleCells)]
    pub fn visible_cells(&self) -> arc::R<ns::Array<ui::CollectionViewCell>>;

    #[objc::msg_send(indexPathsForVisibleItems)]
    pub fn index_paths_for_visible_items(&self) -> arc::R<ns::Array<ns::IndexPath>>;

    #[objc::msg_send(supplementaryViewForElementKind:atIndexPath:)]
    pub fn supplementary_view_for_element_kind_at_index_path(
        &self,
        element_kind: &ns::String,
        index_path: &ns::IndexPath,
    ) -> Option<arc::R<ui::CollectionReusableView>>;

    #[objc::msg_send(visibleSupplementaryViewsOfKind:)]
    pub fn visible_supplementary_views_of_kind(
        &self,
        element_kind: &ns::String,
    ) -> arc::R<ns::Array<ui::CollectionReusableView>>;

    #[objc::msg_send(indexPathsForVisibleSupplementaryElementsOfKind:)]
    pub fn index_paths_for_visible_supplementary_elements_of_kind(
        &self,
        element_kind: &ns::String,
    ) -> arc::R<ns::Array<ns::IndexPath>>;

    #[objc::msg_send(scrollToItemAtIndexPath:atScrollPosition:animated:)]
    pub fn scroll_to_item_at_index_path_scroll_position_animated(
        &self,
        index_path: &ns::IndexPath,
        scroll_position: CollectionViewScrollPos,
        animated: bool,
    );

    #[objc::msg_send(insertSections:)]
    pub fn insert_sections(&mut self, sections: &ns::IndexSet);

    #[objc::msg_send(deleteSections:)]
    pub fn delete_sections(&mut self, sections: &ns::IndexSet);

    #[objc::msg_send(moveSection:toSection:)]
    pub fn move_section_to_section(&mut self, section: ns::Integer, new_section: ns::Integer);

    #[objc::msg_send(reloadSections:)]
    pub fn reload_sections(&mut self, sections: &ns::IndexSet);

    #[objc::msg_send(insertItemsAtIndexPaths:)]
    pub fn insert_items_at_index_paths(&mut self, index_paths: &ns::Array<ns::IndexPath>);

    #[objc::msg_send(deleteItemsAtIndexPaths:)]
    pub fn delete_items_at_index_paths(&mut self, index_paths: &ns::Array<ns::IndexPath>);

    #[objc::msg_send(moveItemAtIndexPath:toIndexPath:)]
    pub fn move_item_at_index_path_to_index_path(
        &mut self,
        index_path: &ns::IndexPath,
        new_index_path: &ns::IndexPath,
    );

    #[objc::msg_send(reloadItemsAtIndexPaths:)]
    pub fn reload_items_at_index_paths(&mut self, index_paths: &ns::Array<ns::IndexPath>);

    #[objc::msg_send(reconfigureItemsAtIndexPaths:)]
    pub fn reconfigure_items_at_index_paths(&mut self, index_paths: &ns::Array<ns::IndexPath>);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(performBatchUpdates:completion:)]
    pub fn perform_batch_updates_completion_block(
        &mut self,
        updates: Option<&mut blocks::EscBlock<fn()>>,
        completion: Option<&mut blocks::EscBlock<fn(bool)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn perform_batch_updates(&mut self, updates: impl FnMut() + 'static) {
        let mut updates = blocks::EscBlock::new0(updates);
        self.perform_batch_updates_completion_block(Some(&mut updates), None);
    }

    #[cfg(feature = "blocks")]
    pub fn perform_batch_updates_with_completion(
        &mut self,
        updates: impl FnMut() + 'static,
        completion: impl FnMut(bool) + 'static,
    ) {
        let mut updates = blocks::EscBlock::new0(updates);
        let mut completion = blocks::EscBlock::new1(completion);
        self.perform_batch_updates_completion_block(Some(&mut updates), Some(&mut completion));
    }

    #[objc::msg_send(beginInteractiveMovementForItemAtIndexPath:)]
    pub fn begin_interactive_movement_for_item_at_index_path(
        &mut self,
        index_path: &ns::IndexPath,
    ) -> bool;

    #[objc::msg_send(updateInteractiveMovementTargetPosition:)]
    pub fn update_interactive_movement_target_position(&mut self, target_position: cg::Point);

    #[objc::msg_send(endInteractiveMovement)]
    pub fn end_interactive_movement(&mut self);

    #[objc::msg_send(cancelInteractiveMovement)]
    pub fn cancel_interactive_movement(&mut self);

    #[objc::msg_send(remembersLastFocusedIndexPath)]
    pub fn remembers_last_focused_index_path(&self) -> bool;

    #[objc::msg_send(setRemembersLastFocusedIndexPath:)]
    pub fn set_remembers_last_focused_index_path(&mut self, val: bool);

    #[objc::msg_send(selectionFollowsFocus)]
    pub fn selection_follows_focus(&self) -> bool;

    #[objc::msg_send(setSelectionFollowsFocus:)]
    pub fn set_selection_follows_focus(&mut self, val: bool);

    #[objc::msg_send(allowsFocus)]
    pub fn allows_focus(&self) -> bool;

    #[objc::msg_send(setAllowsFocus:)]
    pub fn set_allows_focus(&mut self, val: bool);

    #[objc::msg_send(allowsFocusDuringEditing)]
    pub fn allows_focus_during_editing(&self) -> bool;

    #[objc::msg_send(setAllowsFocusDuringEditing:)]
    pub fn set_allows_focus_during_editing(&mut self, val: bool);

    #[objc::msg_send(hasActiveDrag)]
    pub fn has_active_drag(&self) -> bool;

    #[objc::msg_send(hasActiveDrop)]
    pub fn has_active_drop(&self) -> bool;

    #[objc::msg_send(isEditing)]
    pub fn is_editing(&self) -> bool;

    #[objc::msg_send(setEditing:)]
    pub fn set_editing(&mut self, val: bool);

    #[objc::msg_send(allowsSelectionDuringEditing)]
    pub fn allows_selection_during_editing(&self) -> bool;

    #[objc::msg_send(setAllowsSelectionDuringEditing:)]
    pub fn set_allows_selection_during_editing(&mut self, val: bool);

    #[objc::msg_send(allowsMultipleSelectionDuringEditing)]
    pub fn allows_multiple_selection_during_editing(&self) -> bool;

    #[objc::msg_send(setAllowsMultipleSelectionDuringEditing:)]
    pub fn set_allows_multiple_selection_during_editing(&mut self, val: bool);
}

impl arc::A<CollectionViewDropProposal> {
    #[objc::msg_send(initWithDropOperation:intent:)]
    pub fn init_with_drop_operation_intent(
        self,
        operation: ui::DropOperation,
        intent: CollectionViewDropIntent,
    ) -> arc::R<CollectionViewDropProposal>;
}

impl CollectionViewDropProposal {
    define_cls!(UI_COLLECTION_VIEW_DROP_PROPOSAL);

    #[objc::msg_send(intent)]
    pub fn intent(&self) -> CollectionViewDropIntent;
}

impl arc::A<CollectionViewPlaceholder> {
    #[objc::msg_send(initWithInsertionIndexPath:reuseIdentifier:)]
    pub fn init_with_insertion_index_path_reuse_id(
        self,
        insertion_index_path: &ns::IndexPath,
        reuse_id: &ns::String,
    ) -> arc::R<CollectionViewPlaceholder>;
}

impl CollectionViewPlaceholder {
    define_cls!(UI_COLLECTION_VIEW_PLACEHOLDER);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setCellUpdateHandler:)]
    pub fn set_cell_update_handler_block(
        &mut self,
        handler: Option<&mut blocks::EscBlock<fn(&ui::CollectionViewCell)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn set_cell_update_handler(
        &mut self,
        handler: impl FnMut(&ui::CollectionViewCell) + 'static,
    ) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_cell_update_handler_block(Some(&mut handler));
    }
}

impl arc::A<CollectionViewDropPlaceholder> {
    #[objc::msg_send(initWithInsertionIndexPath:reuseIdentifier:)]
    pub fn init_with_insertion_index_path_reuse_id(
        self,
        insertion_index_path: &ns::IndexPath,
        reuse_id: &ns::String,
    ) -> arc::R<CollectionViewDropPlaceholder>;
}

impl CollectionViewDropPlaceholder {
    define_cls!(UI_COLLECTION_VIEW_DROP_PLACEHOLDER);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setPreviewParametersProvider:)]
    pub fn set_preview_parameters_provider_block(
        &mut self,
        handler: Option<
            &mut blocks::EscBlock<
                fn(&ui::CollectionViewCell) -> Option<arc::R<ui::DragPreviewParameters>>,
            >,
        >,
    );

    #[cfg(feature = "blocks")]
    pub fn set_preview_parameters_provider(
        &mut self,
        handler: impl FnMut(&ui::CollectionViewCell) -> Option<arc::R<ui::DragPreviewParameters>>
        + 'static,
    ) {
        let mut handler = blocks::EscBlock::new1(handler);
        self.set_preview_parameters_provider_block(Some(&mut handler));
    }
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_COLLECTION_VIEW: &'static objc::Class<CollectionView>;
    static UI_COLLECTION_VIEW_DROP_PROPOSAL: &'static objc::Class<CollectionViewDropProposal>;
    static UI_COLLECTION_VIEW_PLACEHOLDER: &'static objc::Class<CollectionViewPlaceholder>;
    static UI_COLLECTION_VIEW_DROP_PLACEHOLDER: &'static objc::Class<CollectionViewDropPlaceholder>;
}
