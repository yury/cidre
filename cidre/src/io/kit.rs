pub mod audio;

pub mod _return;
pub use _return::Return;

pub mod pm;

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {}
