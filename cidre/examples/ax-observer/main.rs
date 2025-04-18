/// # AX Observer Example
///
/// This example demonstrates how to observe accessibility notifications from an application using macOS's
/// Accessibility API through the cidre crate.
///
/// Key features:
///
/// - Checks and requests accessibility permissions
/// - Attaches to a target application by PID
/// - Observes application activation/deactivation events
/// - Runs in a Core Foundation run loop to receive notifications
///
use cidre::{ax, cf};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// The pid to attach to
    pid: u32,
}

extern "C" fn observer_cb(
    _observer: &mut ax::Observer,
    _elem: &mut ax::UiElement,
    notification: &ax::Notification,
    _context: *mut std::ffi::c_void,
) {
    println!("{:?}", notification);
}

fn main() {
    if !ax::is_process_trusted_with_prompt(true) {
        println!("Not trusted");
        return;
    }

    let args = Args::parse();
    let app = ax::UiElement::with_app_pid(args.pid as i32);

    let mut observer = ax::Observer::with_cb(args.pid as i32, observer_cb).unwrap();
    observer
        .add_notification(
            &app,
            ax::notification::app_activated(),
            std::ptr::null_mut(),
        )
        .unwrap();

    observer
        .add_notification(
            &app,
            ax::notification::app_deactivated(),
            std::ptr::null_mut(),
        )
        .unwrap();

    cf::RunLoop::current().add_src(observer.run_loop_src(), cf::RunLoopMode::default());

    cf::RunLoop::run();
}
