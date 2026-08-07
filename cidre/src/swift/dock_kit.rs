//! DockKit.framework Swift ABI bindings.

mod accessory;
mod accessory_manager;
pub mod err;

pub use accessory::{
    Accessory, AccessoryEvent, AccessoryEvents, Animation, BatteryChargeState, BatteryState,
    BatteryStates, CameraOrientation, Category, FramingMode, Identifier, Limit, Limits,
    MotionState, MotionStates, Observation, ObservationType, State, StateChange, StateChanges,
    TrackedObject, TrackedPerson, TrackedSubject, TrackingState, TrackingStates,
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

    /// The awaited half puts its state in the task's own allocation, which the
    /// Swift task and the future both own. Dropping the future first has to
    /// leave the task able to finish and free it, and dropping it after the
    /// call has landed has to free it too — a mistake either way is a
    /// use-after-free or a leak rather than a wrong answer.
    #[cfg(feature = "async")]
    #[test]
    fn dropping_an_awaited_call_early_leaves_the_task_able_to_finish() {
        let manager = AccessoryManager::shared();
        let before = manager.is_system_tracking_enabled();

        // Dropped without ever being polled, so the task is still on its way to
        // the callee when the future's reference goes.
        for _ in 0..200 {
            drop(manager.set_system_tracking_enabled(before));
        }

        // Dropped after a poll, which is what leaves a waker registered for a
        // wake-up that now has nobody to reach.
        for _ in 0..200 {
            let mut pending = Box::pin(manager.set_system_tracking_enabled(before));
            let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
            let _ = pending.as_mut().poll(&mut cx);
            drop(pending);
        }

        // Nothing to assert beyond surviving: a double free or a task writing
        // into a freed context shows up as a crash here.
        std::thread::sleep(std::time::Duration::from_millis(200));
        let _ = manager.is_system_tracking_enabled();
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
