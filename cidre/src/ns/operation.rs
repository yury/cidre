use crate::{arc, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "dispatch")]
use crate::dispatch;

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

    /// A bool value indicating whether the operation is currently executing.
    #[objc::msg_send(isExecuting)]
    pub fn is_executing(&self) -> bool;

    #[objc::msg_send(isFinished)]
    pub fn is_finished(&self) -> bool;

    #[objc::msg_send(isReady)]
    pub fn is_ready(&self) -> bool;

    /// Makes the receiver dependent on the completion of the specified operation.
    ///
    /// The receiver is not considered ready to execute until all of its dependent operations have finished executing.
    /// If the receiver is already executing its task, adding dependencies has no practical effect. This method
    ///  may change the isReady and dependencies properties of the receiver.
    ///
    /// It is a programmer error to create any circular dependencies among a set of operations.
    ///  Doing so can cause a deadlock among the operations and may freeze your program.
    #[objc::msg_send(addDependency:)]
    pub fn add_dependency(&mut self, val: &ns::Op);

    /// Removes the receiver’s dependence on the specified operation.
    #[objc::msg_send(removeDependency:)]
    pub fn remove_dependency(&mut self, val: &ns::Op);

    /// An array of the operation objects that must finish executing before the current object can begin executing.
    #[objc::msg_send(dependencies)]
    pub fn dependencies(&self) -> arc::R<ns::Array<ns::Op>>;

    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, val: Option<&ns::String>);

    /// Begins the execution of the operation.
    #[objc::msg_send(start)]
    pub fn start(&mut self);

    /// Performs the receiver’s non-concurrent task.
    #[objc::msg_send(main)]
    pub fn main(&mut self);
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

impl ns::KvObserverRegistration for Op {}
impl ns::KvObserverRegistration for BlockOp {}

define_obj_type!(
    #[doc(alias = "NSOperationQueue")]
    pub OpQueue(ns::Id), NS_OPERATION_QUEUE
);

impl ns::KvObserverRegistration for OpQueue {}

impl OpQueue {
    /// The default maximum number of operations to invoke concurrently in a queue.
    ///
    /// The operation queue determines this number dynamically based on current system conditions.
    #[doc(alias = "NSOperationQueueDefaultMaxConcurrentOperationCount")]
    pub const DEFAULT_MAX_CONCURRENT_OP_COUNT: isize = -1;

    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, name: Option<&ns::String>);

    #[objc::msg_send(addOperation:)]
    pub fn add_op(&mut self, op: &Op);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(addOperationWithBlock:)]
    pub fn add_op_with_block(&mut self, block: &mut blocks::WorkBlock);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(addBarrierBlock:)]
    pub fn add_barrier_block(&mut self, barrier: &mut blocks::WorkBlock);

    /// The maximum number of queued operations that can run at the same time.
    ///
    /// The value in this property affects only the operations that the current queue has executing at the same time.
    /// Other operation queues can also execute their maximum number of operations in parallel.
    /// Reducing the number of concurrent operations does not affect any operations that are currently executing.
    /// Specifying the value NSOperationQueueDefaultMaxConcurrentOperationCount (which is recommended) causes the
    /// system to set the maximum number of operations based on system conditions.
    /// The default value of this property is defaultMaxConcurrentOperationCount. You may monitor changes to the value
    /// of this property using Key-value observing. Configure an observer to monitor the maxConcurrentOperationCount
    /// key path of the operation queue.
    #[objc::msg_send(maxConcurrentOperationCount)]
    pub fn max_concurrent_ops(&self) -> isize;

    #[objc::msg_send(setMaxConcurrentOperationCount:)]
    pub fn set_max_concurrent_ops(&mut self, val: isize);

    /// A bool value indicating whether the queue is actively scheduling operations for execution.
    ///
    /// When the value of this property is false, the queue actively starts operations that are in the queue and ready
    /// to execute. Setting this property to true prevents the queue from starting any queued operations,
    /// but already executing operations continue to execute. You may continue to add operations to a queue
    /// that is suspended but those operations are not scheduled for execution until you change this property to false.
    ///
    /// Operations are removed from the queue only when they finish executing. However, in order to finish executing,
    /// an operation must first be started. Because a suspended queue does not start any new operations, it does not
    /// remove any operations (including cancelled operations) that are currently queued and not executing.
    ///
    /// The default value of this property is false.
    #[objc::msg_send(isSuspended)]
    pub fn is_suspended(&self) -> bool;

    #[objc::msg_send(setSuspended:)]
    pub fn set_suspended(&mut self, val: bool);

    #[objc::msg_send(cancelAllOperations)]
    pub fn cancel_all_ops(&mut self);

    #[cfg(feature = "dispatch")]
    #[objc::msg_send(underlyingQueue)]
    pub fn underlying_queue(&self) -> Option<arc::R<dispatch::Queue>>;

    #[cfg(feature = "dispatch")]
    #[objc::msg_send(setUnderlyingQueue:)]
    pub fn set_underlying_queue(&mut self, val: Option<&dispatch::Queue>);

    #[objc::msg_send(currentQueue)]
    pub fn current() -> Option<arc::R<Self>>;

    #[objc::msg_send(mainQueue)]
    pub fn main() -> arc::R<Self>;
}

/// ProgressReporting
impl OpQueue {
    #[objc::msg_send(progress)]
    pub fn progress(&self) -> arc::R<ns::Progress>;
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
        assert_eq!(
            ns::OpQueue::DEFAULT_MAX_CONCURRENT_OP_COUNT,
            queue.max_concurrent_ops()
        );
        assert!(queue.underlying_queue().is_none());
        let dqueue = dispatch::Queue::new();
        queue.set_underlying_queue(Some(&dqueue));
        assert_eq!(queue.underlying_queue().as_ref(), Some(&dqueue));
        assert_eq!(-1, queue.max_concurrent_ops());
        queue.set_max_concurrent_ops(10);
        assert_eq!(10, queue.max_concurrent_ops());
        assert_eq!(queue.underlying_queue().as_ref(), Some(&dqueue));

        let main_op_queue = ns::OpQueue::main();
        println!("tid {main_op_queue:?}");
    }
}
