use crate::{arc, cf, define_obj_type, ns, sec};

define_obj_type!(
    #[doc(alias = "sec_identity_t")]
    #[doc(alias = "OS_sec_identity")]
    pub ProtocolIdentity(ns::Id)
);

impl sec::Identity {
    pub fn type_id() -> cf::TypeId {
        unsafe { SecIdentityGetTypeID() }
    }

    #[doc(alias = "SecIdentityCreate")]
    #[inline]
    pub fn with_cert_key(cert: &sec::Cert, private_key: &sec::Key) -> Option<arc::R<Self>> {
        unsafe { SecIdentityCreate(None, cert, private_key) }
    }
}

impl ProtocolIdentity {
    #[doc(alias = "sec_identity_create")]
    #[inline]
    pub fn with_identity(identity: &sec::Identity) -> Option<arc::R<Self>> {
        unsafe { sec_identity_create(identity) }
    }
}

unsafe extern "C" {
    fn SecIdentityGetTypeID() -> cf::TypeId;
    fn SecIdentityCreate(
        allocator: Option<&cf::Allocator>,
        cert: &sec::Cert,
        private_key: &sec::Key,
    ) -> Option<arc::R<sec::Identity>>;
    fn sec_identity_create(identity: &sec::Identity) -> Option<arc::R<ProtocolIdentity>>;
}
