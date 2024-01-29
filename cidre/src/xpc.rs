use crate::{blocks, define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "xpc_object_t")]
    pub Object(ns::Id)
);

#[doc(alias = "xpc_handler_t")]
pub type Handler = blocks::Block<fn(&Object)>;

define_obj_type!(
    #[doc(alias = "xpc_connection_t")]
    pub Connection(Object)
);

#[doc(alias = "xpc_connection_handler_t")]
pub type ConnectionHandler = extern "C" fn(&Connection);

define_obj_type!(
    #[doc(alias = "xpc_endpoint_t")]
    pub Endpoint(Object)
);

define_obj_type!(
    pub Bool(Object)
);

impl Bool {
    #[inline]
    pub fn value(&self) -> bool {
        unsafe { xpc_bool_get_value(&self) }
    }

    #[inline]
    pub fn true_value() -> &'static Self {
        Self::new(true)
    }

    #[inline]
    pub fn false_value() -> &'static Self {
        Self::new(false)
    }

    #[doc(alias = "xpc_bool_create")]
    #[inline]
    pub fn new(val: bool) -> &'static Self {
        unsafe { xpc_bool_create(val) }
    }
}

extern "C" {
    fn xpc_bool_create(val: bool) -> &'static Bool;
    fn xpc_bool_get_value(xbool: &Bool) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::xpc;

    #[test]
    fn bool_basics() {
        let bool = xpc::Bool::new(true);

        assert!(bool.value());
        bool.as_type_ref().show();
        let bool = xpc::Bool::new(true);
        bool.as_type_ref().show();
        assert!(bool.value());

        assert_ne!(
            xpc::Bool::false_value().value(),
            xpc::Bool::true_value().value()
        );
    }
}
