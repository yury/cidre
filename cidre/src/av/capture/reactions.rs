use crate::{cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVCaptureReactionType")]
    pub ReactionType(ns::String)
);

impl ReactionType {
    /// Indicates a reaction which features a thumbs-up symbol.
    #[doc(alias = "AVCaptureReactionTypeThumbsUp")]
    #[inline]
    pub fn thumbs_up() -> &'static Self {
        unsafe { AVCaptureReactionTypeThumbsUp }
    }

    /// Indicates a reaction which features a thumbs-down symbol.
    #[doc(alias = "AVCaptureReactionTypeThumbsDown")]
    #[inline]
    pub fn thumbs_down() -> &'static Self {
        unsafe { AVCaptureReactionTypeThumbsDown }
    }

    /// Indicates a reaction which features balloons rising through the scene.
    #[doc(alias = "AVCaptureReactionTypeBalloons")]
    #[inline]
    pub fn ballons() -> &'static Self {
        unsafe { AVCaptureReactionTypeBalloons }
    }

    /// Indicates a reaction which features one or more heart symbols.
    #[doc(alias = "AVCaptureReactionTypeHeart")]
    #[inline]
    pub fn heart() -> &'static Self {
        unsafe { AVCaptureReactionTypeHeart }
    }

    #[doc(alias = "AVCaptureReactionTypeFireworks")]
    #[inline]
    pub fn fireworks() -> &'static Self {
        unsafe { AVCaptureReactionTypeFireworks }
    }

    #[doc(alias = "AVCaptureReactionTypeRain")]
    #[inline]
    pub fn rain() -> &'static Self {
        unsafe { AVCaptureReactionTypeRain }
    }

    #[doc(alias = "AVCaptureReactionTypeConfetti")]
    #[inline]
    pub fn confetti() -> &'static Self {
        unsafe { AVCaptureReactionTypeConfetti }
    }

    #[doc(alias = "AVCaptureReactionTypeLasers")]
    #[inline]
    pub fn lasers() -> &'static Self {
        unsafe { AVCaptureReactionTypeLasers }
    }

    #[doc(alias = "AVCaptureReactionSystemImageNameForType")]
    #[inline]
    pub fn sys_image_name(&self) -> &ns::String {
        unsafe { AVCaptureReactionSystemImageNameForType(self) }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVCaptureReactionTypeThumbsUp: &'static ReactionType;
    static AVCaptureReactionTypeThumbsDown: &'static ReactionType;
    static AVCaptureReactionTypeBalloons: &'static ReactionType;
    static AVCaptureReactionTypeHeart: &'static ReactionType;
    static AVCaptureReactionTypeFireworks: &'static ReactionType;
    static AVCaptureReactionTypeRain: &'static ReactionType;
    static AVCaptureReactionTypeConfetti: &'static ReactionType;
    static AVCaptureReactionTypeLasers: &'static ReactionType;

    fn AVCaptureReactionSystemImageNameForType(reacton_type: &ReactionType) -> &ns::String;
}

define_obj_type!(
    #[doc(alias = "AVCaptureReactionEffectState")]
    pub EffectState(ns::Id)
);

impl EffectState {
    /// Indicates the reaction which is running.
    #[objc::msg_send(reactionType)]
    pub fn reaction_type(&self) -> &ReactionType;

    /// Provides the presentation time of the first frame where the effect is being rendered.
    #[objc::msg_send(startTime)]
    pub fn start_time(&self) -> cm::Time;

    /// Provides the presentation time of the frame following
    /// the last frame where the effect is seen.
    #[objc::msg_send(endTime)]
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
