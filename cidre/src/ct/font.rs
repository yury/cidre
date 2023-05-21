use crate::{arc, cf, cg, ct, define_cf_type, define_options, UniChar};

define_cf_type!(Font(cf::Type));
impl Font {
    #[doc(alias = "CTFontGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CTFontGetTypeID() }
    }

    #[doc(alias = "CTFontCreateWithName")]
    #[inline]
    pub fn with_name_size_matrix(
        name: &cf::String,
        size: cg::Float,
        matrix: Option<&cg::AffineTransform>,
    ) -> arc::R<Self> {
        unsafe { CTFontCreateWithName(name, size, matrix) }
    }

    #[doc(alias = "CTFontCreateWithName")]
    #[inline]
    pub fn with_name_size(name: &cf::String, size: cg::Float) -> arc::R<Self> {
        unsafe { CTFontCreateWithName(name, size, None) }
    }

    #[doc(alias = "CTFontCreateWithName")]
    #[inline]
    pub fn with_name_matrix(name: &cf::String, matrix: &cg::AffineTransform) -> arc::R<Self> {
        unsafe { CTFontCreateWithName(name, 1.0, Some(matrix)) }
    }

    #[doc(alias = "CTFontCreateUIFontForLanguage")]
    #[inline]
    pub fn with_ui_type_size_lang(
        ui_type: UIFontType,
        size: cg::Float,
        language: Option<&cf::String>,
    ) -> Option<arc::R<Self>> {
        unsafe { CTFontCreateUIFontForLanguage(ui_type, size, language) }
    }

    #[doc(alias = "CTFontCreateUIFontForLanguage")]
    #[inline]
    pub fn with_ui_type_size(ui_type: UIFontType, size: cg::Float) -> Option<arc::R<Self>> {
        unsafe { CTFontCreateUIFontForLanguage(ui_type, size, None) }
    }

    #[doc(alias = "CTFontGetSize")]
    #[inline]
    pub fn size(&self) -> cg::Float {
        unsafe { CTFontGetSize(self) }
    }

    #[doc(alias = "CTFontGetMatrix")]
    #[inline]
    pub fn matrix(&self) -> cg::AffineTransform {
        unsafe { CTFontGetMatrix(self) }
    }

    #[doc(alias = "CTFontGetSymbolicTraits")]
    #[inline]
    pub fn symbolic_traits(&self) -> ct::FontSymbolicTraits {
        unsafe { CTFontGetSymbolicTraits(self) }
    }

    #[doc(alias = "CTFontGetGlyphsForCharacters")]
    #[inline]
    pub fn glyphs_for_characters(
        &self,
        charactes: &[UniChar],
        glyphs: &mut [cg::Glyph],
    ) -> Result<(), ()> {
        let len = charactes.len();
        assert!(len <= glyphs.len());
        unsafe {
            if CTFontGetGlyphsForCharacters(
                self,
                charactes.as_ptr(),
                glyphs.as_mut_ptr(),
                len as isize,
            ) {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    #[doc(alias = "CTFontGetBoundingRectsForGlyphs")]
    #[inline]
    pub fn bounding_rect_for_glyphs(
        &self,
        orientation: ct::FontOrientation,
        glyphs: &[cg::Glyph],
    ) -> cg::Rect {
        unsafe {
            CTFontGetBoundingRectsForGlyphs(
                self,
                orientation,
                glyphs.as_ptr(),
                std::ptr::null_mut(),
                glyphs.len() as _,
            )
        }
    }

    #[doc(alias = "CTFontGetBoundingRectsForGlyphs")]
    #[inline]
    pub fn bounding_rects_for_glyphs(
        &self,
        orientation: ct::FontOrientation,
        glyphs: &[cg::Glyph],
        bounding_rects: &mut [cg::Rect],
    ) -> cg::Rect {
        let len = glyphs.len();
        assert!(len <= bounding_rects.len());
        unsafe {
            CTFontGetBoundingRectsForGlyphs(
                self,
                orientation,
                glyphs.as_ptr(),
                bounding_rects.as_mut_ptr(),
                glyphs.len() as _,
            )
        }
    }

    #[doc(alias = "CTFontGetOpticalBoundsForGlyphs")]
    #[inline]
    pub fn optical_bounds_for_glyphs(
        &self,
        glyphs: &[cg::Glyph],
        bounding_rects: &mut [cg::Rect],
    ) -> cg::Rect {
        let len = glyphs.len();
        assert!(len <= bounding_rects.len());
        unsafe {
            CTFontGetOpticalBoundsForGlyphs(
                self,
                glyphs.as_ptr(),
                bounding_rects.as_mut_ptr(),
                len as _,
                Default::default(),
            )
        }
    }

    /// This function returns the summed glyph advance of an array of glyphs.
    #[doc(alias = "CTFontGetAdvancesForGlyphs")]
    #[inline]
    pub fn advance_for_glyphs(
        &self,
        orientation: ct::FontOrientation,
        glyphs: &[cg::Glyph],
    ) -> f64 {
        unsafe {
            CTFontGetAdvancesForGlyphs(
                self,
                orientation,
                glyphs.as_ptr(),
                std::ptr::null_mut(),
                glyphs.len() as _,
            )
        }
    }

    /// This function returns the summed glyph advance of an array of glyphs.
    /// Individual glyph advances are passed back via the advances parameter.
    /// These are the ideal metrics for each glyph scaled and transformed in font space.
    #[doc(alias = "CTFontGetAdvancesForGlyphs")]
    #[inline]
    pub fn advances_for_glyphs(
        &self,
        orientation: ct::FontOrientation,
        glyphs: &[cg::Glyph],
        advances: &mut [cg::Size],
    ) -> f64 {
        let len = glyphs.len();
        assert!(len <= advances.len());
        unsafe {
            CTFontGetAdvancesForGlyphs(
                self,
                orientation,
                glyphs.as_ptr(),
                advances.as_mut_ptr(),
                glyphs.len() as _,
            )
        }
    }

    #[doc(alias = "CTFontCreatePathForGlyph")]
    #[inline]
    pub fn path_for_glyph(
        &self,
        glyph: cg::Glyph,
        matrix: Option<&cg::AffineTransform>,
    ) -> Option<arc::R<cg::Path>> {
        unsafe { CTFontCreatePathForGlyph(self, glyph, matrix) }
    }

    #[doc(alias = "CTFontGetCapHeight")]
    #[inline]
    pub fn cap_height(&self) -> cg::Float {
        unsafe { CTFontGetCapHeight(self) }
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

    fn CTFontCreatePathForGlyph(
        font: &Font,
        glyph: cg::Glyph,
        matrix: Option<&cg::AffineTransform>,
    ) -> Option<arc::R<cg::Path>>;

    fn CTFontGetSize(font: &Font) -> cg::Float;
    fn CTFontGetMatrix(font: &Font) -> cg::AffineTransform;
    fn CTFontGetSymbolicTraits(font: &Font) -> ct::FontSymbolicTraits;
    fn CTFontGetGlyphsForCharacters(
        font: &Font,
        charactes: *const UniChar,
        glyphs: *mut cg::Glyph,
        count: cf::Index,
    ) -> bool;

    fn CTFontGetAdvancesForGlyphs(
        font: &Font,
        orientation: ct::FontOrientation,
        glyphs: *const cg::Glyph,
        advances: *mut cg::Size,
        count: cf::Index,
    ) -> f64;

    fn CTFontGetOpticalBoundsForGlyphs(
        font: &Font,
        glyphs: *const cg::Glyph,
        bounding_rects: *mut cg::Rect,
        count: cf::Index,
        options: cf::OptionFlags,
    ) -> cg::Rect;

    fn CTFontGetBoundingRectsForGlyphs(
        font: &Font,
        orientation: ct::FontOrientation,
        glyphs: *const cg::Glyph,
        bounding_rects: *mut cg::Rect,
        count: cf::Index,
    ) -> cg::Rect;

    fn CTFontGetCapHeight(font: &Font) -> cg::Float;

}

#[cfg(test)]
mod tests {
    use crate::{cf, cg, ct};

    #[test]
    fn basics() {
        let _font = ct::Font::with_name_size_matrix(&cf::String::from_str("None"), 28.0, None);
        let font = ct::Font::with_name_size(&cf::String::from_str("None"), 28.0);
        font.show();

        let j_glyph = cg::Glyph::new(45);
        let path = font.path_for_glyph(j_glyph, None).unwrap();
        path.show();

        let utf16 = "Jabcdef ".encode_utf16().collect::<Vec<u16>>();
        let mut glyphs = vec![cg::Glyph::new(0); utf16.len()];
        font.glyphs_for_characters(&utf16, &mut glyphs).unwrap();
        assert_eq!(j_glyph, glyphs[0]);
        for g in glyphs {
            if let Some(path) = font.path_for_glyph(g, None) {
                path.show();
            } else {
                eprintln!("no path for glyph {:?}", g);
            }
        }
    }
}
