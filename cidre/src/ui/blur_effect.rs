use crate::{arc, define_cls, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIBlurEffect")]
    pub BlurEffect(ui::VisualEffect)
);

#[cfg(not(target_os = "watchos"))]
impl BlurEffect {
    define_cls!(UI_BLUR_EFFECT);

    #[objc::msg_send(effectWithStyle:)]
    pub fn with_style(style: ui::BlurEffectStyle) -> arc::R<Self>;
}

#[doc(alias = "UIBlurEffectStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum BlurEffectStyle {
    ExtraLight,
    Light,
    ExtraDark,
    Regular,
    Prominent,
    SysUltraThinMaterial,
    SysThinMaterial,
    SysMaterial,
    SysThickMaterial,
    SysChromeMaterial,
    SysUltraThinMaterialLight,
    SysThinMaterialLight,
    SysMaterialLight,
    SysThickMaterialLight,
    SysChromeMaterialLight,
    SysUltraThinMaterialDark,
    SysThinMaterialDark,
    SysMaterialDark,
    SysThickMaterialDark,
    SysChromeMaterialDark,
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_BLUR_EFFECT: &'static objc::Class<BlurEffect>;
}
