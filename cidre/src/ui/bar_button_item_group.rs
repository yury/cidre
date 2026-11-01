use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIBarButtonItemGroup")]
    pub BarButtonItemGroup(ns::Id),
    sym UIBarButtonItemGroup
);

impl BarButtonItemGroup {
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
