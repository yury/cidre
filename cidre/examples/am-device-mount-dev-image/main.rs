#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::c_void;

    use cidre::{
        am::{self, device::discovery::NotificationInfo},
        cf,
    };

    const DEVICE_ID: &str = "00008120-000168A20210C01E";

    extern "C" fn _callback(info: &NotificationInfo, _context: *mut c_void) {
        match info.safe() {
            am::device::discovery::SafeInfo::Attached(device) => {
                let id = device.id().to_string();
                if id.eq(DEVICE_ID) {
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

    #[tokio::main]
    pub async fn main() {
        let devices = am::device::QueryBuilder::new_match_all()
            .udids(&[DEVICE_ID])
            // .udids(&["00008120-000168A20210C01E"])
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
        let connection = session.start_debug_server().expect("debug");

        unsafe {
            extern "C" fn cb(
                _s: &cf::Socket,
                _cb_type: cf::SocketCbType,
                _address: &cf::Data,
                _data: *const u8,
                _info: *mut c_void,
            ) {
                println!("??????????")
            }
            let sock = cf::Socket::create_with_native(
                connection.socket().unwrap(),
                cf::SocketCbType::READ,
                cb,
                None,
            )
            .unwrap();
            let source = sock.run_loop_src(0).unwrap();
            cf::RunLoop::main().add_src(&source, cf::RunLoopMode::common());
            // source.invalidate();

            let n = connection.send(&[1]).unwrap();
            println!("sent {}", n);
        }

        // println!("pre send");
        // let n = connection.send(&[0,0,0,0]).unwrap();
        // println!("post send {}", n);
        // connection.http_proxy().await.unwrap();
        // let fd = service.socket().expect("valid socket");
        // println!("FD {:?}", fd);

        // device::start_lldb_proxy(fd).await.unwrap();

        // let note = am::device::Notification::with(
        //     callback,
        //     DeviceSpeed::ANY,
        //     DeviceInterfaceConnectionType::Any,
        //     std::ptr::null_mut(),
        // )
        // .unwrap();

        cf::RunLoop::run();
    }
}

#[cfg(target_os = "macos")]
pub use macos::main;

#[cfg(not(target_os = "macos"))]
fn main() {
    todo!()
}
