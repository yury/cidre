use crate::{api, arc, cg, define_obj_type, define_opts, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "NSTableViewDropOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TableViewDropOp {
    On = 0,
    Above = 1,
}

#[doc(alias = "NSTableViewColumnAutoresizingStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TableViewColumnAutoresizingStyle {
    None = 0,
    Uniform = 1,
    Sequential = 2,
    ReverseSequential = 3,
    LastColumnOnly = 4,
    FirstColumnOnly = 5,
}

define_opts!(
    #[doc(alias = "NSTableViewGridLineStyle")]
    pub TableViewGridLineStyle(usize)
);

impl TableViewGridLineStyle {
    #[doc(alias = "NSTableViewGridNone")]
    pub const NONE: Self = Self(0);

    #[doc(alias = "NSTableViewSolidVerticalGridLineMask")]
    pub const SOLID_VERTICAL: Self = Self(1 << 0);

    #[doc(alias = "NSTableViewSolidHorizontalGridLineMask")]
    pub const SOLID_HORIZONTAL: Self = Self(1 << 1);

    #[doc(alias = "NSTableViewDashedHorizontalGridLineMask")]
    pub const DASHED_HORIZONTAL: Self = Self(1 << 3);
}

#[doc(alias = "NSTableViewRowSizeStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TableViewRowSizeStyle {
    Default = -1,
    Custom = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

#[doc(alias = "NSTableViewStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TableViewStyle {
    Automatic = 0,
    FullWidth = 1,
    Inset = 2,
    SourceList = 3,
    Plain = 4,
}

#[doc(alias = "NSTableViewSelectionHighlightStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TableViewSelectionHighlightStyle {
    None = -1,
    Regular = 0,
    SourceList = 1,
}

#[doc(alias = "NSTableViewDraggingDestinationFeedbackStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TableViewDraggingDestinationFeedbackStyle {
    None = -1,
    Regular = 0,
    SourceList = 1,
    Gap = 2,
}

#[doc(alias = "NSTableRowActionEdge")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TableRowActionEdge {
    Leading = 0,
    Trailing = 1,
}

define_opts!(
    #[doc(alias = "NSTableViewAnimationOptions")]
    pub TableViewAnimationOpts(usize)
);

impl TableViewAnimationOpts {
    #[doc(alias = "NSTableViewAnimationEffectNone")]
    pub const NONE: Self = Self(0);

    #[doc(alias = "NSTableViewAnimationEffectFade")]
    pub const FADE: Self = Self(0x1);

    #[doc(alias = "NSTableViewAnimationEffectGap")]
    pub const GAP: Self = Self(0x2);

    #[doc(alias = "NSTableViewAnimationSlideUp")]
    pub const SLIDE_UP: Self = Self(0x10);

    #[doc(alias = "NSTableViewAnimationSlideDown")]
    pub const SLIDE_DOWN: Self = Self(0x20);

    #[doc(alias = "NSTableViewAnimationSlideLeft")]
    pub const SLIDE_LEFT: Self = Self(0x30);

    #[doc(alias = "NSTableViewAnimationSlideRight")]
    pub const SLIDE_RIGHT: Self = Self(0x40);
}

#[doc(alias = "NSTableViewAutosaveName")]
pub type TableViewAutosaveName = ns::String;

define_obj_type!(
    #[doc(alias = "NSTableView")]
    pub TableView(ns::Control),
    NS_TABLE_VIEW
);

