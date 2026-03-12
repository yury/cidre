use crate::define_opts;

#[doc(alias = "CGButtonCount")]
pub type ButtonCount = u32;

#[doc(alias = "CGWheelCount")]
pub type WheelCount = u32;

#[doc(alias = "CGCharCode")]
pub type CharCode = u16;

#[doc(alias = "CGKeyCode")]
pub type KeyCode = u16;

pub mod key_code {
    use crate::cg::KeyCode;

    // Virtual keycodes labeled for an ANSI-standard US keyboard.
    #[doc(alias = "kVK_ANSI_A")]
    pub const ANSI_A: KeyCode = 0x00;

    #[doc(alias = "kVK_ANSI_S")]
    pub const ANSI_S: KeyCode = 0x01;

    #[doc(alias = "kVK_ANSI_D")]
    pub const ANSI_D: KeyCode = 0x02;

    #[doc(alias = "kVK_ANSI_F")]
    pub const ANSI_F: KeyCode = 0x03;

    #[doc(alias = "kVK_ANSI_H")]
    pub const ANSI_H: KeyCode = 0x04;

    #[doc(alias = "kVK_ANSI_G")]
    pub const ANSI_G: KeyCode = 0x05;

    #[doc(alias = "kVK_ANSI_Z")]
    pub const ANSI_Z: KeyCode = 0x06;

    #[doc(alias = "kVK_ANSI_X")]
    pub const ANSI_X: KeyCode = 0x07;

    #[doc(alias = "kVK_ANSI_C")]
    pub const ANSI_C: KeyCode = 0x08;

    #[doc(alias = "kVK_ANSI_V")]
    pub const ANSI_V: KeyCode = 0x09;

    #[doc(alias = "kVK_ANSI_B")]
    pub const ANSI_B: KeyCode = 0x0B;

    #[doc(alias = "kVK_ANSI_Q")]
    pub const ANSI_Q: KeyCode = 0x0C;

    #[doc(alias = "kVK_ANSI_W")]
    pub const ANSI_W: KeyCode = 0x0D;

    #[doc(alias = "kVK_ANSI_E")]
    pub const ANSI_E: KeyCode = 0x0E;

    #[doc(alias = "kVK_ANSI_R")]
    pub const ANSI_R: KeyCode = 0x0F;

    #[doc(alias = "kVK_ANSI_Y")]
    pub const ANSI_Y: KeyCode = 0x10;

    #[doc(alias = "kVK_ANSI_T")]
    pub const ANSI_T: KeyCode = 0x11;

    #[doc(alias = "kVK_ANSI_1")]
    pub const ANSI_1: KeyCode = 0x12;

    #[doc(alias = "kVK_ANSI_2")]
    pub const ANSI_2: KeyCode = 0x13;

    #[doc(alias = "kVK_ANSI_3")]
    pub const ANSI_3: KeyCode = 0x14;

    #[doc(alias = "kVK_ANSI_4")]
    pub const ANSI_4: KeyCode = 0x15;

    #[doc(alias = "kVK_ANSI_6")]
    pub const ANSI_6: KeyCode = 0x16;

    #[doc(alias = "kVK_ANSI_5")]
    pub const ANSI_5: KeyCode = 0x17;

    #[doc(alias = "kVK_ANSI_Equal")]
    pub const ANSI_EQUAL: KeyCode = 0x18;

    #[doc(alias = "kVK_ANSI_9")]
    pub const ANSI_9: KeyCode = 0x19;

    #[doc(alias = "kVK_ANSI_7")]
    pub const ANSI_7: KeyCode = 0x1A;

    #[doc(alias = "kVK_ANSI_Minus")]
    pub const ANSI_MINUS: KeyCode = 0x1B;

    #[doc(alias = "kVK_ANSI_8")]
    pub const ANSI_8: KeyCode = 0x1C;

    #[doc(alias = "kVK_ANSI_0")]
    pub const ANSI_0: KeyCode = 0x1D;

    #[doc(alias = "kVK_ANSI_RightBracket")]
    pub const ANSI_RIGHT_BRACKET: KeyCode = 0x1E;

    #[doc(alias = "kVK_ANSI_O")]
    pub const ANSI_O: KeyCode = 0x1F;

