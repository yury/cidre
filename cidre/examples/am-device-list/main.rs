use cidre::am;
fn main() {
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
