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

    #[objc::msg_send(state)]
    #[objc::available(ios = 13.0)]
    pub fn state(&self) -> ui::MenuElementState;

    #[objc::msg_send(setState:)]
    #[objc::available(ios = 13.0)]
    pub fn set_state(&mut self, val: ui::MenuElementState);
}

unsafe extern "C" {
    static UI_ACTION: &'static objc::Class<Action>;
}
