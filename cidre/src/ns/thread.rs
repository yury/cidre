use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSThread")]
    pub Thread(ns::Id),
    NS_THREAD
);

impl Thread {
    #[objc::msg_send(currentThread)]
    pub fn current() -> arc::R<Self>;

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
    }
}
