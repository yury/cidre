use crate::{arc, cg, define_obj_type, define_opts, ns, objc, ui};

#[doc(alias = "UITouchPhase")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TouchPhase {
    Began,
    Moved,
    Stationary,
    Ended,
    Cancelled,
    RegionEntered,
    RegionMoved,
    RegionExited,
}

#[doc(alias = "UIForceTouchCapability")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ForceTouchCapability {
    Unknown = 0,
    Unavailable = 1,
    Available = 2,
}

#[doc(alias = "UITouchType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TouchType {
    Direct,
    Indirect,
    Pencil,
    IndirectPointer,
}

define_opts!(
    #[doc(alias = "UITouchProperties")]
    pub TouchProps(isize)
);

impl TouchProps {
    pub const FORCE: Self = Self(1 << 0);
    pub const AZIMUTH: Self = Self(1 << 1);
    pub const ALTITUDE: Self = Self(1 << 2);
    pub const LOCATION: Self = Self(1 << 3);
    pub const ROLL: Self = Self(1 << 4);
}

define_obj_type!(
    pub Touch(ns::Id)
);

impl Touch {
    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> ns::TimeInterval;

    #[objc::msg_send(phase)]
    pub fn phase(&self) -> TouchPhase;

    #[objc::msg_send(tapCount)]
    pub fn tap_count(&self) -> usize;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> TouchType;

    #[objc::msg_send(majorRadius)]
    pub fn major_radius(&self) -> cg::Float;

    #[objc::msg_send(majorRadiusTolerance)]
    pub fn major_radius_tolerance(&self) -> cg::Float;

    #[objc::msg_send(window)]
    pub fn window(&self) -> Option<arc::R<ui::Window>>;

    #[objc::msg_send(view)]
    pub fn view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(gestureRecognizers)]
    pub fn gesture_recognizers(&self) -> Option<arc::R<ns::Array<ui::GestureRecognizer>>>;

    #[objc::msg_send(locationInView:)]
    pub fn location_in_view(&self, view: Option<&ui::View>) -> cg::Point;

    #[objc::msg_send(previousLocationInView:)]
    pub fn prev_location_in_view(&self, view: Option<&ui::View>) -> cg::Point;

    #[objc::msg_send(preciseLocationInView:)]
    pub fn precise_location_in_view(&self, view: Option<&ui::View>) -> cg::Point;

    #[objc::msg_send(precisePreviousLocationInView:)]
    pub fn precise_prev_location_in_view(&self, view: Option<&ui::View>) -> cg::Point;

    #[objc::msg_send(force)]
    pub fn force(&self) -> cg::Float;

    #[objc::msg_send(maximumPossibleForce)]
    pub fn max_possible_force(&self) -> cg::Float;

    #[objc::msg_send(azimuthAngleInView:)]
    pub fn azimuth_angle_in_view(&self, view: Option<&ui::View>) -> cg::Float;

    #[objc::msg_send(azimuthUnitVectorInView:)]
    pub fn azimuth_unit_vec_in_view(&self, view: Option<&ui::View>) -> cg::Vector;

    #[objc::msg_send(altitudeAngle)]
    pub fn altitude_angle(&self) -> cg::Float;

    #[objc::msg_send(estimationUpdateIndex)]
    pub fn estimation_update_index(&self) -> Option<arc::R<ns::Number>>;

    #[objc::msg_send(estimatedProperties)]
    pub fn estimated_props(&self) -> TouchProps;

    #[objc::msg_send(estimatedPropertiesExpectingUpdates)]
    pub fn estimated_props_expecting_updates(&self) -> TouchProps;

    #[objc::msg_send(rollAngle)]
    pub fn roll_angle(&self) -> cg::Float;
}
