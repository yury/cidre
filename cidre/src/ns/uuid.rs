use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSUUID")]
    pub Uuid(ns::Id), NS_UUID
);

unsafe impl Send for Uuid {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_UUID: &'static objc::Class<Uuid>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let _uuid = ns::Uuid::new();
    }
}