impl TableView {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<TableView>;

    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(dataSource)]
    pub fn data_src(&self) -> Option<arc::R<AnyTableViewDataSrc>>;

    #[objc::msg_send(setDataSource:)]
    pub fn set_data_src<D: TableViewDataSrc>(&mut self, val: Option<&D>);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyTableViewDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: TableViewDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(headerView)]
    pub fn header_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setHeaderView:)]
    pub fn set_header_view(&mut self, val: Option<&ns::View>);

    #[objc::msg_send(cornerView)]
    pub fn corner_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setCornerView:)]
    pub fn set_corner_view(&mut self, val: Option<&ns::View>);

    #[objc::msg_send(allowsColumnReordering)]
    pub fn allows_column_reordering(&self) -> bool;

    #[objc::msg_send(setAllowsColumnReordering:)]
    pub fn set_allows_column_reordering(&mut self, val: bool);

    #[objc::msg_send(allowsColumnResizing)]
    pub fn allows_column_resizing(&self) -> bool;

    #[objc::msg_send(setAllowsColumnResizing:)]
    pub fn set_allows_column_resizing(&mut self, val: bool);

    #[objc::msg_send(columnAutoresizingStyle)]
    pub fn column_autoresizing_style(&self) -> TableViewColumnAutoresizingStyle;

    #[objc::msg_send(setColumnAutoresizingStyle:)]
    pub fn set_column_autoresizing_style(&mut self, val: TableViewColumnAutoresizingStyle);

    #[objc::msg_send(gridStyleMask)]
    pub fn grid_style_mask(&self) -> TableViewGridLineStyle;

    #[objc::msg_send(setGridStyleMask:)]
    pub fn set_grid_style_mask(&mut self, val: TableViewGridLineStyle);

    #[objc::msg_send(intercellSpacing)]
    pub fn intercell_spacing(&self) -> ns::Size;

    #[objc::msg_send(setIntercellSpacing:)]
    pub fn set_intercell_spacing(&mut self, val: ns::Size);

    #[objc::msg_send(usesAlternatingRowBackgroundColors)]
    pub fn uses_alternating_row_bg_colors(&self) -> bool;

    #[objc::msg_send(setUsesAlternatingRowBackgroundColors:)]
    pub fn set_uses_alternating_row_bg_colors(&mut self, val: bool);

    #[objc::msg_send(backgroundColor)]
    pub fn bg_color(&self) -> arc::R<ns::Color>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_bg_color(&mut self, val: &ns::Color);

    #[objc::msg_send(gridColor)]
    pub fn grid_color(&self) -> arc::R<ns::Color>;

    #[objc::msg_send(setGridColor:)]
    pub fn set_grid_color(&mut self, val: &ns::Color);

    #[objc::msg_send(rowSizeStyle)]
    #[objc::available(macos = 10.7)]
    pub fn row_size_style(&self) -> TableViewRowSizeStyle;

    #[objc::msg_send(setRowSizeStyle:)]
    #[objc::available(macos = 10.7)]
    pub fn set_row_size_style(&mut self, val: TableViewRowSizeStyle);

    #[objc::msg_send(effectiveRowSizeStyle)]
    #[objc::available(macos = 10.7)]
    pub fn effective_row_size_style(&self) -> TableViewRowSizeStyle;

    #[objc::msg_send(rowHeight)]
    pub fn row_height(&self) -> cg::Float;

    #[objc::msg_send(setRowHeight:)]
    pub fn set_row_height(&mut self, val: cg::Float);

    #[objc::msg_send(noteHeightOfRowsWithIndexesChanged:)]
    pub fn note_height_of_rows_with_indexes_changed(&mut self, indexes: &ns::IndexSet);

    /// The returned objects are `NSTableColumn` instances.
    #[objc::msg_send(tableColumns)]
    pub fn table_columns(&self) -> arc::R<ns::Array<ns::Id>>;

    #[objc::msg_send(numberOfColumns)]
    pub fn columns_n(&self) -> ns::Integer;

    #[objc::msg_send(numberOfRows)]
    pub fn rows_n(&self) -> ns::Integer;

    /// `table_column` must be an `NSTableColumn`.
    #[objc::msg_send(addTableColumn:)]
    pub fn add_table_column(&mut self, table_column: &ns::Id);

    /// `table_column` must be an `NSTableColumn`.
    #[objc::msg_send(removeTableColumn:)]
    pub fn remove_table_column(&mut self, table_column: &ns::Id);

    #[objc::msg_send(moveColumn:toColumn:)]
    pub fn move_column_to_column(&mut self, old_index: ns::Integer, new_index: ns::Integer);

    #[objc::msg_send(columnWithIdentifier:)]
    pub fn column_with_id(&self, identifier: &ns::UiItemId) -> ns::Integer;

    /// The returned object is an `NSTableColumn`.
    #[objc::msg_send(tableColumnWithIdentifier:)]
    pub fn table_column_with_id(&self, identifier: &ns::UiItemId) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(tile)]
    pub fn tile(&mut self);

    #[objc::msg_send(sizeToFit)]
    pub fn size_to_fit(&mut self);

    #[objc::msg_send(sizeLastColumnToFit)]
    pub fn size_last_column_to_fit(&mut self);

    #[objc::msg_send(scrollRowToVisible:)]
    pub fn scroll_row_to_visible(&mut self, row: ns::Integer);

    #[objc::msg_send(scrollColumnToVisible:)]
    pub fn scroll_column_to_visible(&mut self, column: ns::Integer);

    #[objc::msg_send(reloadData)]
    pub fn reload_data(&mut self);

    #[objc::msg_send(noteNumberOfRowsChanged)]
    pub fn note_rows_n_changed(&mut self);

    #[objc::msg_send(reloadDataForRowIndexes:columnIndexes:)]
    #[objc::available(macos = 10.6)]
    pub fn reload_data_for_rows_columns(&mut self, rows: &ns::IndexSet, columns: &ns::IndexSet);

    #[objc::msg_send(editedColumn)]
    pub fn edited_column(&self) -> ns::Integer;

    #[objc::msg_send(editedRow)]
    pub fn edited_row(&self) -> ns::Integer;

    #[objc::msg_send(clickedColumn)]
    pub fn clicked_column(&self) -> ns::Integer;

    #[objc::msg_send(clickedRow)]
    pub fn clicked_row(&self) -> ns::Integer;

    #[objc::msg_send(doubleAction)]
    pub fn double_action(&self) -> *const objc::Sel;

    #[objc::msg_send(setDoubleAction:)]
    pub fn set_double_action(&mut self, val: *const objc::Sel);

    /// The array elements are `NSSortDescriptor` instances.
    #[objc::msg_send(sortDescriptors)]
    pub fn sort_descriptors(&self) -> arc::R<ns::Array<ns::Id>>;

    /// The array elements must be `NSSortDescriptor` instances.
    #[objc::msg_send(setSortDescriptors:)]
    pub fn set_sort_descriptors(&mut self, val: &ns::Array<ns::Id>);

    #[objc::msg_send(setIndicatorImage:inTableColumn:)]
    pub fn set_indicator_image_in_table_column(
        &mut self,
        image: Option<&ns::Image>,
        table_column: &ns::Id,
    );

    #[objc::msg_send(indicatorImageInTableColumn:)]
    pub fn indicator_image_in_table_column(
        &self,
        table_column: &ns::Id,
    ) -> Option<arc::R<ns::Image>>;

    /// The returned object is an `NSTableColumn`.
    #[objc::msg_send(highlightedTableColumn)]
    pub fn highlighted_table_column(&self) -> Option<arc::R<ns::Id>>;

    /// `val` must be an `NSTableColumn`.
    #[objc::msg_send(setHighlightedTableColumn:)]
    pub fn set_highlighted_table_column(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(verticalMotionCanBeginDrag)]
    pub fn vertical_motion_can_begin_drag(&self) -> bool;

    #[objc::msg_send(setVerticalMotionCanBeginDrag:)]
    pub fn set_vertical_motion_can_begin_drag(&mut self, val: bool);

    #[objc::msg_send(canDragRowsWithIndexes:atPoint:)]
    pub fn can_drag_rows_at_point(
        &self,
        row_indexes: &ns::IndexSet,
        mouse_down_point: ns::Point,
    ) -> bool;

    /// `table_columns` must contain `NSTableColumn` instances.
    #[objc::msg_send(dragImageForRowsWithIndexes:tableColumns:event:offset:)]
    pub fn drag_image_for_rows(
        &self,
        rows: &ns::IndexSet,
        table_columns: &ns::Array<ns::Id>,
        event: &ns::Event,
        offset: *mut ns::Point,
    ) -> arc::R<ns::Image>;

    #[objc::msg_send(setDraggingSourceOperationMask:forLocal:)]
    pub fn set_dragging_src_op_mask_for_local(&mut self, mask: ns::UInteger, is_local: bool);

    #[objc::msg_send(setDropRow:dropOperation:)]
    pub fn set_drop_row_op(&mut self, row: ns::Integer, op: TableViewDropOp);

    #[objc::msg_send(allowsMultipleSelection)]
    pub fn allows_multiple_selection(&self) -> bool;

    #[objc::msg_send(setAllowsMultipleSelection:)]
    pub fn set_allows_multiple_selection(&mut self, val: bool);

    #[objc::msg_send(allowsEmptySelection)]
    pub fn allows_empty_selection(&self) -> bool;

    #[objc::msg_send(setAllowsEmptySelection:)]
    pub fn set_allows_empty_selection(&mut self, val: bool);

    #[objc::msg_send(allowsColumnSelection)]
    pub fn allows_column_selection(&self) -> bool;

    #[objc::msg_send(setAllowsColumnSelection:)]
    pub fn set_allows_column_selection(&mut self, val: bool);

    #[objc::msg_send(selectAll:)]
    pub fn select_all(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(deselectAll:)]
    pub fn deselect_all(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(selectColumnIndexes:byExtendingSelection:)]
    pub fn select_column_indexes(&mut self, indexes: &ns::IndexSet, extend: bool);

    #[objc::msg_send(selectRowIndexes:byExtendingSelection:)]
    pub fn select_row_indexes(&mut self, indexes: &ns::IndexSet, extend: bool);

    #[objc::msg_send(selectedColumnIndexes)]
    pub fn selected_column_indexes(&self) -> arc::R<ns::IndexSet>;

    #[objc::msg_send(selectedRowIndexes)]
    pub fn selected_row_indexes(&self) -> arc::R<ns::IndexSet>;

    #[objc::msg_send(deselectColumn:)]
    pub fn deselect_column(&mut self, column: ns::Integer);

    #[objc::msg_send(deselectRow:)]
    pub fn deselect_row(&mut self, row: ns::Integer);

    #[objc::msg_send(selectedColumn)]
    pub fn selected_column(&self) -> ns::Integer;

    #[objc::msg_send(selectedRow)]
    pub fn selected_row(&self) -> ns::Integer;

    #[objc::msg_send(isColumnSelected:)]
    pub fn is_column_selected(&self, column: ns::Integer) -> bool;

    #[objc::msg_send(isRowSelected:)]
    pub fn is_row_selected(&self, row: ns::Integer) -> bool;

    #[objc::msg_send(numberOfSelectedColumns)]
    pub fn selected_columns_n(&self) -> ns::Integer;

    #[objc::msg_send(numberOfSelectedRows)]
    pub fn selected_rows_n(&self) -> ns::Integer;

    #[objc::msg_send(allowsTypeSelect)]
    #[objc::available(macos = 10.5)]
    pub fn allows_type_select(&self) -> bool;

    #[objc::msg_send(setAllowsTypeSelect:)]
    #[objc::available(macos = 10.5)]
    pub fn set_allows_type_select(&mut self, val: bool);

    #[objc::msg_send(style)]
    #[objc::available(macos = 11.0)]
    pub fn style(&self) -> TableViewStyle;

    #[objc::msg_send(setStyle:)]
    #[objc::available(macos = 11.0)]
    pub fn set_style(&mut self, val: TableViewStyle);

    #[objc::msg_send(effectiveStyle)]
    #[objc::available(macos = 11.0)]
    pub fn effective_style(&self) -> TableViewStyle;

    #[objc::msg_send(selectionHighlightStyle)]
    #[objc::available(macos = 10.5)]
    pub fn selection_highlight_style(&self) -> TableViewSelectionHighlightStyle;

    #[objc::msg_send(setSelectionHighlightStyle:)]
    #[objc::available(macos = 10.5)]
    pub fn set_selection_highlight_style(&mut self, val: TableViewSelectionHighlightStyle);

    #[objc::msg_send(draggingDestinationFeedbackStyle)]
    #[objc::available(macos = 10.6)]
    pub fn dragging_dst_feedback_style(&self) -> TableViewDraggingDestinationFeedbackStyle;

    #[objc::msg_send(setDraggingDestinationFeedbackStyle:)]
    #[objc::available(macos = 10.6)]
    pub fn set_dragging_dst_feedback_style(
        &mut self,
        val: TableViewDraggingDestinationFeedbackStyle,
    );

    #[objc::msg_send(rectOfColumn:)]
    pub fn rect_of_column(&self, column: ns::Integer) -> ns::Rect;

    #[objc::msg_send(rectOfRow:)]
    pub fn rect_of_row(&self, row: ns::Integer) -> ns::Rect;

    #[objc::msg_send(columnIndexesInRect:)]
    #[objc::available(macos = 10.5)]
    pub fn column_indexes_in_rect(&self, rect: ns::Rect) -> arc::R<ns::IndexSet>;

    #[objc::msg_send(rowsInRect:)]
    pub fn rows_in_rect(&self, rect: ns::Rect) -> ns::Range;

    #[objc::msg_send(columnAtPoint:)]
    pub fn column_at_point(&self, point: ns::Point) -> ns::Integer;

    #[objc::msg_send(rowAtPoint:)]
    pub fn row_at_point(&self, point: ns::Point) -> ns::Integer;

    #[objc::msg_send(frameOfCellAtColumn:row:)]
    pub fn frame_of_cell(&self, column: ns::Integer, row: ns::Integer) -> ns::Rect;

    #[objc::msg_send(autosaveName)]
    pub fn autosave_name(&self) -> Option<arc::R<TableViewAutosaveName>>;

    #[objc::msg_send(setAutosaveName:)]
    pub fn set_autosave_name(&mut self, val: Option<&TableViewAutosaveName>);

    #[objc::msg_send(autosaveTableColumns)]
    pub fn autosaves_table_columns(&self) -> bool;

    #[objc::msg_send(setAutosaveTableColumns:)]
    pub fn set_autosaves_table_columns(&mut self, val: bool);

    #[objc::msg_send(editColumn:row:withEvent:select:)]
    pub fn edit_column_row(
        &mut self,
        column: ns::Integer,
        row: ns::Integer,
        event: Option<&ns::Event>,
        select: bool,
    );

    #[objc::msg_send(drawRow:clipRect:)]
    pub fn draw_row(&mut self, row: ns::Integer, clip_rect: ns::Rect);

    #[objc::msg_send(highlightSelectionInClipRect:)]
    pub fn highlight_selection_in_clip_rect(&mut self, clip_rect: ns::Rect);

    #[objc::msg_send(drawGridInClipRect:)]
    pub fn draw_grid_in_clip_rect(&mut self, clip_rect: ns::Rect);

    #[objc::msg_send(drawBackgroundInClipRect:)]
    pub fn draw_bg_in_clip_rect(&mut self, clip_rect: ns::Rect);

    #[objc::msg_send(viewAtColumn:row:makeIfNecessary:)]
    #[objc::available(macos = 10.7)]
    pub fn view_at_column_row(
        &self,
        column: ns::Integer,
        row: ns::Integer,
        make_if_necessary: bool,
    ) -> Option<arc::R<ns::View>>;

    /// The returned view is an `NSTableRowView`.
    #[objc::msg_send(rowViewAtRow:makeIfNecessary:)]
    #[objc::available(macos = 10.7)]
    pub fn row_view_at_row(
        &self,
        row: ns::Integer,
        make_if_necessary: bool,
    ) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(rowForView:)]
    #[objc::available(macos = 10.7)]
    pub fn row_for_view(&self, view: &ns::View) -> ns::Integer;

    #[objc::msg_send(columnForView:)]
    #[objc::available(macos = 10.7)]
    pub fn column_for_view(&self, view: &ns::View) -> ns::Integer;

    #[objc::msg_send(makeViewWithIdentifier:owner:)]
    #[objc::available(macos = 10.7)]
    pub fn make_view_with_id(
        &self,
        identifier: &ns::UiItemId,
        owner: Option<&ns::Id>,
    ) -> Option<arc::R<ns::View>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(enumerateAvailableRowViewsUsingBlock:)]
    #[objc::available(macos = 10.7)]
    pub fn enumerate_available_row_views(
        &self,
        block: &mut blocks::NoEscBlock<fn(&ns::View, ns::Integer)>,
    );

    #[objc::msg_send(floatsGroupRows)]
    #[objc::available(macos = 10.7)]
    pub fn floats_group_rows(&self) -> bool;

    #[objc::msg_send(setFloatsGroupRows:)]
    #[objc::available(macos = 10.7)]
    pub fn set_floats_group_rows(&mut self, val: bool);

    #[objc::msg_send(rowActionsVisible)]
    #[objc::available(macos = 10.11)]
    pub fn row_actions_visible(&self) -> bool;

    #[objc::msg_send(setRowActionsVisible:)]
    #[objc::available(macos = 10.11)]
    pub fn set_row_actions_visible(&mut self, val: bool);

    #[objc::msg_send(beginUpdates)]
    #[objc::available(macos = 10.7)]
    pub fn begin_updates(&mut self);

    #[objc::msg_send(endUpdates)]
    #[objc::available(macos = 10.7)]
    pub fn end_updates(&mut self);

    #[objc::msg_send(insertRowsAtIndexes:withAnimation:)]
    #[objc::available(macos = 10.7)]
    pub fn insert_rows(&mut self, indexes: &ns::IndexSet, animation: TableViewAnimationOpts);

    #[objc::msg_send(removeRowsAtIndexes:withAnimation:)]
    #[objc::available(macos = 10.7)]
    pub fn remove_rows(&mut self, indexes: &ns::IndexSet, animation: TableViewAnimationOpts);

    #[objc::msg_send(moveRowAtIndex:toIndex:)]
    #[objc::available(macos = 10.7)]
    pub fn move_row(&mut self, old_index: ns::Integer, new_index: ns::Integer);

    #[objc::msg_send(hideRowsAtIndexes:withAnimation:)]
    #[objc::available(macos = 10.11)]
    pub fn hide_rows(&mut self, indexes: &ns::IndexSet, animation: TableViewAnimationOpts);

    #[objc::msg_send(unhideRowsAtIndexes:withAnimation:)]
    #[objc::available(macos = 10.11)]
    pub fn unhide_rows(&mut self, indexes: &ns::IndexSet, animation: TableViewAnimationOpts);

    #[objc::msg_send(hiddenRowIndexes)]
    #[objc::available(macos = 10.11)]
    pub fn hidden_row_indexes(&self) -> arc::R<ns::IndexSet>;

    /// `nib` must be an `NSNib`.
    #[objc::msg_send(registerNib:forIdentifier:)]
    #[objc::available(macos = 10.8)]
    pub fn register_nib(&mut self, nib: Option<&ns::Id>, identifier: &ns::UiItemId);

    /// The dictionary values are `NSNib` instances.
    #[objc::msg_send(registeredNibsByIdentifier)]
    #[objc::available(macos = 10.8)]
    pub fn registered_nibs_by_id(&self) -> Option<arc::R<ns::Dictionary<ns::UiItemId, ns::Id>>>;

    /// `row_view` must be an `NSTableRowView`.
    #[objc::msg_send(didAddRowView:forRow:)]
    #[objc::available(macos = 10.7)]
    pub fn did_add_row_view(&mut self, row_view: &ns::View, row: ns::Integer);

    /// `row_view` must be an `NSTableRowView`.
    #[objc::msg_send(didRemoveRowView:forRow:)]
    #[objc::available(macos = 10.7)]
    pub fn did_remove_row_view(&mut self, row_view: &ns::View, row: ns::Integer);

    #[objc::msg_send(usesStaticContents)]
    #[objc::available(macos = 10.10)]
    pub fn uses_static_contents(&self) -> bool;

    #[objc::msg_send(setUsesStaticContents:)]
    #[objc::available(macos = 10.10)]
    pub fn set_uses_static_contents(&mut self, val: bool);

    #[objc::msg_send(usesAutomaticRowHeights)]
    #[objc::available(macos = 10.13)]
    pub fn uses_automatic_row_heights(&self) -> bool;

    #[objc::msg_send(setUsesAutomaticRowHeights:)]
    #[objc::available(macos = 10.13)]
    pub fn set_uses_automatic_row_heights(&mut self, val: bool);
}

