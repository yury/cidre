use crate::{cf, os};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Status(os::Status);

impl Status {
    /// NULL or invalid parameter passed to API
    pub const PARAMAMETER_ERROR: Self = Self(os::Status(-22140));

    /// An image cannot be read from the given source
    pub const CORRUPT_INPUT_IMAGE: Self = Self(os::Status(-22141));

    /// The image format is not applicable to animation
    pub const UNSUPPORTED_FORMAT: Self = Self(os::Status(-22142));

    /// An image can be read from the given source, but it is incomplete
    pub const INCOMPLETE_INPUT_IMAGE: Self = Self(os::Status(-22143));

    /// A required resource could not be created
    pub const ALLOCATION_FAILURE: Self = Self(os::Status(-22143));
}

pub fn start_index() -> &'static cf::String {
    unsafe { kCGImageAnimationStartIndex }
}

pub fn delay_time() -> &'static cf::String {
    unsafe { kCGImageAnimationDelayTime }
}

pub fn loop_count() -> &'static cf::String {
    unsafe { kCGImageAnimationLoopCount }
}

#[link(name = "ImageIO", kind = "framework")]
extern "C" {
    static kCGImageAnimationStartIndex: &'static cf::String;
    static kCGImageAnimationDelayTime: &'static cf::String;
    static kCGImageAnimationLoopCount: &'static cf::String;
}
