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

define_obj_type!(
    #[doc(alias = "UICollectionLayoutListConfiguration")]
    pub CollectionLayoutListCfg(ns::Id),
    UI_COLLECTION_LAYOUT_LIST_CONFIGURATION
);

impl CollectionLayoutListCfg {
    #[objc::init(initWithAppearance:)]
    #[objc::available(ios = 14.0)]
    pub fn init_with_appearance(self, val: Appearance) -> arc::R<CollectionLayoutListCfg>;

    #[objc::available(ios = 14.0)]
    pub fn with_appearance(val: Appearance) -> arc::R<Self> {
        Self::alloc().init_with_appearance(val)
    }
}

impl ui::CollectionViewCompositionalLayout {
    #[objc::msg_send(layoutWithListConfiguration:)]
    #[objc::available(ios = 14.0)]
    pub fn with_list_cfg(cfg: &CollectionLayoutListCfg) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_COLLECTION_LAYOUT_LIST_CONFIGURATION: &'static objc::Class<CollectionLayoutListCfg>;
}
