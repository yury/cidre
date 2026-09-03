use crate::{arc, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIWindow")]
    pub Window(ui::View),
    UI_WINDOW
);

impl Window {
    #[objc::init(initWithWindowScene:)]
    pub fn init_with_window_scene(self, scene: &ui::WindowScene) -> arc::R<Window>;

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

unsafe extern "C" {
    static UI_WINDOW: &'static objc::Class<Window>;
}
