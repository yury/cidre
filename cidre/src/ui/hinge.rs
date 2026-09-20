use crate::{cg, define_obj_type, ns, objc};

/// The status of an individual hinge.
#[doc(alias = "UIHingeStatus")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum HingeStatus {
    /// The status of the hinge is unknown.
    Unknown = 0,
    /// The hinge is closed.
    Closed = 1,
    /// The hinge is partially open.
    PartiallyOpen = 2,
    /// The hinge is open as far as the device allows.
    FullyOpen = 3,
}

define_obj_type!(
    /// The state of a single hinge.
    ///
    /// Observe it by adding a [`ui::HingeInteraction`](crate::ui::HingeInteraction)
    /// to a view and reading it from the update delivered to its handler.
    #[doc(alias = "UIHinge")]
    pub Hinge(ns::Id)
);

impl ns::Copying for Hinge {}

impl Hinge {
    #[objc::msg_send(status)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn status(&self) -> HingeStatus;

    /// The current angle of the hinge, in radians.
    ///
    /// The rate and granularity of angle updates are system policy. If you only
    /// need to know whether the hinge is closed, partially open, or fully open,
    /// prefer `status` over the angle.
    #[objc::msg_send(angle)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn angle(&self) -> cg::Float;
}
