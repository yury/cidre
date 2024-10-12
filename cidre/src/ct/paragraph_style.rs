use std::ffi::c_void;

use crate::{arc, cf, define_cf_type};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum TextAlignment {
    /// Text is visually left-aligned.
    Left = 0,

    /// Text is visually right-aligned.
    Right = 1,

    /// Text is visually center-aligned.
    Center = 2,

    /// Text is fully justified. The last line in a paragraph is
    /// naturally aligned.
    Justified = 3,

    /// Use the natural alignment of the text's script.
    Natural = 4,
}

#[repr(u8)]
pub enum LineBreakMode {
    /// Wrapping occurs at word boundaries, unless the word itself doesn't
    /// fit on a single line.
    WordWrapping = 0,

    /// Wrapping occurs before the first character that doesn't fit.
    CharWrapping = 1,

    /// Lines are simply not drawn past the edge of the frame.
    Clipping = 2,

    /// Each line is displayed so that the end fits in the frame and the
    /// missing text is indicated by some kind of ellipsis glyph.
    TruncatingHead = 3,

    /// Each line is displayed so that the beginning fits in the
    /// container and the missing text is indicated by some kind of
    /// ellipsis glyph.
    TruncatingTail = 4,

    /// Each line is displayed so that the beginning and end fit in the
    /// container and the missing text is indicated by some kind of
    /// ellipsis glyph in the middle.
    TruncatingMiddle = 5,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i8)]
pub enum WritingDirection {
    /// The writing direction is algorithmically determined
    /// using the Unicode Bidirectional Algorithm rules P2 and P3.
    Natural = -1,

    /// The writing direction is left to right.
    LeftToRight = 0,

    /// The writing direction is right to left.
    RightToLeft = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ParagraphStyleSpecifier {
    Alignment = 0,
    FirstLineHeadIndent = 1,
    HeadIndent = 2,
    TailIndent = 3,
    TabStops = 4,
    DefaultTabInterval = 5,
    LineBreakMode = 6,
    LineHeightMultiple = 7,
    MaximumLineHeight = 8,
    MinimumLineHeight = 9,
    ParagraphSpacing = 11,
    ParagraphSpacingBefore = 12,
    BaseWritingDirection = 13,
    MaximumLineSpacing = 14,
    MinimumLineSpacing = 15,
    LineSpacingAdjustment = 16,
    LineBoundsOptions = 17,
}

#[repr(C)]
pub struct ParagraphStyleSetting {
    pub spec: ParagraphStyleSpecifier,
    pub value_size: usize,
    pub value: *const c_void,
}

define_cf_type!(ParagraphStyle(cf::Type));
impl ParagraphStyle {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CTParagraphStyleGetTypeID() }
    }

    #[inline]
    pub fn with_settings(settings: &[ParagraphStyleSetting]) -> Option<arc::R<Self>> {
        unsafe { CTParagraphStyleCreate(settings.as_ptr(), settings.len()) }
    }

    #[inline]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { CTParagraphStyleCreateCopy(self) }
    }

    #[inline]
    pub fn value_for_spec(
        &self,
        spec: ParagraphStyleSpecifier,
        value_buf_size: usize,
        value_buf: *mut c_void,
    ) -> bool {
        unsafe { CTParagraphStyleGetValueForSpecifier(self, spec, value_buf_size, value_buf) }
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "C-unwind" {
    fn CTParagraphStyleGetTypeID() -> cf::TypeId;
    fn CTParagraphStyleCreate(
        settings: *const ParagraphStyleSetting,
        setting_count: usize,
    ) -> Option<arc::R<ParagraphStyle>>;

    fn CTParagraphStyleCreateCopy(ps: &ParagraphStyle) -> arc::R<ParagraphStyle>;

    fn CTParagraphStyleGetValueForSpecifier(
        ps: &ParagraphStyle,
        spec: ParagraphStyleSpecifier,
        value_buf_size: usize,
        value_buf: *mut c_void,
    ) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::ct;

    #[test]
    fn basics() {
        let style = ct::ParagraphStyle::with_settings(&[]).unwrap();
        let _style_copy = style.copy();
        style.show();
    }
}
