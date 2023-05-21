use crate::{arc, cf, cg, define_cf_type};

define_cf_type!(Font(cf::Type));

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct Index(pub u16);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct Glyph(pub Index);

impl Glyph {
    pub fn new(index: u16) -> Self {
        Self(Index(index))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum FontPostScriptFormat {
    Type1 = 1,
    Type3 = 3,
    Type42 = 42,
}

impl Index {
    #[doc(alias = "kCGFontIndexMax")]
    pub const MAX: Self = Self(u16::MAX - 1);

    #[doc(alias = "kCGFontIndexInvalid")]
    pub const INVALID: Self = Self(u16::MAX);
}

impl Font {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGFontGetTypeID() }
    }

    #[inline]
    pub fn with_name(name: &cf::String) -> Option<arc::R<Self>> {
        unsafe { CGFontCreateWithFontName(name) }
    }

    #[inline]
    pub fn copy_with_variations(
        &self,
        variations: Option<&cf::Dictionary>,
    ) -> Option<arc::R<Self>> {
        unsafe { CGFontCreateCopyWithVariations(self, variations) }
    }

    #[inline]
    pub fn nglyphs(&self) -> usize {
        unsafe { CGFontGetNumberOfGlyphs(self) }
    }

    #[inline]
    pub fn units_per_em(&self) -> i32 {
        unsafe { CGFontGetUnitsPerEm(self) }
    }

    #[inline]
    pub fn post_script_name(&self) -> arc::R<cf::String> {
        unsafe { CGFontCopyPostScriptName(self) }
    }

    #[inline]
    pub fn full_name(&self) -> arc::R<cf::String> {
        unsafe { CGFontCopyFullName(self) }
    }

    /// The ascent is the maximum distance above the
    /// baseline of glyphs in a font. The value is specified in glyph space
    /// units.
    #[inline]
    pub fn ascent(&self) -> i32 {
        unsafe { CGFontGetAscent(self) }
    }

    /// The descent is the maximum distance below
    /// the baseline of glyphs in a font. The value is specified in glyph space
    /// units.
    #[inline]
    pub fn descent(&self) -> i32 {
        unsafe { CGFontGetDescent(self) }
    }

    /// The leading is the spacing between
    /// consecutive lines of text in a font. The value is specified in glyph
    /// space units.
    #[inline]
    pub fn leading(&self) -> i32 {
        unsafe { CGFontGetLeading(self) }
    }

    /// The cap height is the distance above the
    /// baseline of the top of flat capital letters of glyphs in a font. The
    /// value is specified in glyph space units.
    #[inline]
    pub fn cap_height(&self) -> i32 {
        unsafe { CGFontGetCapHeight(self) }
    }

    /// The x-height is the distance above the
    /// baseline of the top of flat, non-ascending lowercase letters (such as
    /// "x") of glyphs in a font. The value is specified in glyph space units.
    #[inline]
    pub fn x_height(&self) -> i32 {
        unsafe { CGFontGetXHeight(self) }
    }

    /// The font bounding box is the
    /// union of all of the bounding boxes for all the glyphs in a font. The
    /// value is specified in glyph space units.
    #[inline]
    pub fn bbox(&self) -> cg::Rect {
        unsafe { CGFontGetFontBBox(self) }
    }

    #[inline]
    pub fn italic_angle(&self) -> cg::Float {
        unsafe { CGFontGetItalicAngle(self) }
    }

    /// The thickness of the dominant vertical stems of glyphs in font.
    /// The value is specified in glyph space units.
    #[inline]
    pub fn stem_v(&self) -> cg::Float {
        unsafe { CGFontGetStemV(self) }
    }

    /// An array of the variation axis dictionaries for font. Each
    /// variation axis dictionary contains values for the variation axis keys
    /// listed below. This function returns None if font doesn't support
    /// variations.
    #[inline]
    pub fn variation_axes(&self) -> Option<arc::R<cf::Array>> {
        unsafe { CGFontCopyVariationAxes(self) }
    }

    /// The variation specification dictionary from font. This
    /// dictionary contains keys corresponding the variation axis names of the
    /// font. Each key is a variation axis name; the value for each key is the
    /// value specified for that particular variation axis represented as a
    /// cf::Number. This function returns None if font doesn't support
    /// variations.
    #[inline]
    pub fn variations(&self) -> Option<arc::R<cf::Array>> {
        unsafe { CGFontCopyVariations(self) }
    }

    #[inline]
    pub fn glyph_advances(&self, glyphs: &[Glyph]) -> Result<Vec<i32>, ()> {
        let len = glyphs.len();
        let mut result = Vec::<std::mem::MaybeUninit<i32>>::with_capacity(len);
        unsafe {
            result.set_len(len);
            if CGFontGetGlyphAdvances(self, glyphs.as_ptr(), len, result.as_mut_ptr() as _) {
                Ok(std::mem::transmute(result))
            } else {
                Err(())
            }
        }
    }

    #[inline]
    pub fn glyph_advances_u16(&self, glyphs: &[u16]) -> Result<Vec<i32>, ()> {
        unsafe { self.glyph_advances(std::mem::transmute(glyphs)) }
    }

