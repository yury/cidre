use crate::{arc, core_motion as cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CMAltitudeData")]
    pub AltitudeData(cm::LogItem)
);

impl AltitudeData {
    /// The relative altitude in meters to the starting altitude.
    #[objc::msg_send(relativeAltitude)]
    pub fn relative_altitude(&self) -> arc::R<ns::Number>;

    /// The pressure in kPa.
    #[objc::msg_send(pressure)]
    pub fn pressure(&self) -> arc::R<ns::Number>;
}
