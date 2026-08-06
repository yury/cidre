use crate::{
    api, arc, ns, swift,
    swift::dock_kit::StateChanges,
    swift::concurrency,
};

crate::define_swift!(
    #[swift::class("DockKit.DockAccessoryManager")]
    pub AccessoryManager
);

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0A16AccessoryManagerC24setSystemTrackingEnabledyySbYaKFTj"]
    fn dock_accessory_manager_set_system_tracking_enabled();

    #[link_name = "$s7DockKit0A16AccessoryManagerC24setSystemTrackingEnabledyySbYaKFTjTu"]
    static SET_SYSTEM_TRACKING_ENABLED_ASYNC_FN: u8;
}

impl AccessoryManager {
    /// DockKit `DockAccessoryManager.shared`.
    ///
    /// Swift emits a class metadata access before calling a static member, and
    /// the generated call does the same.
    #[swift::call(sym = "$s7DockKit0A16AccessoryManagerC6sharedACvgZ")]
    pub fn shared() -> arc::R<Self>;

    #[swift::call("DockKit.DockAccessoryManager(class).isSystemTrackingEnabled: Bool { get } thunk")]
    pub fn is_system_tracking_enabled(&self) -> bool;

    /// Bridging returns the error box itself, so the reference the call gets
    /// back becomes the `ns::Error`'s and is not released again.
    #[swift::call(sym = "$s7DockKit0A16AccessoryManagerC21accessoryStateChangesAA0aC0C0fG0VvgTj")]
    pub fn accessory_state_changes(&self) -> Result<StateChanges, arc::R<ns::Error>>;

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
