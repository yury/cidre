/// # Workspace Notifications Example
///
/// This example demonstrates how to observe macOS workspace notifications using the cidre library.
/// It registers observers for various application events (hiding, launching, activating, and deactivating)
/// and prints information about these events when they occur.
///
use cidre::{blocks, ns, objc::Obj};

fn main() {
    let block = |n: &ns::Notification| {
        // let workspace = n.id().try_cast(ns::Workspace::cls()).unwrap();
        println!("{:?}", n.name());
        let user_info = n.user_info().unwrap();

        if let Some(app) = user_info.get(ns::workspace::notification::app_key()) {
            if let Some(app) = app.try_cast(ns::RunningApp::cls()) {
                println!("{app:?}");
            }
        }

        let name = n.name().as_ref() as *const _;
        use ns::workspace::notification as names;
        match () {
            _ if name == names::did_activate_app() as *const _ => {
                println!("activating!");
            }
            _ if name == names::did_deactivate_app() as *const _ => {
                println!("deactivating!");
            }
            _ => {
                panic!("unknwon event");
            }
        }
    };

    // One block for all notifications
    let mut block = blocks::SyncBlock::new1(block);

    let notifications = [
        ns::workspace::notification::did_hide_app(),
        ns::workspace::notification::did_launch_app(),
        ns::workspace::notification::did_activate_app(),
        ns::workspace::notification::did_deactivate_app(),
    ];
    let mut observers = Vec::with_capacity(notifications.len());

    let mut nc = ns::Workspace::shared().notification_center();

    for name in notifications {
        observers.push(nc.add_observer_block(name, None, None, &mut block));
    }

    ns::App::shared().run();
}
