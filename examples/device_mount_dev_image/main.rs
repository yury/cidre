use std::ffi::c_void;

use cidre::{
    am::{self, device::discovery::NotificationInfo, DeviceInterfaceConnectionType, DeviceSpeed},
    cf,
};

pub extern "C" fn callback(info: &NotificationInfo, context: *mut c_void) {
    match info.safe() {
        am::device::discovery::SafeInfo::Attached(device) => {
            let id = device.identifier().to_string();
            if id.eq("00008103-001505940231001E") {
                println!("identifier {:?}", id);
                device.show();
                let connected = device.connected().expect("connected");
                println!("Device: {}", connected.name().to_string());

                let session = connected.start_session().expect("started session");
                println!(
                    "batery level {:?}",
                    session.battery_level().unwrap().to_i64()
                );
                // println!("batery level {}", session.battery_level().expect("level").to_string());
                session.mount_developer_image().expect("mounted");
                session.start_debug_server().expect("debug");
                println!("disk mounted");
            }
        }
        am::device::discovery::SafeInfo::Detached(_) => {}
        am::device::discovery::SafeInfo::NotificationStopped => {}
        am::device::discovery::SafeInfo::Paired(_) => {}
    }
}

fn main() {
    let devices = am::device::QueryBuilder::new_match_all()
        // .udids(&["00008103-001505940231001E"])
        .udids(&["00008110-00124CDE3AB8801E"])
        .matching_list(None)
        .unwrap();

    if devices.is_empty() {
        println!("device not found");
        return;
    }

    let ipad = &devices[0].retained();
    let connected = ipad.connected().expect("connected");
    println!("Device: {}", connected.name().to_string());

    let session = connected.start_session().expect("started session");
    session.mount_developer_image().expect("mounted");
    println!("disk mounted");
    let service = session.start_debug_server().expect("debug");

    println!("FD {:?}", service.socket().expect("valid socket"));

    

    // let note = am::device::Notification::with(
    //     callback,
    //     DeviceSpeed::ANY,
    //     DeviceInterfaceConnectionType::Any,
    //     std::ptr::null_mut(),
    // )
    // .unwrap();

    // cf::RunLoop::run()
}
