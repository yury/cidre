use crate::{api, arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UITabAccessory")]
    pub TabAccessory(ns::Id)
);

impl TabAccessory {
    #[api::available(ios = 26.0)]
    crate::define_cls!(UI_TAB_ACCESSORY);

    #[objc::init(initWithContentView:)]
    pub fn init_with_content_view(self, content_view: &ui::View) -> arc::R<TabAccessory>;

    #[objc::available(ios = 26.0)]
    pub fn with_content_view(content_view: &ui::View) -> arc::R<Self> {
        Self::alloc().init_with_content_view(content_view)
    }

    #[objc::msg_send(contentView)]
    #[objc::available(ios = 26.0)]
    pub fn content_view(&self) -> arc::R<ui::View>;
}

#[doc(alias = "UITabAccessoryEnvironment")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TabAccessoryEnv {
    /// Indicates the absence of any information about whether or not the trait collection is
    /// from a view that is in a tab accessory.
    Unspecified = 0,

    /// The trait collection is from a view that is not in an active tab accessory environment.
    None = 1,

    /// The environment for when the accessory is laid out either above the bottom tab bar
    /// when it is visible, or at the bottom of the `ui::TabBarController`'s view.
    Regular = 2,

    /// The environment for when the accessory is laid out inline with the collapsed bottom tab bar.
    Inline = 3,
}

unsafe extern "C" {
    static UI_TAB_ACCESSORY: &'static objc::Class<TabAccessory>;
}
