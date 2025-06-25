use std::ffi::c_void;

use crate::{arc, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSKeyValueObservingOptions")]
    pub KvoOpts(usize)
);
impl KvoOpts {
    pub const NEW: Self = Self(0x01);
    pub const OLD: Self = Self(0x02);
    pub const INITIAL: Self = Self(0x04);
    pub const PRIOR: Self = Self(0x08);
}

#[doc(alias = "NSKeyValueChange")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum KvChange {
    Setting = 1,
    Insertion = 2,
    Removal = 3,
    Replacement = 4,
}

#[doc(alias = "NSKeyValueSetMutationKind")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum KvSetMutationKind {
    Union = 1,
    Minus = 2,
    Intersect = 3,
    Set = 4,
}

define_obj_type!(
    #[doc(alias = "NSKeyValueChangeKey")]
    pub KvChangeKey(ns::String)
);
impl KvChangeKey {
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

pub trait KvObserving {
    #[objc::msg_send(observeValueForKeyPath:ofObject:change:context:)]
    fn observe_value_for_key_path(
        &mut self,
        key_path: Option<&ns::String>,
        of_object: Option<&ns::Id>,
        change: Option<&ns::Dictionary<KvChangeKey, ns::Id>>,
        context: *mut c_void,
    );
}

pub struct Observer<F>
where
    F: FnMut(Option<&ns::String>, Option<&ns::Id>, Option<&ns::Dictionary<KvChangeKey, ns::Id>>),
{
    closure: F,
    cidre_observer: Option<arc::R<CidreObserver>>,
}

impl<F> std::fmt::Debug for Observer<F>
where
    F: FnMut(Option<&ns::String>, Option<&ns::Id>, Option<&ns::Dictionary<KvChangeKey, ns::Id>>),
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Observer")
            .field("cidre_observer", &self.cidre_observer)
            .finish()
    }
}

impl<F> Observer<F>
where
    F: FnMut(Option<&ns::String>, Option<&ns::Id>, Option<&ns::Dictionary<KvChangeKey, ns::Id>>),
{
    extern "C" fn change_handler(
        &mut self,
        key_path: Option<&ns::String>,
        object: Option<&ns::Id>,
        change: Option<&ns::Dictionary<KvChangeKey, ns::Id>>,
    ) {
        (self.closure)(key_path, object, change)
    }

    pub fn with_obj<'ar, O: objc::Obj>(
        obj: &mut O,
        key_path: &ns::String,
        options: KvoOpts,
        closure: F,
    ) -> Result<Box<Self>, &'ar ns::Exception>
    where
        O: objc::Obj + KvObserverRegistration,
    {
        let mut bx = Box::new(Self {
            closure,
            cidre_observer: None,
        });

        let o = ns::try_catch(|| unsafe {
            cidre_create_observer(
                obj.as_id_ref(),
                key_path,
                options,
                bx.as_mut() as *mut Self as *mut c_void,
                Self::change_handler as _,
            )
        })?;

        bx.cidre_observer = Some(o);
        Ok(bx)
    }
}

impl<F> Drop for Observer<F>
where
    F: FnMut(Option<&ns::String>, Option<&ns::Id>, Option<&ns::Dictionary<KvChangeKey, ns::Id>>),
{
    fn drop(&mut self) {
        if let Some(o) = self.cidre_observer.take() {
            o.invalide()
        }
    }
}

#[objc::protocol(CidreKVObserverRegistration)]
pub trait KvObserverRegistration {
    #[objc::msg_send(addObserver:forKeyPath:options:context:)]
    unsafe fn add_observer_throws(
        &mut self,
        observer: &ns::Id,
        for_key_path: &ns::String,
        options: KvoOpts,
        context: *mut c_void,
    );

    #[objc::msg_send(removeObserver:forKeyPath:context:)]
    unsafe fn remove_observer_ctx_throws(
        &mut self,
        observer: &ns::Id,
        key_path: &ns::String,
        context: *mut c_void,
    );

    #[objc::msg_send(removeObserver:forKeyPath:)]
    unsafe fn remove_observer_throws(&mut self, observer: &ns::Id, key_path: &ns::String);

    fn add_observer<'ear>(
        &mut self,
        observer: &ns::Id,
        for_key_path: &ns::String,
        options: KvoOpts,
        context: *mut c_void,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.add_observer_throws(observer, for_key_path, options, context)
        })
    }

    fn remove_observer_ctx<'ear>(
        &mut self,
        observer: &ns::Id,
        for_key_path: &ns::String,
        context: *mut c_void,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.remove_observer_ctx_throws(observer, for_key_path, context)
        })
    }

    fn remove_observer(&mut self, observer: &ns::Id, for_key_path: &ns::String) -> ns::ExResult {
        ns::try_catch(|| unsafe { self.remove_observer_throws(observer, for_key_path) })
    }
}

define_obj_type!(pub CidreObserver(ns::Id));
impl CidreObserver {
    #[objc::msg_send(invalidate)]
    pub fn invalide(&self);
}

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    static NSKeyValueChangeKindKey: &'static KvChangeKey;
    static NSKeyValueChangeNewKey: &'static KvChangeKey;
    static NSKeyValueChangeOldKey: &'static KvChangeKey;
    static NSKeyValueChangeIndexesKey: &'static KvChangeKey;
    static NSKeyValueChangeNotificationIsPriorKey: &'static KvChangeKey;
}

#[link(name = "ns", kind = "static")]
unsafe extern "C-unwind" {
    fn cidre_create_observer(
        obj: &ns::Id,
        key_path: &ns::String,
        options: KvoOpts,
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
        let mut q = ns::OpQueue::new();

        let _observer = ns::Observer::with_obj(
            q.as_mut(),
            ns::str!(c"name"),
            ns::KvoOpts::NEW,
            |_key, _obj, _change| unsafe {
                CALLS_COUNT += 1;
            },
        )
        .unwrap();

        q.set_name(Some(ns::str!(c"nice")));

        let pi = ns::ProcessInfo::current();

        let _observer = ns::Observer::with_obj(
            pi,
            ns::str!(c"thermalState"),
            ns::KvoOpts::INITIAL,
            |_key, _obj, _change| unsafe {
                CALLS_COUNT += 1;
            },
        )
        .unwrap();

        assert_eq!(unsafe { CALLS_COUNT }, 2);
    }

    #[test]
    fn fail() {
        let mut q = ns::OpQueue::new();
        let _observer = ns::Observer::with_obj(
            q.as_mut(),
            ns::str!(c"wrong. name"),
            ns::KvoOpts::NEW,
            |_key, _obj, _change| {},
        )
        .expect_err("should fail");
    }
}
