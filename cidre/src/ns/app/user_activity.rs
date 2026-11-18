use crate::{define_obj_type, ns, objc};

#[objc::protocol(NSUserActivityRestoring)]
pub trait UserActivityRestoring: objc::Obj {
    #[objc::msg_send(restoreUserActivityState:)]
    fn restore_user_activity_state(&mut self, activity: &ns::UserActivity);
}

define_obj_type!(pub AnyUserActivityRestoring(ns::Id));
impl UserActivityRestoring for AnyUserActivityRestoring {}
