use crate::{api, arc, ca, cf, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CAMetalDisplayLinkUpdate")]
    pub MetalDisplayLinkUpdate(ns::Id)
);

impl MetalDisplayLinkUpdate {
    #[objc::msg_send(drawable)]
    pub fn drawable(&self) -> arc::R<ca::AnyMetalDrawable>;

    #[objc::msg_send(targetTimestamp)]
    pub fn target_ts(&self) -> cf::TimeInterval;

    #[objc::msg_send(targetPresentationTimestamp)]
    pub fn target_pts(&self) -> cf::TimeInterval;
}

#[objc::protocol(CAMetalDisplayLinkDelegate)]
pub trait MetalDisplayLinkDelegate: objc::Obj {
    #[objc::msg_send(metalDisplayLink:needsUpdate:)]
    fn metal_display_link_needs_update(
        &mut self,
        link: &mut ca::MetalDisplayLink,
        update: &ca::MetalDisplayLinkUpdate,
    );
}

define_obj_type!(
    pub AnyMetalDisplayLinkDelegate(ns::Id)
);

impl MetalDisplayLinkDelegate for AnyMetalDisplayLinkDelegate {}

define_obj_type!(
    #[doc(alias = "CAMetalDisplayLink")]
    pub MetalDisplayLink(ns::Id),
    CA_METAL_DISPLAY_LINK,
    #[api::available(macos = 14.0, maccatalyst = 17.0, ios = 17.0, tvos = 17.0, visionos = 1.0)]
);

impl arc::A<MetalDisplayLink> {
    #[objc::msg_send(initWithMetalLayer:)]
    pub fn init_with_metal_layer(self, layer: &ca::MetalLayer) -> arc::R<MetalDisplayLink>;
}

impl MetalDisplayLink {
    pub fn with_layer(layer: &ca::MetalLayer) -> arc::R<Self> {
        Self::alloc().init_with_metal_layer(layer)
    }

    #[objc::msg_send(addToRunLoop:forMode:)]
    pub fn add_to_run_loop(&self, run_loop: &ns::RunLoop, mode: &ns::RunLoopMode);

    #[objc::msg_send(removeFromRunLoop:forMode:)]
    pub fn remove_from_run_loop(&self, run_loop: &ns::RunLoop, mode: &ns::RunLoopMode);

    #[objc::msg_send(invalidate)]
    pub fn invalidate(&mut self);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyMetalDisplayLinkDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: MetalDisplayLinkDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(preferredFrameLatency)]
    pub fn preferred_frame_latency(&self) -> f32;

    #[objc::msg_send(setPreferredFrameLatency:)]
    pub fn set_preferred_frame_latency(&mut self, val: f32);

    #[objc::msg_send(preferredFrameRateRange)]
    pub fn preferred_frame_rate_range(&self) -> ca::FrameRateRange;

    #[objc::msg_send(setPreferredFrameRateRange:)]
    pub fn set_preferred_frame_rate_range(&mut self, val: ca::FrameRateRange);

    #[objc::msg_send(isPaused)]
    pub fn is_paused(&self) -> bool;

    #[objc::msg_send(setPaused:)]
    pub fn set_paused(&mut self, val: bool);
}

unsafe extern "C" {
    static CA_METAL_DISPLAY_LINK: &'static objc::Class<MetalDisplayLink>;
}
