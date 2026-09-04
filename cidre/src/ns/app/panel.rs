use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSPanel")]
    pub Panel(ns::Window),
    NS_PANEL
);

unsafe extern "C" {
    static NS_PANEL: &'static objc::Class<Panel>;
}
