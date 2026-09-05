use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIBarButtonItem")]
    pub BarButtonItem(ns::Id),
    UI_BAR_BUTTON_ITEM
);

impl BarButtonItem {
    #[objc::init(initWithCustomView:)]
    pub fn init_with_custom_view(self, view: &ui::View) -> arc::R<BarButtonItem>;

    pub fn with_custom_view(view: &ui::View) -> arc::R<Self> {
        Self::alloc().init_with_custom_view(view)
    }
}

unsafe extern "C" {
    static UI_BAR_BUTTON_ITEM: &'static objc::Class<BarButtonItem>;
}
