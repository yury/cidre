use std::{
    ffi::{c_char, c_void, CStr},
    marker::PhantomData,
    sync::atomic::{fence, AtomicUsize, Ordering},
};

pub struct DlSym<T> {
    name: &'static CStr,
    ptr: AtomicUsize,
    _marker: PhantomData<T>,
}

impl<T> DlSym<T> {
    pub(crate) const fn new(name: &'static CStr) -> Self {
        Self {
            name,
            ptr: AtomicUsize::new(1),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self) -> Option<&T> {
        unsafe {
            match self.ptr.load(Ordering::Relaxed) {
                1 => self.initialize(),
                ptr => {
                    fence(Ordering::Acquire);
                    std::mem::transmute(ptr)
                }
            }
        }
    }

    // Cold because it should only happen during first-time initialization.
    #[cold]
    unsafe fn initialize(&self) -> Option<&T> {
        extern "C" {
            fn dlsym(handle: *const c_void, symbol: *const c_char) -> *mut c_void;
        }
        const RTLD_DEFAULT: isize = -2isize;
        let val = dlsym(RTLD_DEFAULT as _, self.name.as_ptr());
        if val.is_null() {
            return None;
        }

        let val = val as *mut usize;
        let val = *val;

        // This synchronizes with the acquire fence in `get`.
        self.ptr.store(val, Ordering::Release);

        std::mem::transmute(val)
    }
}

unsafe impl<T> Sync for DlSym<T> {}

pub use cidre_macros::api_available as available;
pub use cidre_macros::api_weak as weak;

#[cfg(test)]
mod tests {
    use crate::ns;

    use super::DlSym;

    static NOT_FOUND: DlSym<ns::String> = DlSym::new(c"not_a_symbol");
    static SHOULD_BE_FOUND: DlSym<ns::String> =
        DlSym::new(c"NSInvocationOperationVoidResultException");
    static NOT_FOUND2: DlSym<extern "C" fn(u32)> = DlSym::new(c"not_a_symbol");

    #[test]
    fn basics() {
        assert!(NOT_FOUND.get().is_none());
        assert!(NOT_FOUND.get().is_none());
        assert!(SHOULD_BE_FOUND.get().is_some());
        assert!(SHOULD_BE_FOUND.get().unwrap().len() > 0);
        assert!(SHOULD_BE_FOUND.get().unwrap().len() > 0);
    }
}
