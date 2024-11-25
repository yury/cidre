mod hardware_base;
pub use hardware_base::*;

pub mod hardware;

pub use hardware::aggregate_device_keys;
pub use hardware::AggregateDevice;
pub use hardware::Device;
#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub use hardware::DeviceIoBlock;
pub use hardware::DeviceIoProc;
pub use hardware::DeviceIoProcId;
pub use hardware::Process;
#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub use hardware::PropListenerBlock;
pub use hardware::PropListenerFn;
pub use hardware::System;

mod tap_description;
pub use tap_description::TapDesc;
pub use tap_description::TapMuteBehavior;

#[cfg(feature = "macos_14_2")]
pub mod hardware_tapping;
