use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CLCondition")]
    pub Condition(ns::Id)
);

impl Condition {
    #[objc::available(macos = 14.0, ios = 17.0)]
    crate::define_cls!(CL_CONDITION);
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe extern "C" {
    static CL_CONDITION: &'static objc::Class<Condition>;
}
