use crate::{arc, define_cls, define_obj_type, dispatch, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

use super::KVObserverRegistration;

define_obj_type!(pub Op(ns::Id));
impl Op {
    define_cls!(NS_OPERATION);

    #[objc::msg_send(isCancelled)]
    pub fn is_cancelled(&self) -> bool;

    #[objc::msg_send(cancel)]
    pub fn cancel(&mut self);
}

define_obj_type!(pub BlockOp(Op));
impl BlockOp {
    define_cls!(NS_BLOCK_OPERATION);

    #[cfg(feature = "blocks")]
    #[objc::cls_msg_send(blockOperationWithBlock:)]
    pub fn with_block_ar<F>(block: &'static mut blocks::Block<F>) -> arc::Rar<Self>
    where
        F: FnOnce();

    #[cfg(feature = "blocks")]
    #[objc::cls_rar_retain]
    pub fn with_block<F>(block: &'static mut blocks::Block<F>) -> arc::R<Self>
    where
        F: FnOnce();
}

impl KVObserverRegistration for Op {}
impl KVObserverRegistration for BlockOp {}

define_obj_type!(pub OpQueue(ns::Id), NS_OPERATION_QUEUE);

impl KVObserverRegistration for OpQueue {}

impl OpQueue {
    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<&ns::String>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, name: Option<&ns::String>);

    #[objc::msg_send(addOperation:)]
    pub fn add_op(&mut self, op: &Op);

    #[objc::msg_send(cancelAllOperations)]
    pub fn cancel_all_ops(&mut self);

    #[objc::msg_send(underlyingQueue)]
    pub fn underlying_queue(&self) -> Option<&dispatch::Queue>;

    #[objc::cls_msg_send(currentQueue)]
    pub fn current<'a>() -> Option<&'a Self>;

    #[objc::cls_msg_send(mainQueue)]
    pub fn main() -> &'static Self;
}

extern "C" {
    static NS_OPERATION: &'static objc::Class<Op>;
    static NS_BLOCK_OPERATION: &'static objc::Class<BlockOp>;
    static NS_OPERATION_QUEUE: &'static objc::Class<OpQueue>;
}
