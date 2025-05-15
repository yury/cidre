use std::{
    ffi::{CStr, c_char, c_void},
    marker::PhantomData,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering, fence},
};

pub struct DlSym<T> {
    name: &'static CStr,
    ptr: AtomicUsize,
    _marker: PhantomData<T>,
}

impl<T> DlSym<T> {
    #[allow(dead_code)]
    pub const fn new(name: &'static CStr) -> Self {
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
                1 => std::mem::transmute(&self.initialize_fn()),
                ptr => {
                    fence(Ordering::Acquire);
                    std::mem::transmute(&ptr)
                }
            }
        }
    }
    #[inline]
    pub fn get_var(&self) -> Option<&T> {
        unsafe {
            match self.ptr.load(Ordering::Relaxed) {
                1 => std::mem::transmute(self.initialize_var()),
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
        unsafe extern "C" {
            fn dlsym(handle: *const c_void, symbol: *const c_char) -> *mut c_void;
        }
        const RTLD_DEFAULT: isize = -2isize;
        let val = unsafe { dlsym(RTLD_DEFAULT as _, self.name.as_ptr()) };
        if val.is_null() {
            return 0;
        }

        let val = val as usize;

        // This synchronizes with the acquire fence in `get`.
        self.ptr.store(val, Ordering::Release);

        val
    }

    #[cold]
    unsafe fn initialize_var(&self) -> usize {
        unsafe extern "C" {
            fn dlsym(handle: *const c_void, symbol: *const c_char) -> *mut c_void;
        }
        const RTLD_DEFAULT: isize = -2isize;
        let val = unsafe { dlsym(RTLD_DEFAULT as _, self.name.as_ptr()) };
        if val.is_null() {
            return 0;
        }

        let val = val as *mut usize;
        let val = unsafe { *val };

        // This synchronizes with the acquire fence in `get`.
        self.ptr.store(val, Ordering::Release);

        val
    }
}

unsafe impl<T> Sync for DlSym<T> {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OsVersion {
    pub major: isize,
    pub minor: isize,
    pub patch: isize,
}

#[repr(u32)]
pub enum Platform {
    MacOs = 1,
    IOs = 2,
    TvOs = 3,
    WatchOs = 4,
    // DriverKit = 10,
    VisionOs = 11,
}

unsafe extern "C" {
    fn __isPlatformVersionAtLeast(platform: u32, major: u32, minor: u32, patch: u32) -> i32;
}

impl OsVersion {
    #[inline]
    pub fn platform_at_least(&self, platform: Platform) -> bool {
        unsafe {
            __isPlatformVersionAtLeast(
                platform as _,
                self.major as _,
                self.minor as _,
                self.patch as _,
            ) != 0
        }
    }

    #[cfg(target_os = "macos")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::MacOs)
    }

    #[cfg(target_os = "ios")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::IOs)
    }

    #[cfg(target_os = "tvos")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::TvOs)
    }

    #[cfg(target_os = "watchos")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::WatchOs)
    }

    #[cfg(target_os = "visionos")]
    #[inline]
    pub fn at_least(&self) -> bool {
        self.platform_at_least(Platform::VisionOs)
    }
}

#[derive(Debug)]
pub struct VersionError;

impl FromStr for OsVersion {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((major, rest)) = s.split_once('.') else {
            let Ok(major) = s.parse::<isize>() else {
                return Err(VersionError);
            };
            return Ok(Self {
                major,
                minor: 0,
                patch: 0,
            });
        };
        let Ok(major) = major.parse::<isize>() else {
            return Err(VersionError);
        };
        if let Some((minor, patch)) = rest.split_once('.') {
            let Ok(minor) = minor.parse::<isize>() else {
                return Err(VersionError);
            };
            let Ok(patch) = patch.parse::<isize>() else {
                return Err(VersionError);
            };
            return Ok(Self {
                major,
                minor,
                patch,
            });
        };
        let Ok(minor) = rest.parse::<isize>() else {
            return Err(VersionError);
        };
        Ok(Self {
            major,
            minor,
            patch: 0,
        })
    }
}

