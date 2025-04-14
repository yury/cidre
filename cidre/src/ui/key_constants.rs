#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[repr(isize)]
pub enum KeyboardHidUsage {
    KeyboardErrorRollOver = 0x01,  /* ErrorRollOver */
    KeyboardPOSTFail = 0x02,       /* POSTFail */
    KeyboardErrorUndefined = 0x03, /* ErrorUndefined */

    KeyboardA = 0x04, /* a or A */
    KeyboardB = 0x05, /* b or B */
    KeyboardC = 0x06, /* c or C */
    KeyboardD = 0x07, /* d or D */
    KeyboardE = 0x08, /* e or E */
    KeyboardF = 0x09, /* f or F */
    KeyboardG = 0x0A, /* g or G */
    KeyboardH = 0x0B, /* h or H */
    KeyboardI = 0x0C, /* i or I */
    KeyboardJ = 0x0D, /* j or J */
    KeyboardK = 0x0E, /* k or K */
    KeyboardL = 0x0F, /* l or L */
    KeyboardM = 0x10, /* m or M */
    KeyboardN = 0x11, /* n or N */
    KeyboardO = 0x12, /* o or O */
    KeyboardP = 0x13, /* p or P */
    KeyboardQ = 0x14, /* q or Q */
    KeyboardR = 0x15, /* r or R */
    KeyboardS = 0x16, /* s or S */
    KeyboardT = 0x17, /* t or T */
    KeyboardU = 0x18, /* u or U */
    KeyboardV = 0x19, /* v or V */
    KeyboardW = 0x1A, /* w or W */
    KeyboardX = 0x1B, /* x or X */
    KeyboardY = 0x1C, /* y or Y */
    KeyboardZ = 0x1D, /* z or Z */
    Keyboard1 = 0x1E, /* 1 or ! */
    Keyboard2 = 0x1F, /* 2 or @ */
    Keyboard3 = 0x20, /* 3 or # */
    Keyboard4 = 0x21, /* 4 or $ */
    Keyboard5 = 0x22, /* 5 or % */
    Keyboard6 = 0x23, /* 6 or ^ */
    Keyboard7 = 0x24, /* 7 or & */
    Keyboard8 = 0x25, /* 8 or * */
    Keyboard9 = 0x26, /* 9 or ( */
    Keyboard0 = 0x27, /* 0 or ) */

    KeyboardReturnOrEnter = 0x28,     /* Return (Enter) */
    KeyboardEscape = 0x29,            /* Escape */
    KeyboardDeleteOrBackspace = 0x2A, /* Delete (Backspace) */
    KeyboardTab = 0x2B,               /* Tab */
    KeyboardSpacebar = 0x2C,          /* Spacebar */
    KeyboardHyphen = 0x2D,            /* - or _ */
    KeyboardEqualSign = 0x2E,         /* = or + */
    KeyboardOpenBracket = 0x2F,       /* [ or { */
    KeyboardCloseBracket = 0x30,      /* ] or } */
    KeyboardBackslash = 0x31,         /* \ or | */
    KeyboardNonUSPound = 0x32,        /* Non-US # or _ */

    KeyboardSemicolon = 0x33,           /* ; or : */
    KeyboardQuote = 0x34,               /* ' or " */
    KeyboardGraveAccentAndTilde = 0x35, /* Grave Accent and Tilde */
    KeyboardComma = 0x36,               /* , or < */
    KeyboardPeriod = 0x37,              /* . or > */
    KeyboardSlash = 0x38,               /* / or ? */
    KeyboardCapsLock = 0x39,            /* Caps Lock */

