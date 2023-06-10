use std::ffi::c_void;

use crate::{arc, define_obj_type, define_options, ns, objc};

define_options!(KVOOptions(usize));
impl KVOOptions {
    pub const NEW: Self = Self(0x01);
    pub const OLD: Self = Self(0x02);
    pub const INITIAL: Self = Self(0x04);
    pub const PRIOR: Self = Self(0x08);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum KVChange {
    Setting = 1,
    Insertion = 2,
    Removal = 3,
    Replacement = 4,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum KVSetMutationKind {
    Union = 1,
    Minus = 2,
    Intersect = 3,
    Set = 4,
}

define_obj_type!(KVChangeKey(ns::String));
impl KVChangeKey {
    pub fn kind() -> &'static Self {
        unsafe { NSKeyValueChangeKindKey }
    }

    pub fn new() -> &'static Self {
        unsafe { NSKeyValueChangeNewKey }
    }

    pub fn old() -> &'static Self {
        unsafe { NSKeyValueChangeOldKey }
    }

    pub fn indexes() -> &'static Self {
        unsafe { NSKeyValueChangeIndexesKey }
    }

    pub fn is_prior_key() -> &'static Self {
        unsafe { NSKeyValueChangeNotificationIsPriorKey }
    }
}

#[objc::obj_trait]
pub trait KVObserving {
    #[objc::msg_send(observeValueForKeyPath:ofObject:change:context:)]
    fn observe_value_for_key_path(
        &mut self,
        key_path: Option<&ns::String>,
        of_object: Option<&ns::Id>,
        change: Option<&ns::Dictionary<KVChangeKey, ns::Id>>,
        context: *mut c_void,
    );
}

define_obj_type!(
    Observer + KVObservingImpl,
    Box<dyn FnMut(&ns::String, &ns::Id, &ns::Dictionary<KVChangeKey, ns::Id>)>,
    FRAME_COUNTER
);

impl KVObserving for Observer {}
impl KVObservingImpl for Observer {
    extern "C" fn impl_observe_value_for_key_path(
        &mut self,
        _cmd: Option<&objc::Sel>,
        key_path: Option<&ns::String>,
        of_object: Option<&ns::Id>,
        change: Option<&ns::Dictionary<KVChangeKey, ns::Id>>,
        _context: *mut c_void,
    ) {
        unsafe {
            (self.inner_mut())(
                key_path.unwrap_unchecked(),
                of_object.unwrap_unchecked(),
                change.unwrap_unchecked(),
            )
        }
    }
}

impl Observer {
    pub fn with_obj<O, F>(
        object: &mut O,
        key_path: &ns::String,
        options: KVOOptions,
        closure: F,
    ) -> arc::R<Self>
    where
        O: KVObserverRegistration,
        F: FnMut(&ns::String, &ns::Id, &ns::Dictionary<KVChangeKey, ns::Id>) + 'static,
    {
        let observer = Observer::with(Box::new(closure));
        object.add_observer(&observer, key_path, options, std::ptr::null_mut());
        observer
    }
}

#[objc::obj_trait]
pub trait KVObserverRegistration {
    #[objc::msg_send(addObserver:forKeyPath:options:context:)]
    fn add_observer(
        &mut self,
        observer: &ns::Id,
        for_key_path: &ns::String,
        options: KVOOptions,
        context: *mut c_void,
    );

    #[objc::msg_send(removeObserver:forKeyPath:context:)]
    fn remove_observer_ctx(
        &mut self,
        observer: &ns::Id,
        for_key_path: &ns::String,
        context: *mut c_void,
    );
    #[objc::msg_send(removeObserver:forKeyPath:)]
    fn remove_observer(&mut self, observer: &ns::Id, for_key_path: &ns::String);
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    static NSKeyValueChangeKindKey: &'static KVChangeKey;
    static NSKeyValueChangeNewKey: &'static KVChangeKey;
    static NSKeyValueChangeOldKey: &'static KVChangeKey;
    static NSKeyValueChangeIndexesKey: &'static KVChangeKey;
    static NSKeyValueChangeNotificationIsPriorKey: &'static KVChangeKey;
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {
        // TODO:
    }
}
