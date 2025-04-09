use crate::{api, arc, define_cls, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIUpdateInfo")]
    pub UpdateInfo(ns::Id)
);

/// Contains detailed information about the current state of the UI update. This information may change as UI update
/// progresses through its phases. Note, that single UI update might service views on different displays simultaneously,
/// in which case such views may have different `ui::UpdateInfo` (e.g. `estimatedPresentationTime` may differ).
impl UpdateInfo {
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    define_cls!(UI_UPDATE_INFO);

    #[objc::msg_send(currentUpdateInfoForWindowScene:)]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_window_scene(window_scene: &ui::WindowScene) -> Option<arc::R<Self>>;

    #[objc::msg_send(currentUpdateInfoForView:)]
    #[api::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn with_view(view: &ui::View) -> Option<arc::R<Self>>;

    /// Reference time that is suitable for driving time based model changes, like animations or physics. Use it as "now"
    /// time for the UI update. It's designed to maintain constant latency between model changes and their on screen
    /// presentation. Uses same units as `CACurrentMediaTime()`. Numerically, this time is close to the start of the UI
    /// update, but its exact relation to UI update start time may change depending on frame rate and other UI update
    /// parameters.
    #[objc::msg_send(modelTime)]
    pub fn model_time(&self) -> ns::TimeInterval;

    /// Time by which application has to be done submitting changes to the render server. Missing this completion deadline
    /// will result in a presentation delay. Single miss will look like a frame drop, missing repeatedly will look like
    /// judder.
    #[objc::msg_send(completionDeadlineTime)]
    pub fn completion_deadline_time(&self) -> ns::TimeInterval;

    /// Estimated time when UI update changes will become visible on screen. Actual time when pixels change color may
    /// differ.
    #[objc::msg_send(estimatedPresentationTime)]
    pub fn estimated_presentation_time(&self) -> ns::TimeInterval;

    /// `true` for UI updates that are expected to present immediately upon completion. Use it to minimize amount of work
    /// performed during the UI update. Any processing that is not critical for the frame being presented should be deferred
    /// to after UI update is complete. Note, that immediate presentation still might not happen if strict conditions
    /// imposed by the system, like committing `CATransaction` before the `completionDeadlineTime`, are not satisfied.
    /// Similarly, immediate presentation can be denied at various points of the pipeline, if system detects that current
    /// CPU or GPU load, power state or frame complexity make reliable immediate presentation impossible or unlikely.
    /// Immediate presentation is an extremely challenging mode for the entire system and causes excessive power drain and
    /// has high chances of missing intended presentation time, which results in visual judder. Application that use it
    /// has high chances of missing intended presentation time, which results in visual judder. Applications that use it
    /// should be explicitly designed and tuned to operate in this mode - amount of work in each phase should be precisely
    /// controlled. It is primarily reserved for pencil drawing and writing applications where extra low latency makes a
    /// noticeable improvement to user experience. Returned value can change during the UI update.
    #[objc::msg_send(isImmediatePresentationExpected)]
    pub fn is_immediate_presentation_expected(&self) -> bool;

    /// `true` when it's guaranteed that low-latency event dispatch will happen during the UI update. When `false` is returned,
    /// you can rely on low-latency UI update phases to run for this UI update. Use it to avoid doing the same work more
    /// than once. For example, when rendering a pencil drawing stroke in after event dispatch and
    /// `is_low_latency_event_dispatch_confirmed` is `true`, while `is_performing_low_latency_phases` is `false`, then it would be better
    /// to wait for after low-latency event dispatch to render the stroke. Can change from `false` to `true` during the UI
    /// update, but will never change from `true` to `false`. When `true` is returned, low-latency phases always will be
    /// performed. Note, that checking value of this property might cause system to commit to low-latency event dispatch
    /// unnecessarily as a side effect - call it only when there's an intention to act on returned value.
    #[objc::msg_send(isLowLatencyEventDispatchConfirmed)]
    pub fn is_low_latency_event_dispatch_confirmed(&self) -> bool;

    /// `true` when executing low-latency part of the UI update (specifically between `low_latency_event_dispatch` and
    /// `low_latency_ca_transaction_commit` UI update phases). Work in this part of the UI update should be as minimal as
    /// possible, especially when immediate presentation is to be attempted. Anything that is not critical to the current
    /// UI update must be deferred after `low_latency_ca_transaction_commit`. Try to avoid using `dispatch_after()` types of
    /// deferral as arbitrary delayed work will potentially interfere with following UI updates.
    #[objc::msg_send(isPerformingLowLatencyPhases)]
    pub fn is_performing_low_latency_phases(&self) -> bool;
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_UPDATE_INFO: &'static objc::Class<UpdateInfo>;
}
