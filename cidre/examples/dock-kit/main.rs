// Run this from the Mac and cargo-box will install and launch it on the iPhone.
// Press Ctrl-C in this terminal to stop it:
//
// BOX_ORG_ID=org.cidre.dockkit DEVELOPMENT_TEAM=<TEAM_ID> DEVICE_ID=<DEVICE_ID> \
//   cargo run -p cidre --target aarch64-apple-ios --example dock-kit --no-default-features --features dk
//
// Useful setup commands:
//   cargo box teams
//   cargo box devices

use std::time::Duration;

fn main() {
    println!("dock-kit: starting");
    println!("dock-kit: press Ctrl-C to stop");

    let manager = cidre::dk::AccessoryManager::shared();
    let accessory_state_changes_available = manager.accessory_state_changes().is_ok();
    if let Ok(state_changes) = manager.accessory_state_changes() {
        state_changes.next(|change| {
            print_state_change(change);
            std::thread::sleep(Duration::from_millis(250));
        });
    }

    let mut tick = 0u64;
    loop {
        print_status(tick, &manager, accessory_state_changes_available);
        print_observations(tick);
        tick += 1;
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn print_state_change(change: Option<cidre::dk::StateChange>) {
    match change {
        Some(change) => {
            println!("state change:");
            println!("  state hash={}", change.state.hash_value());
            println!(
                "  tracking_button_enabled={}",
                change.tracking_button_enabled
            );
        }
        None => println!("state changes ended"),
    }
}

fn print_status(
    tick: u64,
    manager: &cidre::swift::Object<cidre::dk::AccessoryManager>,
    accessory_state_changes_available: bool,
) {
    println!("status tick={tick}:");
    println!(
        "  system_tracking_enabled={}",
        manager.is_system_tracking_enabled()
    );
    println!(
        "  accessory_state_changes={}",
        if accessory_state_changes_available {
            "available"
        } else {
            "unavailable"
        }
    );

    let docked = cidre::dk::State::docked();
    let undocked = cidre::dk::State::undocked();
    println!(
        "  state.docked hash={} debug_count={}",
        docked.hash_value(),
        docked.debug_desc().count()
    );
    println!(
        "  state.undocked hash={} debug_count={}",
        undocked.hash_value(),
        undocked.debug_desc().count()
    );
}

fn print_observations(tick: u64) {
    println!("observations tick={tick}:");
    print_observation("human_face", cidre::dk::ObservationType::human_face());
    print_observation("human_body", cidre::dk::ObservationType::human_body());
    print_observation("object", cidre::dk::ObservationType::object());
}

fn print_observation(name: &str, ty: cidre::dk::ObservationType) {
    println!("  type.{name} hash={}", ty.hash_value());
}
