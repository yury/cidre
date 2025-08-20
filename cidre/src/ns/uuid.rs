use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSUUID")]
    pub Uuid(ns::Id), NS_UUID
);

unsafe impl Send for Uuid {}

impl arc::A<Uuid> {
    #[objc::msg_send(initWithUUIDString:)]
    fn init_with_uuid_string(self, str: &ns::String) -> Option<arc::R<Uuid>>;
}

impl Uuid {
    #[objc::msg_send(UUIDString)]
    pub fn string(&self) -> arc::R<ns::String>;

    pub fn with_string(str: &ns::String) -> Option<arc::R<Self>> {
        Self::alloc().init_with_uuid_string(str)
    }
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
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

        let _uuid =
            ns::Uuid::with_string(ns::str!(c"F8B644CC-B944-4996-A321-CA129AFB18FE")).unwrap();
    }
}
