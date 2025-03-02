/// Input terminal types
pub mod input_term {
    #[doc(alias = "INPUT_UNDEFINED")]
    pub const UNDEFINED: u32 = 0x0200;

    #[doc(alias = "INPUT_MICROPHONE")]
    pub const MIC: u32 = 0x0201;

    #[doc(alias = "INPUT_DESKTOP_MICROPHONE")]
    pub const DESKTOP_MIC: u32 = 0x0202;

    #[doc(alias = "INPUT_PERSONAL_MICROPHONE")]
    pub const PERSONAL_MIC: u32 = 0x0203;

    #[doc(alias = "INPUT_OMNIDIRECTIONAL_MICROPHONE")]
    pub const OMNIDIRECTIONAL_MIC: u32 = 0x0204;

    #[doc(alias = "INPUT_MICROPHONE_ARRAY")]
    pub const MIC_ARRAY: u32 = 0x0205;

    #[doc(alias = "INPUT_PROCESSING_MICROPHONE_ARRAY")]
    pub const PROCESSING_MIC_ARRAY: u32 = 0x0206;

    #[doc(alias = "INPUT_MODEM_AUDIO")]
    pub const MODEM_AUDIO: u32 = 0x0207; //  0x207 in header....
}

/// Output terminal types
pub mod output_term {
    #[doc(alias = "OUTPUT_UNDEFINED")]
    pub const UNDEFINED: u32 = 0x0300;

    #[doc(alias = "OUTPUT_SPEAKER")]
    pub const SPEAKER: u32 = 0x0301;

    #[doc(alias = "OUTPUT_HEADPHONES")]
    pub const HEADPHONES: u32 = 0x0302;

    #[doc(alias = "OUTPUT_HEAD_MOUNTED_DISPLAY_AUDIO")]
    pub const HEAD_MOUNTED_DISPLAY_AUDIO: u32 = 0x0303;

    #[doc(alias = "OUTPUT_DESKTOP_SPEAKER")]
    pub const DESKTOP_SPEAKER: u32 = 0x0304;

    #[doc(alias = "OUTPUT_ROOM_SPEAKER")]
    pub const ROOM_SPEAKER: u32 = 0x0305;

    #[doc(alias = "OUTPUT_COMMUNICATION_SPEAKER")]
    pub const COMMUNICATION_SPEAKER: u32 = 0x0306;

    #[doc(alias = "OUTPUT_LOW_FREQUENCY_EFFECTS_SPEAKER")]
    pub const LOW_FREQUENCY_EFFECTS_SPEAKER: u32 = 0x0307;
}
