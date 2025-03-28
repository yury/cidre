use crate::{blocks, cf, cg, define_cf_type, os};

#[doc(alias = "CGImageSourceAnimationBlock")]
pub type AnimationBlock = blocks::EscBlock<fn(index: usize, image: &cg::Image, stop: &mut bool)>;

pub mod err {
    use crate::os::Error;

    /// None/null or invalid parameter passed to API
    #[doc(alias = "kCGImageAnimationStatus_ParameterError")]
    pub const PARAM_ERR: Error = Error::new_unchecked(-22140);

    /// An image cannot be read from the given source
    #[doc(alias = "kCGImageAnimationStatus_CorruptInputImage")]
    pub const CORRUPT_INPUT_IMAGE: Error = Error::new_unchecked(-22141);

    /// The image format is not applicable to animation
    #[doc(alias = "kCGImageAnimationStatus_UnsupportedFormat")]
    pub const UNSUPPORTED_FORMAT: Error = Error::new_unchecked(-22142);

    /// An image can be read from the given source, but it is incomplete
    #[doc(alias = "kCGImageAnimationStatus_IncompleteInputImage")]
    pub const INCOMPLETE_INPUT_IMAGE: Error = Error::new_unchecked(-22143);

    /// A required resource could not be created
    #[doc(alias = "kCGImageAnimationStatus_AllocationFailure")]
    pub const ALLOC_FAILURE: Error = Error::new_unchecked(-22143);
}

define_cf_type!(
    #[doc(alias = "CGImageAnimationOptions")]
    OptKey(cf::String)
);

pub type Opts = cf::DictionaryOf<OptKey, cf::Number>;

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

#[doc(alias = "CGAnimateImageAtURLWithBlock")]
#[inline]
pub fn animate_image_at_url_with_block(
    url: &cf::Url,
    options: Option<&Opts>,
    block: &mut cg::ImageAnimationBlock,
) -> os::Result {
    unsafe { CGAnimateImageAtURLWithBlock(url, options, block).result() }
}

#[doc(alias = "CGAnimateImageAtURLWithBlock")]
#[inline]
pub fn animate_image_at_url(
    url: &cf::Url,
    options: Option<&Opts>,
    f: impl FnMut(/* index: */ usize, /* image: */ &cg::Image, /* stop: */ &mut bool) + 'static,
) -> os::Result {
    let mut block = cg::ImageAnimationBlock::new3(f);
    unsafe { CGAnimateImageAtURLWithBlock(url, options, &mut block).result() }
}

#[doc(alias = "CGAnimateImageDataWithBlock")]
#[inline]
pub fn animate_image_data_with_block(
    data: &cf::Data,
    options: Option<&Opts>,
    block: &mut cg::ImageAnimationBlock,
) -> os::Result {
    unsafe { CGAnimateImageDataWithBlock(data, options, block).result() }
}

#[doc(alias = "CGAnimateImageDataWithBlock")]
#[inline]
pub fn animate_image_data(
    data: &cf::Data,
    options: Option<&Opts>,
    f: impl FnMut(/* index: */ usize, /* image: */ &cg::Image, /* stop: */ &mut bool) + 'static,
) -> os::Result {
    let mut block = cg::ImageAnimationBlock::new3(f);
    unsafe { CGAnimateImageDataWithBlock(data, options, &mut block).result() }
}

#[link(name = "ImageIO", kind = "framework")]
unsafe extern "C-unwind" {
    static kCGImageAnimationStartIndex: &'static OptKey;
    static kCGImageAnimationDelayTime: &'static OptKey;
    static kCGImageAnimationLoopCount: &'static OptKey;

    fn CGAnimateImageAtURLWithBlock(
        url: &cf::Url,
        options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
        block: &mut cg::ImageAnimationBlock,
    ) -> os::Status;

    fn CGAnimateImageDataWithBlock(
        data: &cf::Data,
        options: Option<&cf::DictionaryOf<OptKey, cf::Number>>,
        block: &mut cg::ImageAnimationBlock,
    ) -> os::Status;

}

#[cfg(test)]
mod tests {
    use crate::{cf, cg, os};

    const ERROR: os::Error = os::Error::new_unchecked(-50);

    #[test]
    fn error() {
        let url = cf::Url::from_str("foo").unwrap();
        match cg::animate_image_at_url(&url, None, |_idx, _img, _stp| {
            panic!("should not be called");
        }) {
            Err(ERROR) => {}
            x => panic!("failed {x:?}"),
        }

        let data = cf::Data::from_slice(&[]).unwrap();
        match cg::animate_image_data(&data, None, |_idx, _img, _stp| {
            panic!("should not be called");
        }) {
            Err(cg::image_animation_err::CORRUPT_INPUT_IMAGE) => {}
            x => panic!("failed {x:?}"),
        }
    }
}
