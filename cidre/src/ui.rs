mod application;
pub use application::App;
pub use application::AppDelegate;
pub use application::AppDelegateImpl;

mod device;
pub use device::BatteryState as DeviceBatteryState;
pub use device::Device;
pub use device::Idiom;
pub use device::Orientation as DeviceOrientation;
pub use device::notifications as device_notifications;

mod geometry;
pub use geometry::EdgeInsets;
pub use geometry::Offset;

mod responder;
pub use responder::Responder;

mod view;
pub use view::AnyCoordinateSpace;
pub use view::CoordinateSpace;
pub use view::View;

mod window;
pub use window::Window;

mod view_controller;
pub use view_controller::ViewController;

mod color;
pub use color::Color;

mod image;
pub use image::Image;

mod interaction;
pub use interaction::AnyInteraction;
pub use interaction::Interaction;
pub use interaction::InteractionImpl;

mod scene_definitions;
pub use scene_definitions::SceneSessionRole;

mod scene;
pub use scene::AnySceneDelegate;
pub use scene::Scene;
pub use scene::SceneDelegate;
pub use scene::SceneDelegateImpl;

mod scene_session;
pub use scene_session::SceneCfg;
pub use scene_session::SceneSession;

mod scene_options;
pub use scene_options::SceneConnectionOpts;

mod screen;
pub use screen::Screen;

mod screen_mode;
pub use screen_mode::ScreenMode;

mod window_scene;
pub use window_scene::WindowScene;
pub use window_scene::WindowSceneDelegate;
pub use window_scene::WindowSceneDelegateImpl;

#[inline]
pub fn app_main(
    principal_class_name: Option<&crate::ns::String>,
    delegate_class_name: Option<&crate::ns::String>,
) -> std::ffi::c_int {
    // may be make them public
    unsafe extern "C" {
        fn _NSGetArgc() -> *mut std::ffi::c_int;
        fn _NSGetArgv() -> *mut *mut *mut std::ffi::c_char;
        fn UIApplicationMain(
            argc: std::ffi::c_int,
            argv: *mut *mut std::ffi::c_char,
            principal_class_name: Option<&crate::ns::String>,
            delegate_class_name: Option<&crate::ns::String>,
        ) -> std::ffi::c_int;
    }

    unsafe {
        let argc = *_NSGetArgc();
        let argv = *_NSGetArgv();
        UIApplicationMain(argc, argv, principal_class_name, delegate_class_name)
    }
}
