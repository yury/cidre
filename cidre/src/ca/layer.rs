use crate::{cg, define_obj_type, ns, objc};

define_obj_type!(Layer(ns::Id));

impl Layer {
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, value: cg::Rect);
}
