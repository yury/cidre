use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSUUID")]
    pub Uuid(ns::Id), NS_UUID
);

unsafe impl Send for Uuid {}

impl Uuid {
    #[objc::msg_send(UUIDString)]
    pub fn string(&self) -> arc::R<ns::String>;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_UUID: &'static objc::Class<Uuid>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let uuid = ns::Uuid::new();

        let string = uuid.string();
        assert!(!string.is_empty());
    }
}
