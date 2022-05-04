use crate::{cf, define_cf_type};

define_cf_type!(Set(cf::Type));
define_cf_type!(MutableSet(Set));
