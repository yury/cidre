use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIFont")]
    pub Font(ns::Id),
    UI_FONT
);

impl Font {
    #[objc::msg_send(preferredFontForTextStyle:)]
    pub fn preferred_for_text_style(style: &ui::TextStyle) -> arc::R<Self>;

    #[objc::msg_send(preferredFontForTextStyle:compatibleWithTraitCollection:)]
    pub fn preferred_for_text_style_trait(
        style: &ui::TextStyle,
        trait_collection: &ui::TraitCollection,
    );

    /// Returns a font using CSS name matching semantics.
    #[objc::msg_send(fontWithName:size:)]
    pub fn with_name_size(name: &ns::String, size: cg::Float) -> Option<arc::R<Self>>;

    /// Returns an array of font family names for all installed fonts
    #[objc::msg_send(familyNames)]
    pub fn family_names(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(fontNamesForFamilyName:)]
    pub fn font_names_for_family_name(
        &self,
        family_name: &ns::String,
    ) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(systemFontOfSize:)]
    pub fn sys_font_of_size(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(boldSystemFontOfSize:)]
    pub fn bold_sys_font_of_size(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(italicSystemFontOfSize:)]
    pub fn italic_sys_font_of_size(size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(systemFontOfSize:weight:)]
    pub fn sys_font_of_size_weight(size: cg::Float, weight: ui::FontWeight) -> arc::R<Self>;

    #[objc::msg_send(monospacedDigitSystemFontOfSize:weight:)]
    pub fn monospaced_digit_sys_font_of_size_weight(
        size: cg::Float,
        weight: ui::FontWeight,
    ) -> arc::R<Self>;

    #[objc::msg_send(systemFontOfSize:weight:width:)]
    pub fn sys_font_of_size_weight_width(
        size: cg::Float,
        weight: ui::FontWeight,
        width: ui::FontWidth,
    ) -> arc::R<Self>;

    #[objc::msg_send(monospacedSystemFontOfSize:weight:)]
    pub fn monospaced_sys_font_of_size_weight(
        size: cg::Float,
        weight: ui::FontWeight,
    ) -> arc::R<Self>;

    #[objc::msg_send(familyName)]
    pub fn family_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(fontName)]
    pub fn font_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(pointSize)]
    pub fn point_size(&self) -> cg::Float;

    #[objc::msg_send(ascender)]
    pub fn ascender(&self) -> cg::Float;

    #[objc::msg_send(descender)]
    pub fn descender(&self) -> cg::Float;

    #[objc::msg_send(capHeight)]
    pub fn cap_height(&self) -> cg::Float;

    #[objc::msg_send(xHeight)]
    pub fn x_height(&self) -> cg::Float;

    #[objc::msg_send(lineHeight)]
    pub fn line_height(&self) -> cg::Float;

    #[objc::msg_send(leading)]
    pub fn leading(&self) -> cg::Float;

    #[objc::msg_send(fontWithSize:)]
    pub fn with_size(&self, size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(fontWithDescriptor:size:)]
    pub fn with_desc_size(&self, descriptor: &ui::FontDesc, size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(fontDescriptor)]
    pub fn descriptor(&self) -> arc::R<ui::FontDesc>;
}

#[cfg(all(
    feature = "ct",
    any(target_os = "ios", target_os = "tvos", target_os = "visionos")
))]
impl Font {
    pub fn as_ct(&self) -> &crate::ct::Font {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(all(
    feature = "ct",
    any(target_os = "ios", target_os = "tvos", target_os = "visionos")
))]
impl AsRef<crate::ct::Font> for Font {
    fn as_ref(&self) -> &crate::ct::Font {
        self.as_ct()
    }
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_FONT: &'static objc::Class<Font>;
}
