use crate::{arc, cf, define_cf_type, define_opts};

define_cf_type!(Notification(cf::String));

impl Notification {
    #[inline]
    pub fn with_string(str: &cf::String) -> &Self {
        unsafe { std::mem::transmute(str) }
    }

    #[inline]
    pub fn with_raw<S: AsRef<cf::String>>(raw: &S) -> arc::R<Self> {
        Self::with_string(raw.as_ref()).retained()
    }
}

impl AsRef<Notification> for cf::String {
    #[inline]
    fn as_ref(&self) -> &Notification {
        Notification::with_string(self)
    }
}

pub mod notification {
    use crate::{ax::Notification, cf};

    #[doc(alias = "kAXMainWindowChangedNotification")]
    pub fn main_window_changed() -> &'static Notification {
        cf::str!(c"AXMainWindowChanged").as_ref()
    }

    #[doc(alias = "kAXFocusedWindowChangedNotification")]
    pub fn focused_window_changed() -> &'static Notification {
        cf::str!(c"AXFocusedWindowChanged").as_ref()
    }

    #[doc(alias = "kAXFocusedUIElementChangedNotification")]
    pub fn focused_ui_element_changed() -> &'static Notification {
        cf::str!(c"AXFocusedUIElementChanged").as_ref()
    }

    #[doc(alias = "kAXApplicationActivatedNotification")]
    pub fn app_activated() -> &'static Notification {
        cf::str!(c"AXApplicationActivated").as_ref()
    }

    #[doc(alias = "kAXApplicationDeactivatedNotification")]
    pub fn app_deactivated() -> &'static Notification {
        cf::str!(c"AXApplicationDeactivated").as_ref()
    }

    #[doc(alias = "kAXApplicationHiddenNotification")]
    pub fn app_hidden() -> &'static Notification {
        cf::str!(c"AXApplicationHidden").as_ref()
    }

    #[doc(alias = "kAXApplicationShownNotification")]
    pub fn app_shown() -> &'static Notification {
        cf::str!(c"AXApplicationShown").as_ref()
    }

    #[doc(alias = "kAXWindowCreatedNotification")]
    pub fn window_created() -> &'static Notification {
        cf::str!(c"AXWindowCreated").as_ref()
    }

    #[doc(alias = "kAXWindowMovedNotification")]
    pub fn window_moved() -> &'static Notification {
        cf::str!(c"AXWindowMoved").as_ref()
    }

    #[doc(alias = "kAXWindowResizedNotification")]
    pub fn window_resized() -> &'static Notification {
        cf::str!(c"AXWindowResized").as_ref()
    }

    #[doc(alias = "kAXWindowMiniaturizedNotification")]
    pub fn window_miniaturized() -> &'static Notification {
        cf::str!(c"AXWindowMiniaturized").as_ref()
    }

    #[doc(alias = "kAXWindowDeminiaturizedNotification")]
    pub fn window_deminiaturized() -> &'static Notification {
        cf::str!(c"AXWindowDeminiaturized").as_ref()
    }

    #[doc(alias = "kAXDrawerCreatedNotification")]
    pub fn drawer_created() -> &'static Notification {
        cf::str!(c"AXDrawerCreated").as_ref()
    }

    #[doc(alias = "kAXSheetCreatedNotification")]
    pub fn sheet_created() -> &'static Notification {
        cf::str!(c"AXSheetCreated").as_ref()
    }

    #[doc(alias = "kAXHelpTagCreatedNotification")]
    pub fn help_tag_created() -> &'static Notification {
        cf::str!(c"AXHelpTagCreated").as_ref()
    }

    #[doc(alias = "kAXValueChangedNotification")]
    pub fn value_changed() -> &'static Notification {
        cf::str!(c"AXValueChanged").as_ref()
    }

    #[doc(alias = "kAXUIElementDestroyedNotification")]
    pub fn ui_element_destroyed() -> &'static Notification {
        cf::str!(c"AXUIElementDestroyed").as_ref()
    }

    #[doc(alias = "kAXElementBusyChangedNotification")]
    pub fn element_busy_changed() -> &'static Notification {
        cf::str!(c"AXElementBusyChanged").as_ref()
    }

    #[doc(alias = "kAXMenuOpenedNotification")]
    pub fn menu_opened() -> &'static Notification {
        cf::str!(c"AXMenuOpened").as_ref()
    }

    #[doc(alias = "kAXMenuClosedNotification")]
    pub fn menu_closed() -> &'static Notification {
        cf::str!(c"AXMenuClosed").as_ref()
    }

    #[doc(alias = "kAXMenuItemSelectedNotification")]
    pub fn menu_item_selected() -> &'static Notification {
        cf::str!(c"AXMenuItemSelected").as_ref()
    }

    #[doc(alias = "kAXRowCountChangedNotification")]
    pub fn row_count_changed() -> &'static Notification {
        cf::str!(c"AXRowCountChanged").as_ref()
    }

    #[doc(alias = "kAXRowExpandedNotification")]
    pub fn row_expanded() -> &'static Notification {
        cf::str!(c"AXRowExpanded").as_ref()
    }

    #[doc(alias = "kAXRowCollapsedNotification")]
    pub fn row_collapsed() -> &'static Notification {
        cf::str!(c"AXRowCollapsed").as_ref()
    }

    #[doc(alias = "kAXSelectedCellsChangedNotification")]
    pub fn selected_cells_changed() -> &'static Notification {
        cf::str!(c"AXSelectedCellsChanged").as_ref()
    }

    #[doc(alias = "kAXUnitsChangedNotification")]
    pub fn units_changed() -> &'static Notification {
        cf::str!(c"AXUnitsChanged").as_ref()
    }

    #[doc(alias = "kAXSelectedChildrenMovedNotification")]
    pub fn selected_children_moved() -> &'static Notification {
        cf::str!(c"AXSelectedChildrenMoved").as_ref()
    }

    #[doc(alias = "kAXSelectedChildrenChangedNotification")]
    pub fn selected_children_changed() -> &'static Notification {
        cf::str!(c"AXSelectedChildrenChanged").as_ref()
    }

    #[doc(alias = "kAXResizedNotification")]
    pub fn resized() -> &'static Notification {
        cf::str!(c"AXResized").as_ref()
    }

    #[doc(alias = "kAXMovedNotification")]
    pub fn moved() -> &'static Notification {
        cf::str!(c"AXMoved").as_ref()
    }

    #[doc(alias = "kAXCreatedNotification")]
    pub fn created() -> &'static Notification {
        cf::str!(c"AXCreated").as_ref()
    }

    #[doc(alias = "kAXSelectedRowsChangedNotification")]
    pub fn selected_rows_changed() -> &'static Notification {
        cf::str!(c"AXSelectedRowsChanged").as_ref()
    }

    #[doc(alias = "kAXSelectedColumnsChangedNotification")]
    pub fn selected_columns_changed() -> &'static Notification {
        cf::str!(c"AXSelectedColumnsChanged").as_ref()
    }

    #[doc(alias = "kAXSelectedTextChangedNotification")]
    pub fn selected_text_changed() -> &'static Notification {
        cf::str!(c"AXSelectedTextChanged").as_ref()
    }

    #[doc(alias = "kAXTitleChangedNotification")]
    pub fn title_changed() -> &'static Notification {
        cf::str!(c"AXTitleChanged").as_ref()
    }

    #[doc(alias = "kAXLayoutChangedNotification")]
    pub fn layout_changed() -> &'static Notification {
        cf::str!(c"AXLayoutChanged").as_ref()
    }

    #[doc(alias = "kAXAnnouncementRequestedNotification")]
    pub fn announcement_requested() -> &'static Notification {
        cf::str!(c"AXAnnouncementRequested").as_ref()
    }

    #[doc(alias = "kAXUIElementsKey")]
    pub fn ui_elements_key() -> &'static Notification {
        cf::str!(c"AXUIElements").as_ref()
    }

    #[doc(alias = "kAXPriorityKey")]
    pub fn priority_key() -> &'static Notification {
        cf::str!(c"AXPriority").as_ref()
    }

    #[doc(alias = "kAXAnnouncementKey")]
    pub fn announcement_key() -> &'static Notification {
        cf::str!(c"AXAnnouncement").as_ref()
    }

    #[doc(alias = "kAXUIElementTitleKey")]
    pub fn ui_element_title_key() -> &'static Notification {
        cf::str!(c"AXUIElementTitle").as_ref()
    }
}

define_opts!(
    pub Priority(isize)
);

impl Priority {
    pub const LOW: Self = Self(10);
    pub const MEDIUM: Self = Self(50);
    pub const HIGH: Self = Self(90);
}
