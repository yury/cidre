mod color;
pub use color::Color;

mod device;
pub use device::Device;

mod device_light;
pub use device_light::DeviceLight;

mod controller;
pub use controller::Controller;
pub use controller::PlayerIndex as ControllerPlayerIndex;

#[link(name = "GameController", kind = "framework")]
unsafe extern "C" {}

#[link(name = "gc", kind = "static")]
unsafe extern "C" {}
