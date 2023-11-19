use crate::{define_cls, define_obj_type, ns, objc};

define_obj_type!(pub Application(ns::Id));

#[objc::obj_trait]
pub trait Delegate {}

impl Application {
    define_cls!(NS_APPLICATION);

    #[objc::cls_msg_send(sharedApplication)]
    pub fn shared() -> &'static mut Self;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, delegate: Option<&D>);

    #[objc::msg_send(run)]
    pub fn run(&mut self);
}

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_APPLICATION: &'static objc::Class<Application>;
}