impl ns::AnimatablePropContainer for TableView {}

#[objc::protocol(NSTableViewDelegate)]
pub trait TableViewDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(tableView:viewForTableColumn:row:)]
    fn table_view_view_for_table_column_row(
        &mut self,
        table_view: &mut TableView,
        table_column: Option<&ns::Id>,
        row: ns::Integer,
    ) -> Option<arc::R<ns::View>>;

    /// The returned view must be an `NSTableRowView`.
    #[objc::optional]
    #[objc::msg_send(tableView:rowViewForRow:)]
    fn table_view_row_view_for_row(
        &mut self,
        table_view: &mut TableView,
        row: ns::Integer,
    ) -> Option<arc::R<ns::View>>;

    /// `row_view` is an `NSTableRowView`.
    #[objc::optional]
    #[objc::msg_send(tableView:didAddRowView:forRow:)]
    fn table_view_did_add_row_view(
        &mut self,
        table_view: &mut TableView,
        row_view: &mut ns::View,
        row: ns::Integer,
    );

    /// `row_view` is an `NSTableRowView`.
    #[objc::optional]
    #[objc::msg_send(tableView:didRemoveRowView:forRow:)]
    fn table_view_did_remove_row_view(
        &mut self,
        table_view: &mut TableView,
        row_view: &mut ns::View,
        row: ns::Integer,
    );

    #[objc::optional]
    #[objc::msg_send(selectionShouldChangeInTableView:)]
    fn selection_should_change_in_table_view(&mut self, table_view: &mut TableView) -> bool;

    #[objc::optional]
    #[objc::msg_send(tableView:shouldSelectRow:)]
    fn table_view_should_select_row(
        &mut self,
        table_view: &mut TableView,
        row: ns::Integer,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(tableView:selectionIndexesForProposedSelection:)]
    fn table_view_selection_indexes_for_proposed_selection(
        &mut self,
        table_view: &mut TableView,
        proposed_selection_indexes: &ns::IndexSet,
    ) -> arc::R<ns::IndexSet>;

    #[objc::optional]
    #[objc::msg_send(tableView:shouldSelectTableColumn:)]
    fn table_view_should_select_table_column(
        &mut self,
        table_view: &mut TableView,
        table_column: Option<&ns::Id>,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(tableView:mouseDownInHeaderOfTableColumn:)]
    fn table_view_mouse_down_in_header_of_table_column(
        &mut self,
        table_view: &mut TableView,
        table_column: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(tableView:didClickTableColumn:)]
    fn table_view_did_click_table_column(
        &mut self,
        table_view: &mut TableView,
        table_column: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(tableView:didDragTableColumn:)]
    fn table_view_did_drag_table_column(
        &mut self,
        table_view: &mut TableView,
        table_column: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(tableView:heightOfRow:)]
    fn table_view_height_of_row(
        &mut self,
        table_view: &mut TableView,
        row: ns::Integer,
    ) -> cg::Float;

    #[objc::optional]
    #[objc::msg_send(tableView:typeSelectStringForTableColumn:row:)]
    fn table_view_type_select_string_for_table_column_row(
        &mut self,
        table_view: &mut TableView,
        table_column: Option<&ns::Id>,
        row: ns::Integer,
    ) -> Option<arc::R<ns::String>>;

    #[objc::optional]
    #[objc::msg_send(tableView:nextTypeSelectMatchFromRow:toRow:forString:)]
    fn table_view_next_type_select_match(
        &mut self,
        table_view: &mut TableView,
        start_row: ns::Integer,
        end_row: ns::Integer,
        search_string: &ns::String,
    ) -> ns::Integer;

    #[objc::optional]
    #[objc::msg_send(tableView:shouldTypeSelectForEvent:withCurrentSearchString:)]
    fn table_view_should_type_select(
        &mut self,
        table_view: &mut TableView,
        event: &ns::Event,
        search_string: Option<&ns::String>,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(tableView:isGroupRow:)]
    fn table_view_is_group_row(&mut self, table_view: &mut TableView, row: ns::Integer) -> bool;

    #[objc::optional]
    #[objc::msg_send(tableView:sizeToFitWidthOfColumn:)]
    fn table_view_size_to_fit_width_of_column(
        &mut self,
        table_view: &mut TableView,
        column: ns::Integer,
    ) -> cg::Float;

    #[objc::optional]
    #[objc::msg_send(tableView:shouldReorderColumn:toColumn:)]
    fn table_view_should_reorder_column(
        &mut self,
        table_view: &mut TableView,
        column: ns::Integer,
        new_column: ns::Integer,
    ) -> bool;

    /// The returned array contains `NSTableViewRowAction` instances.
    #[objc::optional]
    #[objc::msg_send(tableView:rowActionsForRow:edge:)]
    fn table_view_row_actions_for_row(
        &mut self,
        table_view: &mut TableView,
        row: ns::Integer,
        edge: TableRowActionEdge,
    ) -> arc::R<ns::Array<ns::Id>>;

    #[objc::optional]
    #[objc::msg_send(tableView:userCanChangeVisibilityOfTableColumn:)]
    fn table_view_user_can_change_visibility_of_table_column(
        &mut self,
        table_view: &mut TableView,
        table_column: &ns::Id,
    ) -> bool;

    /// `table_columns` contains `NSTableColumn` instances.
    #[objc::optional]
    #[objc::msg_send(tableView:userDidChangeVisibilityOfTableColumns:)]
    fn table_view_user_did_change_visibility_of_table_columns(
        &mut self,
        table_view: &mut TableView,
        table_columns: &ns::Array<ns::Id>,
    );

    #[objc::optional]
    #[objc::msg_send(tableViewSelectionDidChange:)]
    fn table_view_selection_did_change(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(tableViewColumnDidMove:)]
    fn table_view_column_did_move(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(tableViewColumnDidResize:)]
    fn table_view_column_did_resize(&mut self, notification: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(tableViewSelectionIsChanging:)]
    fn table_view_selection_is_changing(&mut self, notification: &ns::Notification);
}

