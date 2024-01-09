use crate::{arc, define_cls, define_obj_type, gc, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum PlayerIndex {
    Unset = -1,
    _1 = 0,
    _2 = 1,
    _3 = 2,
    _4 = 3,
}

define_obj_type!(pub Controller(gc::Device));

impl Controller {
    define_cls!(GC_CONTROLLER);

    #[objc::cls_msg_send(current)]
    pub fn current_ar() -> Option<arc::Rar<Controller>>;

    #[objc::cls_rar_retain]
    pub fn current() -> Option<arc::R<Controller>>;

    #[objc::cls_msg_send(controllers)]
    pub fn controllers_ar() -> arc::Rar<ns::Array<Self>>;

    #[objc::cls_rar_retain]
    pub fn controllers() -> arc::R<ns::Array<Self>>;

    #[objc::msg_send(playerIndex)]
    pub fn player_index(&self) -> PlayerIndex;

    #[objc::msg_send(setPlayerIndex:)]
    pub fn set_player_index(&mut self, val: PlayerIndex) -> PlayerIndex;

    #[objc::msg_send(light)]
    pub fn light(&self) -> Option<&gc::DeviceLight>;

    #[objc::msg_send(capture)]
    pub fn capture_ar(&self) -> arc::Rar<Self>;

    #[objc::rar_retain]
    pub fn capture(&self) -> arc::R<Self>;
}

#[link(name = "gc", kind = "static")]
extern "C" {
    static GC_CONTROLLER: &'static objc::Class<Controller>;
}
