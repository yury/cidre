use crate::{api, arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSPreviewRepresentingActivityItem")]
    pub PreviewRepresentingActivityItem(ns::Id)
);

impl PreviewRepresentingActivityItem {
    #[api::available(macos = 13.0)]
    crate::define_cls!(NS_PREVIEW_REPRESENTING_ACTIVITY_ITEM);

    #[objc::init(initWithItem:title:imageProvider:iconProvider:)]
    pub fn init_with_item(
        self,
        item: &ns::Id,
        title: Option<&ns::String>,
        image: Option<&ns::ItemProvider>,
        icon: Option<&ns::ItemProvider>,
    ) -> arc::R<PreviewRepresentingActivityItem>;
}

unsafe extern "C" {
    static NS_PREVIEW_REPRESENTING_ACTIVITY_ITEM:
        &'static objc::Class<PreviewRepresentingActivityItem>;
}
