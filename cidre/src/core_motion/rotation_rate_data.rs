use crate::{core_motion as cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CMRotationRateData")]
    pub RotationRateData(cm::LogItem)
);

impl RotationRateData {
    #[objc::msg_send(rotationRate)]
    pub fn rotation_rate(&self) -> cm::RotationRate;
}

define_obj_type!(
    #[doc(alias = "CMRecordedRotationRateData")]
    pub RecordedRotationRateData(cm::RotationRateData)
);

impl RecordedRotationRateData {
    #[objc::msg_send(startDate)]
    pub fn start_date(&self) -> &ns::Date;
}
