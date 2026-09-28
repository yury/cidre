//! NSCursor.h standard cursor images.
use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSCursor")]
    pub Cursor(ns::Id),
    sym NSCursor
);
impl Cursor {
    #[objc::msg_send(arrowCursor)]
    pub fn arrow() -> arc::R<Self>;
    #[objc::msg_send(openHandCursor)]
    pub fn open_hand() -> arc::R<Self>;
    #[objc::msg_send(closedHandCursor)]
    pub fn closed_hand() -> arc::R<Self>;
    #[objc::msg_send(set)]
    pub fn set(&self);
}
