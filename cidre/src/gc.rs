mod color;
pub use color::Color;

mod controller_element;
pub use controller_element::ControllerElement;

mod controller_button_input;
pub use controller_button_input::ControllerButtonInput;

mod device;
pub use device::Device;

mod device_light;
pub use device_light::DeviceLight;

mod key_codes;
pub use key_codes::KeyCode;

mod physical_input_profile;
pub use physical_input_profile::DeviceButtonInput;
pub use physical_input_profile::DeviceElement;
pub use physical_input_profile::PhysicalInputProfile;

mod keyboard_input;
pub use keyboard_input::KeyboardInput;
#[cfg(feature = "blocks")]
pub use keyboard_input::ValueChangedHandler as KeyboardValueChangedHandler;

mod keyboard;
pub use keyboard::Keyboard;

mod controller;
pub use controller::Controller;
pub use controller::PlayerIndex as ControllerPlayerIndex;

#[link(name = "GameController", kind = "framework")]
unsafe extern "C" {}

#[link(name = "gc", kind = "static")]
unsafe extern "C" {}
