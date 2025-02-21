use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVPlaybackSpeed")]
    pub PlaybackSpeed(ns::Id)
);

impl arc::A<PlaybackSpeed> {
    #[objc::msg_send(initWithRate:localizedName:)]
    pub fn init_with_rate(self, rate: f32, localized_name: &ns::String) -> arc::R<PlaybackSpeed>;
}

impl PlaybackSpeed {
    define_cls!(AV_PLAYBACK_SPEED);

    #[objc::msg_send(systemDefaultSpeeds)]
    fn sys_default_speeds() -> arc::R<ns::Array<Self>>;

    pub fn with_playback_rate(rate: f32, localized_name: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_rate(rate, localized_name)
    }

    #[objc::msg_send(rate)]
    pub fn rate(&self) -> f32;

    /// This name will be used to represent this playback speed in playback UIs where more space is available.
    #[objc::msg_send(localizedName)]
    pub fn localized_name(&self) -> arc::R<ns::String>;

    /// This name will be used to represent this playback speed in playback UIs where limited space is available.
    #[objc::msg_send(localizedNumericName)]
    pub fn localized_numeric_name(&self) -> arc::R<ns::String>;
}

#[link(name = "av_kit", kind = "static")]
unsafe extern "C" {
    static AV_PLAYBACK_SPEED: &'static objc::Class<PlaybackSpeed>;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let speeds = av::PlaybackSpeed::sys_default_speeds();
        assert!(speeds.len() > 0);
    }
}
