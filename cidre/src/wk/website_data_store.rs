use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "WKWebsiteDataStore")]
    pub WebsiteDataStore(ns::Id)
);

impl WebsiteDataStore {
    define_cls!(WK_WEBSITE_DATA_STORE);

    #[objc::msg_send(defaultDataStore)]
    pub fn default() -> arc::R<Self>;

    #[objc::msg_send(nonPersistentDataStore)]
    pub fn non_persistent() -> arc::R<Self>;

    #[objc::msg_send(isPersistent)]
    pub fn is_persistent(&self) -> bool;

    #[objc::msg_send(allWebsiteDataTypes)]
    pub fn all_website_data_types(&self) -> arc::R<ns::Set<ns::String>>;

    // ...

    #[objc::msg_send(identifier)]
    #[objc::available(macos = 14.0, ios = 17.0)]
    pub fn id(&self) -> Option<arc::R<ns::Uuid>>;

    #[objc::msg_send(dataStoreForIdentifier:)]
    #[objc::available(macos = 14.0, ios = 17.0)]
    pub unsafe fn with_id_throws(id: &ns::Uuid) -> arc::R<Self>;

    #[objc::available(macos = 14.0, ios = 17.0)]
    pub fn with_id<'ear>(id: &ns::Uuid) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe { Self::with_id_throws(id) })
    }
}

#[link(name = "wk", kind = "static")]
unsafe extern "C" {
    static WK_WEBSITE_DATA_STORE: &'static objc::Class<WebsiteDataStore>;
}
