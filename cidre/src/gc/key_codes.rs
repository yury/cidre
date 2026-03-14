use crate::ns;

/// These values match the keyboard and keypad usage IDs in `IOHIDUsageTables.h`.
#[doc(alias = "GCKeyCode")]
pub type KeyCode = ns::Integer;

// Letter keys
#[doc(alias = "GCKeyCodeKeyA")]
pub const KEY_A: KeyCode = 0x04; // a or A
#[doc(alias = "GCKeyCodeKeyB")]
pub const KEY_B: KeyCode = 0x05; // b or B
#[doc(alias = "GCKeyCodeKeyC")]
pub const KEY_C: KeyCode = 0x06; // c or C
#[doc(alias = "GCKeyCodeKeyD")]
pub const KEY_D: KeyCode = 0x07; // d or D
#[doc(alias = "GCKeyCodeKeyE")]
pub const KEY_E: KeyCode = 0x08; // e or E
#[doc(alias = "GCKeyCodeKeyF")]
pub const KEY_F: KeyCode = 0x09; // f or F
#[doc(alias = "GCKeyCodeKeyG")]
pub const KEY_G: KeyCode = 0x0A; // g or G
#[doc(alias = "GCKeyCodeKeyH")]
pub const KEY_H: KeyCode = 0x0B; // h or H
#[doc(alias = "GCKeyCodeKeyI")]
pub const KEY_I: KeyCode = 0x0C; // i or I
#[doc(alias = "GCKeyCodeKeyJ")]
pub const KEY_J: KeyCode = 0x0D; // j or J
#[doc(alias = "GCKeyCodeKeyK")]
pub const KEY_K: KeyCode = 0x0E; // k or K
#[doc(alias = "GCKeyCodeKeyL")]
pub const KEY_L: KeyCode = 0x0F; // l or L
#[doc(alias = "GCKeyCodeKeyM")]
pub const KEY_M: KeyCode = 0x10; // m or M
#[doc(alias = "GCKeyCodeKeyN")]
pub const KEY_N: KeyCode = 0x11; // n or N
#[doc(alias = "GCKeyCodeKeyO")]
pub const KEY_O: KeyCode = 0x12; // o or O
#[doc(alias = "GCKeyCodeKeyP")]
pub const KEY_P: KeyCode = 0x13; // p or P
#[doc(alias = "GCKeyCodeKeyQ")]
pub const KEY_Q: KeyCode = 0x14; // q or Q
#[doc(alias = "GCKeyCodeKeyR")]
pub const KEY_R: KeyCode = 0x15; // r or R
#[doc(alias = "GCKeyCodeKeyS")]
pub const KEY_S: KeyCode = 0x16; // s or S
#[doc(alias = "GCKeyCodeKeyT")]
pub const KEY_T: KeyCode = 0x17; // t or T
#[doc(alias = "GCKeyCodeKeyU")]
pub const KEY_U: KeyCode = 0x18; // u or U
#[doc(alias = "GCKeyCodeKeyV")]
pub const KEY_V: KeyCode = 0x19; // v or V
#[doc(alias = "GCKeyCodeKeyW")]
pub const KEY_W: KeyCode = 0x1A; // w or W
#[doc(alias = "GCKeyCodeKeyX")]
pub const KEY_X: KeyCode = 0x1B; // x or X
#[doc(alias = "GCKeyCodeKeyY")]
pub const KEY_Y: KeyCode = 0x1C; // y or Y
#[doc(alias = "GCKeyCodeKeyZ")]
pub const KEY_Z: KeyCode = 0x1D; // z or Z

// Number row
#[doc(alias = "GCKeyCodeOne")]
pub const ONE: KeyCode = 0x1E; // 1 or !
#[doc(alias = "GCKeyCodeTwo")]
pub const TWO: KeyCode = 0x1F; // 2 or @
#[doc(alias = "GCKeyCodeThree")]
pub const THREE: KeyCode = 0x20; // 3 or #
#[doc(alias = "GCKeyCodeFour")]
pub const FOUR: KeyCode = 0x21; // 4 or $
#[doc(alias = "GCKeyCodeFive")]
pub const FIVE: KeyCode = 0x22; // 5 or %
#[doc(alias = "GCKeyCodeSix")]
pub const SIX: KeyCode = 0x23; // 6 or ^
#[doc(alias = "GCKeyCodeSeven")]
pub const SEVEN: KeyCode = 0x24; // 7 or &
#[doc(alias = "GCKeyCodeEight")]
pub const EIGHT: KeyCode = 0x25; // 8 or *
#[doc(alias = "GCKeyCodeNine")]
pub const NINE: KeyCode = 0x26; // 9 or (
#[doc(alias = "GCKeyCodeZero")]
pub const ZERO: KeyCode = 0x27; // 0 or )