#[inline]
pub fn macos_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "macos"))]
    return false;
    #[cfg(target_os = "macos")]
    OsVersion::from_str(_ver).unwrap().at_least()
}

#[inline]
pub fn ios_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "ios"))]
    return false;
    #[cfg(target_os = "ios")]
    OsVersion::from_str(_ver).unwrap().at_least()
}

#[inline]
pub fn tvos_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "tvos"))]
    return false;
    #[cfg(target_os = "tvos")]
    OsVersion::from_str(_ver).unwrap().at_least()
}

#[inline]
pub fn watchos_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "watchos"))]
    return false;
    #[cfg(target_os = "watchos")]
    OsVersion::from_str(_ver).unwrap().at_least()
}

#[inline]
pub fn visionos_available(_ver: &str) -> bool {
    #[cfg(not(target_os = "visionos"))]
    return false;
    #[cfg(target_os = "visionos")]
    OsVersion::from_str(_ver).unwrap().at_least()
}

#[inline]
pub fn maccatalyst_available(_ver: &str) -> bool {
    #[cfg(not(all(target_os = "ios", target_abi = "macabi")))]
    return false;
    #[cfg(all(target_os = "ios", target_abi = "macabi"))]
    OsVersion::from_str(_ver).unwrap().at_least()
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
        $(, maccatalyst = $maccatalyst_ver:literal)?
        $(, tvos = $tvos_ver:literal)?
        $(, watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
    ) => {
        $($crate::api::version!(macos = $macos_ver))?
        $( || $crate::api::version!(ios = $ios_ver))?
        $( || $crate::api::version!(maccatalyst = $maccatalyst_ver))?
        $( || $crate::api::version!(tvos = $tvos_ver))?
        $( || $crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
    };
    (
        $(ios = $ios_ver:literal)?
        $(, maccatalyst = $maccatalyst_ver:literal)?
        $(, tvos = $tvos_ver:literal)?
        $(, watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
    ) => {
        $($crate::api::version!(ios = $ios_ver))?
        $( || $crate::api::version!(maccatalyst = $maccatalyst_ver))?
        $( || $crate::api::version!(tvos = $tvos_ver))?
        $( || $crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
    };
    (
        $(maccatalyst = $maccatalyst_ver:literal)?
        $(, tvos = $tvos_ver:literal)?
        $(, watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
    ) => {
        $($crate::api::version!(maccatalyst = $maccatalyst_ver))?
        $( || $crate::api::version!(tvos = $tvos_ver))?
        $( || $crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
    };
    (
        $(tvos = $tvos_ver:literal)?
        $(, watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
    ) => {
        $($crate::api::version!(tvos = $tvos_ver))?
        $( || $crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
    };
    (
        $(watchos = $watchos_ver:literal)?
        $(, visionos = $visionos_ver:literal)?
    ) => {
        $($crate::api::version!(watchos = $watchos_ver))?
        $( || $crate::api::version!(visionos = $visionos_ver))?
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
        assert!(NOT_FOUND.get_var().is_none());
        assert!(NOT_FOUND.get_var().is_none());
        assert!(SHOULD_BE_FOUND.get_var().is_some());
        assert!(SHOULD_BE_FOUND.get_var().unwrap().len() > 0);
        assert!(SHOULD_BE_FOUND.get_var().unwrap().len() > 0);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn version() {
        assert!(api::macos_available("13.0"));
        assert!(api::macos_available("13.1.1"));
        assert!(api::version!(macos = 13.0, ios = 14.0));
        assert!(!api::version!(visionos = 1.0));
        assert!(!api::version!(macos = 20.0));
    }

    #[cfg(target_os = "ios")]
    #[test]
    fn version() {
        assert!(api::ios_available("18.0"));
        assert!(api::ios_available("18.2"));
        assert!(api::version!(macos = 13.0, ios = 14.0));
        assert!(!api::version!(visionos = 1.0));
        assert!(!api::version!(macos = 20.0));
    }
}
