use crate::{arc, define_cls, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLDictionaryFeatureProvider")]
    pub DictionaryFeatureProvider(ns::Id)
);

impl ml::FeatureProvider for DictionaryFeatureProvider {}

impl arc::A<DictionaryFeatureProvider> {
    #[objc::msg_send(initWithDictionary:error:)]
    pub unsafe fn init_with_dictionary_err<'ear>(
        self,
        dictionary: &ns::Dictionary<ns::String, ns::Id>,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<DictionaryFeatureProvider>>;
}

impl DictionaryFeatureProvider {
    define_cls!(ML_DICTIONARY_FEATURE_PROVIDER);

    #[objc::msg_send(dictionary)]
    pub fn dictionary(&self) -> arc::R<ns::Dictionary<ns::String, ml::FeatureValue>>;

    #[objc::msg_send(objectForKeyedSubscript:)]
    pub fn object_for_keyed_subscript(
        &self,
        feature_name: &ns::String,
    ) -> Option<arc::R<ml::FeatureValue>>;

    #[inline]
    pub fn get(&self, feature_name: &ns::String) -> Option<arc::R<ml::FeatureValue>> {
        self.object_for_keyed_subscript(feature_name)
    }

    pub fn with_dictionary<'ear>(
        dictionary: &ns::Dictionary<ns::String, ns::Id>,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_dictionary_err(dictionary, err) })
    }
}

unsafe extern "C" {
    static ML_DICTIONARY_FEATURE_PROVIDER: &'static objc::Class<DictionaryFeatureProvider>;
}
