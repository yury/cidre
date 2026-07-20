use crate::{dk::StateChanges, swift, swift::abi};

#[repr(C)]
pub struct AccessoryManager {
    _priv: [u8; 0],
}

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0A16AccessoryManagerCMa"]
    fn dock_accessory_manager_metadata();

    #[link_name = "$s7DockKit0A16AccessoryManagerC6sharedACvgZ"]
    fn dock_accessory_manager_shared();

    #[link_name = "$s7DockKit0A16AccessoryManagerC23isSystemTrackingEnabledSbvgTj"]
    fn dock_accessory_manager_is_system_tracking_enabled();

    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesVMa"]
    fn dock_accessory_state_changes_metadata();

    #[link_name = "$s7DockKit0A16AccessoryManagerC21accessoryStateChangesAA0aC0C0fG0VvgTj"]
    fn dock_accessory_manager_accessory_state_changes();
}

impl AccessoryManager {
    /// DockKit `DockAccessoryManager.shared`.
    #[doc(alias = "DockAccessoryManager.shared")]
    #[inline]
    pub fn shared() -> swift::Object<Self> {
        unsafe {
            // Swift emits a class metadata access before calling this static getter.
            let metadata =
                abi::call_int_to_int(dock_accessory_manager_metadata as *const (), 0) as *const ();
            swift::Object::from_raw(
                abi::call_static0_object(dock_accessory_manager_shared as *const (), metadata)
                    .cast(),
            )
        }
    }
}

impl swift::Object<AccessoryManager> {
    #[doc(alias = "DockAccessoryManager.isSystemTrackingEnabled")]
    #[inline]
    pub fn is_system_tracking_enabled(&self) -> bool {
        unsafe {
            abi::call_object_to_bool(
                dock_accessory_manager_is_system_tracking_enabled as *const (),
                self.as_raw().cast_const().cast(),
            )
        }
    }

    #[doc(alias = "DockAccessoryManager.accessoryStateChanges")]
    #[inline]
    pub fn accessory_state_changes(&self) -> Result<StateChanges, ()> {
        unsafe {
            let metadata =
                abi::call_int_to_int(dock_accessory_state_changes_metadata as *const (), 0)
                    as *const abi::TypeMetadata;
            let mut value = StateChanges::alloc(metadata);
            let error = abi::call_object_to_throwing_value(
                dock_accessory_manager_accessory_state_changes as *const (),
                self.as_raw().cast_const().cast(),
                value.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(value)
            } else {
                value.dealloc_uninit();
                abi::error_release(error.cast_const());
                Err(())
            }
        }
    }
}
