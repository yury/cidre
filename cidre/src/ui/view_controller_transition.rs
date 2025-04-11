use crate::{arc, blocks, define_cls, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIViewControllerTransition")]
    pub ViewControllerTransition(ns::Id)
);

#[cfg(not(target_os = "watchos"))]
impl ViewControllerTransition {
    define_cls!(UI_VIEW_CONTROLLER_TRANSITION);

    #[objc::msg_send(zoomWithOptions:sourceViewProvider:)]
    #[objc::available(ios = 18.0, maccatalyst = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn zoom_block(
        options: Option<&ui::ZoomTransitionOpts>,
        src_view_provider: &mut blocks::EscBlock<
            fn(ctx: &mut ui::ZoomTransitionSrcViewProviderCtx) -> Option<arc::Rar<ui::View>>,
        >,
    ) -> arc::R<Self>;

    #[objc::available(ios = 18.0, maccatalyst = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn zoom(
        options: Option<&ui::ZoomTransitionOpts>,
        src_view_provider: impl FnMut(
            &mut ui::ZoomTransitionSrcViewProviderCtx,
        ) -> Option<arc::Rar<ui::View>>
        + 'static,
    ) -> arc::R<Self> {
        let mut block = blocks::EscBlock::new1(src_view_provider);
        Self::zoom_block(options, &mut block)
    }

    /// View slides up from the bottom of the screen. Same as `UIModalTransitionStyle.coverVertical`.
    #[objc::msg_send(flipHorizontalTransition)]
    #[objc::available(ios = 18.0, maccatalyst = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn flip_horizontal() -> arc::R<Self>;

    /// Fades out the current view while fading in the new view. Same as `UIModalTransitionStyle.crossDissolve`.
    #[objc::msg_send(crossDissolveTransition)]
    #[objc::available(ios = 18.0, maccatalyst = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn cross_dissolve() -> arc::R<Self>;

    /// One corner of the current view curls up to reveal the presented view underneath. Same as `UIModalTransitionStyle.partialCurl`.
    #[objc::msg_send(partialCurlTransition)]
    #[objc::available(ios = 18.0, maccatalyst = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn partial_curl() -> arc::R<Self>;
}

define_obj_type!(
    #[doc(alias = "UIZoomTransitionSourceViewProviderContext")]
    pub ZoomTransitionSrcViewProviderCtx(ns::Id)
);

#[cfg(not(target_os = "watchos"))]
impl ZoomTransitionSrcViewProviderCtx {
    #[objc::msg_send(sourceViewController)]
    pub fn src_vc(&self) -> arc::R<ui::ViewController>;

    #[objc::msg_send(zoomedViewController)]
    pub fn zoomed_vc(&self) -> arc::R<ui::ViewController>;
}

#[cfg(not(target_os = "watchos"))]
#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_VIEW_CONTROLLER_TRANSITION: &'static objc::Class<ViewControllerTransition>;
}
