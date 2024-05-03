use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    pub Locale(ns::Id)
);

impl arc::A<Locale> {
    #[objc::msg_send(initWithLocaleIdentifier:)]
    pub fn init_with_locale_id(self, id: &ns::String) -> arc::R<Locale>;
}

impl Locale {
    define_cls!(NS_LOCALE);

    #[objc::cls_msg_send(availableLocaleIdentifiers)]
    pub fn available_locale_ids_ar() -> arc::Rar<ns::Array<ns::String>>;

    #[objc::cls_rar_retain]
    pub fn available_locale_ids() -> arc::R<ns::Array<ns::String>>;

    pub fn with_locale_id(id: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_locale_id(id)
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_LOCALE: &'static objc::Class<Locale>;
}
