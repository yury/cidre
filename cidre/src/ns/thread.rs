#[cfg(feature = "blocks")]
use crate::blocks;
use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSThread")]
    pub Thread(ns::Id),
    NS_THREAD
);

#[objc::protocol(CidreThreadTarget)]
pub trait ThreadTarget: objc::Obj {
    #[objc::msg_send(main:)]
    fn main(&mut self, arg: Option<&mut ns::Id>);
}

impl arc::A<Thread> {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(initWithBlock:)]
    pub fn init_with_block(self, block: &mut blocks::SendBlock<fn()>) -> arc::R<Thread>;

    #[objc::msg_send(initWithTarget:selector:object:)]
    pub fn init_with_target_selector(
        self,
        target: &ns::Id,
        selector: &objc::Sel,
        arg: Option<&mut ns::Id>,
    ) -> arc::R<Thread>;
}

impl Thread {
    #[objc::msg_send(currentThread)]
    pub fn current() -> arc::R<Self>;

    #[cfg(feature = "blocks")]
    pub fn with_block(block: &mut blocks::SendBlock<fn()>) -> arc::R<Self> {
        Self::alloc().init_with_block(block)
    }

    #[cfg(feature = "blocks")]
    pub fn with_mut(f: impl FnMut() + Send + 'static) -> arc::R<Self> {
        let mut block = blocks::SendBlock::new0(f);
        Self::with_block(&mut block)
    }

    pub fn with_target_selector(
        target: &ns::Id,
        selector: &objc::Sel,
        arg: Option<&mut ns::Id>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_target_selector(target, selector, arg)
    }

    pub fn with_target<T: ThreadTargetImpl>(
        target: &mut T,
        arg: Option<&mut ns::Id>,
    ) -> arc::R<Self> {
        Self::with_target_selector(target.as_id_ref(), T::sel_main(), arg)
    }

    #[objc::msg_send(isMultiThreaded)]
    pub fn is_mutli_threaded() -> bool;

    #[objc::msg_send(threadDictionary)]
    pub fn thread_dictionary_mut(&mut self) -> arc::R<ns::DictionaryMut<ns::Id, ns::Id>>;

    #[objc::msg_send(threadDictionary)]
    pub fn thread_dictionary(&self) -> arc::R<ns::Dictionary<ns::Id, ns::Id>>;

    #[objc::msg_send(isExecuting)]
    pub fn is_executing(&self) -> bool;

    #[objc::msg_send(isFinished)]
    pub fn is_finished(&self) -> bool;

    #[objc::msg_send(isCancelled)]
    pub fn is_cancelled(&self) -> bool;

    #[objc::msg_send(isMainThread)]
    pub fn is_main() -> bool;

    #[objc::msg_send(isMainThread)]
    pub fn is_main_thread(&self) -> bool;

    #[objc::msg_send(threadPriority)]
    pub fn priority() -> f64;

    #[objc::msg_send(setThreadPriority:)]
    pub fn set_priority(val: f64) -> bool;

    #[objc::msg_send(qualityOfService)]
    pub fn qos(&self) -> ns::QualityOfService;

    #[objc::msg_send(setQualityOfService:)]
    pub fn set_qos(&mut self, val: ns::QualityOfService);

    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, val: Option<&ns::String>);

    /// The stack size of the receiver, in bytes.
    #[objc::msg_send(stackSize)]
    pub fn stack_size(&self) -> usize;

    /// The stack size of the receiver, in bytes.
    ///
    /// This value must be in bytes and a multiple of 4KB.
    ///
    /// To change the stack size, you must set this property before starting your thread.
    /// Setting the stack size after the thread has started changes the attribute size (which is reflected by the `stack_size` method),
    /// but it does not affect the actual number of pages set aside for the thread.
    #[objc::msg_send(setStackSize:)]
    pub fn set_stack_size(&mut self, val: usize);

    /// Changes the cancelled state of the receiver to indicate that it should exit.
    #[objc::msg_send(cancel)]
    pub fn cancel(&mut self);

    #[objc::msg_send(start)]
    pub fn start(&mut self);

    /// Blocks the current thread until the time specified.
    ///
    /// No run loop processing occurs while the thread is blocked.
    #[objc::msg_send(sleepUntilDate:)]
    pub fn sleep_until_date(date: &ns::Date);

    #[objc::msg_send(sleepForTimeInterval:)]
    pub fn sleep(ti: ns::TimeInterval);

    #[objc::msg_send(exit)]
    pub fn exit();
}

unsafe extern "C" {
    static NS_THREAD: &'static objc::Class<Thread>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let thread = ns::Thread::current();
        assert_eq!(thread.as_type_ref().retain_count(), 2);
        assert!(thread.is_executing());
        let dict = thread.thread_dictionary();
        assert_eq!(dict.as_type_ref().retain_count(), 2);

        assert!(ns::Thread::is_mutli_threaded());

        assert!(!ns::Thread::is_main());

        let thread = ns::Thread::with_mut(|| {});
        assert!(!thread.is_main_thread());
        assert!(!thread.is_executing());
        assert!(!thread.is_finished());
    }
}
