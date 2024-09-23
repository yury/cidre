use crate::{blocks, cf, cg, define_cf_type, os};

#[doc(alias = "CGImageSourceAnimationBlock")]
pub type AnimationBlock<Attr> = blocks::Block<fn(usize, &cg::Image, &mut bool), Attr>;

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

define_cf_type!(OptKey(cf::String));

impl OptKey {
    /// Starts the animation at the given index. Defaults to 0
    /// Value is a 'cf::Number'
    #[doc(alias = "kCGImageAnimationStartIndex")]
    pub fn start_index() -> &'static OptKey {
        unsafe { kCGImageAnimationStartIndex }
    }

    /// The value of this key overrides the `delay time' specified by the image
    /// Value is a 'cf::Number' of 'cf::NumberType::DOUBLE'
    #[doc(alias = "kCGImageAnimationDelayTime")]
    pub fn delay_time() -> &'static OptKey {
        unsafe { kCGImageAnimationDelayTime }
    }

    /// The value of this key overrides the `loop count' specified by the image
    /// Value is a 'cf::Number';  'cf::Number::positive_infinity()' may be used.
    #[doc(alias = "kCGImageAnimationLoopCount")]
    pub fn loop_count() -> &'static OptKey {
        unsafe { kCGImageAnimationLoopCount }
    }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CGAnimateImageAtURLWithBlock")]
#[inline]
pub fn animate_image_at_url_with_block(
    url: &cf::Url,
    options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
    block: &mut cg::ImageAnimationBlock<blocks::Esc>,
) -> os::Result {
    unsafe { CGAnimateImageAtURLWithBlock(url, options, block).result() }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CGAnimateImageAtURLWithBlock")]
#[inline]
pub fn animate_image_at_url(
    url: &cf::Url,
    options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
    block: &mut cg::ImageAnimationBlock<blocks::Esc>,
) -> os::Result {
    unsafe { CGAnimateImageAtURLWithBlock(url, options, block).result() }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CGAnimateImageDataWithBlock")]
#[inline]
pub fn animate_image_data_with_block(
    data: &cf::Data,
    options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
    block: &mut cg::ImageAnimationBlock<blocks::Esc>,
) -> os::Result {
    unsafe { CGAnimateImageDataWithBlock(data, options, block).result() }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CGAnimateImageDataWithBlock")]
#[inline]
pub fn animate_image_data(
    data: &cf::Data,
    options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
    block: &mut cg::ImageAnimationBlock<blocks::Esc>,
) -> os::Result {
    unsafe { CGAnimateImageDataWithBlock(data, options, block).result() }
}

#[link(name = "ImageIO", kind = "framework")]
extern "C-unwind" {
    static kCGImageAnimationStartIndex: &'static OptKey;
    static kCGImageAnimationDelayTime: &'static OptKey;
    static kCGImageAnimationLoopCount: &'static OptKey;

    fn CGAnimateImageAtURLWithBlock(
        url: &cf::Url,
        options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
        block: &mut cg::ImageAnimationBlock<blocks::Esc>,
    ) -> os::Status;

    fn CGAnimateImageDataWithBlock(
        data: &cf::Data,
        options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
        block: &mut cg::ImageAnimationBlock<blocks::Esc>,
    ) -> os::Status;

}
