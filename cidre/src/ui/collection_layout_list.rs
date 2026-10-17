use crate::{arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UICollectionLayoutListAppearance")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum Appearance {
    Plain,
    Grouped,
    InsetGrouped,
    Sidebar,
    SidebarPlain,
}

#[doc(alias = "UICollectionLayoutListHeaderMode")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum HeaderMode {
    None,
    /// Headers from the data source's supplementary view provider.
    Supplementary,
    /// Each section's first item is its header, configured as one.
    FirstItemInSection,
}

define_obj_type!(
    #[doc(alias = "UICollectionLayoutListConfiguration")]
    pub CollectionLayoutListCfg(ns::Id),
    sym UICollectionLayoutListConfiguration
);

impl CollectionLayoutListCfg {
    #[objc::init(initWithAppearance:)]
    #[objc::available(ios = 14.0)]
    pub fn init_with_appearance(self, val: Appearance) -> arc::R<CollectionLayoutListCfg>;

    #[objc::available(ios = 14.0)]
    pub fn with_appearance(val: Appearance) -> arc::R<Self> {
        Self::alloc().init_with_appearance(val)
    }

    #[objc::msg_send(headerMode)]
    #[objc::available(ios = 14.0)]
    pub fn header_mode(&self) -> HeaderMode;

    #[objc::msg_send(setHeaderMode:)]
    #[objc::available(ios = 14.0)]
    pub fn set_header_mode(&mut self, val: HeaderMode);

    #[objc::msg_send(showsSeparators)]
    #[objc::available(ios = 14.0)]
    pub fn shows_separators(&self) -> bool;

    #[objc::msg_send(setShowsSeparators:)]
    #[objc::available(ios = 14.0)]
    pub fn set_shows_separators(&mut self, val: bool);

    #[objc::msg_send(backgroundColor)]
    #[objc::available(ios = 14.0)]
    pub fn bg_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setBackgroundColor:)]
    #[objc::available(ios = 14.0)]
    pub fn set_bg_color(&mut self, val: Option<&ui::Color>);

    /// Space above each section's header (`UICollectionViewListSectionHeaderTopPaddingAutomatic`
    /// by default).
    #[objc::msg_send(headerTopPadding)]
    #[objc::available(ios = 15.0)]
    pub fn header_top_padding(&self) -> crate::cg::Float;

    #[objc::msg_send(setHeaderTopPadding:)]
    #[objc::available(ios = 15.0)]
    pub fn set_header_top_padding(&mut self, val: crate::cg::Float);
}

impl ui::CollectionViewCompositionalLayout {
    #[objc::msg_send(layoutWithListConfiguration:)]
    #[objc::available(ios = 14.0)]
    pub fn with_list_cfg(cfg: &CollectionLayoutListCfg) -> arc::R<Self>;
}
