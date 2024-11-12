#[cfg(feature = "mtl")]
use crate::{arc, mtl};

pub type Id = u32;

pub type RefreshRate = f64;

pub const NULL: Id = 0;

#[cfg(target_os = "macos")]
pub fn main_display_id() -> Id {
    unsafe { CGMainDisplayID() }
}

#[doc(alias = "CGDirectDisplayCopyCurrentMetalDevice")]
#[cfg(all(target_os = "macos", feature = "mtl"))]
pub fn direct_display_current_mtl_device(display: Id) -> Option<arc::R<mtl::Device>> {
    unsafe { CGDirectDisplayCopyCurrentMetalDevice(display) }
}

extern "C-unwind" {
    #[cfg(target_os = "macos")]
    fn CGMainDisplayID() -> Id;

    #[cfg(all(target_os = "macos", feature = "mtl"))]
    fn CGDirectDisplayCopyCurrentMetalDevice(display: Id) -> Option<arc::R<mtl::Device>>;
}

#[cfg(test)]
mod tests {
    use super::{direct_display_current_mtl_device, main_display_id};

    #[test]
    fn basics() {
        let display = main_display_id();
        let _device = direct_display_current_mtl_device(display).expect("Failed to get device");
    }
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
