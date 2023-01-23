use std::ffi::c_void;

use crate::{arc, ca, cf, define_obj_type, ns, objc, objc::Delegate};

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
    #[objc::msg_send(addToRunLoop:forMode:)]
    pub fn add_to_run_loop_for_mode(&self, runloop: &cf::RunLoop, mode: &cf::RunLoopMode);

    /// Removes the receiver from the given mode of the runloop. This will
    /// implicitly release it when removed from the last mode it has been
    /// registered for.
    #[objc::msg_send(removeFromRunLoop:forMode:)]
    pub fn remove_from_run_loop_for_mode(&self, runloop: &cf::RunLoop, mode: &cf::RunLoopMode);

    /// Removes the object from all runloop modes (releasing the receiver if
    /// it has been implicitly retained) and releases the 'target' object
    #[objc::msg_send(invalidate)]
    pub fn invalidate(&self);

    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> cf::TimeInterval;

    #[objc::msg_send(duration)]
    pub fn duration(&self) -> cf::TimeInterval;

    /// The time interval that represents when the next frame displays.
    #[objc::msg_send(targetTimestamp)]
    pub fn target_timestamp(&self) -> cf::TimeInterval;

    /// Defines the range of desired callback rate in frames-per-second for this
    /// display link. If the range contains the same minimum and maximum frame rate,
    /// this property is identical as preferredFramesPerSecond. Otherwise, the actual
    /// callback rate will be dynamically adjusted to better align with other
    /// animation sources.
    #[objc::msg_send(preferredFrameRateRange)]
    pub fn preferred_frame_rate_range(&self) -> ca::FrameRateRange;

    #[objc::msg_send(setPreferredFrameRateRange:)]
    pub fn set_preferred_frame_rate_range(&self, value: ca::FrameRateRange);
}

#[link(name = "ca", kind = "static")]
extern "C" {
    fn make_display_link_delegate(vtable: *const *const c_void) -> arc::R<ns::Id>;
}
