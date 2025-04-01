use crate::{arc, cf, define_cf_type};

define_cf_type!(Role(cf::String));

impl Role {
    #[inline]
    pub fn with_string(str: &cf::String) -> &Self {
        unsafe { std::mem::transmute(str) }
    }

    #[inline]
    pub fn with_raw<S: AsRef<cf::String>>(raw: &S) -> arc::R<Self> {
        Self::with_string(raw.as_ref()).retained()
    }
}

impl AsRef<Role> for cf::String {
    #[inline]
    fn as_ref(&self) -> &Role {
        Role::with_string(self)
    }
}

pub mod role {
    use crate::{ax::Role, cf};

    #[doc(alias = "kAXApplicationRole")]
    pub fn app() -> &'static Role {
        cf::str!(c"AXApplication").as_ref()
    }

    #[doc(alias = "kAXSystemWideRole")]
    pub fn sys_wide() -> &'static Role {
        cf::str!(c"AXSystemWide").as_ref()
    }

    #[doc(alias = "kAXWindowRole")]
    pub fn window() -> &'static Role {
        cf::str!(c"AXWindow").as_ref()
    }

    #[doc(alias = "kAXSheetRole")]
    pub fn sheet() -> &'static Role {
        cf::str!(c"AXSheet").as_ref()
    }

    #[doc(alias = "kAXDrawerRole")]
    pub fn drawer() -> &'static Role {
        cf::str!(c"AXDrawer").as_ref()
    }

    #[doc(alias = "kAXGrowAreaRole")]
    pub fn grow_area() -> &'static Role {
        cf::str!(c"AXGrowArea").as_ref()
    }

    #[doc(alias = "kAXImageRole")]
    pub fn image() -> &'static Role {
        cf::str!(c"AXImage").as_ref()
    }

    #[doc(alias = "kAXUnknownRole")]
    pub fn unknown() -> &'static Role {
        cf::str!(c"AXUnknown").as_ref()
    }

    #[doc(alias = "kAXButtonRole")]
    pub fn button() -> &'static Role {
        cf::str!(c"AXButton").as_ref()
    }

    #[doc(alias = "kAXRadioButtonRole")]
    pub fn radio_button() -> &'static Role {
        cf::str!(c"AXRadioButton").as_ref()
    }

    #[doc(alias = "kAXCheckBoxRole")]
    pub fn check_box() -> &'static Role {
        cf::str!(c"AXCheckBox").as_ref()
    }

    #[doc(alias = "kAXPopUpButtonRole")]
    pub fn pop_up_button() -> &'static Role {
        cf::str!(c"AXPopUpButton").as_ref()
    }

    #[doc(alias = "kAXMenuButtonRole")]
    pub fn menu_button() -> &'static Role {
        cf::str!(c"AXMenuButton").as_ref()
    }

    #[doc(alias = "kAXTabGroupRole")]
    pub fn tab_group() -> &'static Role {
        cf::str!(c"AXTabGroup").as_ref()
    }

    #[doc(alias = "kAXTableRole")]
    pub fn table() -> &'static Role {
        cf::str!(c"AXTable").as_ref()
    }

    #[doc(alias = "kAXColumnRole")]
    pub fn column() -> &'static Role {
        cf::str!(c"AXColumn").as_ref()
    }

    #[doc(alias = "kAXRowRole")]
    pub fn row() -> &'static Role {
        cf::str!(c"AXRow").as_ref()
    }

    #[doc(alias = "kAXOutlineRole")]
    pub fn outline() -> &'static Role {
        cf::str!(c"AXOutline").as_ref()
    }

    #[doc(alias = "kAXBrowserRole")]
    pub fn browser() -> &'static Role {
        cf::str!(c"AXBrowser").as_ref()
    }

    #[doc(alias = "kAXScrollAreaRole")]
    pub fn scroll_area() -> &'static Role {
        cf::str!(c"AXScrollArea").as_ref()
    }

    #[doc(alias = "kAXScrollBarRole")]
    pub fn scroll_bar() -> &'static Role {
        cf::str!(c"AXScrollBar").as_ref()
    }

    #[doc(alias = "kAXRadioGroupRole")]
    pub fn radio_group() -> &'static Role {
        cf::str!(c"AXRadioGroup").as_ref()
    }

    #[doc(alias = "kAXListRole")]
    pub fn list() -> &'static Role {
        cf::str!(c"AXList").as_ref()
    }

    #[doc(alias = "kAXGroupRole")]
    pub fn group() -> &'static Role {
        cf::str!(c"AXGroup").as_ref()
    }

    #[doc(alias = "kAXValueIndicatorRole")]
    pub fn value_indicator() -> &'static Role {
        cf::str!(c"AXValueIndicator").as_ref()
    }

    #[doc(alias = "kAXComboBoxRole")]
    pub fn combo_box() -> &'static Role {
        cf::str!(c"AXComboBox").as_ref()
    }

    #[doc(alias = "kAXSliderRole")]
    pub fn slider() -> &'static Role {
        cf::str!(c"AXSlider").as_ref()
    }

    #[doc(alias = "kAXIncrementorRole")]
    pub fn incrementor() -> &'static Role {
        cf::str!(c"AXIncrementor").as_ref()
    }

    #[doc(alias = "kAXBusyIndicatorRole")]
    pub fn busy_indicator() -> &'static Role {
        cf::str!(c"AXBusyIndicator").as_ref()
    }

    #[doc(alias = "kAXProgressIndicatorRole")]
    pub fn progress_indicator() -> &'static Role {
        cf::str!(c"AXProgressIndicator").as_ref()
    }

    #[doc(alias = "kAXRelevanceIndicatorRole")]
    pub fn relevance_indicator() -> &'static Role {
        cf::str!(c"AXRelevanceIndicator").as_ref()
    }

    #[doc(alias = "kAXToolbarRole")]
    pub fn toolbar() -> &'static Role {
        cf::str!(c"AXToolbar").as_ref()
    }

    #[doc(alias = "kAXDisclosureTriangleRole")]
    pub fn disclosure_triangle() -> &'static Role {
        cf::str!(c"AXDisclosureTriangle").as_ref()
    }

    #[doc(alias = "kAXTextFieldRole")]
    pub fn text_field() -> &'static Role {
        cf::str!(c"AXTextField").as_ref()
    }

    #[doc(alias = "kAXTextAreaRole")]
    pub fn text_area() -> &'static Role {
        cf::str!(c"AXTextArea").as_ref()
    }

    #[doc(alias = "kAXStaticTextRole")]
    pub fn static_text() -> &'static Role {
        cf::str!(c"AXStaticText").as_ref()
    }

    #[doc(alias = "kAXHeadingRole")]
    pub fn heading() -> &'static Role {
        cf::str!(c"AXHeading").as_ref()
    }

    #[doc(alias = "kAXMenuBarRole")]
    pub fn menu_bar() -> &'static Role {
        cf::str!(c"AXMenuBar").as_ref()
    }

    #[doc(alias = "kAXMenuBarItemRole")]
    pub fn menu_bar_item() -> &'static Role {
        cf::str!(c"AXMenuBarItem").as_ref()
    }

    #[doc(alias = "kAXMenuRole")]
    pub fn menu() -> &'static Role {
        cf::str!(c"AXMenu").as_ref()
    }

    #[doc(alias = "kAXMenuItemRole")]
    pub fn menu_item() -> &'static Role {
        cf::str!(c"AXMenuItem").as_ref()
    }

    #[doc(alias = "kAXSplitGroupRole")]
    pub fn split_group() -> &'static Role {
        cf::str!(c"AXSplitGroup").as_ref()
    }

    #[doc(alias = "kAXSplitterRole")]
    pub fn splitter() -> &'static Role {
        cf::str!(c"AXSplitter").as_ref()
    }

    #[doc(alias = "kAXColorWellRole")]
    pub fn color_well() -> &'static Role {
        cf::str!(c"AXColorWell").as_ref()
    }

    #[doc(alias = "kAXTimeFieldRole")]
    pub fn time_field() -> &'static Role {
        cf::str!(c"AXTimeField").as_ref()
    }

    #[doc(alias = "kAXDateFieldRole")]
    pub fn date_field() -> &'static Role {
        cf::str!(c"AXDateField").as_ref()
    }

    #[doc(alias = "kAXHelpTagRole")]
    pub fn help_tag() -> &'static Role {
        cf::str!(c"AXHelpTag").as_ref()
    }

    #[doc(alias = "kAXMatteRole")]
    pub fn matte() -> &'static Role {
        cf::str!(c"AXMatte").as_ref()
    }

    #[doc(alias = "kAXDockItemRole")]
    pub fn dock_item() -> &'static Role {
        cf::str!(c"AXDockItem").as_ref()
    }

    #[doc(alias = "kAXRulerRole")]
    pub fn ruler() -> &'static Role {
        cf::str!(c"AXRuler").as_ref()
    }

    #[doc(alias = "kAXRulerMarkerRole")]
    pub fn ruler_marker() -> &'static Role {
        cf::str!(c"AXRulerMarker").as_ref()
    }

    #[doc(alias = "kAXGridRole")]
    pub fn grid() -> &'static Role {
        cf::str!(c"AXGrid").as_ref()
    }

    #[doc(alias = "kAXLevelIndicatorRole")]
    pub fn level_indicator() -> &'static Role {
        cf::str!(c"AXLevelIndicator").as_ref()
    }

    #[doc(alias = "kAXCellRole")]
    pub fn cell() -> &'static Role {
        cf::str!(c"AXCell").as_ref()
    }

    #[doc(alias = "kAXLayoutAreaRole")]
    pub fn layout_area() -> &'static Role {
        cf::str!(c"AXLayoutArea").as_ref()
    }

    #[doc(alias = "kAXLayoutItemRole")]
    pub fn layout_item() -> &'static Role {
        cf::str!(c"AXLayoutItem").as_ref()
    }

    #[doc(alias = "kAXHandleRole")]
    pub fn handle() -> &'static Role {
        cf::str!(c"AXHandle").as_ref()
    }

    #[doc(alias = "kAXPopoverRole")]
    pub fn popover() -> &'static Role {
        cf::str!(c"AXPopover").as_ref()
    }
}

