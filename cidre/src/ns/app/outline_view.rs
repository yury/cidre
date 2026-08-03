use crate::{api, arc, cg, define_obj_type, ns, objc};

#[doc(alias = "NSOutlineViewDropOnItemIndex")]
pub const OUTLINE_VIEW_DROP_ON_ITEM_INDEX: ns::Integer = -1;

define_obj_type!(
    #[doc(alias = "NSOutlineView")]
    pub OutlineView(ns::TableView),
    NS_OUTLINE_VIEW
);

impl arc::A<OutlineView> {
    #[objc::msg_send(initWithFrame:)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<OutlineView>;
}

impl OutlineView {
    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyOutlineViewDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: OutlineViewDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(dataSource)]
    pub fn data_src(&self) -> Option<arc::R<AnyOutlineViewDataSrc>>;

    #[objc::msg_send(setDataSource:)]
    pub fn set_data_src<D: OutlineViewDataSrc>(&mut self, val: Option<&D>);

    /// The returned object is an `NSTableColumn`.
    #[objc::msg_send(outlineTableColumn)]
    pub fn outline_table_column(&self) -> Option<arc::R<ns::Id>>;

    /// `val` must be an `NSTableColumn`.
    #[objc::msg_send(setOutlineTableColumn:)]
    pub fn set_outline_table_column(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(isExpandable:)]
    pub fn is_expandable(&self, item: Option<&ns::Id>) -> bool;

    #[objc::msg_send(numberOfChildrenOfItem:)]
    #[objc::available(macos = 10.10)]
    pub fn children_n_of_item(&self, item: Option<&ns::Id>) -> ns::Integer;

    #[objc::msg_send(child:ofItem:)]
    #[objc::available(macos = 10.10)]
    pub fn child_of_item(
        &self,
        index: ns::Integer,
        item: Option<&ns::Id>,
    ) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(expandItem:expandChildren:)]
    pub fn expand_item_with_children(&mut self, item: Option<&ns::Id>, expand_children: bool);

    #[objc::msg_send(expandItem:)]
    pub fn expand_item(&mut self, item: Option<&ns::Id>);

    #[objc::msg_send(collapseItem:collapseChildren:)]
    pub fn collapse_item_with_children(&mut self, item: Option<&ns::Id>, collapse_children: bool);

    #[objc::msg_send(collapseItem:)]
    pub fn collapse_item(&mut self, item: Option<&ns::Id>);

    #[objc::msg_send(reloadItem:reloadChildren:)]
    pub fn reload_item_with_children(&mut self, item: Option<&ns::Id>, reload_children: bool);

    #[objc::msg_send(reloadItem:)]
    pub fn reload_item(&mut self, item: Option<&ns::Id>);

    #[objc::msg_send(parentForItem:)]
    pub fn parent_for_item(&self, item: Option<&ns::Id>) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(childIndexForItem:)]
    #[objc::available(macos = 10.11)]
    pub fn child_index_for_item(&self, item: &ns::Id) -> ns::Integer;

    #[objc::msg_send(itemAtRow:)]
    pub fn item_at_row(&self, row: ns::Integer) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(rowForItem:)]
    pub fn row_for_item(&self, item: Option<&ns::Id>) -> ns::Integer;

    #[objc::msg_send(levelForItem:)]
    pub fn level_for_item(&self, item: Option<&ns::Id>) -> ns::Integer;

    #[objc::msg_send(levelForRow:)]
    pub fn level_for_row(&self, row: ns::Integer) -> ns::Integer;

    #[objc::msg_send(isItemExpanded:)]
    pub fn is_item_expanded(&self, item: Option<&ns::Id>) -> bool;

    #[objc::msg_send(indentationPerLevel)]
    pub fn indentation_per_level(&self) -> cg::Float;

    #[objc::msg_send(setIndentationPerLevel:)]
    pub fn set_indentation_per_level(&mut self, val: cg::Float);

    #[objc::msg_send(indentationMarkerFollowsCell)]
    pub fn indentation_marker_follows_cell(&self) -> bool;

    #[objc::msg_send(setIndentationMarkerFollowsCell:)]
    pub fn set_indentation_marker_follows_cell(&mut self, val: bool);

    #[objc::msg_send(autoresizesOutlineColumn)]
    pub fn autoresizes_outline_column(&self) -> bool;

    #[objc::msg_send(setAutoresizesOutlineColumn:)]
    pub fn set_autoresizes_outline_column(&mut self, val: bool);

    #[objc::msg_send(frameOfOutlineCellAtRow:)]
    #[objc::available(macos = 10.5)]
    pub fn frame_of_outline_cell_at_row(&self, row: ns::Integer) -> ns::Rect;

    #[objc::msg_send(setDropItem:dropChildIndex:)]
    pub fn set_drop_item_child_index(&mut self, item: Option<&ns::Id>, index: ns::Integer);

    #[objc::msg_send(shouldCollapseAutoExpandedItemsForDeposited:)]
    pub fn should_collapse_auto_expanded_items(&self, deposited: bool) -> bool;

    #[objc::msg_send(autosaveExpandedItems)]
    pub fn autosaves_expanded_items(&self) -> bool;

    #[objc::msg_send(setAutosaveExpandedItems:)]
    pub fn set_autosaves_expanded_items(&mut self, val: bool);

    #[objc::msg_send(insertItemsAtIndexes:inParent:withAnimation:)]
    #[objc::available(macos = 10.7)]
    pub fn insert_items(
        &mut self,
        indexes: &ns::IndexSet,
        parent: Option<&ns::Id>,
        animation: ns::TableViewAnimationOpts,
    );

    #[objc::msg_send(removeItemsAtIndexes:inParent:withAnimation:)]
    #[objc::available(macos = 10.7)]
    pub fn remove_items(
        &mut self,
        indexes: &ns::IndexSet,
        parent: Option<&ns::Id>,
        animation: ns::TableViewAnimationOpts,
    );

    #[objc::msg_send(moveItemAtIndex:inParent:toIndex:inParent:)]
    #[objc::available(macos = 10.7)]
    pub fn move_item(
        &mut self,
        from_index: ns::Integer,
        old_parent: Option<&ns::Id>,
        to_index: ns::Integer,
        new_parent: Option<&ns::Id>,
    );

    #[objc::msg_send(stronglyReferencesItems)]
    #[objc::available(macos = 10.12)]
    pub fn strongly_references_items(&self) -> bool;

    #[objc::msg_send(setStronglyReferencesItems:)]
    #[objc::available(macos = 10.12)]
    pub fn set_strongly_references_items(&mut self, val: bool);
}

