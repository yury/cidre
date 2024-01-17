use crate::{define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSApplication")]
    pub App(ns::Id)
);

#[objc::obj_trait]
pub trait Delegate {}

impl App {
    define_cls!(NS_APPLICATION);

    #[objc::cls_msg_send(sharedApplication)]
    pub fn shared() -> &'static mut Self;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, delegate: Option<&D>);

    #[objc::msg_send(run)]
    pub fn run(&mut self);

    #[objc::msg_send(terminate:)]
    pub fn terminate(&mut self, sender: Option<&ns::Id>);
}

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_APPLICATION: &'static objc::Class<App>;
}
