use crate::{arc, define_cls, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "UIContextMenuConfiguration")]
    pub ContextMenuCfg(ns::Id)
);

impl ContextMenuCfg {
    define_cls!(UI_CONTEXT_MENU_CONFIGURATION);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(configurationWithIdentifier:previewProvider:actionProvider:)]
    #[objc::available(ios = 13.0, tvos = 17.0)]
    pub fn with_id_preview_action_provider(
        id: Option<&ns::Id>,
        preview: Option<&mut ContentPreviewProvider>,
        actions: Option<&mut ActionProvider>,
    ) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_CONTEXT_MENU_CONFIGURATION: &'static objc::Class<ContextMenuCfg>;
}

#[cfg(feature = "blocks")]
pub type ContentPreviewProvider = blocks::EscBlock<fn() -> Option<arc::Rar<ui::ViewController>>>;

#[cfg(feature = "blocks")]
pub type ActionProvider =
    blocks::EscBlock<fn(&ns::Array<ui::MenuElement>) -> Option<arc::Rar<ui::Menu>>>;
