use crate::{arc, cf, define_cf_type, define_opts};

define_cf_type!(Attr(cf::String));

impl Attr {
    #[inline]
    pub fn with_string(str: &cf::String) -> &Self {
        unsafe { std::mem::transmute(str) }
    }

    #[inline]
    pub fn with_raw<S: AsRef<cf::String>>(raw: &S) -> arc::R<Self> {
        Self::with_string(raw.as_ref()).retained()
    }
}

impl AsRef<Attr> for cf::String {
    #[inline]
    fn as_ref(&self) -> &Attr {
        Attr::with_string(self)
    }
}

pub mod attr {
    use crate::{ax::Attr, cf};

    pub fn frame() -> &'static Attr {
        cf::str!(c"AXFrame").as_ref()
    }

    /// Identifies the basic type of an element.
    #[doc(alias = "kAXRoleAttribute")]
    pub fn role() -> &'static Attr {
        cf::str!(c"AXRole").as_ref()
    }

    /// More specifically identifies the type of an element beyond the basic type provided
    /// by kAXRoleAttribute.
    #[doc(alias = "kAXSubroleAttribute")]
    pub fn subrole() -> &'static Attr {
        cf::str!(c"AXSubrole").as_ref()
    }

    #[doc(alias = "kAXRoleDescriptionAttribute")]
    pub fn role_desc() -> &'static Attr {
        cf::str!(c"AXRoleDescription").as_ref()
    }

    ///  A localized, human-readable cf::String ref that offers help content for an element.
    #[doc(alias = "kAXHelpAttribute")]
    pub fn help() -> &'static Attr {
        cf::str!(c"AXHelp").as_ref()
    }

    #[doc(alias = "kAXTitleAttribute")]
    pub fn title() -> &'static Attr {
        cf::str!(c"AXTitle").as_ref()
    }

    #[doc(alias = "kAXValueAttribute")]
    pub fn value() -> &'static Attr {
        cf::str!(c"AXValue").as_ref()
    }

    #[doc(alias = "kAXValueDescriptionAttribute")]
    pub fn value_desc() -> &'static Attr {
        cf::str!(c"AXValueDescription").as_ref()
    }

    #[doc(alias = "kAXMinValueAttribute")]
    pub fn min_value() -> &'static Attr {
        cf::str!(c"AXMinValue").as_ref()
    }

    #[doc(alias = "kAXMaxValueAttribute")]
    pub fn max_value() -> &'static Attr {
        cf::str!(c"AXMaxValue").as_ref()
    }

    #[doc(alias = "kAXValueIncrementAttribute")]
    pub fn value_inc() -> &'static Attr {
        cf::str!(c"AXValueIncrement").as_ref()
    }

    /// A cf::Array ref of whatever type the element uses for its kAXValueAttribute.
    #[doc(alias = "kAXAllowedValuesAttribute")]
    pub fn allowed_values() -> &'static Attr {
        cf::str!(c"AXAllowedValues").as_ref()
    }

    #[doc(alias = "kAXPlaceholderValueAttribute")]
    pub fn placeholder_value() -> &'static Attr {
        cf::str!(c"AXPlaceholderValue").as_ref()
    }

    #[doc(alias = "kAXEnabledAttribute")]
    pub fn enabled() -> &'static Attr {
        cf::str!(c"AXEnabled").as_ref()
    }

    #[doc(alias = "kAXElementBusyAttribute")]
    pub fn element_busy() -> &'static Attr {
        cf::str!(c"AXElementBusy").as_ref()
    }

    #[doc(alias = "kAXFocusedAttribute")]
    pub fn focused() -> &'static Attr {
        cf::str!(c"AXFocused").as_ref()
    }

    #[doc(alias = "kAXParentAttribute")]
    pub fn parent() -> &'static Attr {
        cf::str!(c"AXParent").as_ref()
    }

    #[doc(alias = "kAXChildrenAttribute")]
    pub fn children() -> &'static Attr {
        cf::str!(c"AXChildren").as_ref()
    }

    #[doc(alias = "kAXSelectedChildrenAttribute")]
    pub fn selected_children() -> &'static Attr {
        cf::str!(c"AXSelectedChildren").as_ref()
    }

    #[doc(alias = "kAXVisibleChildrenAttribute")]
    pub fn visible_children() -> &'static Attr {
        cf::str!(c"AXVisibleChildren").as_ref()
    }

    #[doc(alias = "kAXWindowAttribute")]
    pub fn window() -> &'static Attr {
        cf::str!(c"AXWindow").as_ref()
    }

    #[doc(alias = "kAXTopLevelUIElementAttribute")]
    pub fn top_level_ui_element() -> &'static Attr {
        cf::str!(c"AXTopLevelUIElement").as_ref()
    }

    #[doc(alias = "kAXPositionAttribute")]
    pub fn position() -> &'static Attr {
        cf::str!(c"AXPosition").as_ref()
    }

    #[doc(alias = "kAXSizeAttribute")]
    pub fn size() -> &'static Attr {
        cf::str!(c"AXSize").as_ref()
    }

    #[doc(alias = "kAXOrientationAttribute")]
    pub fn orientation() -> &'static Attr {
        cf::str!(c"AXOrientation").as_ref()
    }

    #[doc(alias = "kAXDescriptionAttribute")]
    pub fn desc() -> &'static Attr {
        cf::str!(c"AXDescription").as_ref()
    }

    #[doc(alias = "kAXSelectedTextAttribute")]
    pub fn selected_text() -> &'static Attr {
        cf::str!(c"AXSelectedText").as_ref()
    }

    #[doc(alias = "kAXSelectedTextRangeAttribute")]
    pub fn selected_text_range() -> &'static Attr {
        cf::str!(c"AXSelectedTextRange").as_ref()
    }

    #[doc(alias = "kAXSelectedTextRangesAttribute")]
    pub fn selected_text_ranges() -> &'static Attr {
        cf::str!(c"AXSelectedTextRanges").as_ref()
    }

    #[doc(alias = "kAXVisibleCharacterRangeAttribute")]
    pub fn visible_character_range() -> &'static Attr {
        cf::str!(c"AXVisibleCharacterRange").as_ref()
    }

    #[doc(alias = "kAXNumberOfCharactersAttribute")]
    pub fn num_chars() -> &'static Attr {
        cf::str!(c"AXNumberOfCharacters").as_ref()
    }

    #[doc(alias = "kAXSharedTextUIElementsAttribute")]
    pub fn shared_text_ui_elements() -> &'static Attr {
        cf::str!(c"AXSharedTextUIElements").as_ref()
    }

    #[doc(alias = "kAXSharedCharacterRangeAttribute")]
    pub fn shared_character_range() -> &'static Attr {
        cf::str!(c"AXSharedCharacterRange").as_ref()
    }

    #[doc(alias = "kAXSharedFocusElementsAttribute")]
    pub fn shared_focus_elements() -> &'static Attr {
        cf::str!(c"AXSharedFocusElements").as_ref()
    }

    #[doc(alias = "kAXInsertionPointLineNumberAttribute")]
    pub fn insertion_point_line_num() -> &'static Attr {
        cf::str!(c"AXInsertionPointLineNumber").as_ref()
    }

    #[doc(alias = "kAXMainAttribute")]
    pub fn main() -> &'static Attr {
        cf::str!(c"AXMain").as_ref()
    }

    #[doc(alias = "kAXMinimizedAttribute")]
    pub fn minimized() -> &'static Attr {
        cf::str!(c"AXMinimized").as_ref()
    }

    #[doc(alias = "kAXCloseButtonAttribute")]
    pub fn close_button() -> &'static Attr {
        cf::str!(c"AXCloseButton").as_ref()
    }

    #[doc(alias = "kAXZoomButtonAttribute")]
    pub fn zoom_button() -> &'static Attr {
        cf::str!(c"AXZoomButton").as_ref()
    }

    #[doc(alias = "kAXMinimizeButtonAttribute")]
    pub fn minimize_button() -> &'static Attr {
        cf::str!(c"AXMinimizeButton").as_ref()
    }

    #[doc(alias = "kAXToolbarButtonAttribute")]
    pub fn toolbar_button() -> &'static Attr {
        cf::str!(c"AXToolbarButton").as_ref()
    }

    #[doc(alias = "kAXFullScreenButtonAttribute")]
    pub fn full_screen_button() -> &'static Attr {
        cf::str!(c"AXFullScreenButton").as_ref()
    }

    #[doc(alias = "kAXProxyAttribute")]
    pub fn proxy() -> &'static Attr {
        cf::str!(c"AXProxy").as_ref()
    }

    #[doc(alias = "kAXGrowAreaAttribute")]
    pub fn grow_area() -> &'static Attr {
        cf::str!(c"AXGrowArea").as_ref()
    }

    #[doc(alias = "kAXModalAttribute")]
    pub fn modal() -> &'static Attr {
        cf::str!(c"AXModal").as_ref()
    }

    #[doc(alias = "kAXDefaultButtonAttribute")]
    pub fn default_button() -> &'static Attr {
        cf::str!(c"AXDefaultButton").as_ref()
    }

    #[doc(alias = "kAXCancelButtonAttribute")]
    pub fn cancel_button() -> &'static Attr {
        cf::str!(c"AXCancelButton").as_ref()
    }

    #[doc(alias = "kAXMenuItemCmdCharAttribute")]
    pub fn menu_item_cmd_char() -> &'static Attr {
        cf::str!(c"AXMenuItemCmdChar").as_ref()
    }

    #[doc(alias = "kAXMenuItemCmdVirtualKeyAttribute")]
    pub fn menu_item_cmd_virtual_key() -> &'static Attr {
        cf::str!(c"AXMenuItemCmdVirtualKey").as_ref()
    }

    #[doc(alias = "kAXMenuItemCmdGlyphAttribute")]
    pub fn menu_item_cmd_glyph() -> &'static Attr {
        cf::str!(c"AXMenuItemCmdGlyph").as_ref()
    }

    #[doc(alias = "kAXMenuItemCmdModifiersAttribute")]
    pub fn menu_item_cmd_modifiers() -> &'static Attr {
        cf::str!(c"AXMenuItemCmdModifiers").as_ref()
    }

    #[doc(alias = "kAXMenuItemMarkCharAttribute")]
    pub fn menu_item_mark_char() -> &'static Attr {
        cf::str!(c"AXMenuItemMarkChar").as_ref()
    }

    #[doc(alias = "kAXMenuItemPrimaryUIElementAttribute")]
    pub fn menu_item_primary_ui_element() -> &'static Attr {
        cf::str!(c"AXMenuItemPrimaryUIElement").as_ref()
    }

    #[doc(alias = "kAXMenuBarAttribute")]
    pub fn menu_bar() -> &'static Attr {
        cf::str!(c"AXMenuBar").as_ref()
    }

    #[doc(alias = "kAXWindowsAttribute")]
    pub fn windows() -> &'static Attr {
        cf::str!(c"AXWindows").as_ref()
    }

    #[doc(alias = "kAXFrontmostAttribute")]
    pub fn frontmost() -> &'static Attr {
        cf::str!(c"AXFrontmost").as_ref()
    }

    #[doc(alias = "kAXHiddenAttribute")]
    pub fn hidden() -> &'static Attr {
        cf::str!(c"AXHidden").as_ref()
    }

    #[doc(alias = "kAXMainWindowAttribute")]
    pub fn main_window() -> &'static Attr {
        cf::str!(c"AXMainWindow").as_ref()
    }

    #[doc(alias = "kAXFocusedWindowAttribute")]
    pub fn focused_window() -> &'static Attr {
        cf::str!(c"AXFocusedWindow").as_ref()
    }

    #[doc(alias = "kAXFocusedUIElementAttribute")]
    pub fn focused_ui_element() -> &'static Attr {
        cf::str!(c"AXFocusedUIElement").as_ref()
    }

    #[doc(alias = "kAXExtrasMenuBarAttribute")]
    pub fn extras_menu_bar() -> &'static Attr {
        cf::str!(c"AXExtrasMenuBar").as_ref()
    }

    #[doc(alias = "kAXHeaderAttribute")]
    pub fn header() -> &'static Attr {
        cf::str!(c"AXHeader").as_ref()
    }

    #[doc(alias = "kAXEditedAttribute")]
    pub fn edited() -> &'static Attr {
        cf::str!(c"AXEdited").as_ref()
    }

    #[doc(alias = "kAXValueWrapsAttribute")]
    pub fn value_wraps() -> &'static Attr {
        cf::str!(c"AXValueWraps").as_ref()
    }

    #[doc(alias = "kAXTabsAttribute")]
    pub fn tabs() -> &'static Attr {
        cf::str!(c"AXTabs").as_ref()
    }

    #[doc(alias = "kAXTitleUIElementAttribute")]
    pub fn title_ui_element() -> &'static Attr {
        cf::str!(c"AXTitleUIElement").as_ref()
    }

    #[doc(alias = "kAXHorizontalScrollBarAttribute")]
    pub fn horizontal_scroll_bar() -> &'static Attr {
        cf::str!(c"AXHorizontalScrollBar").as_ref()
    }

    #[doc(alias = "kAXVerticalScrollBarAttribute")]
    pub fn vertical_scroll_bar() -> &'static Attr {
        cf::str!(c"AXVerticalScrollBar").as_ref()
    }

    #[doc(alias = "kAXOverflowButtonAttribute")]
    pub fn overflow_button() -> &'static Attr {
        cf::str!(c"AXOverflowButton").as_ref()
    }

    #[doc(alias = "kAXFilenameAttribute")]
    pub fn filename() -> &'static Attr {
        cf::str!(c"AXFilename").as_ref()
    }

    #[doc(alias = "kAXExpandedAttribute")]
    pub fn expanded() -> &'static Attr {
        cf::str!(c"AXExpanded").as_ref()
    }

    #[doc(alias = "kAXSelectedAttribute")]
    pub fn selected() -> &'static Attr {
        cf::str!(c"AXSelected").as_ref()
    }

    #[doc(alias = "kAXSplittersAttribute")]
    pub fn splitters() -> &'static Attr {
        cf::str!(c"AXSplitters").as_ref()
    }

    #[doc(alias = "kAXNextContentsAttribute")]
    pub fn next_contents() -> &'static Attr {
        cf::str!(c"AXNextContents").as_ref()
    }

    #[doc(alias = "kAXDocumentAttribute")]
    pub fn document() -> &'static Attr {
        cf::str!(c"AXDocument").as_ref()
    }

    #[doc(alias = "kAXDecrementButtonAttribute")]
    pub fn decrement_button() -> &'static Attr {
        cf::str!(c"AXDecrementButton").as_ref()
    }

    #[doc(alias = "kAXIncrementButtonAttribute")]
    pub fn increment_button() -> &'static Attr {
        cf::str!(c"AXIncrementButton").as_ref()
    }

    #[doc(alias = "kAXPreviousContentsAttribute")]
    pub fn previous_contents() -> &'static Attr {
        cf::str!(c"AXPreviousContents").as_ref()
    }

    #[doc(alias = "kAXContentsAttribute")]
    pub fn contents() -> &'static Attr {
        cf::str!(c"AXContents").as_ref()
    }

    #[doc(alias = "kAXIncrementorAttribute")]
    pub fn incrementor() -> &'static Attr {
        cf::str!(c"AXIncrementor").as_ref()
    }

    #[doc(alias = "kAXHourFieldAttribute")]
    pub fn hour_field() -> &'static Attr {
        cf::str!(c"AXHourField").as_ref()
    }

    #[doc(alias = "kAXMinuteFieldAttribute")]
    pub fn minute_field() -> &'static Attr {
        cf::str!(c"AXMinuteField").as_ref()
    }

    #[doc(alias = "kAXSecondFieldAttribute")]
    pub fn second_field() -> &'static Attr {
        cf::str!(c"AXSecondField").as_ref()
    }

    #[doc(alias = "kAXAMPMFieldAttribute")]
    pub fn ampm_field() -> &'static Attr {
        cf::str!(c"AXAMPMField").as_ref()
    }

    #[doc(alias = "kAXDayFieldAttribute")]
    pub fn day_field() -> &'static Attr {
        cf::str!(c"AXDayField").as_ref()
    }

    #[doc(alias = "kAXMonthFieldAttribute")]
    pub fn month_field() -> &'static Attr {
        cf::str!(c"AXMonthField").as_ref()
    }

    #[doc(alias = "kAXYearFieldAttribute")]
    pub fn year_field() -> &'static Attr {
        cf::str!(c"AXYearField").as_ref()
    }

    #[doc(alias = "kAXColumnTitleAttribute")]
    pub fn column_title() -> &'static Attr {
        cf::str!(c"AXColumnTitle").as_ref()
    }

    #[doc(alias = "kAXURLAttribute")]
    pub fn url() -> &'static Attr {
        cf::str!(c"AXURL").as_ref()
    }

    #[doc(alias = "kAXLabelUIElementsAttribute")]
    pub fn label_ui_elements() -> &'static Attr {
        cf::str!(c"AXLabelUIElements").as_ref()
    }

    #[doc(alias = "kAXLabelValueAttribute")]
    pub fn label_value() -> &'static Attr {
        cf::str!(c"AXLabelValue").as_ref()
    }

    #[doc(alias = "kAXShownMenuUIElementAttribute")]
    pub fn shown_menu_ui_element() -> &'static Attr {
        cf::str!(c"AXShownMenuUIElement").as_ref()
    }

    #[doc(alias = "kAXServesAsTitleForUIElementsAttribute")]
    pub fn serves_as_title_for_ui_elements() -> &'static Attr {
        cf::str!(c"AXServesAsTitleForUIElements").as_ref()
    }

    #[doc(alias = "kAXLinkedUIElementsAttribute")]
    pub fn linked_ui_elements() -> &'static Attr {
        cf::str!(c"AXLinkedUIElements").as_ref()
    }

    #[doc(alias = "kAXRowsAttribute")]
    pub fn rows() -> &'static Attr {
        cf::str!(c"AXRows").as_ref()
    }

    #[doc(alias = "kAXVisibleRowsAttribute")]
    pub fn visible_rows() -> &'static Attr {
        cf::str!(c"AXVisibleRows").as_ref()
    }

    #[doc(alias = "kAXSelectedRowsAttribute")]
    pub fn selected_rows() -> &'static Attr {
        cf::str!(c"AXSelectedRows").as_ref()
    }

    #[doc(alias = "kAXColumnsAttribute")]
    pub fn columns() -> &'static Attr {
        cf::str!(c"AXColumns").as_ref()
    }

    #[doc(alias = "kAXVisibleColumnsAttribute")]
    pub fn visible_columns() -> &'static Attr {
        cf::str!(c"AXVisibleColumns").as_ref()
    }

    #[doc(alias = "kAXSelectedColumnsAttribute")]
    pub fn selected_columns() -> &'static Attr {
        cf::str!(c"AXSelectedColumns").as_ref()
    }

    #[doc(alias = "kAXRowAttribute")]
    pub fn row() -> &'static Attr {
        cf::str!(c"AXRow").as_ref()
    }

    #[doc(alias = "kAXIndexAttribute")]
    pub fn index() -> &'static Attr {
        cf::str!(c"AXIndex").as_ref()
    }

    #[doc(alias = "kAXDisclosingAttribute")]
    pub fn disclosing() -> &'static Attr {
        cf::str!(c"AXDisclosing").as_ref()
    }

    #[doc(alias = "kAXDisclosedRowsAttribute")]
    pub fn disclosed_rows() -> &'static Attr {
        cf::str!(c"AXDisclosedRows").as_ref()
    }

    #[doc(alias = "kAXDisclosedByRowAttribute")]
    pub fn disclosed_by_row() -> &'static Attr {
        cf::str!(c"AXDisclosedByRow").as_ref()
    }

    #[doc(alias = "kAXDisclosureLevelAttribute")]
    pub fn disclosure_level() -> &'static Attr {
        cf::str!(c"AXDisclosureLevel").as_ref()
    }

    #[doc(alias = "kAXMatteHoleAttribute")]
    pub fn matte_hole() -> &'static Attr {
        cf::str!(c"AXMatteHole").as_ref()
    }

    #[doc(alias = "kAXMatteContentUIElementAttribute")]
    pub fn matte_content_ui_element() -> &'static Attr {
        cf::str!(c"AXMatteContentUIElement").as_ref()
    }

    #[doc(alias = "kAXMarkerUIElementsAttribute")]
    pub fn marker_ui_elements() -> &'static Attr {
        cf::str!(c"AXMarkerUIElements").as_ref()
    }

    #[doc(alias = "kAXUnitsAttribute")]
    pub fn units() -> &'static Attr {
        cf::str!(c"AXUnits").as_ref()
    }

    #[doc(alias = "kAXUnitDescriptionAttribute")]
    pub fn unit_desc() -> &'static Attr {
        cf::str!(c"AXUnitDescription").as_ref()
    }

    #[doc(alias = "kAXMarkerTypeAttribute")]
    pub fn marker_type() -> &'static Attr {
        cf::str!(c"AXMarkerType").as_ref()
    }

    #[doc(alias = "kAXMarkerTypeDescriptionAttribute")]
    pub fn marker_type_desc() -> &'static Attr {
        cf::str!(c"AXMarkerTypeDescription").as_ref()
    }

    #[doc(alias = "kAXIsApplicationRunningAttribute")]
    pub fn is_app_running() -> &'static Attr {
        cf::str!(c"AXIsApplicationRunning").as_ref()
    }

    #[doc(alias = "kAXSearchButtonAttribute")]
    pub fn search_button() -> &'static Attr {
        cf::str!(c"AXSearchButton").as_ref()
    }

    #[doc(alias = "kAXClearButtonAttribute")]
    pub fn clear_button() -> &'static Attr {
        cf::str!(c"AXClearButton").as_ref()
    }

    #[doc(alias = "kAXFocusedApplicationAttribute")]
    pub fn focused_app() -> &'static Attr {
        cf::str!(c"AXFocusedApplication").as_ref()
    }

    #[doc(alias = "kAXRowCountAttribute")]
    pub fn row_count() -> &'static Attr {
        cf::str!(c"AXRowCount").as_ref()
    }

    #[doc(alias = "kAXColumnCountAttribute")]
    pub fn column_count() -> &'static Attr {
        cf::str!(c"AXColumnCount").as_ref()
    }

    #[doc(alias = "kAXOrderedByRowAttribute")]
    pub fn ordered_by_row() -> &'static Attr {
        cf::str!(c"AXOrderedByRow").as_ref()
    }

    #[doc(alias = "kAXWarningValueAttribute")]
    pub fn warning_value() -> &'static Attr {
        cf::str!(c"AXWarningValue").as_ref()
    }

    #[doc(alias = "kAXCriticalValueAttribute")]
    pub fn critical_value() -> &'static Attr {
        cf::str!(c"AXCriticalValue").as_ref()
    }

    #[doc(alias = "kAXSelectedCellsAttribute")]
    pub fn selected_cells() -> &'static Attr {
        cf::str!(c"AXSelectedCells").as_ref()
    }

    #[doc(alias = "kAXVisibleCellsAttribute")]
    pub fn visible_cells() -> &'static Attr {
        cf::str!(c"AXVisibleCells").as_ref()
    }

    #[doc(alias = "kAXRowHeaderUIElementsAttribute")]
    pub fn row_header_ui_elements() -> &'static Attr {
        cf::str!(c"AXRowHeaderUIElements").as_ref()
    }

    #[doc(alias = "kAXColumnHeaderUIElementsAttribute")]
    pub fn column_header_ui_elements() -> &'static Attr {
        cf::str!(c"AXColumnHeaderUIElements").as_ref()
    }

    #[doc(alias = "kAXRowIndexRangeAttribute")]
    pub fn row_index_range() -> &'static Attr {
        cf::str!(c"AXRowIndexRange").as_ref()
    }

    #[doc(alias = "kAXColumnIndexRangeAttribute")]
    pub fn column_index_range() -> &'static Attr {
        cf::str!(c"AXColumnIndexRange").as_ref()
    }

    #[doc(alias = "kAXHorizontalUnitsAttribute")]
    pub fn horizontal_units() -> &'static Attr {
        cf::str!(c"AXHorizontalUnits").as_ref()
    }

    #[doc(alias = "kAXVerticalUnitsAttribute")]
    pub fn vertical_units() -> &'static Attr {
        cf::str!(c"AXVerticalUnits").as_ref()
    }

    #[doc(alias = "kAXHorizontalUnitDescriptionAttribute")]
    pub fn horizontal_unit_desc() -> &'static Attr {
        cf::str!(c"AXHorizontalUnitDescription").as_ref()
    }

    #[doc(alias = "kAXVerticalUnitDescriptionAttribute")]
    pub fn vertical_unit_desc() -> &'static Attr {
        cf::str!(c"AXVerticalUnitDescription").as_ref()
    }

    #[doc(alias = "kAXHandlesAttribute")]
    pub fn handles() -> &'static Attr {
        cf::str!(c"AXHandles").as_ref()
    }

    #[doc(alias = "kAXTextAttribute")]
    pub fn text() -> &'static Attr {
        cf::str!(c"AXText").as_ref()
    }

    #[doc(alias = "kAXVisibleTextAttribute")]
    pub fn visible_text() -> &'static Attr {
        cf::str!(c"AXVisibleText").as_ref()
    }

    #[doc(alias = "kAXIsEditableAttribute")]
    pub fn is_editable() -> &'static Attr {
        cf::str!(c"AXIsEditable").as_ref()
    }

    #[doc(alias = "kAXColumnTitlesAttribute")]
    pub fn column_titles() -> &'static Attr {
        cf::str!(c"AXColumnTitles").as_ref()
    }

    #[doc(alias = "kAXIdentifierAttribute")]
    pub fn id() -> &'static Attr {
        cf::str!(c"AXIdentifier").as_ref()
    }

    #[doc(alias = "kAXAlternateUIVisibleAttribute")]
    pub fn alternate_ui_visible() -> &'static Attr {
        cf::str!(c"AXAlternateUIVisible").as_ref()
    }
}

