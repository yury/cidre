use std::ffi::c_void;

use crate::{ca, cf, define_obj_type, msg_send, ns, objc::Delegate};

define_obj_type!(DisplayLink(ns::Id));

pub trait DisplayLinkDelegate {
    extern "C" fn on_display_link(&mut self, link: &DisplayLink);

    fn delegate(self) -> Delegate<Self>
    where
        Self: Sized,
    {
        let b = Box::new(self);
        let table: [*const c_void; 2] = [
            b.as_ref() as *const _ as _,
            // &self as *const _ as *const _,
            Self::on_display_link as _,
        ];

        let ptr = table.as_ptr();
        let obj = unsafe { make_display_link_delegate(ptr as _) };

        Delegate { delegate: b, obj }
    }
}

impl DisplayLink {
    /// Adds the receiver to the given run-loop and mode. Unless paused, it
    /// will fire every vsync until removed. Each object may only be added
    /// to a single run-loop, but it may be added in multiple modes at once.
    /// While added to a run-loop it will implicitly be retained.
    pub fn add_to_run_loop_for_mode(&self, runloop: &cf::RunLoop, mode: &cf::RunLoopMode) {
        unsafe { wsel_addToRunLoop_forMode(self, runloop, mode) }
    }

    /// Removes the receiver from the given mode of the runloop. This will
    /// implicitly release it when removed from the last mode it has been
    /// registered for.
    pub fn add_remove_from_run_loop_for_mode(&self, runloop: &cf::RunLoop, mode: &cf::RunLoopMode) {
        unsafe { wsel_removeFromRunLoop_forMode(self, runloop, mode) }
    }

    /// Removes the object from all runloop modes (releasing the receiver if
    /// it has been implicitly retained) and releases the 'target' object
    #[inline]
    pub fn invalidate(&self) {
        msg_send!("common", self, sel_invalidate)
    }

    #[inline]
    pub fn timestamp(&self) -> cf::TimeInterval {
        unsafe { rsel_timestamp(self) }
    }

    #[inline]
    pub fn duration(&self) -> cf::TimeInterval {
        unsafe { rsel_duration(self) }
    }

    /// The time interval that represents when the next frame displays.
    #[inline]
    pub fn target_timestamp(&self) -> cf::TimeInterval {
        unsafe { rsel_targetTimestamp(self) }
    }

    /// Defines the range of desired callback rate in frames-per-second for this
    /// display link. If the range contains the same minimum and maximum frame rate,
    /// this property is identical as preferredFramesPerSecond. Otherwise, the actual
    /// callback rate will be dynamically adjusted to better align with other
    /// animation sources.
    #[inline]
    pub fn preferred_frame_rate_range(&self) -> ca::FrameRateRange {
        unsafe { rsel_preferredFrameRateRange(self) }
    }

    #[inline]
    pub fn set_preferred_frame_rate_range(&self, value: ca::FrameRateRange) {
        unsafe { wsel_setPreferredFrameRateRange(self, value) }
    }
}

#[link(name = "ca", kind = "static")]
extern "C" {
    fn make_display_link_delegate(vtable: *const *const c_void) -> cf::Retained<ns::Id>;

    fn wsel_addToRunLoop_forMode(link: &DisplayLink, runloop: &cf::RunLoop, mode: &cf::RunLoopMode);
    fn wsel_removeFromRunLoop_forMode(
        link: &DisplayLink,
        runloop: &cf::RunLoop,
        mode: &cf::RunLoopMode,
    );

    fn rsel_timestamp(link: &DisplayLink) -> cf::TimeInterval;
    fn rsel_duration(link: &DisplayLink) -> cf::TimeInterval;
    fn rsel_targetTimestamp(link: &DisplayLink) -> cf::TimeInterval;
    fn rsel_preferredFrameRateRange(link: &DisplayLink) -> ca::FrameRateRange;
    fn wsel_setPreferredFrameRateRange(link: &DisplayLink, value: ca::FrameRateRange);
}
