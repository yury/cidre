use crate::{arc, cx, define_obj_type, dispatch, ns, objc};

define_obj_type!(
    /// Tells of the calls going on and their changes, the device's phone calls among them.
    #[doc(alias = "CXCallObserver")]
    pub CallObserver(ns::Id), sym CXCallObserver
);

impl CallObserver {
    /// The calls going on now.
    #[objc::msg_send(calls)]
    pub fn calls(&self) -> arc::R<ns::Array<cx::Call>>;

    /// `delegate` hears of every call's change on `queue` (the main queue when `None`). The
    /// observer does not retain it.
    #[objc::msg_send(setDelegate:queue:)]
    pub fn set_delegate<D: CallObserverDelegate>(
        &mut self,
        delegate: Option<&D>,
        queue: Option<&dispatch::Queue>,
    );
}

#[objc::protocol(CXCallObserverDelegate)]
pub trait CallObserverDelegate: objc::Obj {
    /// A call began, connected, went on or off hold, or ended.
    #[objc::msg_send(callObserver:callChanged:)]
    fn call_observer_call_changed(&mut self, observer: &CallObserver, call: &cx::Call);
}

define_obj_type!(pub AnyCallObserverDelegate(ns::Id));
impl CallObserverDelegate for AnyCallObserverDelegate {}

#[cfg(test)]
mod tests {
    use crate::cx;

    #[test]
    fn basics() {
        let observer = cx::CallObserver::new();
        assert!(observer.calls().is_empty());
    }
}
