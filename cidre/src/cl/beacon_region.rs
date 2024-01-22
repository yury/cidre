use crate::{cl, define_obj_type, ns, objc};

define_obj_type!(
    pub BeaconRegion(cl::Region)
);

define_obj_type!(
    pub Beacon(ns::Id)
);

impl Beacon {
    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> &ns::Date;

    #[objc::msg_send(UUID)]
    pub fn uuid(&self) -> &ns::Uuid;

    #[objc::msg_send(major)]
    pub fn major(&self) -> &ns::Number;

    #[objc::msg_send(minor)]
    pub fn minor(&self) -> &ns::Number;

    #[objc::msg_send(proximity)]
    pub fn proximity(&self) -> cl::Proximity;

    #[objc::msg_send(accuracy)]
    pub fn accuracy(&self) -> cl::LocationAccuracy;

    #[objc::msg_send(rssi)]
    pub fn rssi(&self) -> isize;
}
