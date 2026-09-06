use crate::{arc, blocks, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIAction")]
    pub Action(ui::MenuElement),
    UI_ACTION
);

impl Action {
    #[objc::msg_send(actionWithTitle:image:identifier:handler:)]
    #[objc::available(ios = 13.0)]
    pub fn with_title_image_id_handler(
        title: &ns::String,
        image: Option<&ui::Image>,
        id: Option<&ns::String>,
        handler: &mut blocks::EscBlock<fn(&Action)>,
    ) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_ACTION: &'static objc::Class<Action>;
}
