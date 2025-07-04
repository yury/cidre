use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSToolbarIdentifier")]
    pub ToolbarId(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSToolbarItemIdentifier")]
    pub ToolbarItemId(ns::String)
);

#[doc(alias = "NSToolbarDisplayMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(usize)]
pub enum ToolbarDisplayMode {
    Default,
    IconAndLabel,
    IconOnly,
    LabelOnly,
}

define_obj_type!(
    #[doc(alias = "NSToolbar")]
    pub Toolbar(ns::Id),
    NS_TOOLBAR
);

impl arc::A<Toolbar> {
    #[objc::msg_send(initWithIdentifier:)]
    pub fn init_with_id(self, id: &ns::ToolbarId) -> arc::R<Toolbar>;
}

impl Toolbar {
    pub fn with_id(id: &ns::ToolbarId) -> arc::R<Self> {
        Self::alloc().init_with_id(id)
    }

    #[objc::msg_send(insertItemWithItemIdentifier:atIndex:)]
    pub fn insert_item_with_id(&mut self, item_id: &ns::ToolbarItemId, index: isize);

    #[objc::msg_send(removeItemAtIndex:)]
    pub fn remove_item(&mut self, index: isize);

    #[objc::msg_send(removeItemWithItemIdentifier:)]
    pub fn remove_item_with_id(&mut self, item_id: &ns::ToolbarItemId);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyToolbarDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: ToolbarDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(isVisible)]
    pub fn is_visible(&self) -> bool;

    #[objc::msg_send(setVisible:)]
    pub fn set_visible(&mut self, val: bool);

    #[objc::msg_send(runCustomizationPalette:)]
    pub fn run_customization_palette(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(customizationPaletteIsRunning)]
    pub fn customization_palette_is_running(&self) -> bool;

    #[objc::msg_send(displayMode)]
    pub fn display_mode(&self) -> ns::ToolbarDisplayMode;

    #[objc::msg_send(setDisplayMode:)]
    pub fn set_display_mode(&mut self, val: ns::ToolbarDisplayMode);

    #[objc::msg_send(selectedItemIdentifier)]
    pub fn selected_item_id(&self) -> Option<arc::R<ns::ToolbarItemId>>;

    #[objc::msg_send(setSelectedItemIdentifier:)]
    pub fn set_selected_item_id(&self, val: Option<&ns::ToolbarItemId>);

    #[objc::msg_send(allowsUserCustomization)]
    pub fn allows_user_customization(&self) -> bool;

    #[objc::msg_send(setAllowsUserCustomization:)]
    pub fn set_allows_user_customization(&mut self, val: bool);

    #[objc::msg_send(allowsDisplayModeCustomization)]
    #[objc::available(macos = 15.0)]
    pub fn allows_display_mode_customization(&self) -> bool;

    #[objc::msg_send(setAllowsDisplayModeCustomization:)]
    #[objc::available(macos = 15.0)]
    pub fn set_allows_display_mode_customization(&mut self, val: bool);
}

/// Accessing toolbar info
impl Toolbar {
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::ToolbarId>;

    #[objc::msg_send(items)]
    pub fn items(&self) -> arc::R<ns::Array<ns::ToolbarItem>>;

    #[objc::msg_send(visibleItems)]
    pub fn visible_items(&self) -> Option<arc::R<ns::Array<ns::ToolbarItem>>>;

    #[objc::msg_send(itemIdentifiers)]
    #[objc::available(macos = 15.0)]
    pub fn item_ids(&self) -> arc::R<ns::Array<ns::ToolbarItemId>>;

    #[objc::msg_send(setItemIdentifiers:)]
    #[objc::available(macos = 15.0)]
    pub fn set_item_ids(&mut self, val: &ns::Array<ns::ToolbarItemId>);

    #[objc::msg_send(centeredItemIdentifiers)]
    #[objc::available(macos = 13.0)]
    pub fn centered_item_ids(&self) -> arc::R<ns::Array<ns::ToolbarItemId>>;

    #[objc::msg_send(setCenteredItemIdentifiers:)]
    #[objc::available(macos = 13.0)]
    pub fn set_centered_item_ids(&mut self, val: &ns::Array<ns::ToolbarItemId>);

    #[objc::msg_send(autosavesConfiguration)]
    pub fn autosaves_cfg(&self) -> bool;

    #[objc::msg_send(setAutosavesConfiguration:)]
    pub fn set_autosaves_cfg(&mut self, val: bool);

    #[objc::msg_send(validateVisibleItems)]
    pub fn validate_visible_items(&mut self);

    #[objc::msg_send(allowsExtensionItems)]
    pub fn allows_extension_items(&self) -> bool;

    #[objc::msg_send(setAllowsExtensionItems:)]
    pub fn set_allows_extension_items(&mut self, val: bool);
}

#[objc::protocol(NSToolbarDelegate)]
pub trait ToolbarDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:)]
    fn toolbar_item_for_id_will_be_inserted_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
        id: &ns::ToolbarItemId,
        flag: bool,
    ) -> Option<arc::Rar<ns::ToolbarItem>>;

    #[objc::optional]
    #[objc::msg_send(toolbarDefaultItemIdentifiers:)]
    fn toolbar_default_item_ids_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
    ) -> arc::Rar<ns::Array<ns::ToolbarItemId>>;

    #[objc::optional]
    #[objc::msg_send(toolbarAllowedItemIdentifiers:)]
    fn toolbar_allowed_item_ids_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
    ) -> arc::Rar<ns::Array<ns::ToolbarItemId>>;

    #[objc::optional]
    #[objc::msg_send(toolbarSelectableItemIdentifiers:)]
    fn toolbar_selectable_item_ids_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
    ) -> arc::Rar<ns::Array<ns::ToolbarItemId>>;

    #[objc::optional]
    #[objc::msg_send(toolbarImmovableItemIdentifiers:)]
    fn toolbar_immovable_item_ids_ar(
        &mut self,
        toolbar: &mut ns::Toolbar,
    ) -> arc::Rar<ns::Array<ns::ToolbarItemId>>;

    #[objc::optional]
    #[objc::msg_send(toolbar:itemIdentifier:canBeInsertedAtIndex:)]
    fn toolbar_item_id_can_be_inserted(
        &mut self,
        toolbar: &mut ns::Toolbar,
        item_id: &ns::ToolbarId,
        index: isize,
    ) -> bool;

    #[objc::msg_send(toolbarWillAddItem:)]
    fn toolbar_will_add_item(&mut self, n: &ns::Notification);

    #[objc::msg_send(toolbarDidRemoveItem:)]
    fn toolbar_did_remove_item(&mut self, n: &ns::Notification);
}

define_obj_type!(
    pub AnyToolbarDelegate(ns::Id)
);

impl ToolbarDelegate for AnyToolbarDelegate {}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_TOOLBAR: &'static objc::Class<Toolbar>;
}
