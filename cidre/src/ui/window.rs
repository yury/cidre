use crate::{arc, cg, define_obj_type, objc, ui};

/// Windows stack by level, higher in front; within a level the key window is in front.
#[doc(alias = "UIWindowLevel")]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Level(pub cg::Float);

impl Level {
    #[doc(alias = "UIWindowLevelNormal")]
    #[inline]
    pub fn normal() -> Self {
        unsafe { UIWindowLevelNormal }
    }

    #[doc(alias = "UIWindowLevelAlert")]
    #[inline]
    pub fn alert() -> Self {
        unsafe { UIWindowLevelAlert }
    }

    #[doc(alias = "UIWindowLevelStatusBar")]
    #[inline]
    pub fn status_bar() -> Self {
        unsafe { UIWindowLevelStatusBar }
    }
}

#[link(name = "UIKit", kind = "framework")]
unsafe extern "C" {
    static UIWindowLevelNormal: Level;
    static UIWindowLevelAlert: Level;
    static UIWindowLevelStatusBar: Level;
}

define_obj_type!(
    #[doc(alias = "UIWindow")]
    pub Window(ui::View),
    sym UIWindow
);

impl Window {
    #[objc::init(initWithWindowScene:)]
    pub fn init_with_window_scene(self, scene: &ui::WindowScene) -> arc::R<Window>;

    pub fn with_window_scene(scene: &ui::WindowScene) -> arc::R<Self> {
        Self::alloc().init_with_window_scene(scene)
    }

    #[objc::msg_send(windowScene)]
    pub fn window_scene(&self) -> Option<arc::R<ui::WindowScene>>;

    #[objc::msg_send(rootViewController)]
    pub fn root_vc(&self) -> Option<arc::R<ui::ViewController>>;

    #[objc::msg_send(setRootViewController:)]
    pub fn set_root_vc(&mut self, val: Option<&ui::ViewController>);

    #[objc::msg_send(makeKeyAndVisible)]
    pub fn make_key_and_visible(&self);

    #[objc::msg_send(windowLevel)]
    pub fn level(&self) -> Level;

    #[objc::msg_send(setWindowLevel:)]
    pub fn set_level(&mut self, val: Level);
}
