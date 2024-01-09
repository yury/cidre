use crate::{define_obj_type, dispatch, ns, objc};

define_obj_type!(pub Device(ns::Id));

impl Device {
    #[objc::msg_send(handlerQueue)]
    pub fn handler_queue(&self) -> &dispatch::Queue;

    #[objc::msg_send(setHandlerQueue:)]
    pub fn set_handler_queue(&mut self, val: &dispatch::Queue);

    #[objc::msg_send(vendorName)]
    pub fn vendor_name(&self) -> Option<&ns::String>;

    #[objc::msg_send(productCategory)]
    pub fn product_category(&self) -> &ns::String;
}
