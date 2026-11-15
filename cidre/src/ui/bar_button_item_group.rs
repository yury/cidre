use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIBarButtonItemGroup")]
    pub BarButtonItemGroup(ns::Id),
    sym UIBarButtonItemGroup
);

impl BarButtonItemGroup {
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(fixedGroupWithRepresentativeItem:items:)]
    #[objc::available(ios = 16.0)]
    pub fn fixed(
        representative: Option<&ui::BarButtonItem>,
        items: &ns::Array<ui::BarButtonItem>,
    ) -> arc::R<Self>;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(movableGroupWithCustomizationIdentifier:representativeItem:items:)]
    #[objc::available(ios = 16.0)]
    pub fn movable(
        id: &ns::String,
        representative: Option<&ui::BarButtonItem>,
        items: &ns::Array<ui::BarButtonItem>,
    ) -> arc::R<Self>;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(optionalGroupWithCustomizationIdentifier:inDefaultCustomization:representativeItem:items:)]
    #[objc::available(ios = 16.0)]
    pub fn optional(
        id: &ns::String,
        in_default: bool,
        representative: Option<&ui::BarButtonItem>,
        items: &ns::Array<ui::BarButtonItem>,
    ) -> arc::R<Self>;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos", target_os = "visionos")))]
    #[objc::msg_send(groupWithFixedSpace)]
    #[objc::available(ios = 26.0)]
    pub fn fixed_space() -> arc::R<Self>;

    #[objc::msg_send(barButtonItems)]
    pub fn items(&self) -> arc::R<ns::Array<ui::BarButtonItem>>;

    #[objc::msg_send(setBarButtonItems:)]
    pub fn set_items(&mut self, val: &ns::Array<ui::BarButtonItem>);

    #[objc::msg_send(representativeItem)]
    pub fn representative_item(&self) -> Option<arc::R<ui::BarButtonItem>>;

    #[objc::msg_send(setRepresentativeItem:)]
    pub fn set_representative_item(&mut self, val: Option<&ui::BarButtonItem>);

    #[objc::msg_send(isDisplayingRepresentativeItem)]
    pub fn is_displaying_representative_item(&self) -> bool;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(alwaysAvailable)]
    #[objc::available(ios = 16.0)]
    pub fn always_available(&self) -> bool;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(setAlwaysAvailable:)]
    #[objc::available(ios = 16.0)]
    pub fn set_always_available(&mut self, val: bool);

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(setMenuRepresentation:)]
    #[objc::available(ios = 16.0)]
    pub fn set_menu_representation(&mut self, val: Option<&ui::MenuElement>);

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(isHidden)]
    #[objc::available(ios = 16.0)]
    pub fn is_hidden(&self) -> bool;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[objc::msg_send(setHidden:)]
    #[objc::available(ios = 16.0)]
    pub fn set_hidden(&mut self, val: bool);

    #[objc::init(initWithBarButtonItems:representativeItem:)]
    #[objc::available(ios = 9.0, tvos = 9.0)]
    pub fn init_with_items_representative_item(
        self,
        items: &ns::Array<ui::BarButtonItem>,
        representative_item: Option<&ui::BarButtonItem>,
    ) -> arc::R<BarButtonItemGroup>;

    pub fn with_items(items: &ns::Array<ui::BarButtonItem>) -> arc::R<Self> {
        Self::alloc().init_with_items_representative_item(items, None)
    }
}

// UIBarButtonItem's category declared by UIBarButtonItemGroup.h.
impl ui::BarButtonItem {
    #[objc::msg_send(buttonGroup)]
    pub fn button_group(&self) -> Option<arc::R<BarButtonItemGroup>>;
}
