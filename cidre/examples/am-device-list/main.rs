#[cfg(target_os = "macos")]
mod macos {
    use cidre::am;

    pub fn main() {
        let devices = am::Device::list().unwrap();

        for d in devices.iter() {
            let f = d.connected().expect("Failed to connect to device");
            println!(
                "uuid: {:?}, if: {:?} {:?}",
                f.id().to_string(),
                f.iface_type(),
                f.name().to_string()
            );

            let s = f.start_session().expect("started session");
            s.show();
        }
    }
}

#[cfg(target_os = "macos")]
pub use macos::main;

#[cfg(not(target_os = "macos"))]
fn main() {
    todo!()
}
