use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIMenu")]
    pub Menu(ui::MenuElement),
    UI_MENU
);

impl Menu {
    #[objc::msg_send(menuWithTitle:children:)]
    #[objc::available(ios = 13.0)]
    pub fn with_title_children(
        title: &ns::String,
        children: &ns::Array<ui::MenuElement>,
    ) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_MENU: &'static objc::Class<Menu>;
}