// Control and symbol keys
#[doc(alias = "GCKeyCodeReturnOrEnter")]
pub const RETURN_OR_ENTER: KeyCode = 0x28; // Return (Enter)
#[doc(alias = "GCKeyCodeEscape")]
pub const ESCAPE: KeyCode = 0x29; // Escape
#[doc(alias = "GCKeyCodeDeleteOrBackspace")]
pub const DELETE_OR_BACKSPACE: KeyCode = 0x2A; // Delete (Backspace)
#[doc(alias = "GCKeyCodeTab")]
pub const TAB: KeyCode = 0x2B; // Tab
#[doc(alias = "GCKeyCodeSpacebar")]
pub const SPACEBAR: KeyCode = 0x2C; // Spacebar
#[doc(alias = "GCKeyCodeHyphen")]
pub const HYPHEN: KeyCode = 0x2D; // - or _
#[doc(alias = "GCKeyCodeEqualSign")]
pub const EQUAL_SIGN: KeyCode = 0x2E; // = or +
#[doc(alias = "GCKeyCodeOpenBracket")]
pub const OPEN_BRACKET: KeyCode = 0x2F; // [ or {
#[doc(alias = "GCKeyCodeCloseBracket")]
pub const CLOSE_BRACKET: KeyCode = 0x30; // ] or }
#[doc(alias = "GCKeyCodeBackslash")]
pub const BACKSLASH: KeyCode = 0x31; // \ or |
#[doc(alias = "GCKeyCodeNonUSPound")]
pub const NON_US_POUND: KeyCode = 0x32; // Non-US # or _

// Punctuation and lock keys
#[doc(alias = "GCKeyCodeSemicolon")]
pub const SEMICOLON: KeyCode = 0x33; // ; or :
#[doc(alias = "GCKeyCodeQuote")]
pub const QUOTE: KeyCode = 0x34; // ' or "
#[doc(alias = "GCKeyCodeGraveAccentAndTilde")]
pub const GRAVE_ACCENT_AND_TILDE: KeyCode = 0x35; // Grave Accent and Tilde
#[doc(alias = "GCKeyCodeComma")]
pub const COMMA: KeyCode = 0x36; // , or <
#[doc(alias = "GCKeyCodePeriod")]
pub const PERIOD: KeyCode = 0x37; // . or >
#[doc(alias = "GCKeyCodeSlash")]
pub const SLASH: KeyCode = 0x38; // / or ?
#[doc(alias = "GCKeyCodeCapsLock")]
pub const CAPS_LOCK: KeyCode = 0x39; // Caps Lock

