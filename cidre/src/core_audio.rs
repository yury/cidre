mod hardware_base;
pub use hardware_base::hardware_err;
pub use hardware_base::*;

pub mod hardware;

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
pub use hardware::Stream;
pub use hardware::StreamDir;
pub use hardware::System;
pub use hardware::aggregate_device_keys;
pub use hardware::device_start;
pub use hardware::sub_device_keys;

mod tap_description;
pub use tap_description::TapDesc;
pub use tap_description::TapMuteBehavior;

pub mod hardware_tapping;
pub use hardware_tapping::Tap;
#[cfg(feature = "macos_14_2")]
pub use hardware_tapping::TapGuard;

mod host_time;
pub use host_time::current_host_time;
pub use host_time::host_clock_frequency;
pub use host_time::host_clock_min_time_delta;
pub use host_time::host_time_to_nanos;
pub use host_time::nanos_to_host_time;

#[link(name = "CoreAudio", kind = "framework")]
unsafe extern "C" {}

#[link(name = "core_audio", kind = "static")]
unsafe extern "C" {}
