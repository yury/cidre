/// # Workspace Notifications Example
///
/// This example demonstrates how to observe macOS workspace notifications using the cidre library.
/// It registers observers for various application events (hiding, launching, activating, and deactivating)
/// and prints information about these events when they occur.
///
use cidre::{blocks, ns, ns::workspace::notification as wsn, objc::Obj};

fn main() {
    let block = |n: &ns::Notification| {
        // let workspace = n.id().try_cast(ns::Workspace::cls()).unwrap();
        println!("{:?}", n.name());
        let user_info = n.user_info().unwrap();

        if let Some(app) = user_info.get(wsn::app_key()) {
            if let Some(app) = app.try_cast(ns::RunningApp::cls()) {
                println!("{app:?}");
            }
        }
    };

    // One block for all notifications
    let mut block = blocks::SyncBlock::new1(block);

    let notifications = [
        wsn::did_hide_app(),
        wsn::did_launch_app(),
        wsn::did_activate_app(),
        wsn::did_deactivate_app(),
    ];
    let mut observers = Vec::with_capacity(notifications.len());

    let mut nc = ns::Workspace::shared().notification_center();

    for name in notifications {
        observers.push(nc.add_observer_block(name, None, None, &mut block));
    }

    ns::App::shared().run();
}
