use crate::{arc, ca, define_obj_type, ns, objc};

define_obj_type!(pub Animation(ns::Id), CA_ANIMATION);
impl Animation {
    #[objc::msg_send(timingFunction)]
    pub fn timing_fn(&self) -> Option<&ca::MediaTimingFn>;

    #[objc::msg_send(setTimingFunction:)]
    pub fn set_timing_fn(&mut self, val: Option<&ca::MediaTimingFn>);
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static CA_ANIMATION: &'static objc::Class<Animation>;
}
