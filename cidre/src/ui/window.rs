use crate::{arc, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIWindow")]
    pub Window(ui::View),
    UI_WINDOW
);

impl arc::A<Window> {
    #[objc::msg_send(initWithWindowScene:)]
    pub fn init_with_window_scene(self, scene: &ui::WindowScene) -> arc::R<Window>;
}

impl Window {
    pub fn with_window_scene(scene: &ui::WindowScene) -> arc::R<Self> {
        Self::alloc().init_with_window_scene(scene)
    }

    #[objc::msg_send(rootViewController)]
    pub fn root_vc(&self) -> Option<&ui::ViewController>;

    #[objc::msg_send(setRootViewController:)]
    pub fn set_root_vc(&mut self, val: Option<&ui::ViewController>);

    #[objc::msg_send(makeKeyAndVisible)]
    pub fn make_key_and_visible(&self);
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_WINDOW: &'static objc::Class<Window>;
}