// Function and navigation keys
#[doc(alias = "GCKeyCodeF1")]
pub const F1: KeyCode = 0x3A; // F1
#[doc(alias = "GCKeyCodeF2")]
pub const F2: KeyCode = 0x3B; // F2
#[doc(alias = "GCKeyCodeF3")]
pub const F3: KeyCode = 0x3C; // F3
#[doc(alias = "GCKeyCodeF4")]
pub const F4: KeyCode = 0x3D; // F4
#[doc(alias = "GCKeyCodeF5")]
pub const F5: KeyCode = 0x3E; // F5
#[doc(alias = "GCKeyCodeF6")]
pub const F6: KeyCode = 0x3F; // F6
#[doc(alias = "GCKeyCodeF7")]
pub const F7: KeyCode = 0x40; // F7
#[doc(alias = "GCKeyCodeF8")]
pub const F8: KeyCode = 0x41; // F8
#[doc(alias = "GCKeyCodeF9")]
pub const F9: KeyCode = 0x42; // F9
#[doc(alias = "GCKeyCodeF10")]
pub const F10: KeyCode = 0x43; // F10
#[doc(alias = "GCKeyCodeF11")]
pub const F11: KeyCode = 0x44; // F11
#[doc(alias = "GCKeyCodeF12")]
pub const F12: KeyCode = 0x45; // F12
#[doc(alias = "GCKeyCodePrintScreen")]
pub const PRINT_SCREEN: KeyCode = 0x46; // Print Screen
#[doc(alias = "GCKeyCodeScrollLock")]
pub const SCROLL_LOCK: KeyCode = 0x47; // Scroll Lock
#[doc(alias = "GCKeyCodePause")]
pub const PAUSE: KeyCode = 0x48; // Pause
#[doc(alias = "GCKeyCodeInsert")]
pub const INSERT: KeyCode = 0x49; // Insert
#[doc(alias = "GCKeyCodeHome")]
pub const HOME: KeyCode = 0x4A; // Home
#[doc(alias = "GCKeyCodePageUp")]
pub const PAGE_UP: KeyCode = 0x4B; // Page Up
#[doc(alias = "GCKeyCodeDeleteForward")]
pub const DELETE_FORWARD: KeyCode = 0x4C; // Delete Forward
#[doc(alias = "GCKeyCodeEnd")]
pub const END: KeyCode = 0x4D; // End
#[doc(alias = "GCKeyCodePageDown")]
pub const PAGE_DOWN: KeyCode = 0x4E; // Page Down
#[doc(alias = "GCKeyCodeRightArrow")]
pub const RIGHT_ARROW: KeyCode = 0x4F; // Right Arrow
#[doc(alias = "GCKeyCodeLeftArrow")]
pub const LEFT_ARROW: KeyCode = 0x50; // Left Arrow
#[doc(alias = "GCKeyCodeDownArrow")]
pub const DOWN_ARROW: KeyCode = 0x51; // Down Arrow
#[doc(alias = "GCKeyCodeUpArrow")]
pub const UP_ARROW: KeyCode = 0x52; // Up Arrow

// Keypad keys
#[doc(alias = "GCKeyCodeKeypadNumLock")]
pub const KEYPAD_NUM_LOCK: KeyCode = 0x53; // Keypad NumLock or Clear
#[doc(alias = "GCKeyCodeKeypadSlash")]
pub const KEYPAD_SLASH: KeyCode = 0x54; // Keypad /
#[doc(alias = "GCKeyCodeKeypadAsterisk")]
pub const KEYPAD_ASTERISK: KeyCode = 0x55; // Keypad *
#[doc(alias = "GCKeyCodeKeypadHyphen")]
pub const KEYPAD_HYPHEN: KeyCode = 0x56; // Keypad -
#[doc(alias = "GCKeyCodeKeypadPlus")]
pub const KEYPAD_PLUS: KeyCode = 0x57; // Keypad +
#[doc(alias = "GCKeyCodeKeypadEnter")]
pub const KEYPAD_ENTER: KeyCode = 0x58; // Keypad Enter
#[doc(alias = "GCKeyCodeKeypad1")]
pub const KEYPAD1: KeyCode = 0x59; // Keypad 1 or End
#[doc(alias = "GCKeyCodeKeypad2")]
pub const KEYPAD2: KeyCode = 0x5A; // Keypad 2 or Down Arrow
#[doc(alias = "GCKeyCodeKeypad3")]
pub const KEYPAD3: KeyCode = 0x5B; // Keypad 3 or Page Down
#[doc(alias = "GCKeyCodeKeypad4")]
pub const KEYPAD4: KeyCode = 0x5C; // Keypad 4 or Left Arrow
#[doc(alias = "GCKeyCodeKeypad5")]
pub const KEYPAD5: KeyCode = 0x5D; // Keypad 5
#[doc(alias = "GCKeyCodeKeypad6")]
pub const KEYPAD6: KeyCode = 0x5E; // Keypad 6 or Right Arrow
#[doc(alias = "GCKeyCodeKeypad7")]
pub const KEYPAD7: KeyCode = 0x5F; // Keypad 7 or Home
#[doc(alias = "GCKeyCodeKeypad8")]
pub const KEYPAD8: KeyCode = 0x60; // Keypad 8 or Up Arrow
#[doc(alias = "GCKeyCodeKeypad9")]
pub const KEYPAD9: KeyCode = 0x61; // Keypad 9 or Page Up
#[doc(alias = "GCKeyCodeKeypad0")]
pub const KEYPAD0: KeyCode = 0x62; // Keypad 0 or Insert
#[doc(alias = "GCKeyCodeKeypadPeriod")]
pub const KEYPAD_PERIOD: KeyCode = 0x63; // Keypad . or Delete
#[doc(alias = "GCKeyCodeNonUSBackslash")]
pub const NON_US_BACKSLASH: KeyCode = 0x64; // Non-US \ or |

