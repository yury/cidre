use crate::{arc, blocks, define_cls, define_obj_type, ns, objc};

use super::KVObserverRegistration;

define_obj_type!(Operation(ns::Id));
impl Operation {
    define_cls!(NS_OPERATION);

    #[objc::msg_send(isCancelled)]
    pub fn is_cancelled(&self) -> bool;

    #[objc::msg_send(cancel)]
    pub fn cancel(&mut self);
}

define_obj_type!(BlockOperation(Operation));
impl BlockOperation {
    define_cls!(NS_BLOCK_OPERATION);

    #[objc::cls_msg_send(blockOperationWithBlock:)]
    pub fn with_block_ar<F>(block: &'static mut blocks::Block<F>) -> arc::Rar<Self>
    where
        F: FnOnce();

    #[objc::cls_rar_retain]
    pub fn with_block<F>(block: &'static mut blocks::Block<F>) -> arc::R<Self>
    where
        F: FnOnce();
}

impl KVObserverRegistration for Operation {}
impl KVObserverRegistration for BlockOperation {}

define_obj_type!(OperationQueue(ns::Id), NS_OPERATION_QUEUE);

impl KVObserverRegistration for OperationQueue {}

impl OperationQueue {
    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<&ns::String>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, name: Option<&ns::String>);
}

extern "C" {
    static NS_OPERATION: &'static objc::Class<Operation>;
    static NS_BLOCK_OPERATION: &'static objc::Class<BlockOperation>;
    static NS_OPERATION_QUEUE: &'static objc::Class<OperationQueue>;
}