define_obj_type!(pub AnyTableViewDelegate(ns::Id));

impl TableViewDelegate for AnyTableViewDelegate {}

#[objc::protocol(NSTableViewDataSource)]
pub trait TableViewDataSrc: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(numberOfRowsInTableView:)]
    fn rows_n_in_table_view(&mut self, table_view: &mut TableView) -> ns::Integer;

    #[objc::optional]
    #[objc::msg_send(tableView:objectValueForTableColumn:row:)]
    fn table_view_obj_value_for_table_column_row(
        &mut self,
        table_view: &mut TableView,
        table_column: Option<&ns::Id>,
        row: ns::Integer,
    ) -> Option<arc::R<ns::Id>>;

    #[objc::optional]
    #[objc::msg_send(tableView:setObjectValue:forTableColumn:row:)]
    fn table_view_set_obj_value_for_table_column_row(
        &mut self,
        table_view: &mut TableView,
        obj: Option<&ns::Id>,
        table_column: Option<&ns::Id>,
        row: ns::Integer,
    );

    /// `old_descriptors` contains `NSSortDescriptor` instances.
    #[objc::optional]
    #[objc::msg_send(tableView:sortDescriptorsDidChange:)]
    fn table_view_sort_descriptors_did_change(
        &mut self,
        table_view: &mut TableView,
        old_descriptors: &ns::Array<ns::Id>,
    );

    #[objc::optional]
    #[objc::msg_send(tableView:pasteboardWriterForRow:)]
    fn table_view_pasteboard_writer_for_row(
        &mut self,
        table_view: &mut TableView,
        row: ns::Integer,
    ) -> Option<arc::R<ns::Id>>;

    /// `session` is an `NSDraggingSession`.
    #[objc::optional]
    #[objc::msg_send(tableView:draggingSession:willBeginAtPoint:forRowIndexes:)]
    fn table_view_dragging_session_will_begin(
        &mut self,
        table_view: &mut TableView,
        session: &mut ns::Id,
        screen_point: ns::Point,
        row_indexes: &ns::IndexSet,
    );

    /// `session` is an `NSDraggingSession`.
    #[objc::optional]
    #[objc::msg_send(tableView:draggingSession:endedAtPoint:operation:)]
    fn table_view_dragging_session_ended(
        &mut self,
        table_view: &mut TableView,
        session: &mut ns::Id,
        screen_point: ns::Point,
        op: ns::UInteger,
    );

    /// `dragging_info` conforms to `NSDraggingInfo`.
    #[objc::optional]
    #[objc::msg_send(tableView:updateDraggingItemsForDrag:)]
    fn table_view_update_dragging_items(
        &mut self,
        table_view: &mut TableView,
        dragging_info: &mut ns::Id,
    );

    /// `dragging_info` conforms to `NSDraggingInfo`.
    #[objc::optional]
    #[objc::msg_send(tableView:validateDrop:proposedRow:proposedDropOperation:)]
    fn table_view_validate_drop(
        &mut self,
        table_view: &mut TableView,
        dragging_info: &ns::Id,
        row: ns::Integer,
        op: TableViewDropOp,
    ) -> ns::UInteger;

    /// `dragging_info` conforms to `NSDraggingInfo`.
    #[objc::optional]
    #[objc::msg_send(tableView:acceptDrop:row:dropOperation:)]
    fn table_view_accept_drop(
        &mut self,
        table_view: &mut TableView,
        dragging_info: &ns::Id,
        row: ns::Integer,
        op: TableViewDropOp,
    ) -> bool;
}

