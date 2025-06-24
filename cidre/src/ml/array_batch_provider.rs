use crate::{arc, define_cls, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLArrayBatchProvider")]
    pub ArrayBatchProvider(ns::Id)
);

impl ml::BatchProvider for ArrayBatchProvider {}

impl arc::A<ArrayBatchProvider> {
    #[objc::msg_send(initWithFeatureProviderArray:)]
    pub fn init_with_feature_provider_array<P: ml::FeatureProvider>(
        self,
        arr: &ns::Array<P>,
    ) -> arc::R<ArrayBatchProvider>;

    #[objc::msg_send(initWithDictionary:error:)]
    pub unsafe fn init_with_dictionary<'ear>(
        self,
        dictionary: &ns::Dictionary<ns::String, ns::Id>,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ArrayBatchProvider>>;
}

impl ArrayBatchProvider {
    define_cls!(ML_ARRAY_BATCH_PROVIDER);

    #[objc::msg_send(array)]
    pub fn array(&self) -> arc::R<ns::Array<ml::AnyFeatureProvider>>;

    pub fn with_feature_provider_array<P: ml::FeatureProvider>(arr: &ns::Array<P>) -> arc::R<Self> {
        Self::alloc().init_with_feature_provider_array(arr)
    }

    pub fn with_dictionary<'ear>(
        dictionary: &ns::Dictionary<ns::String, ns::Id>,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_dictionary(dictionary, err) })
    }
}

#[link(name = "ml", kind = "static")]
unsafe extern "C" {
    static ML_ARRAY_BATCH_PROVIDER: &'static objc::Class<ArrayBatchProvider>;
}
