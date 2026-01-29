#[cfg(feature = "io_surface")]
pub mod surface;

#[cfg(feature = "io_surface")]
pub use surface::ComponentName as SurfComponentName;
#[cfg(feature = "io_surface")]
pub use surface::ComponentRange as SurfComponentRange;
#[cfg(feature = "io_surface")]
pub use surface::LockOpts as SurfLockOpts;
#[cfg(feature = "io_surface")]
pub use surface::Subsampling as SurfSubsampling;
#[cfg(feature = "io_surface")]
pub use surface::Surf;
#[cfg(feature = "io_surface")]
pub use surface::SurfId;

#[cfg(feature = "io_surface")]
#[link(name = "IOSurface", kind = "framework")]
unsafe extern "C" {}

#[cfg(all(target_os = "macos", feature = "io_kit"))]
mod kit;
#[cfg(all(target_os = "macos", feature = "io_kit"))]
pub use kit::{Return, audio};

/// Defines power management constants and keys used by both in-kernel and user space power management.
#[cfg(all(target_os = "macos", feature = "io_kit", feature = "io_pm"))]
pub use kit::pm;
