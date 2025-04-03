use cidre::{blocks, ns, objc::Obj};

fn main() {
    let block = |n: &ns::Notification| {
        println!("{:?}", n.name());
        let user_info = n.user_info().unwrap();
        if let Some(app) = user_info.get(ns::workspace::notification::app_key()) {
            if let Some(app) = app.try_cast(ns::RunningApp::cls()) {
                println!("{app:?}");
            }
        }
    };

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
