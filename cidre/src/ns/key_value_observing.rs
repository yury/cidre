use std::ffi::c_void;

use crate::{arc, define_obj_type, define_options, ns, objc};

define_options!(KVOOpts(usize));
impl KVOOpts {
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

pub struct Observer<F>
where
    F: FnMut(Option<&ns::String>, Option<&ns::Id>, Option<&ns::Dictionary<KVChangeKey, ns::Id>>),
{
    closure: F,
    cidre_observer: Option<arc::R<CidreObserver>>,
}

impl<F> Observer<F>
where
    F: FnMut(Option<&ns::String>, Option<&ns::Id>, Option<&ns::Dictionary<KVChangeKey, ns::Id>>),
{
    extern "C" fn change_handler(
        &mut self,
        key_path: Option<&ns::String>,
        object: Option<&ns::Id>,
        change: Option<&ns::Dictionary<KVChangeKey, ns::Id>>,
    ) {
        (self.closure)(key_path, object, change)
    }

    pub fn with_obj<O>(
        object: &mut O,
        key_path: &ns::String,
        options: KVOOpts,
        closure: F,
    ) -> Box<Self>
    where
        O: objc::Obj + KVObserverRegistration,
    {
        let res = Box::new(Self {
            closure,
            cidre_observer: None,
        });
        let raw = Box::into_raw(res);

        let o = unsafe {
            cidre_create_observer(
                std::mem::transmute(object),
                key_path,
                options,
                raw as *mut c_void,
                Self::change_handler as _,
            )
        };
        let mut res = unsafe { Box::from_raw(raw) };
        res.cidre_observer = Some(o);
        res
    }
}

impl<F> Drop for Observer<F>
where
    F: FnMut(Option<&ns::String>, Option<&ns::Id>, Option<&ns::Dictionary<KVChangeKey, ns::Id>>),
{
    fn drop(&mut self) {
        if let Some(o) = self.cidre_observer.take() {
            o.invalide()
        }
    }
}

#[objc::obj_trait]
pub trait KVObserverRegistration {
    #[objc::msg_send(addObserver:forKeyPath:options:context:)]
    fn add_observer(
        &mut self,
        observer: &ns::Id,
        for_key_path: &ns::String,
        options: KVOOpts,
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

define_obj_type!(CidreObserver(ns::Id));
impl CidreObserver {
    #[objc::msg_send(invalidate)]
    pub fn invalide(&self);
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    static NSKeyValueChangeKindKey: &'static KVChangeKey;
    static NSKeyValueChangeNewKey: &'static KVChangeKey;
    static NSKeyValueChangeOldKey: &'static KVChangeKey;
    static NSKeyValueChangeIndexesKey: &'static KVChangeKey;
    static NSKeyValueChangeNotificationIsPriorKey: &'static KVChangeKey;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn cidre_create_observer(
        obj: &ns::Id,
        key_path: &ns::String,
        options: KVOOpts,
        context: *mut c_void,
        fn_ptr: *const c_void,
    ) -> arc::R<CidreObserver>;

}

#[cfg(test)]
mod tests {
    use crate::ns;

    static mut CALLS_COUNT: usize = 0;

    #[test]
    fn basics() {
        let mut q = ns::OperationQueue::new();

        let _observer = ns::Observer::with_obj(
            q.as_mut(),
            &ns::String::with_str("name"),
            ns::KVOOpts::NEW,
            |key, obj, change| {
                println!("{:?} {:?} {:?}", key, obj, change);
                unsafe {
                    CALLS_COUNT += 1;
                }
            },
        );

        q.set_name(Some(&ns::String::with_str("nice")));

        let pi = ns::ProcessInfo::current();

        let _observer = ns::Observer::with_obj(
            pi,
            &ns::String::with_str("thermalState"),
            ns::KVOOpts::INITIAL,
            |key, obj, change| {
                println!("{:?} {:?} {:?}", key, obj, change);
                unsafe {
                    CALLS_COUNT += 1;
                }
            },
        );

        assert_eq!(unsafe { CALLS_COUNT }, 2);
    }
}
