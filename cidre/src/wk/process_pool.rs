use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "WKProcessPool")]
    pub ProcessPool(ns::Id),
    WK_PROCESS_POOL
);

#[link(name = "wk", kind = "static")]
unsafe extern "C" {
    static WK_PROCESS_POOL: &'static objc::Class<ProcessPool>;
}
