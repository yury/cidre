use crate::{api, cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVCaptureReactionType")]
    pub ReactionType(ns::String)
);

impl ReactionType {
    /// Indicates a reaction which features a thumbs-up symbol.
    #[doc(alias = "AVCaptureReactionTypeThumbsUp")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn thumbs_up() -> &'static Self {
        unsafe { AVCaptureReactionTypeThumbsUp }
    }

    /// Indicates a reaction which features a thumbs-down symbol.
    #[doc(alias = "AVCaptureReactionTypeThumbsDown")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn thumbs_down() -> &'static Self {
        unsafe { AVCaptureReactionTypeThumbsDown }
    }

    /// Indicates a reaction which features balloons rising through the scene.
    #[doc(alias = "AVCaptureReactionTypeBalloons")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn ballons() -> &'static Self {
        unsafe { AVCaptureReactionTypeBalloons }
    }

    /// Indicates a reaction which features one or more heart symbols.
    #[doc(alias = "AVCaptureReactionTypeHeart")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn heart() -> &'static Self {
        unsafe { AVCaptureReactionTypeHeart }
    }

    #[doc(alias = "AVCaptureReactionTypeFireworks")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn fireworks() -> &'static Self {
        unsafe { AVCaptureReactionTypeFireworks }
    }

    #[doc(alias = "AVCaptureReactionTypeRain")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn rain() -> &'static Self {
        unsafe { AVCaptureReactionTypeRain }
    }

    #[doc(alias = "AVCaptureReactionTypeConfetti")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn confetti() -> &'static Self {
        unsafe { AVCaptureReactionTypeConfetti }
    }

    #[doc(alias = "AVCaptureReactionTypeLasers")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn lasers() -> &'static Self {
        unsafe { AVCaptureReactionTypeLasers }
    }

    #[doc(alias = "AVCaptureReactionSystemImageNameForType")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn sys_image_name(&self) -> &ns::String {
        unsafe { AVCaptureReactionSystemImageNameForType(self) }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
#[api::weak]
extern "C" {
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    static AVCaptureReactionTypeThumbsUp: &'static ReactionType;

    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    static AVCaptureReactionTypeThumbsDown: &'static ReactionType;

    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    static AVCaptureReactionTypeBalloons: &'static ReactionType;

    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    static AVCaptureReactionTypeHeart: &'static ReactionType;

    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    static AVCaptureReactionTypeFireworks: &'static ReactionType;

    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    static AVCaptureReactionTypeRain: &'static ReactionType;

    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    static AVCaptureReactionTypeConfetti: &'static ReactionType;

    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    static AVCaptureReactionTypeLasers: &'static ReactionType;

    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    fn AVCaptureReactionSystemImageNameForType(reacton_type: &ReactionType) -> &ns::String;
}

define_obj_type!(
    #[doc(alias = "AVCaptureReactionEffectState")]
    pub EffectState(ns::Id)
);

impl EffectState {
    /// Indicates the reaction which is running.
    #[objc::msg_send(reactionType)]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn reaction_type(&self) -> &ReactionType;

    /// Provides the presentation time of the first frame where the effect is being rendered.
    #[objc::msg_send(startTime)]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn start_time(&self) -> cm::Time;

    /// Provides the presentation time of the frame following
    /// the last frame where the effect is seen.
    #[objc::msg_send(endTime)]
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0)]
    pub fn end_time(&self) -> cm::Time;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let name = av::CaptureReactionType::thumbs_up().sys_image_name();
        assert_eq!(name, "hand.thumbsup.fill");

        let name = av::CaptureReactionType::thumbs_down().sys_image_name();
        assert_eq!(name, "hand.thumbsdown.fill");
    }
}