impl ns::AnimatablePropContainer for OutlineView {}

#[objc::protocol(NSOutlineViewDataSource)]
pub trait OutlineViewDataSrc: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(outlineView:numberOfChildrenOfItem:)]
    fn outline_view_children_n_of_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: Option<&ns::Id>,
    ) -> ns::Integer;

    #[objc::optional]
    #[objc::msg_send(outlineView:child:ofItem:)]
    fn outline_view_child_of_item(
        &mut self,
        outline_view: &mut OutlineView,
        index: ns::Integer,
        item: Option<&ns::Id>,
    ) -> arc::R<ns::Id>;

    #[objc::optional]
    #[objc::msg_send(outlineView:isItemExpandable:)]
    fn outline_view_is_item_expandable(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:objectValueForTableColumn:byItem:)]
    fn outline_view_obj_value_for_table_column_by_item(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: Option<&ns::Id>,
        item: Option<&ns::Id>,
    ) -> Option<arc::R<ns::Id>>;

    #[objc::optional]
    #[objc::msg_send(outlineView:setObjectValue:forTableColumn:byItem:)]
    fn outline_view_set_obj_value_for_table_column_by_item(
        &mut self,
        outline_view: &mut OutlineView,
        obj: Option<&ns::Id>,
        table_column: Option<&ns::Id>,
        item: Option<&ns::Id>,
    );

    #[objc::optional]
    #[objc::msg_send(outlineView:itemForPersistentObject:)]
    fn outline_view_item_for_persistent_obj(
        &mut self,
        outline_view: &mut OutlineView,
        obj: &ns::Id,
    ) -> Option<arc::R<ns::Id>>;

    #[objc::optional]
    #[objc::msg_send(outlineView:persistentObjectForItem:)]
    fn outline_view_persistent_obj_for_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: Option<&ns::Id>,
    ) -> Option<arc::R<ns::Id>>;

    /// `old_descriptors` contains `NSSortDescriptor` instances.
    #[objc::optional]
    #[objc::msg_send(outlineView:sortDescriptorsDidChange:)]
    fn outline_view_sort_descriptors_did_change(
        &mut self,
        outline_view: &mut OutlineView,
        old_descriptors: &ns::Array<ns::Id>,
    );

    #[objc::optional]
    #[objc::msg_send(outlineView:pasteboardWriterForItem:)]
    fn outline_view_pasteboard_writer_for_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> Option<arc::R<ns::Id>>;

    /// `session` is an `NSDraggingSession`.
    #[objc::optional]
    #[objc::msg_send(outlineView:draggingSession:willBeginAtPoint:forItems:)]
    fn outline_view_dragging_session_will_begin(
        &mut self,
        outline_view: &mut OutlineView,
        session: &mut ns::Id,
        screen_point: ns::Point,
        dragged_items: &ns::Array<ns::Id>,
    );

    /// `session` is an `NSDraggingSession`.
    #[objc::optional]
    #[objc::msg_send(outlineView:draggingSession:endedAtPoint:operation:)]
    fn outline_view_dragging_session_ended(
        &mut self,
        outline_view: &mut OutlineView,
        session: &mut ns::Id,
        screen_point: ns::Point,
        op: ns::UInteger,
    );

    /// `dragging_info` conforms to `NSDraggingInfo`.
    #[objc::optional]
    #[objc::msg_send(outlineView:updateDraggingItemsForDrag:)]
    fn outline_view_update_dragging_items(
        &mut self,
        outline_view: &mut OutlineView,
        dragging_info: &mut ns::Id,
    );

    /// `dragging_info` conforms to `NSDraggingInfo`.
    #[objc::optional]
    #[objc::msg_send(outlineView:validateDrop:proposedItem:proposedChildIndex:)]
    fn outline_view_validate_drop(
        &mut self,
        outline_view: &mut OutlineView,
        dragging_info: &ns::Id,
        item: Option<&ns::Id>,
        index: ns::Integer,
    ) -> ns::UInteger;

    /// `dragging_info` conforms to `NSDraggingInfo`.
    #[objc::optional]
    #[objc::msg_send(outlineView:acceptDrop:item:childIndex:)]
    fn outline_view_accept_drop(
        &mut self,
        outline_view: &mut OutlineView,
        dragging_info: &ns::Id,
        item: Option<&ns::Id>,
        index: ns::Integer,
    ) -> bool;
}

