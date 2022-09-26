pub type ID = u32;

pub type RefreshRate = f64;

pub const NULL: ID = 0;

#[cfg(target_os = "macos")]
pub fn main_display_id() -> ID {
    unsafe { CGMainDisplayID() }
}

extern "C" {
    #[cfg(target_os = "macos")]
    fn CGMainDisplayID() -> ID;
}

// typedef uint32_t CGDirectDisplayID;
// typedef uint32_t CGOpenGLDisplayMask;
// typedef double CGRefreshRate;

// typedef struct CF_BRIDGED_TYPE(id) CGDisplayMode *CGDisplayModeRef;

// #define kCGNullDirectDisplay ((CGDirectDisplayID)0)
// #define kCGDirectMainDisplay CGMainDisplayID()

// CF_IMPLICIT_BRIDGING_ENABLED

// CF_ASSUME_NONNULL_BEGIN

// /* Return the display ID of the current main display. */
// CG_EXTERN CGDirectDisplayID CGMainDisplayID(void)
//     CG_AVAILABLE_STARTING(10.2);
