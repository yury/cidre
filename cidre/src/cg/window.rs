use crate::define_opts;

// typedef uint32_t CGWindowID;
pub type Id = u32;

define_opts!(
    #[doc(alias = "CGWindowListOption")]
    pub ListOpt(u32)
);

impl ListOpt {
    #[doc(alias = "kCGWindowListOptionAll")]
    pub const ALL: Self = Self(0);

    #[doc(alias = "kCGWindowListOptionOnScreenOnly")]
    pub const ON_SCREEN_ONLY: Self = Self(1 << 0);

    #[doc(alias = "kCGWindowListOptionOnScreenAboveWindow")]
    pub const ON_SCREEN_ABOVE_WINDOW: Self = Self(1 << 1);

    #[doc(alias = "kCGWindowListOptionOnScreenBelowWindow")]
    pub const ON_SCREEN_BELOW_WINDOW: Self = Self(1 << 2);

    #[doc(alias = "kCGWindowListOptionIncludingWindow")]
    pub const INCLUDING_WINDOW: Self = Self(1 << 3);

    #[doc(alias = "kCGWindowListExcludeDesktopElements")]
    pub const EXCLUDE_DESKTOP_ELEMENTS: Self = Self(1 << 4);
}

#[cfg(target_os = "macos")]
pub mod keys {
    use crate::cf;

    /// The window ID, a unique value within the user session representing the window.
    #[doc(alias = "kCGWindowNumber")]
    #[inline]
    pub fn number() -> &'static cf::String {
        unsafe { kCGWindowNumber }
    }

    /// The backing store type of the window.
    #[doc(alias = "kCGWindowStoreType")]
    #[inline]
    pub fn store_type() -> &'static cf::String {
        unsafe { kCGWindowStoreType }
    }

    /// The window layer number of the window.
    #[doc(alias = "kCGWindowLayer")]
    #[inline]
    pub fn layer() -> &'static cf::String {
        unsafe { kCGWindowLayer }
    }

    /// The bounds of the window in screen space.
    #[doc(alias = "kCGWindowBounds")]
    #[inline]
    pub fn bounds() -> &'static cf::String {
        unsafe { kCGWindowBounds }
    }

    /// The sharing state of the window.
    #[doc(alias = "kCGWindowSharingState")]
    #[inline]
    pub fn sharing_state() -> &'static cf::String {
        unsafe { kCGWindowSharingState }
    }

    /// The alpha fade of the window.
    #[doc(alias = "kCGWindowAlpha")]
    #[inline]
    pub fn alpha() -> &'static cf::String {
        unsafe { kCGWindowAlpha }
    }

    /// The process ID of the process that owns the window.
    #[doc(alias = "kCGWindowOwnerPID")]
    #[inline]
    pub fn owner_pid() -> &'static cf::String {
        unsafe { kCGWindowOwnerPID }
    }

    /// An estimate of the memory in bytes currently used by the window.
    #[doc(alias = "kCGWindowMemoryUsage")]
    #[inline]
    pub fn memory_usage() -> &'static cf::String {
        unsafe { kCGWindowMemoryUsage }
    }

    /// The workspace ID of the workspace associated with the window.
    #[doc(alias = "kCGWindowWorkspace")]
    #[deprecated(note = "No longer supported")]
    #[inline]
    pub fn workspace() -> &'static cf::String {
        unsafe { kCGWindowWorkspace }
    }

    /// The name of the application process which owns the window.
    #[doc(alias = "kCGWindowOwnerName")]
    #[inline]
    pub fn owner_name() -> &'static cf::String {
        unsafe { kCGWindowOwnerName }
    }

    /// The name of the window.
    #[doc(alias = "kCGWindowName")]
    #[inline]
    pub fn name() -> &'static cf::String {
        unsafe { kCGWindowName }
    }

    /// Whether the window is ordered on screen.
    #[doc(alias = "kCGWindowIsOnscreen")]
    #[inline]
    pub fn is_on_screen() -> &'static cf::String {
        unsafe { kCGWindowIsOnscreen }
    }

    /// Whether the window backing store is in video memory.
    #[doc(alias = "kCGWindowBackingLocationVideoMemory")]
    #[inline]
    pub fn backing_location_video_memory() -> &'static cf::String {
        unsafe { kCGWindowBackingLocationVideoMemory }
    }

    unsafe extern "C" {
        static kCGWindowNumber: &'static cf::String;
        static kCGWindowStoreType: &'static cf::String;
        static kCGWindowLayer: &'static cf::String;
        static kCGWindowBounds: &'static cf::String;
        static kCGWindowSharingState: &'static cf::String;
        static kCGWindowAlpha: &'static cf::String;
        static kCGWindowOwnerPID: &'static cf::String;
        static kCGWindowMemoryUsage: &'static cf::String;
        static kCGWindowWorkspace: &'static cf::String;
        static kCGWindowOwnerName: &'static cf::String;
        static kCGWindowName: &'static cf::String;
        static kCGWindowIsOnscreen: &'static cf::String;
        static kCGWindowBackingLocationVideoMemory: &'static cf::String;
    }
}
