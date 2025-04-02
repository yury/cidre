use cidre::{ax, cf};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// The pid to attach to
    pid: u32,
}

extern "C" fn observer_cb(
    observer: &mut ax::Observer,
    elem: &mut ax::UiElement,
    notification: &ax::Notification,
    context: *mut std::ffi::c_void,
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