    #[inline]
    pub fn glyph_bboxes(&self, glyphs: &[Glyph]) -> Result<Vec<cg::Rect>, ()> {
        let len = glyphs.len();
        let mut result = Vec::<std::mem::MaybeUninit<cg::Rect>>::with_capacity(len);
        unsafe {
            result.set_len(len);
            if CGFontGetGlyphBBoxes(self, glyphs.as_ptr(), len, result.as_mut_ptr() as _) {
                Ok(std::mem::transmute(result))
            } else {
                Err(())
            }
        }
    }

    #[inline]
    pub fn glyph_bboxes_u16(&self, glyphs: &[u16]) -> Result<Vec<cg::Rect>, ()> {
        unsafe { self.glyph_bboxes(std::mem::transmute(glyphs)) }
    }

    #[inline]
    pub fn glyph_with_name(&self, name: &cf::String) -> Glyph {
        unsafe { CGFontGetGlyphWithGlyphName(self, name) }
    }

    #[inline]
    pub fn name_for_glyph(&self, glyph: Glyph) -> Option<arc::R<cf::String>> {
        unsafe { CGFontCopyGlyphNameForGlyph(self, glyph) }
    }

    #[inline]
    pub fn name_for_glyph_u16(&self, glyph: u16) -> Option<arc::R<cf::String>> {
        self.name_for_glyph(Glyph::new(glyph))
    }

    #[inline]
    pub fn can_create_post_script_subset(&self, format: FontPostScriptFormat) -> bool {
        unsafe { CGFontCanCreatePostScriptSubset(self, format) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGFontGetTypeID() -> cf::TypeId;
    fn CGFontCreateWithFontName(name: &cf::String) -> Option<arc::R<Font>>;

    fn CGFontCreateCopyWithVariations(
        font: &Font,
        variations: Option<&cf::Dictionary>,
    ) -> Option<arc::R<Font>>;

    fn CGFontGetNumberOfGlyphs(font: &Font) -> usize;

    fn CGFontGetUnitsPerEm(font: &Font) -> i32;

    // NOTE: api mark return type as nullable. But I think it is bc of
    // nullable font parameter. So we remove that optional for now.
    fn CGFontCopyPostScriptName(font: &Font) -> arc::R<cf::String>;

    // NOTE: same as CGFontCopyPostScriptName
    // CG_EXTERN CFStringRef __nullable CGFontCopyFullName(CGFontRef cg_nullable font)
    fn CGFontCopyFullName(font: &Font) -> arc::R<cf::String>;

    fn CGFontGetAscent(font: &Font) -> i32;
    fn CGFontGetDescent(font: &Font) -> i32;
    fn CGFontGetLeading(font: &Font) -> i32;
    fn CGFontGetCapHeight(font: &Font) -> i32;
    fn CGFontGetXHeight(font: &Font) -> i32;
    fn CGFontGetFontBBox(font: &Font) -> cg::Rect;
    fn CGFontGetItalicAngle(font: &Font) -> cg::Float;
    fn CGFontGetStemV(font: &Font) -> cg::Float;

    fn CGFontCopyVariationAxes(font: &Font) -> Option<arc::R<cf::Array>>;
    fn CGFontCopyVariations(font: &Font) -> Option<arc::R<cf::Array>>;

    fn CGFontGetGlyphAdvances(
        font: &Font,
        glyphs: *const Glyph,
        count: usize,
        advances: *mut i32,
    ) -> bool;

    fn CGFontGetGlyphBBoxes(
        font: &Font,
        glyphs: *const Glyph,
        count: usize,
        bboxes: *mut cg::Rect,
    ) -> bool;

    fn CGFontGetGlyphWithGlyphName(font: &Font, name: &cf::String) -> Glyph;
    fn CGFontCopyGlyphNameForGlyph(font: &Font, glyph: Glyph) -> Option<arc::R<cf::String>>;

    fn CGFontCanCreatePostScriptSubset(font: &Font, format: FontPostScriptFormat) -> bool;

}

#[cfg(test)]
mod tests {
    use crate::{cf, cg};

    #[test]
    fn basics() {
        let font = cg::Font::with_name(&cf::String::from_str("Helvetica")).unwrap();
        let copy = font.copy_with_variations(None).unwrap();
        copy.show();

        assert_eq!(font.nglyphs(), 2252);
        assert_eq!(font.units_per_em(), 2048);

        assert_eq!(font.post_script_name().to_string(), "Helvetica");
        assert_eq!(font.full_name().to_string(), "Helvetica");

        assert_eq!(font.ascent(), 1577);
        assert_eq!(font.descent(), -471);
        assert_eq!(font.leading(), 0);
        assert_eq!(font.cap_height(), 1469);
        assert_eq!(font.bbox(), cg::Rect::new(-1947.0, -985.0, 4908.0, 3282.0));
        assert_eq!(font.italic_angle(), 0.0);

        assert!(font.variation_axes().is_none());
        assert!(font.variations().is_none());

        let advances = font.glyph_advances_u16(&[0, 1, 2]).unwrap();
        println!("{:?}", advances);
        let bboxes = font.glyph_bboxes_u16(&[0, 1, 2]).unwrap();
        println!("{:?}", bboxes);

        let name = font.name_for_glyph_u16(45).unwrap();
        assert_eq!(name.to_string(), "J");

        assert!(!font.can_create_post_script_subset(cg::FontPostScriptFormat::Type3));
        assert!(font.can_create_post_script_subset(cg::FontPostScriptFormat::Type1));
        assert!(font.can_create_post_script_subset(cg::FontPostScriptFormat::Type42));
    }
}
