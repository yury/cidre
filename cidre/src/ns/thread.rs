use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSThread")]
    pub Thread(ns::Id),
    NS_THREAD
);

impl Thread {
    #[objc::msg_send(currentThread)]
    pub fn current<'a>() -> &'a Self;

    #[objc::msg_send(isMultiThreaded)]
    pub fn is_mutli_threaded() -> bool;

    #[objc::msg_send(threadDictionary)]
    pub fn thread_dictionary_mut(&mut self) -> &mut ns::DictionaryMut<ns::Id, ns::Id>;

    #[objc::msg_send(threadDictionary)]
    pub fn thread_dictionary(&self) -> &mut ns::Dictionary<ns::Id, ns::Id>;

    #[objc::msg_send(isExecuting)]
    pub fn is_executing(&self) -> bool;

    #[objc::msg_send(isFinished)]
    pub fn is_finished(&self) -> bool;

    #[objc::msg_send(isCancelled)]
    pub fn is_cancelled(&self) -> bool;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_THREAD: &'static objc::Class<Thread>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let thread = ns::Thread::current();
        assert_eq!(thread.as_type_ref().retain_count(), 1);
        assert!(thread.is_executing());
        let dict = thread.thread_dictionary();
        assert_eq!(dict.as_type_ref().retain_count(), 1);

        assert!(ns::Thread::is_mutli_threaded());
    }
}