    #[doc(alias = "kVK_ANSI_U")]
    pub const ANSI_U: KeyCode = 0x20;

    #[doc(alias = "kVK_ANSI_LeftBracket")]
    pub const ANSI_LEFT_BRACKET: KeyCode = 0x21;

    #[doc(alias = "kVK_ANSI_I")]
    pub const ANSI_I: KeyCode = 0x22;

    #[doc(alias = "kVK_ANSI_P")]
    pub const ANSI_P: KeyCode = 0x23;

    #[doc(alias = "kVK_ANSI_L")]
    pub const ANSI_L: KeyCode = 0x25;

    #[doc(alias = "kVK_ANSI_J")]
    pub const ANSI_J: KeyCode = 0x26;

    #[doc(alias = "kVK_ANSI_Quote")]
    pub const ANSI_QUOTE: KeyCode = 0x27;

    #[doc(alias = "kVK_ANSI_K")]
    pub const ANSI_K: KeyCode = 0x28;

    #[doc(alias = "kVK_ANSI_Semicolon")]
    pub const ANSI_SEMICOLON: KeyCode = 0x29;

    #[doc(alias = "kVK_ANSI_Backslash")]
    pub const ANSI_BACKSLASH: KeyCode = 0x2A;

    #[doc(alias = "kVK_ANSI_Comma")]
    pub const ANSI_COMMA: KeyCode = 0x2B;

    #[doc(alias = "kVK_ANSI_Slash")]
    pub const ANSI_SLASH: KeyCode = 0x2C;

    #[doc(alias = "kVK_ANSI_N")]
    pub const ANSI_N: KeyCode = 0x2D;

    #[doc(alias = "kVK_ANSI_M")]
    pub const ANSI_M: KeyCode = 0x2E;

    #[doc(alias = "kVK_ANSI_Period")]
    pub const ANSI_PERIOD: KeyCode = 0x2F;

    #[doc(alias = "kVK_ANSI_Grave")]
    pub const ANSI_GRAVE: KeyCode = 0x32;

    #[doc(alias = "kVK_ANSI_KeypadDecimal")]
    pub const ANSI_KEYPAD_DECIMAL: KeyCode = 0x41;

    #[doc(alias = "kVK_ANSI_KeypadMultiply")]
    pub const ANSI_KEYPAD_MULTIPLY: KeyCode = 0x43;

    #[doc(alias = "kVK_ANSI_KeypadPlus")]
    pub const ANSI_KEYPAD_PLUS: KeyCode = 0x45;

    #[doc(alias = "kVK_ANSI_KeypadClear")]
    pub const ANSI_KEYPAD_CLEAR: KeyCode = 0x47;

    #[doc(alias = "kVK_ANSI_KeypadDivide")]
    pub const ANSI_KEYPAD_DIVIDE: KeyCode = 0x4B;

    #[doc(alias = "kVK_ANSI_KeypadEnter")]
    pub const ANSI_KEYPAD_ENTER: KeyCode = 0x4C;

    #[doc(alias = "kVK_ANSI_KeypadMinus")]
    pub const ANSI_KEYPAD_MINUS: KeyCode = 0x4E;

    #[doc(alias = "kVK_ANSI_KeypadEquals")]
    pub const ANSI_KEYPAD_EQUALS: KeyCode = 0x51;

    #[doc(alias = "kVK_ANSI_Keypad0")]
    pub const ANSI_KEYPAD_0: KeyCode = 0x52;

    #[doc(alias = "kVK_ANSI_Keypad1")]
    pub const ANSI_KEYPAD_1: KeyCode = 0x53;

    #[doc(alias = "kVK_ANSI_Keypad2")]
    pub const ANSI_KEYPAD_2: KeyCode = 0x54;

    #[doc(alias = "kVK_ANSI_Keypad3")]
    pub const ANSI_KEYPAD_3: KeyCode = 0x55;

    #[doc(alias = "kVK_ANSI_Keypad4")]
    pub const ANSI_KEYPAD_4: KeyCode = 0x56;

    #[doc(alias = "kVK_ANSI_Keypad5")]
    pub const ANSI_KEYPAD_5: KeyCode = 0x57;

