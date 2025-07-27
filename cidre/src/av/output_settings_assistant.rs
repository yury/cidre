use crate::{api, arc, av, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "cm")]
use crate::cm;

define_obj_type!(
    #[doc(alias = "AVOutputSettingsPreset")]
    pub OutputSettingsPreset(ns::String)
);

impl OutputSettingsPreset {
    /// A preset for H.264 video at 640 by 480 pixels.
    #[doc(alias = "AVOutputSettingsPreset640x480")]
    pub fn h264_640x480() -> &'static Self {
        unsafe { AVOutputSettingsPreset640x480 }
    }

    /// A preset for H.264 video at 960 by 540 pixels.
    #[doc(alias = "AVOutputSettingsPreset960x540")]
    pub fn h264_960x540() -> &'static Self {
        unsafe { AVOutputSettingsPreset960x540 }
    }

    /// A preset for H.264 video at 1280 by 720 pixels.
    #[doc(alias = "AVOutputSettingsPreset1280x720")]
    pub fn h264_1280x720() -> &'static Self {
        unsafe { AVOutputSettingsPreset1280x720 }
    }

    /// A preset for H.264 video at 1920 by 1080 pixels.
    #[doc(alias = "AVOutputSettingsPreset1920x1080")]
    pub fn h264_1920x1080() -> &'static Self {
        unsafe { AVOutputSettingsPreset1920x1080 }
    }

    /// A preset for H.264 video at 3840 by 2160 pixels.
    #[doc(alias = "AVOutputSettingsPreset3840x2160")]
    pub fn h264_3840x2160() -> &'static Self {
        unsafe { AVOutputSettingsPreset3840x2160 }
    }

    /// A preset for HEVC video at 1920 by 1080 pixels.
    #[doc(alias = "AVOutputSettingsPresetHEVC1920x1080")]
    pub fn hevc_1920x1080() -> &'static Self {
        unsafe { AVOutputSettingsPresetHEVC1920x1080 }
    }

    /// A preset for HEVC with Alpha video at 1920 by 1080 pixels.
    #[doc(alias = "AVOutputSettingsPresetHEVC1920x1080WithAlpha")]
    pub fn hevc_1920x1080_with_alhpa() -> &'static Self {
        unsafe { AVOutputSettingsPresetHEVC1920x1080WithAlpha }
    }

    /// A preset for HEVC video at 3840 by 2160 pixels.
    #[doc(alias = "AVOutputSettingsPresetHEVC3840x2160")]
    pub fn hevc_3840x2160() -> &'static Self {
        unsafe { AVOutputSettingsPresetHEVC3840x2160 }
    }

    /// A preset for HEVC with Alpha video at 3840 by 2160 pixels.
    #[doc(alias = "AVOutputSettingsPresetHEVC3840x2160WithAlpha")]
    pub fn hevc_3840x2160_with_alpha() -> &'static Self {
        unsafe { AVOutputSettingsPresetHEVC3840x2160WithAlpha }
    }

    #[doc(alias = "AVOutputSettingsPresetHEVC4320x2160")]
    #[api::available(macos = 26.0, ios = 26.0, visionos = 26.0)]
    pub fn hevc_4320x2160() -> &'static Self {
        unsafe { AVOutputSettingsPresetHEVC4320x2160 }
    }

    /// A preset for HEVC video at 7680 by 4320 pixels.
    #[doc(alias = "AVOutputSettingsPresetHEVC7680x4320")]
    #[api::available(macos = 12.1, ios = 26.0, visionos = 26.0)]
    pub fn hevc_7680x4320() -> &'static Self {
        unsafe { AVOutputSettingsPresetHEVC7680x4320 }
    }

    /// A preset for MV-HEVC video at 960 by 960 pixels.
    #[doc(alias = "AVOutputSettingsPresetMVHEVC960x960")]
    pub fn mv_hevc_960x960() -> &'static Self {
        unsafe { AVOutputSettingsPresetMVHEVC960x960 }
    }

    /// A preset for MV-HEVC video at 1440 by 1440 pixels.
    #[doc(alias = "AVOutputSettingsPresetMVHEVC1440x1440")]
    pub fn mv_hevc_1440x1440() -> &'static Self {
        unsafe { AVOutputSettingsPresetMVHEVC1440x1440 }
    }

    #[api::available(macos = 26.0, ios = 26.0, visionos = 26.0)]
    pub fn mv_hevc_4320x4320() -> &'static Self {
        unsafe { AVOutputSettingsPresetMVHEVC4320x4320 }
    }
    #[api::available(macos = 26.0, ios = 26.0, visionos = 26.0)]
    pub fn mv_hevc_7680x7680() -> &'static Self {
        unsafe { AVOutputSettingsPresetMVHEVC7680x7680 }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    static AVOutputSettingsPreset640x480: &'static OutputSettingsPreset;
    static AVOutputSettingsPreset960x540: &'static OutputSettingsPreset;
    static AVOutputSettingsPreset1280x720: &'static OutputSettingsPreset;
    static AVOutputSettingsPreset1920x1080: &'static OutputSettingsPreset;
    static AVOutputSettingsPreset3840x2160: &'static OutputSettingsPreset;
    static AVOutputSettingsPresetHEVC1920x1080: &'static OutputSettingsPreset;
    static AVOutputSettingsPresetHEVC1920x1080WithAlpha: &'static OutputSettingsPreset;
    static AVOutputSettingsPresetHEVC3840x2160: &'static OutputSettingsPreset;
    static AVOutputSettingsPresetHEVC3840x2160WithAlpha: &'static OutputSettingsPreset;
    #[api::available(macos = 26.0, ios = 26.0, visionos = 26.0)]
    static AVOutputSettingsPresetHEVC4320x2160: &'static OutputSettingsPreset;
    #[api::available(macos = 12.1, ios = 26.0, visionos = 26.0)]
    static AVOutputSettingsPresetHEVC7680x4320: &'static OutputSettingsPreset;
    static AVOutputSettingsPresetMVHEVC960x960: &'static OutputSettingsPreset;
    static AVOutputSettingsPresetMVHEVC1440x1440: &'static OutputSettingsPreset;
    #[api::available(macos = 26.0, ios = 26.0, visionos = 26.0)]
    static AVOutputSettingsPresetMVHEVC4320x4320: &'static OutputSettingsPreset;
    #[api::available(macos = 26.0, ios = 26.0, visionos = 26.0)]
    static AVOutputSettingsPresetMVHEVC7680x7680: &'static OutputSettingsPreset;

}