define_obj_type!(pub AnyOutlineViewDataSrc(ns::Id));

impl OutlineViewDataSrc for AnyOutlineViewDataSrc {}

#[objc::protocol(NSOutlineViewDelegate)]
pub trait OutlineViewDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(outlineView:viewForTableColumn:item:)]
    fn outline_view_view_for_table_column_item(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
    ) -> Option<arc::R<ns::View>>;

    /// The returned view must be an `NSTableRowView`.
    #[objc::optional]
    #[objc::msg_send(outlineView:rowViewForItem:)]
    fn outline_view_row_view_for_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> Option<arc::R<ns::View>>;

    /// `row_view` is an `NSTableRowView`.
    #[objc::optional]
    #[objc::msg_send(outlineView:didAddRowView:forRow:)]
    fn outline_view_did_add_row_view(
        &mut self,
        outline_view: &mut OutlineView,
        row_view: &mut ns::View,
        row: ns::Integer,
    );

    /// `row_view` is an `NSTableRowView`.
    #[objc::optional]
    #[objc::msg_send(outlineView:didRemoveRowView:forRow:)]
    fn outline_view_did_remove_row_view(
        &mut self,
        outline_view: &mut OutlineView,
        row_view: &mut ns::View,
        row: ns::Integer,
    );

    #[objc::optional]
    #[objc::msg_send(outlineView:willDisplayCell:forTableColumn:item:)]
    fn outline_view_will_display_cell(
        &mut self,
        outline_view: &mut OutlineView,
        cell: &mut ns::Id,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldEditTableColumn:item:)]
    fn outline_view_should_edit_table_column(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(selectionShouldChangeInOutlineView:)]
    fn selection_should_change_in_outline_view(&mut self, outline_view: &mut OutlineView) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldSelectItem:)]
    fn outline_view_should_select_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:selectionIndexesForProposedSelection:)]
    fn outline_view_selection_indexes_for_proposed_selection(
        &mut self,
        outline_view: &mut OutlineView,
        proposed_selection_indexes: &ns::IndexSet,
    ) -> arc::R<ns::IndexSet>;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldSelectTableColumn:)]
    fn outline_view_should_select_table_column(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: Option<&ns::Id>,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:mouseDownInHeaderOfTableColumn:)]
    fn outline_view_mouse_down_in_header_of_table_column(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(outlineView:didClickTableColumn:)]
    fn outline_view_did_click_table_column(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(outlineView:didDragTableColumn:)]
    fn outline_view_did_drag_table_column(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(outlineView:toolTipForCell:rect:tableColumn:item:mouseLocation:)]
    fn outline_view_tool_tip_for_cell(
        &mut self,
        outline_view: &mut OutlineView,
        cell: &ns::Cell,
        rect: *mut ns::Rect,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
        mouse_location: ns::Point,
    ) -> arc::R<ns::String>;

    #[objc::optional]
    #[objc::msg_send(outlineView:heightOfRowByItem:)]
    fn outline_view_height_of_row_by_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> cg::Float;

    /// The returned object is an `NSTintConfiguration`.
    #[objc::optional]
    #[objc::msg_send(outlineView:tintConfigurationForItem:)]
    fn outline_view_tint_cfg_for_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> Option<arc::R<ns::Id>>;

    #[objc::optional]
    #[objc::msg_send(outlineView:typeSelectStringForTableColumn:item:)]
    fn outline_view_type_select_string_for_table_column_item(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
    ) -> Option<arc::R<ns::String>>;

    #[objc::optional]
    #[objc::msg_send(outlineView:nextTypeSelectMatchFromItem:toItem:forString:)]
    fn outline_view_next_type_select_match(
        &mut self,
        outline_view: &mut OutlineView,
        start_item: &ns::Id,
        end_item: &ns::Id,
        search_string: &ns::String,
    ) -> Option<arc::R<ns::Id>>;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldTypeSelectForEvent:withCurrentSearchString:)]
    fn outline_view_should_type_select(
        &mut self,
        outline_view: &mut OutlineView,
        event: &ns::Event,
        search_string: Option<&ns::String>,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldShowCellExpansionForTableColumn:item:)]
    fn outline_view_should_show_cell_expansion(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldTrackCell:forTableColumn:item:)]
    fn outline_view_should_track_cell(
        &mut self,
        outline_view: &mut OutlineView,
        cell: &ns::Cell,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:dataCellForTableColumn:item:)]
    fn outline_view_data_cell_for_table_column_item(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
    ) -> Option<arc::R<ns::Cell>>;

    #[objc::optional]
    #[objc::msg_send(outlineView:isGroupItem:)]
    fn outline_view_is_group_item(&mut self, outline_view: &mut OutlineView, item: &ns::Id)
    -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldExpandItem:)]
    fn outline_view_should_expand_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldCollapseItem:)]
    fn outline_view_should_collapse_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:willDisplayOutlineCell:forTableColumn:item:)]
    fn outline_view_will_display_outline_cell(
        &mut self,
        outline_view: &mut OutlineView,
        cell: &mut ns::Id,
        table_column: Option<&ns::Id>,
        item: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(outlineView:sizeToFitWidthOfColumn:)]
    fn outline_view_size_to_fit_width_of_column(
        &mut self,
        outline_view: &mut OutlineView,
        column: ns::Integer,
    ) -> cg::Float;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldReorderColumn:toColumn:)]
    fn outline_view_should_reorder_column(
        &mut self,
        outline_view: &mut OutlineView,
        column: ns::Integer,
        new_column: ns::Integer,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:shouldShowOutlineCellForItem:)]
    fn outline_view_should_show_outline_cell_for_item(
        &mut self,
        outline_view: &mut OutlineView,
        item: &ns::Id,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(outlineView:userCanChangeVisibilityOfTableColumn:)]
    fn outline_view_user_can_change_visibility_of_table_column(
        &mut self,
        outline_view: &mut OutlineView,
        table_column: &ns::Id,
    ) -> bool;

    /// `table_columns` contains `NSTableColumn` instances.
    #[objc::optional]
    #[objc::msg_send(outlineView:userDidChangeVisibilityOfTableColumns:)]
    fn outline_view_user_did_change_visibility_of_table_columns(
        &mut self,
        outline_view: &mut OutlineView,
        table_columns: &ns::Array<ns::Id>,
    );

    #[objc::optional]
    #[objc::msg_send(outlineViewSelectionDidChange:)]
    fn outline_view_selection_did_change(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(outlineViewColumnDidMove:)]
    fn outline_view_column_did_move(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(outlineViewColumnDidResize:)]
    fn outline_view_column_did_resize(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(outlineViewSelectionIsChanging:)]
    fn outline_view_selection_is_changing(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(outlineViewItemWillExpand:)]
    fn outline_view_item_will_expand(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(outlineViewItemDidExpand:)]
    fn outline_view_item_did_expand(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(outlineViewItemWillCollapse:)]
    fn outline_view_item_will_collapse(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(outlineViewItemDidCollapse:)]
    fn outline_view_item_did_collapse(&mut self, notification: &ns::Notification);
}

