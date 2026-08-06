use crate::{
    api, arc, ns,
    swift::dock_kit::StateChanges,
    swift::{abi, concurrency},
};

crate::define_swift_class!(pub AccessoryManager = accessor dock_accessory_manager_metadata);

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0A16AccessoryManagerCMa"]
    fn dock_accessory_manager_metadata();

    #[link_name = "$s7DockKit0A16AccessoryManagerC6sharedACvgZ"]
    fn dock_accessory_manager_shared();

    #[link_name = "$s7DockKit0A16AccessoryManagerC23isSystemTrackingEnabledSbvgTj"]
    fn dock_accessory_manager_is_system_tracking_enabled();

    #[link_name = "$s7DockKit0A16AccessoryManagerC21accessoryStateChangesAA0aC0C0fG0VvgTj"]
    fn dock_accessory_manager_accessory_state_changes();

    #[link_name = "$s7DockKit0A16AccessoryManagerC24setSystemTrackingEnabledyySbYaKFTj"]
    fn dock_accessory_manager_set_system_tracking_enabled();

    #[link_name = "$s7DockKit0A16AccessoryManagerC24setSystemTrackingEnabledyySbYaKFTjTu"]
    static SET_SYSTEM_TRACKING_ENABLED_ASYNC_FN: u8;
}

impl AccessoryManager {
    /// DockKit `DockAccessoryManager.shared`.
    #[doc(alias = "DockAccessoryManager.shared")]
    #[inline]
    pub fn shared() -> arc::R<Self> {
        unsafe {
            // Swift emits a class metadata access before calling this static getter.
            let metadata = <Self as crate::swift::SwiftMetadata>::metadata();
            arc::R::from_raw(
                abi::call_static0_object(
                    dock_accessory_manager_shared as *const (),
                    metadata.cast(),
                )
                .cast(),
            )
        }
    }

    #[doc(alias = "DockAccessoryManager.isSystemTrackingEnabled")]
    #[inline]
    pub fn is_system_tracking_enabled(&self) -> bool {
        unsafe {
            abi::call_object_to_bool(
                dock_accessory_manager_is_system_tracking_enabled as *const (),
                (self as *const Self).cast(),
            )
        }
    }

    #[doc(alias = "DockAccessoryManager.accessoryStateChanges")]
    #[inline]
    pub fn accessory_state_changes(&self) -> Result<StateChanges, arc::R<ns::Error>> {
        unsafe {
            let mut storage = StateChanges::storage();
            let error = abi::call_object_to_throwing_value(
                dock_accessory_manager_accessory_state_changes as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(StateChanges::from_storage(storage))
            } else {
                // Bridging returns the error box itself, so our reference
                // becomes the `ns::Error`'s and must not be released again.
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    /// Turns system tracking on or off.
    #[doc(alias = "DockAccessoryManager.setSystemTrackingEnabled(_:)")]
    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    pub fn set_system_tracking_enabled_handler<F>(&self, enabled: bool, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            concurrency::call_async_result(
                dock_accessory_manager_set_system_tracking_enabled as *const (),
                &raw const SET_SYSTEM_TRACKING_ENABLED_ASYNC_FN,
                // The call borrows the manager, so it is kept alive for it.
                arc::Retain::retained(self),
                |manager| {
                    concurrency::AsyncCallArgs::new()
                        .swift_self(manager.as_ptr().cast())
                        .arg(0, enabled as usize as *mut ())
                },
                |_, _| (),
                callback,
            );
        }
    }

    #[doc(alias = "DockAccessoryManager.setSystemTrackingEnabled(_:)")]
    #[cfg(feature = "async")]
    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    pub fn set_system_tracking_enabled(
        &self,
        enabled: bool,
    ) -> impl Future<Output = Result<(), arc::R<ns::Error>>> {
        concurrency::future_from(|callback| {
            self.set_system_tracking_enabled_handler(enabled, callback)
        })
    }
}
