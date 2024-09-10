use crate::{arc, cg, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIViewController")]
    pub ViewController(ui::Responder), UI_VIEW_CONTROLLER
);

impl ViewController {
    #[objc::msg_send(view)]
    pub fn view(&self) -> arc::R<ui::View>;

    /// Is used for any container laying out a child view controller.
    #[objc::msg_send(preferredContentSize)]
    pub fn preferred_content_size(&self) -> cg::Size;

    #[objc::msg_send(setPreferredContentSize:)]
    pub fn set_preferred_content_size(&mut self, val: cg::Size);
}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_VIEW_CONTROLLER: &'static objc::Class<ViewController>;
}
