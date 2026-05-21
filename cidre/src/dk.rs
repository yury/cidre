//! DockKit.framework Swift ABI bindings.

mod accessory;
mod accessory_manager;
mod error;

pub use accessory::{
    Accessory, Animation, BatteryChargeState, CameraOrientation, Category, FramingMode,
    ObservationType, State, StateChange, StateChanges, StateChangesIter,
};
pub use accessory_manager::AccessoryManager;
pub use error::Error;

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
        let _ = Error::not_supported().hash_value();
    }

    #[test]
    fn manager_shared_call_uses_swift_class_abi() {
        let manager = AccessoryManager::shared();
        let _ = manager.is_system_tracking_enabled();
    }
}
