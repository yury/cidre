use crate::{api, arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSSplitViewItemAccessoryViewController")]
    pub SplitViewItemAccessoryViewController(ns::ViewController)
);

impl SplitViewItemAccessoryViewController {
    #[api::available(macos = 26.0)]
    crate::define_cls!(NS_SPLIT_VIEW_ITEM_ACCESSORY_VIEW_CONTROLLER);

    #[objc::init(init)]
    pub fn init(self) -> arc::R<SplitViewItemAccessoryViewController>;

    #[api::available(macos = 26.0)]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(isHidden)]
    #[objc::available(macos = 26.0)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    #[objc::available(macos = 26.0)]
    pub fn set_hidden(&mut self, val: bool);

    #[objc::msg_send(automaticallyAppliesContentInsets)]
    #[objc::available(macos = 26.0)]
    pub fn automatically_applies_content_insets(&self) -> bool;

    #[objc::msg_send(setAutomaticallyAppliesContentInsets:)]
    #[objc::available(macos = 26.0)]
    pub fn set_automatically_applies_content_insets(&mut self, val: bool);

    #[objc::msg_send(preferredScrollEdgeEffectStyle)]
    #[objc::available(macos = 26.1)]
    pub fn preferred_scroll_edge_effect_style(&self) -> arc::R<ns::ScrollEdgeEffectStyle>;

    #[objc::msg_send(setPreferredScrollEdgeEffectStyle:)]
    #[objc::available(macos = 26.1)]
    pub fn set_preferred_scroll_edge_effect_style(&mut self, val: &ns::ScrollEdgeEffectStyle);
}

unsafe extern "C" {
    static NS_SPLIT_VIEW_ITEM_ACCESSORY_VIEW_CONTROLLER:
        &'static objc::Class<SplitViewItemAccessoryViewController>;
}
