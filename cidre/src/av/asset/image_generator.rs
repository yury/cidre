use std::future::Future;

use crate::{
    arc, av,
    blocks::{self, Completion, SendBlock, Shared},
    cg, cm, define_obj_type, ns, objc,
};

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

    #[objc::msg_send(generateCGImageAsynchronouslyForTime:completionHandler:)]
    pub fn cg_image_for_time_ch(
        &self,
        request_time: &cm::Time,
        ch: &mut blocks::EscBlock<
            fn(image: Option<&cg::Image>, actual_time: cm::Time, error: *mut Option<&ns::Error>),
        >,
    );

    // pub fn cg_image_for_time(
    //     &self,
    //     request_time: &cm::Time,
    // ) -> impl Future<Output = ns::Result<(arc::R<cg::Image>, cm::Time), arc::R<ns::Error>>> {
    //     let shared = Shared::new();
    //     let (a, b ) = (
    //         Completion(shared.clone()),
    //         SendBlock::new3(move |v: R| shared.lock().ready(v)),
    //     )

    //     b
    // }

    #[objc::msg_send(cancelAllCGImageGeneration)]
    pub fn cancel_all_cg_image_gen(&mut self);
}

extern "C" {
    static AV_ASSET_IMAGE_GENERATOR: &'static objc::Class<ImageGenerator>;
}
