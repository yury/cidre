use crate::{api, arc, blocks, define_cls, define_obj_type, gc, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum PlayerIndex {
    Unset = -1,
    _1 = 0,
    _2 = 1,
    _3 = 2,
    _4 = 3,
}

define_obj_type!(
    #[doc(alias = "GCController")]
    pub Controller(gc::Device)
);

impl Controller {
    define_cls!(GC_CONTROLLER);

    #[objc::msg_send(current)]
    pub fn current() -> Option<arc::R<Controller>>;

    /// Get a list of controllers currently attached to the system.
    #[objc::msg_send(controllers)]
    pub fn controllers() -> arc::R<ns::Array<Self>>;

    #[objc::msg_send(playerIndex)]
    pub fn player_index(&self) -> PlayerIndex;

    #[objc::msg_send(setPlayerIndex:)]
    pub fn set_player_index(&mut self, val: PlayerIndex) -> PlayerIndex;

    #[objc::msg_send(light)]
    pub fn light(&self) -> Option<arc::R<gc::DeviceLight>>;

    #[objc::msg_send(capture)]
    pub fn capture(&self) -> arc::R<Self>;

    #[objc::msg_send(shouldMonitorBackgroundEvents)]
    #[api::available(macos = 11.3, ios = 14.5, tvos = 14.5)]
    pub fn should_monitor_background_events() -> bool;

    #[objc::msg_send(setShouldMonitorBackgroundEvents:)]
    #[api::available(macos = 11.3, ios = 14.5, tvos = 14.5)]
    pub fn set_should_monitor_background_events(val: bool);

    #[objc::msg_send(startWirelessControllerDiscoveryWithCompletionHandler:)]
    pub fn start_wireless_discovery_ch(ch: Option<&mut blocks::CompletionBlock>);

    pub fn start_wireless_discovery(ch: impl FnMut() + 'static) {
        let mut block = blocks::CompletionBlock::new0(ch);
        Self::start_wireless_discovery_ch(Some(&mut block));
    }

    #[objc::msg_send(stopWirelessControllerDiscovery)]
    pub fn stop_wireless_discovery();
}

/// Notifications
impl Controller {
    #[doc(alias = "GCControllerDidConnectNotification")]
    #[api::available(macos = 10.9, ios = 7.0, tvos = 7.0)]
    pub fn did_connect_notification() -> &'static ns::NotificationName {
        unsafe { GCControllerDidConnectNotification }
    }

    #[doc(alias = "GCControllerDidDisconnectNotification")]
    #[api::available(macos = 10.9, ios = 7.0, tvos = 7.0)]
    pub fn did_disconnect_notification() -> &'static ns::NotificationName {
        unsafe { GCControllerDidDisconnectNotification }
    }

    #[doc(alias = "GCControllerDidBecomeCurrentNotification")]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn did_become_current_notification() -> &'static ns::NotificationName {
        unsafe { GCControllerDidBecomeCurrentNotification }
    }

    #[doc(alias = "GCControllerDidStopBeingCurrentNotification")]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn stop_being_current_notification() -> &'static ns::NotificationName {
        unsafe { GCControllerDidStopBeingCurrentNotification }
    }

    #[doc(alias = "GCControllerUserCustomizationsDidChangeNotification")]
    #[api::available(macos = 13.0, ios = 16.0, tvos = 16.0)]
    pub fn user_customizations_did_change_notification() -> &'static ns::NotificationName {
        unsafe { GCControllerUserCustomizationsDidChangeNotification }
    }
}

#[link(name = "gc", kind = "static")]
unsafe extern "C" {
    static GC_CONTROLLER: &'static objc::Class<Controller>;
}

#[link(name = "GameController", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    #[api::available(macos = 10.9, ios = 7.0, tvos = 7.0)]
    static GCControllerDidConnectNotification: &'static ns::NotificationName;
    #[api::available(macos = 10.9, ios = 7.0, tvos = 7.0)]
    static GCControllerDidDisconnectNotification: &'static ns::NotificationName;

    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    static GCControllerDidBecomeCurrentNotification: &'static ns::NotificationName;
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    static GCControllerDidStopBeingCurrentNotification: &'static ns::NotificationName;

    #[api::available(macos = 13.0, ios = 16.0, tvos = 16.0)]
    static GCControllerUserCustomizationsDidChangeNotification: &'static ns::NotificationName;
}
