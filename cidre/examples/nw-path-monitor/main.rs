use cidre::{dispatch, nw};

fn main() {
    println!("try turn on/off wifi or other interfaces");
    let mut monitor = nw::PathMonitor::new();
    monitor.set_update_handler(|path| {
        println!("{path:?}");
    });
    monitor.start();
    dispatch::main();
}
