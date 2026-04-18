pub mod audio;

pub mod _return;
pub use _return::Return;

#[cfg(all(target_os = "macos", feature = "io_kit", feature = "io_pm"))]
pub mod pm;

pub mod hid;

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {}
