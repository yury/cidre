use crate::{arc, av, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVPlayerView")]
    pub PlayerView(ns::View),
    sym AVPlayerView
);

impl PlayerView {
    #[objc::msg_send(setPlayer:)]
    pub fn set_player(&mut self, val: Option<&av::Player>);
}
