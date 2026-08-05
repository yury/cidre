//! DockKit.framework Swift ABI bindings.

mod accessory;
mod accessory_manager;
pub mod err;

pub use accessory::{
    Accessory, AccessoryEvent, AccessoryEvents, Animation, BatteryChargeState, BatteryState,
    BatteryStates, CameraOrientation, Category, FramingMode, Identifier, Limit, Limits,
    MotionState, MotionStates, Observation, ObservationType, Observations, State, StateChange,
    StateChanges, TrackedObject, TrackedPerson, TrackedSubject, TrackedSubjects, TrackingState,
    TrackingStates,
};
#[cfg(feature = "av")]
pub use accessory::{CameraInformation, CameraIntrinsics};
pub use accessory_manager::AccessoryManager;

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_cases_call_dock_kit_symbols() {
        let state = State::docked();
        assert!(state.debug_desc().count() > 0);

        let category = Category::tracking_stand();
        assert!(category.debug_desc().count() > 0);

        let _ = CameraOrientation::portrait().hash_value();
        let _ = ObservationType::human_face().hash_value();
        let _ = BatteryChargeState::charging().hash_value();
        let _ = FramingMode::automatic().hash_value();
        let _ = Animation::wakeup().hash_value();
    }

    /// Completes the manager's surface: the async setter round-trips through a
    /// task and reports whatever DockKit throws.
    #[test]
    fn set_system_tracking_enabled_reports_its_result() {
        use std::sync::mpsc;

        let manager = AccessoryManager::shared();
        let before = manager.is_system_tracking_enabled();

        let (tx, rx) = mpsc::channel();
        manager.set_system_tracking_enabled_handler(before, move |res| {
            let _ = tx.send(res.map_err(|e| (e.code(), e.localized_desc().to_string())));
        });

        match rx
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("tracking callback")
        {
            Ok(()) => println!("system tracking set to {before}"),
            Err((code, desc)) => println!("refused: code {code} :: {desc}"),
        }
    }

    #[test]
    fn manager_shared_call_uses_swift_class_abi() {
        let manager = AccessoryManager::shared();
        let retained = manager.clone();
        drop(manager);
        let _ = retained.is_system_tracking_enabled();
    }

    /// `accessoryStateChanges` throws without a dock accessory, which is the
    /// only way to see what the bridged `DockKitError` actually looks like.
    #[test]
    fn accessory_state_changes_surfaces_the_thrown_error() {
        let manager = AccessoryManager::shared();
        match manager.accessory_state_changes() {
            Ok(_) => println!("a dock accessory is present; no error to inspect"),
            Err(err) => {
                println!(
                    "domain {:?} code {} :: {}",
                    err.domain(),
                    err.code(),
                    err.localized_desc()
                );
                assert_eq!(err::domain(), err.domain().as_ref());
            }
        }
    }
}
