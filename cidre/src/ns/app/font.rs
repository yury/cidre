use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSFont")]
    pub Font(ns::Id),
    NS_FONT
);

impl Font {
    #[objc::msg_send(fontWithName:size:)]
    pub fn with_name_size(name: &ns::String, size: cg::Float) -> Option<arc::R<Self>>;

    #[objc::msg_send(fontWithName:matrix:)]
    pub fn with_name_matrix(name: &ns::String, matrix: *const cg::Float) -> Option<arc::R<Self>>;

    /// Application font
    #[objc::msg_send(userFontOfSize:)]
    pub fn user_font_of_size(size: cg::Float) -> Option<arc::R<Self>>;

    #[objc::msg_send(userFixedPitchFontOfSize:)]
    pub fn user_fixed_pitch_font_of_size(size: cg::Float) -> Option<arc::R<Self>>;

    #[objc::msg_send(setUserFont:)]
    pub fn set_user_ns_font(val: Option<&Self>);

    #[inline]
    pub fn set_user_font<F: AsRef<Self>>(font: Option<&F>) {
        Self::set_user_ns_font(font.map(|f| f.as_ref()));
    }

    #[objc::msg_send(setUserFixedPitchFont:)]
    pub fn set_user_fixed_pitch_ns_font(val: Option<&Self>);

    pub fn set_user_fixed_pitch_font<F: AsRef<Self>>(font: Option<&F>) {
        Self::set_user_fixed_pitch_ns_font(font.map(|f| f.as_ref()));
    }
}

/// UI font settings
impl Font {
    #[objc::msg_send(systemFontOfSize:)]
    pub fn sys_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(boldSystemFontOfSize:)]
    pub fn sys_bold_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(labelFontOfSize:)]
    pub fn label_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(titleBarFontOfSize:)]
    pub fn title_bar_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(menuFontOfSize:)]
    pub fn menu_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(menuBarFontOfSize:)]
    pub fn menu_bar_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(messageFontOfSize:)]
    pub fn message_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(paletteFontOfSize:)]
    pub fn palette_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(toolTipsFontOfSize:)]
    pub fn tool_tips_font(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(controlContentFontOfSize:)]
    pub fn control_content_font(size: cg::Float) -> arc::R<Self>;

    // #[objc::msg_send(systemFontOfSize:weight:)]
    // pub fn sys_font_weight(size: cg::Float, weight: ns::Fon) -> arc::R<Self>;
}

#[cfg(all(target_os = "macos", feature = "ct"))]
impl Font {
    pub fn as_ct(&self) -> &crate::ct::Font {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(all(target_os = "macos", feature = "ct"))]
impl AsRef<crate::ct::Font> for Font {
    fn as_ref(&self) -> &crate::ct::Font {
        self.as_ct()
    }
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_FONT: &'static objc::Class<Font>;
}
