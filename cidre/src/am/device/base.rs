use crate::{cf, define_cf_type};

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct Error(pub i32);

define_cf_type!(Device(cf::Type));
define_cf_type!(ServiceConnection(cf::Type));

define_cf_type!(Notification(cf::Type));

define_cf_type!(PreflightOperation(cf::Type));
