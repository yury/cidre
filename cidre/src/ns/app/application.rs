use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSApplication")]
    pub App(ns::Id)
);

#[objc::protocol(NSApplicationDelegate)]
pub trait Delegate {
    // Notifications

    #[objc::optional]
    #[objc::msg_send(applicationWillFinishLaunching:)]
    fn app_will_finish_launching(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidFinishLaunching:)]
    fn app_did_finish_launching(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationWillTerminate:)]
    fn app_will_terminate(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationWillHide:)]
    fn app_will_hide(&mut self, n: &ns::Notification);

    #[objc::optional]
    #[objc::msg_send(applicationDidHide:)]
    fn app_did_hide(&mut self, n: &ns::Notification);
}

impl App {
    define_cls!(NS_APPLICATION);

    #[objc::msg_send(sharedApplication)]
    pub fn shared() -> arc::R<Self>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, delegate: Option<&D>);

    #[objc::msg_send(run)]
    pub fn run(&mut self);

    #[objc::msg_send(terminate:)]
    pub fn terminate(&mut self, sender: Option<&ns::Id>);

    #[objc::msg_send(dockTile)]
    pub fn dock_tile(&self) -> arc::R<ns::DockTile>;

    #[objc::msg_send(mainWindow)]
    pub fn main_window(&self) -> Option<arc::R<ns::Window>>;

    #[objc::msg_send(keyWindow)]
    pub fn key_window(&self) -> Option<arc::R<ns::Window>>;
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_APPLICATION: &'static objc::Class<App>;
}