define_cf_type!(ParamAttr(cf::String));

impl ParamAttr {
    #[inline]
    pub fn with_string(str: &cf::String) -> &Self {
        unsafe { std::mem::transmute(str) }
    }

    #[inline]
    pub fn with_raw<S: AsRef<cf::String>>(raw: &S) -> arc::R<Self> {
        Self::with_string(raw.as_ref()).retained()
    }
}

impl AsRef<ParamAttr> for cf::String {
    #[inline]
    fn as_ref(&self) -> &ParamAttr {
        ParamAttr::with_string(self)
    }
}

pub mod param_attr {
    use crate::{ax::ParamAttr, cf};

    #[doc(alias = "kAXLineForIndexParameterizedAttribute")]
    pub fn line_for_index() -> &'static ParamAttr {
        cf::str!(c"AXLineForIndex").as_ref()
    }

    #[doc(alias = "kAXRangeForLineParameterizedAttribute")]
    pub fn range_for_line() -> &'static ParamAttr {
        cf::str!(c"AXRangeForLine").as_ref()
    }

    #[doc(alias = "kAXStringForRangeParameterizedAttribute")]
    pub fn string_for_range() -> &'static ParamAttr {
        cf::str!(c"AXStringForRange").as_ref()
    }

    #[doc(alias = "kAXRangeForPositionParameterizedAttribute")]
    pub fn range_for_pos() -> &'static ParamAttr {
        cf::str!(c"AXRangeForPosition").as_ref()
    }

    #[doc(alias = "kAXRangeForIndexParameterizedAttribute")]
    pub fn range_for_index() -> &'static ParamAttr {
        cf::str!(c"AXRangeForIndex").as_ref()
    }

    #[doc(alias = "kAXBoundsForRangeParameterizedAttribute")]
    pub fn bounds_for_range() -> &'static ParamAttr {
        cf::str!(c"AXBoundsForRange").as_ref()
    }

    #[doc(alias = "kAXRTFForRangeParameterizedAttribute")]
    pub fn rtf_for_range() -> &'static ParamAttr {
        cf::str!(c"AXRTFForRange").as_ref()
    }

    #[doc(alias = "kAXAttributedStringForRangeParameterizedAttribute")]
    pub fn attr_string_for_range() -> &'static ParamAttr {
        cf::str!(c"AXAttributedStringForRange").as_ref()
    }

    #[doc(alias = "kAXStyleRangeForIndexParameterizedAttribute")]
    pub fn style_range_for_index() -> &'static ParamAttr {
        cf::str!(c"AXStyleRangeForIndex").as_ref()
    }

    #[doc(alias = "kAXCellForColumnAndRowParameterizedAttribute")]
    pub fn cell_for_col_and_row() -> &'static ParamAttr {
        cf::str!(c"AXCellForColumnAndRow").as_ref()
    }

    #[doc(alias = "kAXLayoutPointForScreenPointParameterizedAttribute")]
    pub fn layout_point_for_screen_point() -> &'static ParamAttr {
        cf::str!(c"AXLayoutPointForScreenPoint").as_ref()
    }

    #[doc(alias = "kAXLayoutSizeForScreenSizeParameterizedAttribute")]
    pub fn layout_size_for_screen_size() -> &'static ParamAttr {
        cf::str!(c"AXLayoutSizeForScreenSize").as_ref()
    }

    #[doc(alias = "kAXScreenPointForLayoutPointParameterizedAttribute")]
    pub fn screen_point_for_layout_point() -> &'static ParamAttr {
        cf::str!(c"AXScreenPointForLayoutPoint").as_ref()
    }

    #[doc(alias = "kAXScreenSizeForLayoutSizeParameterizedAttribute")]
    pub fn screen_size_for_layout_size() -> &'static ParamAttr {
        cf::str!(c"AXScreenSizeForLayoutSize").as_ref()
    }
}

define_opts!(
    #[doc(alias = "AXMenuItemModifiers")]
    pub MenuItemModifiers(i32)
);

impl MenuItemModifiers {
    /// Mask for no modifiers other than the command key (which is used by default)
    #[doc(alias = "kAXMenuItemModifierNone")]
    pub const NONE: Self = Self(0);

    #[doc(alias = "kAXMenuItemModifierShift")]
    pub const SHIFT: Self = Self(1 << 0);

    #[doc(alias = "kAXMenuItemModifierOption")]
    pub const OPTION: Self = Self(1 << 1);

    #[doc(alias = "kAXMenuItemModifierControl")]
    pub const CTRL: Self = Self(1 << 2);

    #[doc(alias = "kAXMenuItemModifierNoCommand")]
    pub const NO_CMD: Self = Self(1 << 3);
}
