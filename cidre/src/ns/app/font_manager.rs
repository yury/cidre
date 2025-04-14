use crate::{arc, define_obj_type, define_opts, ns, objc};

define_opts!(
    pub FontTraitMask(usize)
);

impl FontTraitMask {
    pub const ITALIC: Self = Self(0x00000001);
    pub const BOLD: Self = Self(0x00000002);
    pub const UNBOLD: Self = Self(0x00000004);
    pub const NON_STANDARD_CHAR_SET: Self = Self(0x00000008);
    pub const NARROW: Self = Self(0x00000010);
    pub const EXPANDED: Self = Self(0x00000020);
    pub const CONDENSED: Self = Self(0x00000040);
    pub const SMALL_CAPS: Self = Self(0x00000080);
    pub const POSTER: Self = Self(0x00000100);
    pub const COMPRESSED: Self = Self(0x00000200);
    pub const FIXED_PITCH: Self = Self(0x00000400);
    pub const UNITALIC: Self = Self(0x01000000);
}

define_obj_type!(
    pub FontManager(ns::Id),
    NS_FONT_MANAGER
);

impl FontManager {
    #[objc::msg_send(setFontPanelFactory:)]
    pub fn set_font_panel_factory(factory_id: Option<&objc::Class<ns::Id>>);

    #[objc::msg_send(setFontManagerFactory:)]
    pub fn set_font_manager_factory(factory_id: Option<&objc::Class<ns::FontManager>>);

    #[objc::msg_send(sharedFontManager)]
    pub fn shared() -> arc::R<Self>;

    #[objc::msg_send(isMultiple)]
    pub fn is_multiple(&self) -> bool;

    #[objc::msg_send(setMultiple:)]
    pub fn set_multiple(&mut self, val: bool);

    #[objc::msg_send(selectedFont)]
    pub fn selected_font(&self) -> Option<arc::R<ns::Font>>;

    #[objc::msg_send(setSelectedFont:isMultiple:)]
    pub fn set_selected_font(&mut self, font: &ns::Font, is_mutliple: bool);

    #[objc::msg_send(setFontMenu:)]
    pub fn set_font_menu(&mut self, val: &ns::Menu);

    #[objc::msg_send(fontMenu:)]
    pub fn font_menu(&self, create: bool) -> Option<arc::R<ns::Menu>>;

    #[objc::msg_send(availableFonts)]
    pub fn available_fonts(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(availableFontFamilies)]
    pub fn available_font_families(&self) -> arc::R<ns::Array<ns::String>>;
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_FONT_MANAGER: &'static objc::Class<FontManager>;
}

#[cfg(test)]
mod tests {
    use crate::ns;
    #[test]
    fn basics() {
        let fm = ns::FontManager::new();
        assert!(fm.selected_font().is_none());
        assert!(!fm.available_fonts().is_empty());
        assert!(!fm.available_font_families().is_empty());

        let fm = ns::FontManager::shared();
        assert!(fm.selected_font().is_none());
        assert!(!fm.available_fonts().is_empty());
        assert!(!fm.available_font_families().is_empty());
    }
}
