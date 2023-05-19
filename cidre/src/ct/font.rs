use crate::{arc, cf, cg, define_cf_type, define_options};

define_cf_type!(Font(cf::Type));
impl Font {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CTFontGetTypeID() }
    }

    #[inline]
    pub fn with_name_size_matrix(
        name: &cf::String,
        size: cg::Float,
        matrix: Option<&cg::AffineTransform>,
    ) -> arc::R<Self> {
        unsafe { CTFontCreateWithName(name, size, matrix) }
    }

    #[inline]
    pub fn with_name_size(name: &cf::String, size: cg::Float) -> arc::R<Self> {
        unsafe { CTFontCreateWithName(name, size, None) }
    }

    #[inline]
    pub fn with_name_matrix(name: &cf::String, matrix: &cg::AffineTransform) -> arc::R<Self> {
        unsafe { CTFontCreateWithName(name, 1.0, Some(matrix)) }
    }

    #[inline]
    pub fn with_ui_type_size_lang(
        ui_type: UIFontType,
        size: cg::Float,
        language: Option<&cf::String>,
    ) -> Option<arc::R<Self>> {
        unsafe { CTFontCreateUIFontForLanguage(ui_type, size, language) }
    }

    #[inline]
    pub fn with_ui_type_size(ui_type: UIFontType, size: cg::Float) -> Option<arc::R<Self>> {
        unsafe { CTFontCreateUIFontForLanguage(ui_type, size, None) }
    }
}

define_options!(Options(usize));
impl Options {
    pub const DEFAULT: Self = Self(0);
    pub const PREVENT_AUTO_ACTIVATION: Self = Self(1 << 0);
    pub const PREVENT_AUTO_DOWNLOAD: Self = Self(1 << 1);
    pub const PREFER_SYSTEM_FONT: Self = Self(1 << 2);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum UIFontType {
    None = u32::MAX - 1,
    User = 0,
    UserFixedPitch = 1,
    System = 2,
    EmphasizedSystem = 3,
    SmallSystem = 4,
    SmallEmphasizedSystem = 5,
    MiniSystem = 6,
    MiniEmphasizedSystem = 7,
    Views = 8,
    Application = 9,
    Label = 10,
    MenuTitle = 11,
    MenuItem = 12,
    MenuItemMark = 13,
    MenuItemCmdKey = 14,
    WindowTitle = 15,
    PushButton = 16,
    UtilityWindowTitle = 17,
    AlertHeader = 18,
    SystemDetail = 19,
    EmphasizedSystemDetail = 20,
    Toolbar = 21,
    SmallToolbar = 22,
    Message = 23,
    Palette = 24,
    ToolTip = 25,
    ControlContent = 26,
}

#[link(name = "CoreText", kind = "framework")]
extern "C" {
    fn CTFontGetTypeID() -> cf::TypeId;
    fn CTFontCreateWithName(
        name: &cf::String,
        size: cg::Float,
        matrix: Option<&cg::AffineTransform>,
    ) -> arc::R<Font>;

    fn CTFontCreateUIFontForLanguage(
        ui_type: UIFontType,
        size: cg::Float,
        language: Option<&cf::String>,
    ) -> Option<arc::R<Font>>;

}

#[cfg(test)]
mod tests {
    use crate::{cf, ct};

    #[test]
    fn basics() {
        let _font = ct::Font::with_name_size_matrix(&cf::String::from_str("None"), 12.0, None);
        let font = ct::Font::with_name_size(&cf::String::from_str("None"), 12.0);
        font.show();
    }
}
