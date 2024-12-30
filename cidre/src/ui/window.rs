use crate::{arc, define_obj_type, objc, ui};

define_obj_type!(
    pub Window(ui::View),
    UI_WINDOW
);

impl Window {
    #[objc::msg_send(rootViewController)]
    pub fn root_vc(&self) -> Option<&ui::ViewController>;

    #[objc::msg_send(setRootViewController:)]
    pub fn set_root_vc(&mut self, val: Option<&ui::ViewController>);
}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_WINDOW: &'static objc::Class<Window>;
}
