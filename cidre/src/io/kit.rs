pub mod audio;

pub mod _return;
pub use _return::Return;

pub mod pm;

pub mod hid;

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {}