// Application and extended function keys
#[doc(alias = "GCKeyCodeApplication")]
pub const APPLICATION: KeyCode = 0x65; // Application
#[doc(alias = "GCKeyCodePower")]
pub const POWER: KeyCode = 0x66; // Power
#[doc(alias = "GCKeyCodeKeypadEqualSign")]
pub const KEYPAD_EQUAL_SIGN: KeyCode = 0x67; // Keypad =

#[doc(alias = "GCKeyCodeF13")]
pub const F13: KeyCode = 0x68; // F13
#[doc(alias = "GCKeyCodeF14")]
pub const F14: KeyCode = 0x69; // F14
#[doc(alias = "GCKeyCodeF15")]
pub const F15: KeyCode = 0x6A; // F15
#[doc(alias = "GCKeyCodeF16")]
pub const F16: KeyCode = 0x6B; // F16
#[doc(alias = "GCKeyCodeF17")]
pub const F17: KeyCode = 0x6C; // F17
#[doc(alias = "GCKeyCodeF18")]
pub const F18: KeyCode = 0x6D; // F18
#[doc(alias = "GCKeyCodeF19")]
pub const F19: KeyCode = 0x6E; // F19
#[doc(alias = "GCKeyCodeF20")]
pub const F20: KeyCode = 0x6F; // F20

// International keys
#[doc(alias = "GCKeyCodeInternational1")]
pub const INTERNATIONAL1: KeyCode = 0x87; // International1
#[doc(alias = "GCKeyCodeInternational2")]
pub const INTERNATIONAL2: KeyCode = 0x88; // International2
#[doc(alias = "GCKeyCodeInternational3")]
pub const INTERNATIONAL3: KeyCode = 0x89; // International3
#[doc(alias = "GCKeyCodeInternational4")]
pub const INTERNATIONAL4: KeyCode = 0x8A; // International4
#[doc(alias = "GCKeyCodeInternational5")]
pub const INTERNATIONAL5: KeyCode = 0x8B; // International5
#[doc(alias = "GCKeyCodeInternational6")]
pub const INTERNATIONAL6: KeyCode = 0x8C; // International6
#[doc(alias = "GCKeyCodeInternational7")]
pub const INTERNATIONAL7: KeyCode = 0x8D; // International7
#[doc(alias = "GCKeyCodeInternational8")]
pub const INTERNATIONAL8: KeyCode = 0x8E; // International8
#[doc(alias = "GCKeyCodeInternational9")]
pub const INTERNATIONAL9: KeyCode = 0x8F; // International9

// Language keys
#[doc(alias = "GCKeyCodeLANG1")]
pub const LANG1: KeyCode = 0x90; // LANG1
#[doc(alias = "GCKeyCodeLANG2")]
pub const LANG2: KeyCode = 0x91; // LANG2
#[doc(alias = "GCKeyCodeLANG3")]
pub const LANG3: KeyCode = 0x92; // LANG3
#[doc(alias = "GCKeyCodeLANG4")]
pub const LANG4: KeyCode = 0x93; // LANG4
#[doc(alias = "GCKeyCodeLANG5")]
pub const LANG5: KeyCode = 0x94; // LANG5
#[doc(alias = "GCKeyCodeLANG6")]
pub const LANG6: KeyCode = 0x95; // LANG6
#[doc(alias = "GCKeyCodeLANG7")]
pub const LANG7: KeyCode = 0x96; // LANG7
#[doc(alias = "GCKeyCodeLANG8")]
pub const LANG8: KeyCode = 0x97; // LANG8
#[doc(alias = "GCKeyCodeLANG9")]
pub const LANG9: KeyCode = 0x98; // LANG9

