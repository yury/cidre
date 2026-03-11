use crate::{api, arc, define_cls, define_obj_type, gc, ns, objc};

define_obj_type!(
    #[doc(alias = "GCKeyboard")]
    pub Keyboard(gc::Device)
);

impl Keyboard {
    define_cls!(GC_KEYBOARD);

    #[objc::msg_send(keyboardInput)]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn keyboard_input(&self) -> Option<arc::R<gc::KeyboardInput>>;

    #[objc::msg_send(coalescedKeyboard)]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn coalesced() -> Option<arc::R<Self>>;
}

impl Keyboard {
    #[doc(alias = "GCKeyboardDidConnectNotification")]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn did_connect_notification() -> &'static ns::NotificationName {
        unsafe { GCKeyboardDidConnectNotification }
    }

    #[doc(alias = "GCKeyboardDidDisconnectNotification")]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    pub fn did_disconnect_notification() -> &'static ns::NotificationName {
        unsafe { GCKeyboardDidDisconnectNotification }
    }
}

unsafe extern "C" {
    static GC_KEYBOARD: &'static objc::Class<Keyboard>;
}

#[api::weak]
unsafe extern "C" {
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    static GCKeyboardDidConnectNotification: &'static ns::NotificationName;
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0)]
    static GCKeyboardDidDisconnectNotification: &'static ns::NotificationName;
}