    #[doc(alias = "kVK_ANSI_Keypad6")]
    pub const ANSI_KEYPAD_6: KeyCode = 0x58;

    #[doc(alias = "kVK_ANSI_Keypad7")]
    pub const ANSI_KEYPAD_7: KeyCode = 0x59;

    #[doc(alias = "kVK_ANSI_Keypad8")]
    pub const ANSI_KEYPAD_8: KeyCode = 0x5B;

    #[doc(alias = "kVK_ANSI_Keypad9")]
    pub const ANSI_KEYPAD_9: KeyCode = 0x5C;

    // Keycodes for keys that are independent of keyboard layout.
    #[doc(alias = "kVK_Return")]
    pub const RETURN: KeyCode = 0x24;

    #[doc(alias = "kVK_Tab")]
    pub const TAB: KeyCode = 0x30;

    #[doc(alias = "kVK_Space")]
    pub const SPACE: KeyCode = 0x31;

    #[doc(alias = "kVK_Delete")]
    pub const DELETE: KeyCode = 0x33;

    #[doc(alias = "kVK_Escape")]
    pub const ESCAPE: KeyCode = 0x35;

    #[doc(alias = "kVK_RightCommand")]
    pub const RIGHT_COMMAND: KeyCode = 0x36;

    #[doc(alias = "kVK_Command")]
    pub const COMMAND: KeyCode = 0x37;

    #[doc(alias = "kVK_Shift")]
    pub const SHIFT: KeyCode = 0x38;

    #[doc(alias = "kVK_CapsLock")]
    pub const CAPS_LOCK: KeyCode = 0x39;

    #[doc(alias = "kVK_Option")]
    pub const OPTION: KeyCode = 0x3A;

    #[doc(alias = "kVK_Control")]
    pub const CONTROL: KeyCode = 0x3B;

    #[doc(alias = "kVK_RightShift")]
    pub const RIGHT_SHIFT: KeyCode = 0x3C;

    #[doc(alias = "kVK_RightOption")]
    pub const RIGHT_OPTION: KeyCode = 0x3D;

    #[doc(alias = "kVK_RightControl")]
    pub const RIGHT_CONTROL: KeyCode = 0x3E;

    #[doc(alias = "kVK_Function")]
    pub const FUNCTION: KeyCode = 0x3F;

    #[doc(alias = "kVK_F17")]
    pub const F17: KeyCode = 0x40;

    #[doc(alias = "kVK_VolumeUp")]
    pub const VOLUME_UP: KeyCode = 0x48;

    #[doc(alias = "kVK_VolumeDown")]
    pub const VOLUME_DOWN: KeyCode = 0x49;

    #[doc(alias = "kVK_Mute")]
    pub const MUTE: KeyCode = 0x4A;

    #[doc(alias = "kVK_F18")]
    pub const F18: KeyCode = 0x4F;

    #[doc(alias = "kVK_F19")]
    pub const F19: KeyCode = 0x50;

    #[doc(alias = "kVK_F20")]
    pub const F20: KeyCode = 0x5A;

    #[doc(alias = "kVK_F5")]
    pub const F5: KeyCode = 0x60;

    #[doc(alias = "kVK_F6")]
    pub const F6: KeyCode = 0x61;

    #[doc(alias = "kVK_F7")]
    pub const F7: KeyCode = 0x62;

    #[doc(alias = "kVK_F3")]
    pub const F3: KeyCode = 0x63;

    #[doc(alias = "kVK_F8")]
    pub const F8: KeyCode = 0x64;

    #[doc(alias = "kVK_F9")]
    pub const F9: KeyCode = 0x65;

    #[doc(alias = "kVK_F11")]
    pub const F11: KeyCode = 0x67;

    #[doc(alias = "kVK_F13")]
    pub const F13: KeyCode = 0x69;

    #[doc(alias = "kVK_F16")]
    pub const F16: KeyCode = 0x6A;

    #[doc(alias = "kVK_F14")]
    pub const F14: KeyCode = 0x6B;

    #[doc(alias = "kVK_F10")]
    pub const F10: KeyCode = 0x6D;

