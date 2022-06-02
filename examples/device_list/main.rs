use cidre::am;
fn main() {
    let devices = am::Device::list().unwrap();

    for d in devices.iter() {
        let f = d.connected().expect("connected");
        println!(
            "uuid: {:?}, if: {:?} {:?}",
            f.identifier().to_string(),
            f.interface_type(),
            f.name().to_string()
        );

        let s = f.start_session().expect("started session");
        s.show();
    }
}
