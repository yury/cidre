use crate::{api, arc, ns, swift, swift::dock_kit::StateChanges};

crate::define_swift!(
    #[swift::class("DockKit.DockAccessoryManager")]
    pub AccessoryManager
);

impl AccessoryManager {
    /// DockKit `DockAccessoryManager.shared`.
    ///
    /// Swift emits a class metadata access before calling a static member, and
    /// the generated call does the same.
    #[swift::call(sym = "$s7DockKit0A16AccessoryManagerC6sharedACvgZ")]
    pub fn shared() -> arc::R<Self>;

    #[swift::call(
        "DockKit.DockAccessoryManager(class).isSystemTrackingEnabled: Bool { get } thunk"
    )]
    pub fn is_system_tracking_enabled(&self) -> bool;

    /// Bridging returns the error box itself, so the reference the call gets
    /// back becomes the `ns::Error`'s and is not released again.
    #[swift::call(sym = "$s7DockKit0A16AccessoryManagerC21accessoryStateChangesAA0aC0C0fG0VvgTj")]
    pub fn accessory_state_changes(&self) -> Result<StateChanges, arc::R<ns::Error>>;

    /// Turns system tracking on or off.
    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    #[swift::call(
        "DockKit.DockAccessoryManager(class).setSystemTrackingEnabled(_: Bool) async throws thunk"
    )]
    pub fn set_system_tracking_enabled(&self, enabled: bool) -> Result<(), arc::R<ns::Error>>;
}
