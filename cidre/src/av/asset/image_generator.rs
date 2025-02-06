use crate::{arc, av, cg, cm, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "AVAssetImageGeneratorResult")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ImageGeneratorResult {
    Succeeded = 0,
    Failed = 1,
    Cancelled = 2,
}

#[cfg(feature = "blocks")]
#[doc(alias = "AVAssetImageGeneratorCompletionHandler")]
pub type ImageGeneratorCh = blocks::EscBlock<
    fn(
        requested_time: cm::Time,
        image: Option<&cg::Image>,
        actual_time: cm::Time,
        result: ImageGeneratorResult,
        error: Option<&ns::Error>,
    ),
>;

define_obj_type!(
    #[doc(alias = "AVAssetImageGenerator")]
    pub ImageGenerator(ns::Id)
);

impl arc::A<ImageGenerator> {
    #[objc::msg_send(initWithAsset:)]
    pub fn init_with_asset(self, asset: &av::Asset) -> arc::R<ImageGenerator>;
}

impl ImageGenerator {
    crate::define_cls!(AV_ASSET_IMAGE_GENERATOR);

    pub fn with_asset(asset: &av::Asset) -> arc::R<Self> {
        Self::alloc().init_with_asset(asset)
    }

    /// Indicates the instance of av::Asset with which the av::AssetImageGenerator was initialized
    #[objc::msg_send(asset)]
    pub fn asset(&self) -> arc::R<av::Asset>;

    #[objc::msg_send(appliesPreferredTrackTransform)]
    pub fn applies_preferred_track_transform(&self) -> bool;

    /// Default is false.  Only rotation by 90, 180, or 270 degrees is supported.
    #[objc::msg_send(setAppliesPreferredTrackTransform:)]
    pub fn set_applies_preferred_track_transform(&mut self, val: bool);

    #[objc::msg_send(maximumSize)]
    pub fn max_size(&self) -> cg::Size;

    #[objc::msg_send(setMaximumSize:)]
    pub fn set_max_size(&mut self, val: cg::Size);

    #[objc::msg_send(requestedTimeToleranceBefore)]
    pub fn requested_time_tolerance_before(&self) -> cm::Time;

    #[objc::msg_send(setRequestedTimeToleranceBefore:)]
    pub fn set_requested_time_tolerance_before(&mut self, val: cm::Time);

    #[objc::msg_send(requestedTimeToleranceAfter)]
    pub fn requested_time_tolerance_after(&self) -> cm::Time;

    #[objc::msg_send(setRequestedTimeToleranceAfter:)]
    pub fn set_requested_time_tolerance_after(&mut self, val: cm::Time);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(generateCGImagesAsynchronouslyForTimes:completionHandler:)]
    pub fn cg_images_for_times_ch(
        &self,
        requested_times: &ns::Array<ns::Value>,
        ch: &mut ImageGeneratorCh,
    );

    #[cfg(feature = "blocks")]
    #[objc::msg_send(generateCGImageAsynchronouslyForTime:completionHandler:)]
    pub fn cg_image_for_time_ch(
        &self,
        request_time: cm::Time,
        ch: &mut blocks::EscBlock<
            fn(image: Option<&cg::Image>, actual_time: cm::Time, error: Option<&ns::Error>),
        >,
    );

    #[cfg(feature = "async")]
    pub async fn cg_image_for_time(
        &self,
        request_time: cm::Time,
    ) -> ns::Result<(arc::R<cg::Image>, cm::Time), arc::R<ns::Error>> {
        let shared = blocks::Shared::new();
        let comp = blocks::Completion::new(shared.clone());
        let mut block = blocks::EscBlock::new3(
            move |image: Option<&cg::Image>, actual_time: cm::Time, error: Option<&ns::Error>| {
                if let Some(err) = error {
                    shared.lock().ready(Err(err.retained()));
                } else {
                    let img = unsafe { image.unwrap_unchecked().retained() };
                    shared.lock().ready(Ok((img, actual_time)));
                }
            },
        );
        self.cg_image_for_time_ch(request_time, &mut block);
        comp.await
    }

    #[objc::msg_send(cancelAllCGImageGeneration)]
    pub fn cancel_all_cg_image_gen(&mut self);
}

extern "C" {
    static AV_ASSET_IMAGE_GENERATOR: &'static objc::Class<ImageGenerator>;
}