define_obj_type!(
    #[doc(alias = "AVOutputSettingsAssistant")]
    pub OutputSettingsAssistant(ns::Id)
);

impl OutputSettingsAssistant {
    define_cls!(AV_OUTPUT_SETTINGS_ASSISTANT);

    #[objc::msg_send(availableOutputSettingsPresets)]
    pub fn available_presets() -> arc::R<ns::Array<OutputSettingsPreset>>;

    #[objc::msg_send(outputSettingsAssistantWithPreset:)]
    pub fn with_preset(preset: &OutputSettingsPreset) -> Option<arc::R<Self>>;

    #[objc::msg_send(audioSettings)]
    pub fn audio_settings(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(videoSettings)]
    pub fn video_settings(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(outputFileType)]
    pub fn output_file_type(&self) -> arc::R<av::FileType>;
}

/// AVOutputSettingsAssistant_SourceInformation
#[cfg(feature = "cm")]
impl OutputSettingsAssistant {
    #[objc::msg_send(sourceAudioFormat)]
    pub fn src_audio_format(&self) -> Option<&cm::AudioFormatDesc>;

    #[objc::msg_send(setSourceAudioFormat:)]
    pub fn set_src_audio_format(&mut self, val: Option<&cm::AudioFormatDesc>);

    #[objc::msg_send(sourceVideoFormat)]
    pub fn src_video_format(&self) -> Option<&cm::VideoFormatDesc>;

    #[objc::msg_send(setSourceVideoFormat:)]
    pub fn set_src_video_format(&mut self, val: Option<&cm::VideoFormatDesc>);

    #[objc::msg_send(sourceVideoAverageFrameDuration)]
    pub fn src_video_average_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(setSourceVideoAverageFrameDuration:)]
    pub fn set_src_video_average_frame_duration(&mut self, val: cm::Time);
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_OUTPUT_SETTINGS_ASSISTANT: &'static objc::Class<OutputSettingsAssistant>;
}

#[cfg(test)]
mod tests {
    use crate::{av, objc};

    #[test]
    fn basics() {
        let presets = objc::ar_pool(|| av::OutputSettingsAssistant::available_presets());

        eprintln!(
            "available presets {presets:?} {}",
            presets.as_type_ref().retain_count()
        );
        let presets = av::OutputSettingsAssistant::available_presets();

        eprintln!(
            "available presets {presets:?} {}",
            presets.as_type_ref().retain_count()
        );

        let assistant =
            av::OutputSettingsAssistant::with_preset(&av::OutputSettingsPreset::h264_640x480())
                .unwrap();
        eprintln!(
            "assistant {assistant:?} {}",
            assistant.as_type_ref().retain_count()
        );

        let audio1 = assistant.audio_settings().unwrap();
        let audio2 = assistant.audio_settings().unwrap();
        unsafe {
            assert_ne!(
                audio1.as_type_ref().as_type_ptr(),
                audio2.as_type_ref().as_type_ptr()
            );
        }

        let _ftype = assistant.output_file_type();

        let video1 = assistant.video_settings().unwrap();
        eprintln!("video1 {video1:?}");

        assert!(assistant.src_audio_format().is_none());
        assert!(assistant.src_video_format().is_none());
    }
}
