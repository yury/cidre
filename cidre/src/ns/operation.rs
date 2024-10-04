use crate::{arc, define_cls, define_obj_type, dispatch, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "NSOperation")]
    pub Op(ns::Id)
);

impl Op {
    define_cls!(NS_OPERATION);

    #[objc::msg_send(isCancelled)]
    pub fn is_cancelled(&self) -> bool;

    #[objc::msg_send(cancel)]
    pub fn cancel(&mut self);
}

define_obj_type!(
    #[doc(alias = "NSBlockOperation")]
    pub BlockOp(Op)
);

impl BlockOp {
    define_cls!(NS_BLOCK_OPERATION);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(blockOperationWithBlock:)]
    pub fn with_block(block: &mut blocks::WorkBlock) -> arc::R<Self>;
}

impl ns::KVObserverRegistration for Op {}
impl ns::KVObserverRegistration for BlockOp {}

define_obj_type!(
    #[doc(alias = "NSOperationQueue")]
    pub OpQueue(ns::Id), NS_OPERATION_QUEUE
);

impl ns::KVObserverRegistration for OpQueue {}

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
    pub fn underlying_queue(&self) -> Option<arc::R<dispatch::Queue>>;

    #[objc::msg_send(setUnderlyingQueue:)]
    pub fn set_underlying_queue(&mut self, val: Option<&dispatch::Queue>);

    #[objc::msg_send(currentQueue)]
    pub fn current() -> Option<arc::R<Self>>;

    #[objc::msg_send(mainQueue)]
    pub fn main() -> arc::R<Self>;
}

extern "C" {
    static NS_OPERATION: &'static objc::Class<Op>;
    static NS_BLOCK_OPERATION: &'static objc::Class<BlockOp>;
    static NS_OPERATION_QUEUE: &'static objc::Class<OpQueue>;
}

#[cfg(test)]
mod tests {
    use crate::{dispatch, ns};

    #[test]
    fn basics() {
        assert!(ns::OpQueue::current().is_none());
        let mut queue = ns::OpQueue::new();
        assert!(queue.underlying_queue().is_none());
        let dqueue = dispatch::Queue::new();
        queue.set_underlying_queue(Some(&dqueue));
        assert_eq!(queue.underlying_queue(), Some(dqueue));

        let main_op_queue = ns::OpQueue::main();
        println!("tid {main_op_queue:?}");
    }
}
