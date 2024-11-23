mod hardware_base;
pub use hardware_base::*;

pub mod hardware;

#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub use hardware::AudioDeviceIoBlock;
pub use hardware::AudioDeviceIoProc;
pub use hardware::AudioDeviceIoProcId;
#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub use hardware::AudioObjPropListenerBlock;
pub use hardware::AudioObjPropListenerFn;

mod tap_description;
pub use tap_description::TapDesc;
pub use tap_description::TapMuteBehavior;

#[cfg(feature = "macos_14_2")]
pub mod hardware_tapping;
