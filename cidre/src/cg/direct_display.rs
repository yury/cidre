#[cfg(all(target_os = "macos", feature = "mtl"))]
use crate::{arc, mtl};

#[cfg(target_os = "macos")]
use crate::cg;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "CGDirectDisplayID")]
#[repr(transparent)]
pub struct Id(pub u32);

impl Id {
    #[doc(alias = "kCGNullDirectDisplay")]
    pub const NULL: Self = Self(0);
}

#[cfg(target_os = "macos")]
impl Id {
    pub fn main() -> Id {
        unsafe { CGMainDisplayID() }
    }

    #[doc(alias = "CGDirectDisplayCopyCurrentMetalDevice")]
    #[cfg(feature = "mtl")]
    pub fn current_mtl_device(self) -> Option<arc::R<mtl::Device>> {
        unsafe { CGDirectDisplayCopyCurrentMetalDevice(self) }
    }

    /// Return the screen size and screen origin of `display' in global
    /// coordinates, or `cg::Rect::zero()' if `display' is invalid.
    #[inline]
    pub fn bounds(self) -> cg::Rect {
        unsafe { CGDisplayBounds(self) }
    }

    /// Return the width in pixels of `display'
    #[inline]
    pub fn pixels_wide(self) -> usize {
        unsafe { CGDisplayPixelsWide(self) }
    }

    /// Return the height in pixels of `display'
    #[inline]
    pub fn pixels_high(self) -> usize {
        unsafe { CGDisplayPixelsHigh(self) }
    }
}

pub type RefreshRate = f64;

#[cfg(target_os = "macos")]
unsafe extern "C-unwind" {
    fn CGMainDisplayID() -> Id;

    #[cfg(feature = "mtl")]
    fn CGDirectDisplayCopyCurrentMetalDevice(display: Id) -> Option<arc::R<mtl::Device>>;

    fn CGDisplayBounds(display: Id) -> cg::Rect;
    fn CGDisplayPixelsWide(display: Id) -> usize;
    fn CGDisplayPixelsHigh(display: Id) -> usize;
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::Id;

    #[test]
    fn basics() {
        let display = Id::main();
        let _device = display.current_mtl_device().expect("Failed to get device");
        let bounds = display.bounds();
        assert!(bounds.size.width > 0.0);
        assert!(bounds.size.height > 0.0);
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
