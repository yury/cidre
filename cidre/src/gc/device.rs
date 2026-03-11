use crate::{arc, define_obj_type, ns, objc};

#[cfg(feature = "dispatch")]
use crate::dispatch;

define_obj_type!(
    #[doc(alias = "GCDevice")]
    pub Device(ns::Id)
);

impl Device {
    #[cfg(feature = "dispatch")]
    #[objc::msg_send(handlerQueue)]
    pub fn handler_queue(&self) -> arc::R<dispatch::Queue>;

    #[cfg(feature = "dispatch")]
    #[objc::msg_send(setHandlerQueue:)]
    pub fn set_handler_queue(&mut self, val: &dispatch::Queue);

    #[objc::msg_send(vendorName)]
    pub fn vendor_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(productCategory)]
    pub fn product_category(&self) -> arc::R<ns::String>;
}
