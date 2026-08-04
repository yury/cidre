// Run this from the Mac and cargo-box will install and launch it on the iPhone.
// Press Ctrl-C in this terminal to stop it:
//
// BOX_ORG_ID=org.cidre.dockkit DEVELOPMENT_TEAM=<TEAM_ID> DEVICE_ID=<DEVICE_ID> \
//   cargo run -p cidre --target aarch64-apple-ios --example dock-kit \
//     --no-default-features --features dk,ios_18_0
//
// Useful setup commands:
//   cargo box teams
//   cargo box devices

use std::time::Duration;

fn main() {
    println!("dock-kit: starting");
    println!("dock-kit: press Ctrl-C to stop");

    let manager = cidre::swift::dock_kit::AccessoryManager::shared();
    let accessory_state_changes_available = manager.accessory_state_changes().is_ok();
    if let Ok(state_changes) = manager.accessory_state_changes() {
        state_changes.for_each(|change| {
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

fn print_state_change(change: Option<cidre::swift::dock_kit::StateChange>) {
    match change {
        Some(change) => {
            println!("state change:");
            println!("  state hash={}", change.state.hash_value());
            println!(
                "  tracking_button_enabled={}",
                change.tracking_button_enabled
            );
            if let Some(accessory) = change.accessory {
                print_accessory(&accessory);
                listen_to_accessory(&accessory);
            }
        }
        None => println!("state changes ended"),
    }
}

fn print_accessory(accessory: &cidre::swift::dock_kit::Accessory) {
    let identifier = accessory.identifier();
    println!("  accessory={}", identifier.debug_desc());
    println!("  identifier.name={}", identifier.name());
    println!("  identifier.uuid={}", identifier.uuid());
    println!("  firmware={:?}", accessory.firmware_version());
    println!("  hardware={:?}", accessory.hardware_model());
    println!("  framing_mode={}", accessory.framing_mode().hash_value());
    println!("  region_of_interest={:?}", accessory.region_of_interest());

    if let Ok(limits) = accessory.limits() {
        print_limit("yaw", limits.yaw());
        print_limit("pitch", limits.pitch());
        print_limit("roll", limits.roll());
    }
}

fn print_limit(name: &str, limit: Option<cidre::swift::dock_kit::Limit>) {
    if let Some(limit) = limit {
        println!(
            "  limit.{name} range={:?} maximum_speed={}",
            limit.position_range(),
            limit.maximum_speed()
        );
    }
}

fn listen_to_accessory(accessory: &cidre::swift::dock_kit::Accessory) {
    if let Ok(states) = accessory.motion_states() {
        states.for_each(|state| match state {
            Some(state) => println!(
                "motion t={} position={:?} velocity={:?} error={:?}",
                state.timestamp(),
                state.angular_positions(),
                state.angular_velocities(),
                state
                    .error()
                    .map(|error| error.localized_desc().to_string())
            ),
            None => println!("motion states ended"),
        });
    }

    if let Ok(events) = accessory.accessory_events() {
        events.for_each(|event| match event {
            Some(event) => println!("accessory event: {event:?}"),
            None => println!("accessory events ended"),
        });
    }

    if let Ok(states) = accessory.battery_states() {
        states.for_each(|state| match state {
            Some(state) => println!(
                "battery {} level={} low={} charge={}",
                state.name(),
                state.battery_level(),
                state.is_low_battery(),
                state.charge_state().hash_value()
            ),
            None => println!("battery states ended"),
        });
    }

    if let Ok(states) = accessory.tracking_states() {
        states.for_each(|state| match state {
            Some(state) => {
                println!("tracking: {}", state.description());
                for subject in state.tracked_subjects().iter() {
                    match subject {
                        cidre::swift::dock_kit::TrackedSubject::Person(person) => println!(
                            "  person {} rect={:?} rank={:?}",
                            person.identifier(),
                            person.rect(),
                            person.saliency_rank()
                        ),
                        cidre::swift::dock_kit::TrackedSubject::Object(object) => println!(
                            "  object {} rect={:?} rank={:?}",
                            object.identifier(),
                            object.rect(),
                            object.saliency_rank()
                        ),
                        cidre::swift::dock_kit::TrackedSubject::Unknown(tag) => {
                            println!("  unknown tracked subject tag={tag}")
                        }
                    }
                }
            }
            None => println!("tracking states ended"),
        });
    }
}

fn print_status(
    tick: u64,
    manager: &cidre::swift::dock_kit::AccessoryManager,
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

    let docked = cidre::swift::dock_kit::State::docked();
    let undocked = cidre::swift::dock_kit::State::undocked();
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
    print_observation(
        "human_face",
        cidre::swift::dock_kit::ObservationType::human_face(),
    );
    print_observation(
        "human_body",
        cidre::swift::dock_kit::ObservationType::human_body(),
    );
    print_observation("object", cidre::swift::dock_kit::ObservationType::object());
}

fn print_observation(name: &str, ty: cidre::swift::dock_kit::ObservationType) {
    println!("  type.{name} hash={}", ty.hash_value());
}
