use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIActivityItemsConfiguration")]
    pub ActivityItemsCfg(ns::Id), sym UIActivityItemsConfiguration
);

impl ActivityItemsCfg {
    #[objc::init(initWithItemProviders:)]
    pub fn init_with_item_providers(
        self,
        providers: &ns::Array<ns::ItemProvider>,
    ) -> arc::R<ActivityItemsCfg>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setMetadataProvider:)]
    pub fn set_metadata_provider(
        &mut self,
        provider: Option<&mut crate::blocks::EscBlock<fn(&ns::String) -> Option<arc::Rar<ns::Id>>>>,
    );
}

pub fn title_key() -> &'static ns::String {
    unsafe { UIActivityItemsConfigurationMetadataKeyTitle }
}
unsafe extern "C" {
    static UIActivityItemsConfigurationMetadataKeyTitle: &'static ns::String;
}
