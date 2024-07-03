use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSLocale")]
    pub Locale(ns::Id)
);

unsafe impl Send for Locale {}
unsafe impl Sync for Locale {}

impl arc::A<Locale> {
    #[objc::msg_send(initWithLocaleIdentifier:)]
    pub fn init_with_locale_id(self, id: &ns::String) -> arc::R<Locale>;
}

impl Locale {
    define_cls!(NS_LOCALE);

    #[objc::msg_send2(availableLocaleIdentifiers)]
    pub fn available_locale_ids() -> arc::R<ns::Array<ns::String>>;

    pub fn with_locale_id(id: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_locale_id(id)
    }

    #[objc::msg_send2(autoupdatingCurrentLocale)]
    pub fn autoupdating_current_locale() -> arc::R<Self>;

    #[objc::msg_send2(currentLocale)]
    pub fn current_locale() -> arc::R<Self>;

    #[objc::msg_send2(systemLocale)]
    pub fn sys_locale() -> arc::R<Self>;

    #[objc::msg_send2(languageCode)]
    pub fn lang_code(&self) -> arc::R<ns::String>;

    #[objc::msg_send(languageIdentifier)]
    pub fn lang_id_ar(&self) -> arc::Rar<ns::String>;

    #[objc::rar_retain]
    pub fn lang_id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(regionCode)]
    pub fn region_code_ar(&self) -> Option<arc::Rar<ns::String>>;

    #[objc::rar_retain]
    pub fn region_code(&self) -> Option<arc::R<ns::String>>;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_LOCALE: &'static objc::Class<Locale>;
}
