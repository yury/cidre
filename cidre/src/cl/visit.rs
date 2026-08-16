use crate::{arc, cl, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CLVisit")]
    pub Visit(ns::Id)
);

impl Visit {
    #[objc::available(macos = 10.15, ios = 8.0)]
    crate::define_cls!(CL_VISIT);

    #[objc::msg_send(arrivalDate)]
    pub fn arrival_date(&self) -> arc::R<ns::Date>;

    #[objc::msg_send(departureDate)]
    pub fn departure_date(&self) -> arc::R<ns::Date>;

    #[objc::msg_send(coordinate)]
    pub fn coordinate(&self) -> cl::LocationCoordinate2d;

    #[objc::msg_send(horizontalAccuracy)]
    pub fn horizontal_accuracy(&self) -> cl::LocationAccuracy;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe extern "C" {
    static CL_VISIT: &'static objc::Class<Visit>;
}