define_obj_type!(pub AnyOutlineViewDelegate(ns::Id));

impl OutlineViewDelegate for AnyOutlineViewDelegate {}

#[doc(alias = "NSOutlineViewDisclosureButtonKey")]
#[api::available(macos = 10.9)]
pub fn disclosure_button_key() -> &'static ns::UiItemId {
    unsafe { NSOutlineViewDisclosureButtonKey }
}

#[doc(alias = "NSOutlineViewShowHideButtonKey")]
#[api::available(macos = 10.9)]
pub fn show_hide_button_key() -> &'static ns::UiItemId {
    unsafe { NSOutlineViewShowHideButtonKey }
}

pub mod notifications {
    use crate::ns;

    #[doc(alias = "NSOutlineViewSelectionDidChangeNotification")]
    pub fn selection_did_change() -> &'static ns::NotificationName {
        unsafe { NSOutlineViewSelectionDidChangeNotification }
    }

    #[doc(alias = "NSOutlineViewColumnDidMoveNotification")]
    pub fn column_did_move() -> &'static ns::NotificationName {
        unsafe { NSOutlineViewColumnDidMoveNotification }
    }

    #[doc(alias = "NSOutlineViewColumnDidResizeNotification")]
    pub fn column_did_resize() -> &'static ns::NotificationName {
        unsafe { NSOutlineViewColumnDidResizeNotification }
    }

    #[doc(alias = "NSOutlineViewSelectionIsChangingNotification")]
    pub fn selection_is_changing() -> &'static ns::NotificationName {
        unsafe { NSOutlineViewSelectionIsChangingNotification }
    }

    #[doc(alias = "NSOutlineViewItemWillExpandNotification")]
    pub fn item_will_expand() -> &'static ns::NotificationName {
        unsafe { NSOutlineViewItemWillExpandNotification }
    }

    #[doc(alias = "NSOutlineViewItemDidExpandNotification")]
    pub fn item_did_expand() -> &'static ns::NotificationName {
        unsafe { NSOutlineViewItemDidExpandNotification }
    }

    #[doc(alias = "NSOutlineViewItemWillCollapseNotification")]
    pub fn item_will_collapse() -> &'static ns::NotificationName {
        unsafe { NSOutlineViewItemWillCollapseNotification }
    }

    #[doc(alias = "NSOutlineViewItemDidCollapseNotification")]
    pub fn item_did_collapse() -> &'static ns::NotificationName {
        unsafe { NSOutlineViewItemDidCollapseNotification }
    }

    unsafe extern "C" {
        static NSOutlineViewSelectionDidChangeNotification: &'static ns::NotificationName;
        static NSOutlineViewColumnDidMoveNotification: &'static ns::NotificationName;
        static NSOutlineViewColumnDidResizeNotification: &'static ns::NotificationName;
        static NSOutlineViewSelectionIsChangingNotification: &'static ns::NotificationName;
        static NSOutlineViewItemWillExpandNotification: &'static ns::NotificationName;
        static NSOutlineViewItemDidExpandNotification: &'static ns::NotificationName;
        static NSOutlineViewItemWillCollapseNotification: &'static ns::NotificationName;
        static NSOutlineViewItemDidCollapseNotification: &'static ns::NotificationName;
    }
}

unsafe extern "C" {
    static NS_OUTLINE_VIEW: &'static objc::Class<OutlineView>;
}

#[api::weak]
unsafe extern "C" {
    #[api::available(macos = 10.9)]
    static NSOutlineViewDisclosureButtonKey: &'static ns::UiItemId;

    #[api::available(macos = 10.9)]
    static NSOutlineViewShowHideButtonKey: &'static ns::UiItemId;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let outline_view = ns::OutlineView::with_frame(ns::Rect::new(0.0, 0.0, 320.0, 200.0));
        assert_eq!(outline_view.rows_n(), 0);
        assert_eq!(outline_view.columns_n(), 0);
        assert!(outline_view.outline_table_column().is_none());
        assert!(outline_view.autoresizes_outline_column());
    }
}