define_cf_type!(SubRole(cf::String));

impl SubRole {
    #[inline]
    pub fn with_string(str: &cf::String) -> &Self {
        unsafe { std::mem::transmute(str) }
    }

    #[inline]
    pub fn with_raw<S: AsRef<cf::String>>(raw: &S) -> arc::R<Self> {
        Self::with_string(raw.as_ref()).retained()
    }
}

impl AsRef<SubRole> for cf::String {
    #[inline]
    fn as_ref(&self) -> &SubRole {
        SubRole::with_string(self)
    }
}

pub mod sub_role {
    use crate::{ax::SubRole, cf};

    #[doc(alias = "kAXCloseButtonSubrole")]
    pub fn close_button() -> &'static SubRole {
        cf::str!(c"AXCloseButton").as_ref()
    }

    #[doc(alias = "kAXMinimizeButtonSubrole")]
    pub fn minimize_button() -> &'static SubRole {
        cf::str!(c"AXMinimizeButton").as_ref()
    }

    #[doc(alias = "kAXZoomButtonSubrole")]
    pub fn zoom_button() -> &'static SubRole {
        cf::str!(c"AXZoomButton").as_ref()
    }

    #[doc(alias = "kAXToolbarButtonSubrole")]
    pub fn toolbar_button() -> &'static SubRole {
        cf::str!(c"AXToolbarButton").as_ref()
    }

    #[doc(alias = "kAXFullScreenButtonSubrole")]
    pub fn full_screen_button() -> &'static SubRole {
        cf::str!(c"AXFullScreenButton").as_ref()
    }

    #[doc(alias = "kAXSecureTextFieldSubrole")]
    pub fn secure_text_field() -> &'static SubRole {
        cf::str!(c"AXSecureTextField").as_ref()
    }

    #[doc(alias = "kAXTableRowSubrole")]
    pub fn table_row() -> &'static SubRole {
        cf::str!(c"AXTableRow").as_ref()
    }

    #[doc(alias = "kAXOutlineRowSubrole")]
    pub fn outline_row() -> &'static SubRole {
        cf::str!(c"AXOutlineRow").as_ref()
    }

    #[doc(alias = "kAXUnknownSubrole")]
    pub fn unknown() -> &'static SubRole {
        cf::str!(c"AXUnknown").as_ref()
    }

    #[doc(alias = "kAXStandardWindowSubrole")]
    pub fn standard_window() -> &'static SubRole {
        cf::str!(c"AXStandardWindow").as_ref()
    }

    #[doc(alias = "kAXDialogSubrole")]
    pub fn dialog() -> &'static SubRole {
        cf::str!(c"AXDialog").as_ref()
    }

    #[doc(alias = "kAXSystemDialogSubrole")]
    pub fn system_dialog() -> &'static SubRole {
        cf::str!(c"AXSystemDialog").as_ref()
    }

    #[doc(alias = "kAXFloatingWindowSubrole")]
    pub fn floating_window() -> &'static SubRole {
        cf::str!(c"AXFloatingWindow").as_ref()
    }

    #[doc(alias = "kAXSystemFloatingWindowSubrole")]
    pub fn system_floating_window() -> &'static SubRole {
        cf::str!(c"AXSystemFloatingWindow").as_ref()
    }

    #[doc(alias = "kAXDecorativeSubrole")]
    pub fn decorative() -> &'static SubRole {
        cf::str!(c"AXDecorative").as_ref()
    }

    #[doc(alias = "kAXIncrementArrowSubrole")]
    pub fn increment_arrow() -> &'static SubRole {
        cf::str!(c"AXIncrementArrow").as_ref()
    }

    #[doc(alias = "kAXDecrementArrowSubrole")]
    pub fn decrement_arrow() -> &'static SubRole {
        cf::str!(c"AXDecrementArrow").as_ref()
    }

    #[doc(alias = "kAXIncrementPageSubrole")]
    pub fn increment_page() -> &'static SubRole {
        cf::str!(c"AXIncrementPage").as_ref()
    }

    #[doc(alias = "kAXDecrementPageSubrole")]
    pub fn decrement_page() -> &'static SubRole {
        cf::str!(c"AXDecrementPage").as_ref()
    }

    #[doc(alias = "kAXSortButtonSubrole")]
    pub fn sort_button() -> &'static SubRole {
        cf::str!(c"AXSortButton").as_ref()
    }

    #[doc(alias = "kAXSearchFieldSubrole")]
    pub fn search_field() -> &'static SubRole {
        cf::str!(c"AXSearchField").as_ref()
    }

    #[doc(alias = "kAXTimelineSubrole")]
    pub fn timeline() -> &'static SubRole {
        cf::str!(c"AXTimeline").as_ref()
    }

    #[doc(alias = "kAXRatingIndicatorSubrole")]
    pub fn rating_indicator() -> &'static SubRole {
        cf::str!(c"AXRatingIndicator").as_ref()
    }

    #[doc(alias = "kAXContentListSubrole")]
    pub fn content_list() -> &'static SubRole {
        cf::str!(c"AXContentList").as_ref()
    }

    #[doc(alias = "kAXDescriptionListSubrole")]
    pub fn desc_list() -> &'static SubRole {
        cf::str!(c"AXDescriptionList").as_ref()
    }

    #[doc(alias = "kAXToggleSubrole")]
    pub fn toggle() -> &'static SubRole {
        cf::str!(c"AXToggle").as_ref()
    }

    #[doc(alias = "kAXSwitchSubrole")]
    pub fn switch() -> &'static SubRole {
        cf::str!(c"AXSwitch").as_ref()
    }

    #[doc(alias = "kAXApplicationDockItemSubrole")]
    pub fn app_dock_item() -> &'static SubRole {
        cf::str!(c"AXApplicationDockItem").as_ref()
    }

    #[doc(alias = "kAXDocumentDockItemSubrole")]
    pub fn doc_dock_item() -> &'static SubRole {
        cf::str!(c"AXDocumentDockItem").as_ref()
    }

    #[doc(alias = "kAXFolderDockItemSubrole")]
    pub fn folder_dock_item() -> &'static SubRole {
        cf::str!(c"AXFolderDockItem").as_ref()
    }

    #[doc(alias = "kAXMinimizedWindowDockItemSubrole")]
    pub fn minimized_window_dock_item() -> &'static SubRole {
        cf::str!(c"AXMinimizedWindowDockItem").as_ref()
    }

    #[doc(alias = "kAXURLDockItemSubrole")]
    pub fn url_dock_item() -> &'static SubRole {
        cf::str!(c"AXURLDockItem").as_ref()
    }

    #[doc(alias = "kAXDockExtraDockItemSubrole")]
    pub fn dock_extra_dock_item() -> &'static SubRole {
        cf::str!(c"AXDockExtraDockItem").as_ref()
    }

    #[doc(alias = "kAXTrashDockItemSubrole")]
    pub fn trash_dock_item() -> &'static SubRole {
        cf::str!(c"AXTrashDockItem").as_ref()
    }

    #[doc(alias = "kAXSeparatorDockItemSubrole")]
    pub fn separator_dock_item() -> &'static SubRole {
        cf::str!(c"AXSeparatorDockItem").as_ref()
    }

    #[doc(alias = "kAXProcessSwitcherListSubrole")]
    pub fn process_switcher_list() -> &'static SubRole {
        cf::str!(c"AXProcessSwitcherList").as_ref()
    }
}