// Modifier keys
#[doc(alias = "GCKeyCodeLeftControl")]
pub const LEFT_CONTROL: KeyCode = 0xE0; // Left Control
#[doc(alias = "GCKeyCodeLeftShift")]
pub const LEFT_SHIFT: KeyCode = 0xE1; // Left Shift
#[doc(alias = "GCKeyCodeLeftAlt")]
pub const LEFT_ALT: KeyCode = 0xE2; // Left Alt
#[doc(alias = "GCKeyCodeLeftGUI")]
pub const LEFT_GUI: KeyCode = 0xE3; // Left GUI
#[doc(alias = "GCKeyCodeRightControl")]
pub const RIGHT_CONTROL: KeyCode = 0xE4; // Right Control
#[doc(alias = "GCKeyCodeRightShift")]
pub const RIGHT_SHIFT: KeyCode = 0xE5; // Right Shift
#[doc(alias = "GCKeyCodeRightAlt")]
pub const RIGHT_ALT: KeyCode = 0xE6; // Right Alt
#[doc(alias = "GCKeyCodeRightGUI")]
pub const RIGHT_GUI: KeyCode = 0xE7; // Right GUI

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use std::{fs, process::Command};

    use super::*;

    fn hid_header() -> String {
        let output = Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .expect("xcrun should be available to locate the macOS SDK");
        assert!(
            output.status.success(),
            "xcrun failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let sdk = String::from_utf8(output.stdout).expect("xcrun output should be utf-8");
        let path = format!(
            "{}/System/Library/Frameworks/IOKit.framework/Headers/hid/IOHIDUsageTables.h",
            sdk.trim()
        );
        let bytes = fs::read(path).expect("IOHIDUsageTables.h should exist in the SDK");
        String::from_utf8_lossy(&bytes).into_owned()
    }

    fn hid_usage(header: &str, symbol: &str) -> KeyCode {
        let line = header
            .lines()
            .find(|line| {
                line.split('=')
                    .next()
                    .map(str::trim)
                    .is_some_and(|name| name == symbol)
            })
            .unwrap_or_else(|| panic!("missing {symbol} in IOHIDUsageTables.h"));
        let value = line
            .split('=')
            .nth(1)
            .unwrap_or_else(|| panic!("missing value for {symbol}"))
            .split(',')
            .next()
            .unwrap()
            .trim();

        if value.starts_with("0x") {
            KeyCode::from_str_radix(value.trim_start_matches("0x"), 16)
                .expect("hex value should parse")
        } else {
            value.parse::<KeyCode>().expect("value should parse")
        }
    }

    #[test]
    fn all_gc_key_codes_match_hid_usage_table() {
        let header = hid_header();
        let cases = [
            (KEY_A, "kHIDUsage_KeyboardA"),
            (KEY_B, "kHIDUsage_KeyboardB"),
            (KEY_C, "kHIDUsage_KeyboardC"),
            (KEY_D, "kHIDUsage_KeyboardD"),
            (KEY_E, "kHIDUsage_KeyboardE"),
            (KEY_F, "kHIDUsage_KeyboardF"),
            (KEY_G, "kHIDUsage_KeyboardG"),
            (KEY_H, "kHIDUsage_KeyboardH"),
            (KEY_I, "kHIDUsage_KeyboardI"),
            (KEY_J, "kHIDUsage_KeyboardJ"),
            (KEY_K, "kHIDUsage_KeyboardK"),
            (KEY_L, "kHIDUsage_KeyboardL"),
            (KEY_M, "kHIDUsage_KeyboardM"),
            (KEY_N, "kHIDUsage_KeyboardN"),
            (KEY_O, "kHIDUsage_KeyboardO"),
            (KEY_P, "kHIDUsage_KeyboardP"),
            (KEY_Q, "kHIDUsage_KeyboardQ"),
            (KEY_R, "kHIDUsage_KeyboardR"),
            (KEY_S, "kHIDUsage_KeyboardS"),
            (KEY_T, "kHIDUsage_KeyboardT"),
            (KEY_U, "kHIDUsage_KeyboardU"),
            (KEY_V, "kHIDUsage_KeyboardV"),
            (KEY_W, "kHIDUsage_KeyboardW"),
            (KEY_X, "kHIDUsage_KeyboardX"),
            (KEY_Y, "kHIDUsage_KeyboardY"),
            (KEY_Z, "kHIDUsage_KeyboardZ"),
            (ONE, "kHIDUsage_Keyboard1"),
            (TWO, "kHIDUsage_Keyboard2"),
            (THREE, "kHIDUsage_Keyboard3"),
            (FOUR, "kHIDUsage_Keyboard4"),
            (FIVE, "kHIDUsage_Keyboard5"),
            (SIX, "kHIDUsage_Keyboard6"),
            (SEVEN, "kHIDUsage_Keyboard7"),
            (EIGHT, "kHIDUsage_Keyboard8"),
            (NINE, "kHIDUsage_Keyboard9"),
            (ZERO, "kHIDUsage_Keyboard0"),
            (RETURN_OR_ENTER, "kHIDUsage_KeyboardReturnOrEnter"),
            (ESCAPE, "kHIDUsage_KeyboardEscape"),
            (DELETE_OR_BACKSPACE, "kHIDUsage_KeyboardDeleteOrBackspace"),
            (TAB, "kHIDUsage_KeyboardTab"),
            (SPACEBAR, "kHIDUsage_KeyboardSpacebar"),
            (HYPHEN, "kHIDUsage_KeyboardHyphen"),
            (EQUAL_SIGN, "kHIDUsage_KeyboardEqualSign"),
            (OPEN_BRACKET, "kHIDUsage_KeyboardOpenBracket"),
            (CLOSE_BRACKET, "kHIDUsage_KeyboardCloseBracket"),
            (BACKSLASH, "kHIDUsage_KeyboardBackslash"),
            (NON_US_POUND, "kHIDUsage_KeyboardNonUSPound"),
            (SEMICOLON, "kHIDUsage_KeyboardSemicolon"),
            (QUOTE, "kHIDUsage_KeyboardQuote"),
            (
                GRAVE_ACCENT_AND_TILDE,
                "kHIDUsage_KeyboardGraveAccentAndTilde",
            ),
            (COMMA, "kHIDUsage_KeyboardComma"),
            (PERIOD, "kHIDUsage_KeyboardPeriod"),
            (SLASH, "kHIDUsage_KeyboardSlash"),
            (CAPS_LOCK, "kHIDUsage_KeyboardCapsLock"),
            (F1, "kHIDUsage_KeyboardF1"),
            (F2, "kHIDUsage_KeyboardF2"),
            (F3, "kHIDUsage_KeyboardF3"),
            (F4, "kHIDUsage_KeyboardF4"),
            (F5, "kHIDUsage_KeyboardF5"),
            (F6, "kHIDUsage_KeyboardF6"),
            (F7, "kHIDUsage_KeyboardF7"),
            (F8, "kHIDUsage_KeyboardF8"),
            (F9, "kHIDUsage_KeyboardF9"),
            (F10, "kHIDUsage_KeyboardF10"),
            (F11, "kHIDUsage_KeyboardF11"),
            (F12, "kHIDUsage_KeyboardF12"),
            (PRINT_SCREEN, "kHIDUsage_KeyboardPrintScreen"),
            (SCROLL_LOCK, "kHIDUsage_KeyboardScrollLock"),
            (PAUSE, "kHIDUsage_KeyboardPause"),
            (INSERT, "kHIDUsage_KeyboardInsert"),
            (HOME, "kHIDUsage_KeyboardHome"),
            (PAGE_UP, "kHIDUsage_KeyboardPageUp"),
            (DELETE_FORWARD, "kHIDUsage_KeyboardDeleteForward"),
            (END, "kHIDUsage_KeyboardEnd"),
            (PAGE_DOWN, "kHIDUsage_KeyboardPageDown"),
            (RIGHT_ARROW, "kHIDUsage_KeyboardRightArrow"),
            (LEFT_ARROW, "kHIDUsage_KeyboardLeftArrow"),
            (DOWN_ARROW, "kHIDUsage_KeyboardDownArrow"),
            (UP_ARROW, "kHIDUsage_KeyboardUpArrow"),
            (KEYPAD_NUM_LOCK, "kHIDUsage_KeypadNumLock"),
            (KEYPAD_SLASH, "kHIDUsage_KeypadSlash"),
            (KEYPAD_ASTERISK, "kHIDUsage_KeypadAsterisk"),
            (KEYPAD_HYPHEN, "kHIDUsage_KeypadHyphen"),
            (KEYPAD_PLUS, "kHIDUsage_KeypadPlus"),
            (KEYPAD_ENTER, "kHIDUsage_KeypadEnter"),
            (KEYPAD1, "kHIDUsage_Keypad1"),
            (KEYPAD2, "kHIDUsage_Keypad2"),
            (KEYPAD3, "kHIDUsage_Keypad3"),
            (KEYPAD4, "kHIDUsage_Keypad4"),
            (KEYPAD5, "kHIDUsage_Keypad5"),
            (KEYPAD6, "kHIDUsage_Keypad6"),
            (KEYPAD7, "kHIDUsage_Keypad7"),
            (KEYPAD8, "kHIDUsage_Keypad8"),
            (KEYPAD9, "kHIDUsage_Keypad9"),
            (KEYPAD0, "kHIDUsage_Keypad0"),
            (KEYPAD_PERIOD, "kHIDUsage_KeypadPeriod"),
            (NON_US_BACKSLASH, "kHIDUsage_KeyboardNonUSBackslash"),
            (APPLICATION, "kHIDUsage_KeyboardApplication"),
            (POWER, "kHIDUsage_KeyboardPower"),
            (KEYPAD_EQUAL_SIGN, "kHIDUsage_KeypadEqualSign"),
            (F13, "kHIDUsage_KeyboardF13"),
            (F14, "kHIDUsage_KeyboardF14"),
            (F15, "kHIDUsage_KeyboardF15"),
            (F16, "kHIDUsage_KeyboardF16"),
            (F17, "kHIDUsage_KeyboardF17"),
            (F18, "kHIDUsage_KeyboardF18"),
            (F19, "kHIDUsage_KeyboardF19"),
            (F20, "kHIDUsage_KeyboardF20"),
            (INTERNATIONAL1, "kHIDUsage_KeyboardInternational1"),
            (INTERNATIONAL2, "kHIDUsage_KeyboardInternational2"),
            (INTERNATIONAL3, "kHIDUsage_KeyboardInternational3"),
            (INTERNATIONAL4, "kHIDUsage_KeyboardInternational4"),
            (INTERNATIONAL5, "kHIDUsage_KeyboardInternational5"),
            (INTERNATIONAL6, "kHIDUsage_KeyboardInternational6"),
            (INTERNATIONAL7, "kHIDUsage_KeyboardInternational7"),
            (INTERNATIONAL8, "kHIDUsage_KeyboardInternational8"),
            (INTERNATIONAL9, "kHIDUsage_KeyboardInternational9"),
            (LANG1, "kHIDUsage_KeyboardLANG1"),
            (LANG2, "kHIDUsage_KeyboardLANG2"),
            (LANG3, "kHIDUsage_KeyboardLANG3"),
            (LANG4, "kHIDUsage_KeyboardLANG4"),
            (LANG5, "kHIDUsage_KeyboardLANG5"),
            (LANG6, "kHIDUsage_KeyboardLANG6"),
            (LANG7, "kHIDUsage_KeyboardLANG7"),
            (LANG8, "kHIDUsage_KeyboardLANG8"),
            (LANG9, "kHIDUsage_KeyboardLANG9"),
            (LEFT_CONTROL, "kHIDUsage_KeyboardLeftControl"),
            (LEFT_SHIFT, "kHIDUsage_KeyboardLeftShift"),
            (LEFT_ALT, "kHIDUsage_KeyboardLeftAlt"),
            (LEFT_GUI, "kHIDUsage_KeyboardLeftGUI"),
            (RIGHT_CONTROL, "kHIDUsage_KeyboardRightControl"),
            (RIGHT_SHIFT, "kHIDUsage_KeyboardRightShift"),
            (RIGHT_ALT, "kHIDUsage_KeyboardRightAlt"),
            (RIGHT_GUI, "kHIDUsage_KeyboardRightGUI"),
        ];

        for (actual, symbol) in cases {
            assert_eq!(actual, hid_usage(&header, symbol), "mismatch for {symbol}");
        }
    }

    #[test]
    fn gc_key_codes_do_not_match_cg_virtual_keycodes() {
        let gc = [KEY_A, SPACEBAR, F1, KEYPAD1];
        let cg = [0 as KeyCode, 49 as KeyCode, 122 as KeyCode, 83 as KeyCode];

        assert_ne!(gc, cg);
    }
}
