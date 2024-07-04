use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UTType")]
    pub Type(ns::Id)
);

unsafe impl Send for Type {}

impl Type {
    define_cls!(UT_TYPE);

    #[objc::msg_send(typeWithIdentifier:)]
    pub fn with_id(identifier: &ns::String) -> Option<arc::R<Self>>;

    #[objc::msg_send(typeWithFilenameExtension:)]
    pub fn with_file_ext(file_name_extension: &ns::String) -> Option<arc::R<Self>>;

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> &ns::String;

    #[objc::msg_send(preferredFilenameExtension)]
    pub fn preferred_file_ext(&self) -> Option<&ns::String>;

    #[objc::msg_send(preferredMIMEType)]
    pub fn preferred_mime_type(&self) -> Option<&ns::String>;

    #[objc::msg_send(localizedDescription)]
    pub fn localized_desc(&self) -> Option<&ns::String>;

    #[objc::msg_send(version)]
    pub fn version(&self) -> Option<&ns::String>;

    #[objc::msg_send(referenceURL)]
    pub fn reference_url(&self) -> Option<&ns::Url>;

    #[objc::msg_send(isDynamic)]
    pub fn is_dynamic(&self) -> bool;

    #[objc::msg_send(isDeclared)]
    pub fn is_declared(&self) -> bool;

    #[objc::msg_send(isPublicType)]
    pub fn is_public_type(&self) -> bool;
}

/// Conformance
impl Type {
    #[objc::msg_send(conformsToType:)]
    pub fn conforms_to_type(&self, other: &Self) -> bool;

    #[objc::msg_send(isSupertypeOfType:)]
    pub fn is_supertype_of_type(&self, other: &Self) -> bool;

    #[objc::msg_send(isSubtypeOfType:)]
    pub fn is_subtype_of_type(&self, other: &Self) -> bool;

    #[objc::msg_send(supertypes)]
    pub fn supertypes(&self) -> &ns::Set<Self>;
}

#[link(name = "ut", kind = "static")]
extern "C" {
    static UT_TYPE: &'static objc::Class<Type>;
}

#[cfg(test)]
mod tests {
    use crate::{ns, ut};
    #[test]
    fn basics() {
        let t = ut::Type::with_file_ext(&ns::String::with_str("png")).unwrap();
        assert_eq!(t.id(), "public.png");

        let sup = t.supertypes();
        assert!(sup.len() >= 4);
    }
}