    #[doc(alias = "kVK_ContextualMenu")]
    pub const CONTEXTUAL_MENU: KeyCode = 0x6E;

    #[doc(alias = "kVK_F12")]
    pub const F12: KeyCode = 0x6F;

    #[doc(alias = "kVK_F15")]
    pub const F15: KeyCode = 0x71;

    #[doc(alias = "kVK_Help")]
    pub const HELP: KeyCode = 0x72;

    #[doc(alias = "kVK_Home")]
    pub const HOME: KeyCode = 0x73;

    #[doc(alias = "kVK_PageUp")]
    pub const PAGE_UP: KeyCode = 0x74;

    #[doc(alias = "kVK_ForwardDelete")]
    pub const FORWARD_DELETE: KeyCode = 0x75;

    #[doc(alias = "kVK_F4")]
    pub const F4: KeyCode = 0x76;

    #[doc(alias = "kVK_End")]
    pub const END: KeyCode = 0x77;

    #[doc(alias = "kVK_F2")]
    pub const F2: KeyCode = 0x78;

    #[doc(alias = "kVK_PageDown")]
    pub const PAGE_DOWN: KeyCode = 0x79;

    #[doc(alias = "kVK_F1")]
    pub const F1: KeyCode = 0x7A;

    #[doc(alias = "kVK_LeftArrow")]
    pub const LEFT_ARROW: KeyCode = 0x7B;

    #[doc(alias = "kVK_RightArrow")]
    pub const RIGHT_ARROW: KeyCode = 0x7C;

    #[doc(alias = "kVK_DownArrow")]
    pub const DOWN_ARROW: KeyCode = 0x7D;

    #[doc(alias = "kVK_UpArrow")]
    pub const UP_ARROW: KeyCode = 0x7E;

    // ISO keyboards only.
    #[doc(alias = "kVK_ISO_Section")]
    pub const ISO_SECTION: KeyCode = 0x0A;

    // JIS keyboards only.
    #[doc(alias = "kVK_JIS_Yen")]
    pub const JIS_YEN: KeyCode = 0x5D;

    #[doc(alias = "kVK_JIS_Underscore")]
    pub const JIS_UNDERSCORE: KeyCode = 0x5E;

    #[doc(alias = "kVK_JIS_KeypadComma")]
    pub const JIS_KEYPAD_COMMA: KeyCode = 0x5F;

    #[doc(alias = "kVK_JIS_Eisu")]
    pub const JIS_EISU: KeyCode = 0x66;

    #[doc(alias = "kVK_JIS_Kana")]
    pub const JIS_KANA: KeyCode = 0x68;
}

define_opts!(
    #[doc(alias = "CGEventFilterMask")]
    pub EventFilterMask(u32)
);

impl EventFilterMask {
    #[doc(alias = "kCGEventFilterMaskPermitLocalMouseEvents")]
    pub const PERMIT_LOCAL_MOUSE_EVENTS: Self = Self(0x00000001);

    #[doc(alias = "kCGEventFilterMaskPermitLocalKeyboardEvents")]
    pub const PERMIT_LOCAL_KEYBOARD_EVENTS: Self = Self(0x00000002);

    #[doc(alias = "kCGEventFilterMaskPermitSystemDefinedEvents")]
    pub const PERMIT_SYSTEM_DEFINED_EVENTS: Self = Self(0x00000004);

    #[doc(alias = "kCGEventFilterMaskPermitAllEvents")]
    pub const ALL_EVENTS: Self = Self(
        Self::PERMIT_LOCAL_MOUSE_EVENTS.0
            | Self::PERMIT_LOCAL_KEYBOARD_EVENTS.0
            | Self::PERMIT_SYSTEM_DEFINED_EVENTS.0,
    );
}

#[doc(alias = "CGEventSuppressionState")]
#[repr(u32)]
pub enum EventSuppressionState {
    #[doc(alias = "kCGEventSuppressionStateSuppressionInterval")]
    SuppressionInterval = 0,

    #[doc(alias = "kCGEventSuppressionStateRemoteMouseDrag")]
    RemoteMouseDrag,

    #[doc(alias = "kCGNumberOfEventSuppressionStates")]
    NumberOfEventSuppressionStates,
}
