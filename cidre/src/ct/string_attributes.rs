use crate::{cf, define_cf_type, define_opts, ns};

define_opts!(pub UnderlineStyle(i32));
impl UnderlineStyle {
    pub const NONE: Self = Self(0x00);
    pub const SINGLE: Self = Self(0x01);
    pub const THICK: Self = Self(0x02);
    pub const DOUBLE: Self = Self(0x09);
}

define_opts!(pub UnderlineStyleModifiers(i32));
impl UnderlineStyleModifiers {
    pub const PATTERN_SOLID: Self = Self(0x0000);
    pub const PATTERN_DOT: Self = Self(0x0100);
    pub const PATTERN_DASH: Self = Self(0x0200);
    pub const PATTERN_DASH_DOT: Self = Self(0x0300);
    pub const PATTERN_DASH_DOT_DOT: Self = Self(0x0400);
}

define_cf_type!(AttrName(cf::String));
impl AttrName {
    /// Value must be a &ct::Font. Default is Helvetica 12.
    #[inline]
    pub fn font() -> &'static Self {
        unsafe { kCTFontAttributeName }
    }

    /// Never set a foreground color in the CGContext; use what is set as
    /// the context's fill color.
    ///
    /// Value must be a &cf::Boolean. Default is false. The reason
    /// why this exists is because an NSAttributedString defaults to a
    /// black color if no color attribute is set. This forces CoreText to
    /// set the color in the context. This will allow developers to
    /// sidestep this, making CoreText set nothing but font information
    /// in the CGContext. If set, this attribute also determines the
    /// color used by kCTUnderlineStyleAttributeName, in which case it
    /// overrides the foreground color.
    #[inline]
    pub fn foreground_color_from_context() -> &'static Self {
        unsafe { kCTForegroundColorFromContextAttributeName }
    }

    /// A kerning adjustment.
    ///
    /// Value must be a &cf::Number float. Default is standard kerning.
    /// The kerning attribute indicate how many points the following
    /// character should be shifted from its default offset as defined
    /// by the current character's font in points; a positive kern
    /// indicates a shift farther along and a negative kern indicates a
    /// shift closer to the current character. If this attribute is not
    /// present, standard kerning will be used. If this attribute is
    /// set to 0.0, no kerning will be done at all.
    #[inline]
    pub fn kern() -> &'static Self {
        unsafe { kCTKernAttributeName }
    }

    /// Applies tracking (letterspacing).
    ///
    /// Value must be a &cf::Number. Default is zero (no tracking).
    /// The tracking attribute indicates how much additional space, in
    /// points, should be added to each character cluster after layout.
    /// The effect of this attribute is similar to kCTKernAttributeName
    /// but differs in that the added tracking is treated as trailing
    /// whitespace and a non-zero amount disables non-essential ligatures
    /// unless overridden by kCTLigatureAttributeName being present.
    /// If both kCTKernAttributeName and kCTTrackingAttributeName are
    /// present kCTKernAttributeName will be ignored unless zero;
    /// kCTTrackingAttributeName will still be honored.
    #[inline]
    pub fn tracking() -> &'static Self {
        unsafe { kCTTrackingAttributeName }
    }

    /// Controls ligature formation.
    ///
    /// Value must be a &cf::Number. Default is int value 1. The ligature
    /// attribute determines what kinds of ligatures should be used when
    /// displaying the string. A value of 0 indicates that only ligatures
    /// essential for proper rendering of text should be used, 1
    /// indicates that standard ligatures should be used, and 2 indicates
    /// that all available ligatures should be used. Which ligatures are
    /// standard depends on the script and possibly the font. Arabic
    /// text, for example, requires ligatures for many character
    /// sequences, but has a rich set of additional ligatures that
    /// combine characters. English text has no essential ligatures, and
    /// typically has only two standard ligatures, those for "fi" and
    /// "fl" -- all others being considered more advanced or fancy.
    /// On iOS releases prior to 6.0 essential ligatures are applied
    /// iif the font contains glyphs for any of U+FB00 through U+FB04 and
    /// the font lacks AAT or OpenType shaping tables, but as of 6.0
    /// shaping tables (or the lack thereof) are treated as definitive.
    #[inline]
    pub fn ligature() -> &'static Self {
        unsafe { kCTLigatureAttributeName }
    }

    /// The foreground color.
    ///
    /// Value must be a &cg::Color. Default value is black.
    #[inline]
    pub fn foreground_color() -> &'static Self {
        unsafe { kCTForegroundColorAttributeName }
    }

    #[inline]
    pub fn background_color() -> &'static Self {
        unsafe { kCTBackgroundColorAttributeName }
    }

    /// A ct::ParagraphStyle object which is used to specify things like
    /// line alignment, tab rulers, writing direction, etc.
    ///
    /// Value must be a &ct::ParagraphStyle. Default is an empty
    /// ct::ParagraphStyle object: see paragraph_style.rs for more
    /// information. The value of this attribute must be uniform over
    /// the range of any paragraphs to which it is applied.
    #[inline]
    pub fn paragraph_style() -> &'static Self {
        unsafe { kCTParagraphStyleAttributeName }
    }

    /// The stroke width.
    ///
    /// Value must be a cf::Number. Default value is 0.0, or no stroke.
    /// This attribute, interpreted as a percentage of font point size,
    /// controls the text drawing mode: positive values effect drawing
    /// with stroke only; negative values are for stroke and fill. A
    /// typical value for outlined text is 3.0.
    #[inline]
    pub fn stroke_width() -> &'static Self {
        unsafe { kCTStrokeWidthAttributeName }
    }

    /// The stroke color.
    ///
    /// Value must be a &cg::Color. Default is the foreground color.
    #[inline]
    pub fn stroke_color() -> &'static Self {
        unsafe { kCTStrokeColorAttributeName }
    }

    /// Allows the setting of an underline to be applied at render
    /// time.
    #[inline]
    pub fn underline_style() -> &'static Self {
        unsafe { kCTUnderlineStyleAttributeName }
    }

    #[inline]
    pub fn superscript() -> &'static Self {
        unsafe { kCTSuperscriptAttributeName }
    }

    #[inline]
    pub fn underline_color() -> &'static Self {
        unsafe { kCTUnderlineColorAttributeName }
    }

    #[inline]
    pub fn vertical_forms() -> &'static Self {
        unsafe { kCTVerticalFormsAttributeName }
    }

    #[inline]
    pub fn horizontal_in_vertical_forms() -> &'static Self {
        unsafe { kCTHorizontalInVerticalFormsAttributeName }
    }

    #[inline]
    pub fn glyph_info() -> &'static Self {
        unsafe { kCTGlyphInfoAttributeName }
    }

    #[inline]
    pub fn language() -> &'static Self {
        unsafe { kCTLanguageAttributeName }
    }

    #[inline]
    pub fn run_delegate() -> &'static Self {
        unsafe { kCTRunDelegateAttributeName }
    }

    #[inline]
    pub fn baseline_class() -> &'static Self {
        unsafe { kCTBaselineClassAttributeName }
    }

    #[inline]
    pub fn baseline_info() -> &'static Self {
        unsafe { kCTBaselineInfoAttributeName }
    }

    #[inline]
    pub fn baseline_ref_info() -> &'static Self {
        unsafe { kCTBaselineReferenceInfoAttributeName }
    }

    #[inline]
    pub fn baseline_offset() -> &'static Self {
        unsafe { kCTBaselineOffsetAttributeName }
    }

    #[inline]
    pub fn writing_direction() -> &'static Self {
        unsafe { kCTWritingDirectionAttributeName }
    }

    #[inline]
    pub fn ruby_annotation() -> &'static Self {
        unsafe { kCTRubyAnnotationAttributeName }
    }

    #[inline]
    pub fn as_ns_attr_string_key(&self) -> &'static ns::AttrStringKey {
        unsafe { std::mem::transmute(self) }
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "C" {
    static kCTFontAttributeName: &'static AttrName;
    static kCTForegroundColorFromContextAttributeName: &'static AttrName;
    static kCTKernAttributeName: &'static AttrName;
    static kCTTrackingAttributeName: &'static AttrName;
    static kCTLigatureAttributeName: &'static AttrName;
    static kCTForegroundColorAttributeName: &'static AttrName;
    static kCTBackgroundColorAttributeName: &'static AttrName;
    static kCTParagraphStyleAttributeName: &'static AttrName;
    static kCTStrokeWidthAttributeName: &'static AttrName;
    static kCTStrokeColorAttributeName: &'static AttrName;
    static kCTUnderlineStyleAttributeName: &'static AttrName;
    static kCTSuperscriptAttributeName: &'static AttrName;
    static kCTUnderlineColorAttributeName: &'static AttrName;
    static kCTVerticalFormsAttributeName: &'static AttrName;
    static kCTHorizontalInVerticalFormsAttributeName: &'static AttrName;
    static kCTGlyphInfoAttributeName: &'static AttrName;
    static kCTLanguageAttributeName: &'static AttrName;
    static kCTRunDelegateAttributeName: &'static AttrName;
    static kCTBaselineClassAttributeName: &'static AttrName;
    static kCTBaselineInfoAttributeName: &'static AttrName;
    static kCTBaselineReferenceInfoAttributeName: &'static AttrName;
    static kCTBaselineOffsetAttributeName: &'static AttrName;
    static kCTWritingDirectionAttributeName: &'static AttrName;
    static kCTRubyAnnotationAttributeName: &'static AttrName;
}