define_obj_type!(pub AnyTableViewDataSrc(ns::Id));

impl TableViewDataSrc for AnyTableViewDataSrc {}

#[doc(alias = "NSTableViewRowViewKey")]
#[api::available(macos = 10.7)]
pub fn row_view_key() -> &'static ns::UiItemId {
    unsafe { NSTableViewRowViewKey }
}

pub mod notifications {
    use crate::ns;

    #[doc(alias = "NSTableViewSelectionDidChangeNotification")]
    pub fn selection_did_change() -> &'static ns::NotificationName {
        unsafe { NSTableViewSelectionDidChangeNotification }
    }

    #[doc(alias = "NSTableViewColumnDidMoveNotification")]
    pub fn column_did_move() -> &'static ns::NotificationName {
        unsafe { NSTableViewColumnDidMoveNotification }
    }

    #[doc(alias = "NSTableViewColumnDidResizeNotification")]
    pub fn column_did_resize() -> &'static ns::NotificationName {
        unsafe { NSTableViewColumnDidResizeNotification }
    }

    #[doc(alias = "NSTableViewSelectionIsChangingNotification")]
    pub fn selection_is_changing() -> &'static ns::NotificationName {
        unsafe { NSTableViewSelectionIsChangingNotification }
    }

    unsafe extern "C" {
        static NSTableViewSelectionDidChangeNotification: &'static ns::NotificationName;
        static NSTableViewColumnDidMoveNotification: &'static ns::NotificationName;
        static NSTableViewColumnDidResizeNotification: &'static ns::NotificationName;
        static NSTableViewSelectionIsChangingNotification: &'static ns::NotificationName;
    }
}

unsafe extern "C" {
    static NS_TABLE_VIEW: &'static objc::Class<TableView>;
}

#[api::weak]
unsafe extern "C" {
    #[api::available(macos = 10.7)]
    static NSTableViewRowViewKey: &'static ns::UiItemId;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let table_view = ns::TableView::with_frame(ns::Rect::new(0.0, 0.0, 320.0, 200.0));
        assert_eq!(table_view.rows_n(), 0);
        assert_eq!(table_view.columns_n(), 0);
        assert!(table_view.table_columns().is_empty());
        assert_eq!(
            table_view.selection_highlight_style(),
            ns::TableViewSelectionHighlightStyle::Regular
        );
    }
}
