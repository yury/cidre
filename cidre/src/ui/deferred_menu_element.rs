use crate::{arc, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

/// Hands the deferred element its menu elements; call it when they are available.
#[cfg(feature = "blocks")]
#[doc(alias = "void(^completion)(NSArray<UIMenuElement *> *elements)")]
pub type DeferredMenuElementCompletion = blocks::EscBlock<fn(&ns::Array<ui::MenuElement>)>;

/// Called by the system when the containing menu is presented, with the completion to call.
#[cfg(feature = "blocks")]
pub type DeferredMenuElementProviderBlock =
    blocks::EscBlock<fn(&mut DeferredMenuElementCompletion)>;

define_obj_type!(
    /// A placeholder menu element, replaced with what its provider gives when the menu is
    /// presented; a loading UI takes its place until then.
    #[doc(alias = "UIDeferredMenuElement")]
    pub DeferredMenuElement(ui::MenuElement),
    UI_DEFERRED_MENU_ELEMENT
);

impl DeferredMenuElement {
    #[objc::msg_send(identifier)]
    #[objc::available(ios = 26.0)]
    pub fn id(&self) -> arc::R<ns::String>;

    /// The provider is called only once, when the element is first encountered in a menu;
    /// the element may be stored and reused across menus.
    #[cfg(feature = "blocks")]
    #[objc::msg_send(elementWithProvider:)]
    #[objc::available(ios = 14.0)]
    pub fn with_provider_block(provider: &mut DeferredMenuElementProviderBlock) -> arc::R<Self>;

    /// Like [`Self::with_provider_block`] with a closure.
    #[cfg(feature = "blocks")]
    #[objc::available(ios = 14.0)]
    pub fn with_provider(
        provider: impl FnMut(&mut DeferredMenuElementCompletion) + 'static,
    ) -> arc::R<Self> {
        let mut provider = blocks::EscBlock::new1(provider);
        Self::with_provider_block(&mut provider)
    }

    /// Uncached: the provider is called every time the element is displayed.
    #[cfg(feature = "blocks")]
    #[objc::msg_send(elementWithUncachedProvider:)]
    #[objc::available(ios = 15.0)]
    pub fn with_uncached_provider_block(
        provider: &mut DeferredMenuElementProviderBlock,
    ) -> arc::R<Self>;

    /// Like [`Self::with_uncached_provider_block`] with a closure.
    #[cfg(feature = "blocks")]
    #[objc::available(ios = 15.0)]
    pub fn with_uncached_provider(
        provider: impl FnMut(&mut DeferredMenuElementCompletion) + 'static,
    ) -> arc::R<Self> {
        let mut provider = blocks::EscBlock::new1(provider);
        Self::with_uncached_provider_block(&mut provider)
    }

    /// Replaced with elements provided from the responder chain. With `should_cache_items`
    /// the chain is asked only once, otherwise every time the element is displayed.
    #[objc::msg_send(elementUsingFocusWithIdentifier:shouldCacheItems:)]
    #[objc::available(ios = 26.0)]
    pub fn using_focus_with_id(id: &ns::String, should_cache_items: bool) -> arc::R<Self>;
}

define_obj_type!(
    /// What a responder answers a responder-based deferred element with.
    #[doc(alias = "UIDeferredMenuElementProvider")]
    pub DeferredMenuElementProvider(ns::Id),
    UI_DEFERRED_MENU_ELEMENT_PROVIDER
);

impl DeferredMenuElementProvider {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(providerWithElementProvider:)]
    #[objc::available(ios = 26.0)]
    pub fn with_element_provider_block(
        provider: &mut DeferredMenuElementProviderBlock,
    ) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_DEFERRED_MENU_ELEMENT: &'static objc::Class<DeferredMenuElement>;
    static UI_DEFERRED_MENU_ELEMENT_PROVIDER: &'static objc::Class<DeferredMenuElementProvider>;
}