    KeyboardF1 = 0x3A,            /* F1 */
    KeyboardF2 = 0x3B,            /* F2 */
    KeyboardF3 = 0x3C,            /* F3 */
    KeyboardF4 = 0x3D,            /* F4 */
    KeyboardF5 = 0x3E,            /* F5 */
    KeyboardF6 = 0x3F,            /* F6 */
    KeyboardF7 = 0x40,            /* F7 */
    KeyboardF8 = 0x41,            /* F8 */
    KeyboardF9 = 0x42,            /* F9 */
    KeyboardF10 = 0x43,           /* F10 */
    KeyboardF11 = 0x44,           /* F11 */
    KeyboardF12 = 0x45,           /* F12 */
    KeyboardPrintScreen = 0x46,   /* Print Screen */
    KeyboardScrollLock = 0x47,    /* Scroll Lock */
    KeyboardPause = 0x48,         /* Pause */
    KeyboardInsert = 0x49,        /* Insert */
    KeyboardHome = 0x4A,          /* Home */
    KeyboardPageUp = 0x4B,        /* Page Up */
    KeyboardDeleteForward = 0x4C, /* Delete Forward */
    KeyboardEnd = 0x4D,           /* End */
    KeyboardPageDown = 0x4E,      /* Page Down */
    KeyboardRightArrow = 0x4F,    /* Right Arrow */
    KeyboardLeftArrow = 0x50,     /* Left Arrow */
    KeyboardDownArrow = 0x51,     /* Down Arrow */
    KeyboardUpArrow = 0x52,       /* Up Arrow */

    KeypadNumLock = 0x53,          /* Keypad NumLock or Clear */
    KeypadSlash = 0x54,            /* Keypad / */
    KeypadAsterisk = 0x55,         /* Keypad * */
    KeypadHyphen = 0x56,           /* Keypad - */
    KeypadPlus = 0x57,             /* Keypad + */
    KeypadEnter = 0x58,            /* Keypad Enter */
    Keypad1 = 0x59,                /* Keypad 1 or End */
    Keypad2 = 0x5A,                /* Keypad 2 or Down Arrow */
    Keypad3 = 0x5B,                /* Keypad 3 or Page Down */
    Keypad4 = 0x5C,                /* Keypad 4 or Left Arrow */
    Keypad5 = 0x5D,                /* Keypad 5 */
    Keypad6 = 0x5E,                /* Keypad 6 or Right Arrow */
    Keypad7 = 0x5F,                /* Keypad 7 or Home */
    Keypad8 = 0x60,                /* Keypad 8 or Up Arrow */
    Keypad9 = 0x61,                /* Keypad 9 or Page Up */
    Keypad0 = 0x62,                /* Keypad 0 or Insert */
    KeypadPeriod = 0x63,           /* Keypad . or Delete */
    KeyboardNonUSBackslash = 0x64, /* Non-US \ or | */

    KeyboardApplication = 0x65, /* Application */
    KeyboardPower = 0x66,       /* Power */
    KeypadEqualSign = 0x67,     /* Keypad = */

    KeyboardF13 = 0x68,        /* F13 */
    KeyboardF14 = 0x69,        /* F14 */
    KeyboardF15 = 0x6A,        /* F15 */
    KeyboardF16 = 0x6B,        /* F16 */
    KeyboardF17 = 0x6C,        /* F17 */
    KeyboardF18 = 0x6D,        /* F18 */
    KeyboardF19 = 0x6E,        /* F19 */
    KeyboardF20 = 0x6F,        /* F20 */
    KeyboardF21 = 0x70,        /* F21 */
    KeyboardF22 = 0x71,        /* F22 */
    KeyboardF23 = 0x72,        /* F23 */
    KeyboardF24 = 0x73,        /* F24 */
    KeyboardExecute = 0x74,    /* Execute */
    KeyboardHelp = 0x75,       /* Help */
    KeyboardMenu = 0x76,       /* Menu */
    KeyboardSelect = 0x77,     /* Select */
    KeyboardStop = 0x78,       /* Stop */
    KeyboardAgain = 0x79,      /* Again */
    KeyboardUndo = 0x7A,       /* Undo */
    KeyboardCut = 0x7B,        /* Cut */
    KeyboardCopy = 0x7C,       /* Copy */
    KeyboardPaste = 0x7D,      /* Paste */
    KeyboardFind = 0x7E,       /* Find */
    KeyboardMute = 0x7F,       /* Mute */
    KeyboardVolumeUp = 0x80,   /* Volume Up */
    KeyboardVolumeDown = 0x81, /* Volume Down */

