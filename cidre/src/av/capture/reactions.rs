use crate::{define_obj_type, ns};

define_obj_type!(ReactionType(ns::String));

impl ReactionType {
    /// Indicates a reaction which features a thumbs-up symbol.
    #[inline]
    pub fn thumbs_up() -> &'static Self {
        unsafe { AVCaptureReactionTypeThumbsUp }
    }

    /// Indicates a reaction which features a thumbs-down symbol.
    #[inline]
    pub fn thumbs_down() -> &'static Self {
        unsafe { AVCaptureReactionTypeThumbsDown }
    }

    /// Indicates a reaction which features balloons rising through the scene.
    #[inline]
    pub fn ballons() -> &'static Self {
        unsafe { AVCaptureReactionTypeBalloons }
    }

    /// Indicates a reaction which features one or more heart symbols.
    #[inline]
    pub fn heart() -> &'static Self {
        unsafe { AVCaptureReactionTypeHeart }
    }

    #[inline]
    pub fn fireworks() -> &'static Self {
        unsafe { AVCaptureReactionTypeFireworks }
    }

    #[inline]
    pub fn rain() -> &'static Self {
        unsafe { AVCaptureReactionTypeRain }
    }

    #[inline]
    pub fn confetti() -> &'static Self {
        unsafe { AVCaptureReactionTypeConfetti }
    }

    #[inline]
    pub fn lasers() -> &'static Self {
        unsafe { AVCaptureReactionTypeLasers }
    }

    #[inline]
    pub fn system_image_name(&self) -> &ns::String {
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

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let name = av::CaptureReactionType::thumbs_up().system_image_name();
        assert_eq!(name, "hand.thumbsup.fill");

        let name = av::CaptureReactionType::thumbs_down().system_image_name();
        assert_eq!(name, "hand.thumbsdown.fill");
    }
}
