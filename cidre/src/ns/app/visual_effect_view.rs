use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSVisualEffectView")]
    pub VisualEffectView(ns::View),
    NS_VISUAL_EFFECT_VIEW
);

impl VisualEffectView {
    #[objc::msg_send(setMaterial:)]
    pub fn set_material(&mut self, val: Material);

    #[objc::msg_send(material)]
    pub fn material(&self) -> Material;

    #[objc::msg_send(setBlendingMode:)]
    pub fn set_blending_mode(&mut self, val: BlendingMode);

    #[objc::msg_send(setState:)]
    pub fn set_state(&mut self, val: State);
}

unsafe extern "C" {
    static NS_VISUAL_EFFECT_VIEW: &'static objc::Class<VisualEffectView>;
}

#[doc(alias = "NSVisualEffectMaterial")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum Material {
    Titlebar = 3,
    Selection = 4,
    Menu = 5,
    Popover = 6,
    Sidebar = 7,
    HeaderView = 10,
    Sheet = 11,
    WindowBg = 12,
    HudWindow = 13,
    FullScreenUi = 15,
    ToolTip = 17,
    ContentBg = 18,
    UnderWindowBg = 21,
    UnderPageBg = 22,
}

#[doc(alias = "NSVisualEffectBlendingMode")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum BlendingMode {
    BehindWindow,
    WithinWindow,
}

#[doc(alias = "NSVisualEffectState")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum State {
    FollowsWindowActiveState,
    Active,
    Inactive,
}
