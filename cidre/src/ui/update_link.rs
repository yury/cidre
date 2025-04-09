use crate::{api, arc, ca, define_cls, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "UIUpdateLink")]
    pub UpdateLink(ns::Id)
);

/// Allows to formally participate in UI updates and influence UI update behavior.
impl UpdateLink {
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    define_cls!(UI_UPDATE_LINK);

    #[objc::msg_send(updateLinkForWindowScene:)]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_window_scene(window_scene: &ui::WindowScene) -> arc::R<Self>;

    #[objc::msg_send(updateLinkForView:)]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_view(view: &ui::View) -> arc::R<Self>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(addActionToPhase:handler:)]
    pub fn add_action_to_phase_block(
        &mut self,
        phase: &ui::UpdateActionPhase,
        handler: &mut blocks::EscBlock<fn(update_link: &mut Self, update_info: &ui::UpdateInfo)>,
    );

    #[cfg(feature = "blocks")]
    pub fn add_action_to_phase(
        &mut self,
        phase: &ui::UpdateActionPhase,
        handler: impl FnMut(&mut Self, &ui::UpdateInfo) + 'static,
    ) {
        let mut handler = blocks::EscBlock::new2(handler);
        self.add_action_to_phase_block(phase, &mut handler);
    }

    /// Varitant with target+selector pair. It will create block inside
    #[objc::msg_send(addActionToPhase:target:selector:)]
    pub fn add_action_to_phase_selector(
        &mut self,
        phase: &ui::UpdateActionPhase,
        target: &ns::Id,
        selector: &objc::Sel,
    );

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    /// It's required to enable the Update Link for it to have effect and for its actions to be invoked.
    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(requiresContinuousUpdates)]
    pub fn requires_continuous_updates(&self) -> bool;

    /// By default, `ui::UpdateLink` is a passive UI update observer. Its actions will only be called when UI update is being
    /// produced. When this property is set to `true`, `ui::UpdateLink` will request continuous UI updates by itself.
    #[objc::msg_send(setRequiresContinuousUpdates:)]
    pub fn set_requires_continuous_updates(&mut self, val: bool);

    /// Request dispatch of low-latency eligible events in `low_latency_event_dispatch` phase. Low latency eligible events are
    /// dispatch in the middle of the UI update, meaning that to handle them application has half the time, compared to
    /// events dispatched normally. Consult `ui::UpdateInfo.completion_deadline_time()` for exact completion deadline time.
    #[objc::msg_send(wantsLowLatencyEventDispatch)]
    pub fn wants_low_latency_event_dispatch(&self) -> bool;

    #[objc::msg_send(setWantsLowLatencyEventDispatch:)]
    pub fn set_wants_low_latency_event_dispatch(&mut self, val: bool);

    #[objc::msg_send(wantsImmediatePresentation)]
    pub fn wants_immediate_presentation(&self) -> bool;

    /// Request immediate frame presentation. When enabled, system will request immediate rendering of the display frame
    /// after last `ca::Transaction` commit for the current UI update. This allows to reduce input to display latency, as
    /// rendered display frame will be presented one frame duration sooner. However, for this to happen amount of work
    /// submitted to render server should be minimal, otherwise it will not be able to submit frame for presentation in
    /// time. This capability is primarily useful for pencil drawing applications where low input to display latency is
    /// critical for good user experience. Applications that request immediate presentation must be profiled thoroughly to
    /// ensure that amount of application and render server work is adequate. When application requests immediate
    /// presentation, but fails to keep work complexity at minimum, user will experience on screen judder, as frames will
    /// not be presented at their intended time.
    #[objc::msg_send(setWantsImmediatePresentation:)]
    pub fn set_wants_immediate_presentation(&mut self, val: bool);

    #[objc::msg_send(preferredFrameRateRange)]
    pub fn preferred_frame_rate_range(&self) -> ca::FrameRateRange;

    /// Preferred frame rate range. Even when not forcing periodic updates, this will still express intention to the system.
    /// Use `CAFrameRateRangeDefault` (default value) to not request any specific frame rate range.
    #[objc::msg_send(setPreferredFrameRateRange:)]
    pub fn set_preferred_frame_rate_range(&mut self, val: ca::FrameRateRange);

    /// During UI update, returns `ui::UpdateInfo` instance describing current UI update state. Returns `nil` outside of UI
    /// update.
    #[objc::msg_send(currentUpdateInfo)]
    pub fn current() -> Option<arc::R<Self>>;
}

/// Convenience
impl UpdateLink {
    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[cfg(feature = "blocks")]
    #[objc::msg_send(addActionWithHandler:)]
    pub fn add_action_block(
        &mut self,
        handler: &mut blocks::EscBlock<fn(update_link: &mut Self, update_info: &ui::UpdateInfo)>,
    );

    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[cfg(feature = "blocks")]
    pub fn add_action(&mut self, handler: impl FnMut(&mut Self, &ui::UpdateInfo) + 'static) {
        let mut handler = blocks::EscBlock::new2(handler);
        self.add_action_block(&mut handler);
    }

    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[objc::msg_send(addActionWithTarget:selector:)]
    pub fn add_action_selector(&mut self, target: &ns::Id, selector: &objc::Sel);

    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[cfg(feature = "blocks")]
    #[objc::msg_send(updateLinkForWindowScene:actionHandler:)]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_window_scene_handler_block(
        window_scene: &ui::WindowScene,
        handler: &mut blocks::EscBlock<fn(update_link: &mut Self, update_info: &ui::UpdateInfo)>,
    ) -> arc::R<Self>;

    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[objc::msg_send(updateLinkForWindowScene:actionTarget:selector:)]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_window_scene_selector(
        window_scene: &ui::WindowScene,
        action_target: &ns::Id,
        selector: &objc::Sel,
    ) -> arc::R<Self>;

    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[cfg(feature = "blocks")]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_window_scene_handler(
        window_scene: &ui::WindowScene,
        handler: impl FnMut(&mut Self, &ui::UpdateInfo) + 'static,
    ) -> arc::R<Self> {
        let mut handler = blocks::EscBlock::new2(handler);
        Self::with_window_scene_handler_block(window_scene, &mut handler)
    }

    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[cfg(feature = "blocks")]
    #[objc::msg_send(updateLinkForView:actionHandler:)]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_view_handler_block(
        view: &ui::View,
        handler: &mut blocks::EscBlock<fn(update_link: &mut Self, update_info: &ui::UpdateInfo)>,
    ) -> arc::R<Self>;

    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[cfg(feature = "blocks")]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_view_handler(
        view: &ui::View,
        handler: impl FnMut(&mut Self, &ui::UpdateInfo) + 'static,
    ) -> arc::R<Self> {
        let mut handler = blocks::EscBlock::new2(handler);
        Self::with_view_handler_block(view, &mut handler)
    }

    /// Adds action to `ui::UpdateActionPhase.before_ca_display_link_dispatch` phase
    #[objc::msg_send(updateLinkForView:actionTarget:selector:)]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_view_selector(
        view: &ui::View,
        action_target: &ns::Id,
        selector: &objc::Sel,
    ) -> arc::R<Self>;
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_UPDATE_LINK: &'static objc::Class<UpdateLink>;
}