    KeyboardLockingCapsLock = 0x82, /* Locking Caps Lock */
    KeyboardLockingNumLock = 0x83,  /* Locking Num Lock */

    KeyboardLockingScrollLock = 0x84, /* Locking Scroll Lock */
    KeypadComma = 0x85,               /* Keypad Comma */
    KeypadEqualSignAS400 = 0x86,      /* Keypad Equal Sign for AS/400 */

    KeyboardInternational1 = 0x87, /* International1 */
    KeyboardInternational2 = 0x88, /* International2 */
    KeyboardInternational3 = 0x89, /* International3 */
    KeyboardInternational4 = 0x8A, /* International4 */
    KeyboardInternational5 = 0x8B, /* International5 */
    KeyboardInternational6 = 0x8C, /* International6 */
    KeyboardInternational7 = 0x8D, /* International7 */
    KeyboardInternational8 = 0x8E, /* International8 */
    KeyboardInternational9 = 0x8F, /* International9 */

    /* LANG1: On Apple keyboard for Japanese, this is the kana switch (かな) key */
    /* On Korean keyboards, this is the Hangul/English toggle key. */
    KeyboardLANG1 = 0x90, /* LANG1 */

    /* LANG2: On Apple keyboards for Japanese, this is the alphanumeric (英数) key */
    /* On Korean keyboards, this is the Hanja conversion key. */
    KeyboardLANG2 = 0x91, /* LANG2 */

    /* LANG3: Defines the Katakana key for Japanese USB word-processing keyboards. */
    KeyboardLANG3 = 0x92, /* LANG3 */

    /* LANG4: Defines the Hiragana key for Japanese USB word-processing keyboards. */
    KeyboardLANG4 = 0x93, /* LANG4 */

    /* LANG5: Defines the Zenkaku/Hankaku key for Japanese USB word-processing keyboards. */
    KeyboardLANG5 = 0x94, /* LANG5 */

    /* LANG6-9: Reserved for language-specific functions, such as Front End Processors and Input Method Editors. */
    KeyboardLANG6 = 0x95, /* LANG6 */
    KeyboardLANG7 = 0x96, /* LANG7 */
    KeyboardLANG8 = 0x97, /* LANG8 */
    KeyboardLANG9 = 0x98, /* LANG9 */

    KeyboardAlternateErase = 0x99,    /* AlternateErase */
    KeyboardSysReqOrAttention = 0x9A, /* SysReq/Attention */
    KeyboardCancel = 0x9B,            /* Cancel */
    KeyboardClear = 0x9C,             /* Clear */
    KeyboardPrior = 0x9D,             /* Prior */
    KeyboardReturn = 0x9E,            /* Return */
    KeyboardSeparator = 0x9F,         /* Separator */
    KeyboardOut = 0xA0,               /* Out */
    KeyboardOper = 0xA1,              /* Oper */
    KeyboardClearOrAgain = 0xA2,      /* Clear/Again */
    KeyboardCrSelOrProps = 0xA3,      /* CrSel/Props */
    KeyboardExSel = 0xA4,             /* ExSel */

    KeyboardLeftControl = 0xE0,  /* Left Control */
    KeyboardLeftShift = 0xE1,    /* Left Shift */
    KeyboardLeftAlt = 0xE2,      /* Left Alt */
    KeyboardLeftGUI = 0xE3,      /* Left GUI */
    KeyboardRightControl = 0xE4, /* Right Control */
    KeyboardRightShift = 0xE5,   /* Right Shift */
    KeyboardRightAlt = 0xE6,     /* Right Alt */
    KeyboardRightGUI = 0xE7,     /* Right GUI */

    KeyboardReserved = 0xFFFF,
}
