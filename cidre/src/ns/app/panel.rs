use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSPanel")]
    pub Panel(ns::Window),
    sym NSPanel
);

impl Panel {
    #[objc::init(initWithContentRect:styleMask:backing:defer:)]
    pub fn init_with_content_rect_style_mask_backing_defer(
        self,
        content_rect: ns::Rect,
        style: ns::WindowStyleMask,
        backing: ns::BackingStoreType,
        defer: bool,
    ) -> arc::R<Panel>;

    pub fn with_content_rect_style_mask_backing_defer(
        content_rect: ns::Rect,
        style: ns::WindowStyleMask,
        backing: ns::BackingStoreType,
        defer: bool,
    ) -> arc::R<Self> {
        Self::alloc().init_with_content_rect_style_mask_backing_defer(
            content_rect,
            style,
            backing,
            defer,
        )
    }

    /// Whether it floats over the app's other windows.
    #[objc::msg_send(isFloatingPanel)]
    pub fn is_floating_panel(&self) -> bool;

    #[objc::msg_send(setFloatingPanel:)]
    pub fn set_floating_panel(&mut self, val: bool);

    /// Whether it becomes key only when a view in it needs the keyboard (a text field
    /// clicked), not on any click.
    #[objc::msg_send(becomesKeyOnlyIfNeeded)]
    pub fn becomes_key_only_if_needed(&self) -> bool;

    #[objc::msg_send(setBecomesKeyOnlyIfNeeded:)]
    pub fn set_becomes_key_only_if_needed(&mut self, val: bool);
}
