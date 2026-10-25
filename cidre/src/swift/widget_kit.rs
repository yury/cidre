//! WidgetKit.framework Swift ABI bindings.

mod control_center;

pub use control_center::ControlCenter;

#[link(name = "WidgetKit", kind = "framework")]
unsafe extern "C" {}
