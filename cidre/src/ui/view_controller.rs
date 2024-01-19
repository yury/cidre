use crate::{arc, define_obj_type, objc, ui};

define_obj_type!(
    pub ViewController(ui::Responder), UI_VIEW_CONTROLLER
);

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_VIEW_CONTROLLER: &'static objc::Class<ViewController>;
}
