use std::{
    ffi::{c_char, c_void, CStr},
    marker::PhantomData,
    str::FromStr,
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
    pub fn get_fn(&self) -> Option<&T> {
        unsafe {
            match self.ptr.load(Ordering::Relaxed) {
                1 => {
                    let fnptr = &(self.initialize_fn());
                    std::mem::transmute(fnptr)
                }
                ptr => {
                    fence(Ordering::Acquire);
                    let fnptr = &ptr;
                    std::mem::transmute(fnptr)
                }
            }
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
    unsafe fn initialize_fn(&self) -> usize {
        extern "C" {
            fn dlsym(handle: *const c_void, symbol: *const c_char) -> *mut c_void;
        }
        const RTLD_DEFAULT: isize = -2isize;
        let val = dlsym(RTLD_DEFAULT as _, self.name.as_ptr());
        if val.is_null() {
            return 0;
        }

        // let val = val as *mut usize;
        // let val = *val;
        let val = val as usize;

        // This synchronizes with the acquire fence in `get`.
        self.ptr.store(val, Ordering::Release);

        val
    }

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

use crate::ns;

#[inline]
pub fn macos_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "macos"))]
    return false;
    #[cfg(target_os = "macos")]
    ns::ProcessInfo::current().is_os_at_least_version(ns::OsVersion::from_str(_ver).unwrap())
}

#[inline]
pub fn ios_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "ios"))]
    return false;
    #[cfg(target_os = "ios")]
    ns::ProcessInfo::current().is_os_at_least_version(ns::OsVersion::from_str(_ver).unwrap())
}

#[inline]
pub fn tvos_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "tvos"))]
    return false;
    #[cfg(target_os = "tvos")]
    ns::ProcessInfo::current().is_os_at_least_version(ns::OsVersion::from_str(_ver).unwrap())
}

#[inline]
pub fn watchos_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "watchos"))]
    return false;
    #[cfg(target_os = "watchos")]
    ns::ProcessInfo::current().is_os_at_least_version(ns::OsVersion::from_str(_ver).unwrap())
}

#[inline]
pub fn visionos_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "visionos"))]
    return false;
    #[cfg(target_os = "visionos")]
    ns::ProcessInfo::current().is_os_at_least_version(ns::OsVersion::from_str(_ver).unwrap())
}
#[inline]
pub fn maccatalyst_available(_ver: &str) -> bool {
    #[cfg(not(all(target_os = "ios", target_abi = "macabi")))]
    return false;
    #[cfg(all(target_os = "ios", target_abi = "macabi"))]
    ns::ProcessInfo::current().is_os_at_least_version(ns::OsVersion::from_str(_ver).unwrap())
}

#[macro_export]
macro_rules! version {
    (macos = $ver:literal) => {
        $crate::api::macos_available(stringify!($ver))
    };
    (ios = $ver:literal) => {
        $crate::api::ios_available(stringify!($ver))
    };
    (tvos = $ver:literal) => {
        $crate::api::tvos_available(stringify!($ver))
    };
    (watchos = $ver:literal) => {
        $crate::api::watchos_available(stringify!($ver))
    };
    (visionos = $ver:literal) => {
        $crate::api::visionos_available(stringify!($ver))
    };
    (maccatalyst = $ver:literal) => {
        $crate::api::maccatalyst_available(stringify!($ver))
    };
    (
        $(macos = $macos_ver:literal)?
        $(, ios = $ios_ver:literal)?
        $(, tvos = $tvos_ver:literal)?
        $(, watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
        $(, maccatalyst = $maccatalyst_ver:literal)?
    ) => {
        $($crate::api::version!(macos = $macos_ver))?
        $( || $crate::api::version!(ios = $ios_ver))?
        $( || $crate::api::version!(tvos = $tvos_ver))?
        $( || $crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
        $( || $crate::api::version!(maccatalyst = $maccatalyst_ver))?
    };
    (
        $(ios = $ios_ver:literal)?
        $(, tvos = $tvos_ver:literal)?
        $(, watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
        $(, maccatalyst = $maccatalyst_ver:literal)?
    ) => {
        $($crate::api::version!(ios = $ios_ver))?
        $( || $crate::api::version!(tvos = $tvos_ver))?
        $( || $crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
        $( || $crate::api::version!(maccatalyst = $maccatalyst_ver))?
    };
    (
        $(tvos = $tvos_ver:literal)?
        $(, watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
        $(, maccatalyst = $maccatalyst_ver:literal)?
    ) => {
        $($crate::api::version!(tvos = $tvos_ver))?
        $( || $crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
        $( || $crate::api::version!(maccatalyst = $maccatalyst_ver))?
    };
    (
        $(watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
    ) => {
        $($crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
        $( || $crate::api::version!(maccatalyst = $maccatalyst_ver))?
    };
    (
        $(watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
        $(, maccatalyst = $maccatalyst_ver:literal)?
    ) => {
        $($crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
        $( || $crate::api::version!(maccatalyst = $maccatalyst_ver))?
    };
    (
        $(visionos = $visionos_ver:literal)?
        $(, maccatalyst = $maccatalyst_ver:literal)?
    ) => {
        $( || $crate::api::version!(visionos = $visionos_ver))?
        $( || $crate::api::version!(maccatalyst = $maccatalyst_ver))?
    };
}
pub use cidre_macros::api_available as available;
pub use cidre_macros::api_weak as weak;
pub use version;

#[cfg(test)]
mod tests {
    use crate::{api, ns};

    static NOT_FOUND: api::DlSym<ns::String> = api::DlSym::new(c"not_a_symbol");
    static SHOULD_BE_FOUND: api::DlSym<ns::String> =
        api::DlSym::new(c"NSInvocationOperationVoidResultException");

    #[test]
    fn basics() {
        assert!(NOT_FOUND.get().is_none());
        assert!(NOT_FOUND.get().is_none());
        assert!(SHOULD_BE_FOUND.get().is_some());
        assert!(SHOULD_BE_FOUND.get().unwrap().len() > 0);
        assert!(SHOULD_BE_FOUND.get().unwrap().len() > 0);
    }

    #[test]
    fn version() {
        assert!(api::macos_available("13.0"));
        assert!(api::macos_available("13.1.1"));
        assert!(api::version!(macos = 13.0, ios = 14.0));
        assert!(!api::version!(visionos = 1.0));
        assert!(!api::version!(macos = 20.0));
    }
}
