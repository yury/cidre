use crate::{arc, blocks, cf, cg, ct, define_cf_type, define_opts};

define_opts!(
    #[doc(alias = "CTLineBoundsOptions")]
    pub LineBoundsOpts(usize)
);

impl LineBoundsOpts {
    pub const EXCLUDE_TYPOGRAPHIC_LEADING: Self = Self(1 << 0);
    pub const EXCLUDE_TYPOGRAPHIC_SHIFTS: Self = Self(1 << 1);
    pub const USE_HANGING_PUNCTUATION: Self = Self(1 << 2);
    pub const USE_GLYPH_PATH_BOUNDS: Self = Self(1 << 3);
    pub const USE_OPTICAL_BOUNDS: Self = Self(1 << 4);
    pub const INCLUDE_LANGUAGE_EXTENTS: Self = Self(1 << 5);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum LineTruncationType {
    Start = 0,
    End = 1,
    Middle = 2,
}

define_cf_type!(Line(cf::Type));

impl Line {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CTLineGetTypeID() }
    }

    #[inline]
    pub fn with_attr_string(attr_string: &cf::AttrString) -> arc::R<Self> {
        unsafe { CTLineCreateWithAttributedString(attr_string) }
    }

    #[inline]
    pub fn glyph_count(&self) -> usize {
        unsafe { CTLineGetGlyphCount(self) as _ }
    }

    #[inline]
    pub fn glyph_runs(&self) -> &cf::ArrayOf<ct::Run> {
        unsafe { CTLineGetGlyphRuns(self) }
    }

    #[inline]
    pub fn string_range(&self) -> cf::Range {
        unsafe { CTLineGetStringRange(self) }
    }

    #[inline]
    pub fn pen_offset_for_flush(&self, flush_factor: cg::Float, flush_width: f64) -> f64 {
        unsafe { CTLineGetPenOffsetForFlush(self, flush_factor, flush_width) }
    }

    #[inline]
    pub fn typographic_bounds(
        &self,
        ascent: &mut cg::Float,
        descent: &mut cg::Float,
        leading: &mut cg::Float,
    ) -> f64 {
        unsafe { CTLineGetTypographicBounds(self, ascent, descent, leading) }
    }

    #[inline]
    pub fn bounds(&self, options: LineBoundsOpts) -> cg::Rect {
        unsafe { CTLineGetBoundsWithOptions(self, options) }
    }

    #[inline]
    pub fn trailing_whitspace(&self) -> f64 {
        unsafe { CTLineGetTrailingWhitespaceWidth(self) }
    }
    #[inline]
    pub fn index_for_pos(&self, position: cg::Point) -> cf::Index {
        unsafe { CTLineGetStringIndexForPosition(self, position) }
    }

    #[inline]
    pub fn enumerate_caret_offsets_block<F>(&self, block: &mut blocks::Block<F>)
    where
        F: FnMut(f64, cf::Index, bool, *mut bool),
    {
        unsafe { CTLineEnumerateCaretOffsets(self, block.as_mut_ptr()) }
    }

    #[inline]
    pub fn enumerate_caret_offsets<F>(&self, block: &mut F)
    where
        F: FnMut(f64, cf::Index, bool, *mut bool),
    {
        let mut block = blocks::no_esc4(block);
        unsafe { CTLineEnumerateCaretOffsets(self, block.as_mut_ptr()) }
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "C" {
    fn CTLineGetTypeID() -> cf::TypeId;
    fn CTLineCreateWithAttributedString(attr_string: &cf::AttrString) -> arc::R<Line>;

    fn CTLineGetGlyphCount(line: &Line) -> cf::Index;
    fn CTLineGetGlyphRuns(line: &Line) -> &cf::ArrayOf<ct::Run>;
    fn CTLineGetStringRange(line: &Line) -> cf::Range;

    fn CTLineGetPenOffsetForFlush(line: &Line, flush_factor: cg::Float, flush_width: f64) -> f64;

    fn CTLineGetTypographicBounds(
        line: &Line,
        ascent: *mut cg::Float,
        descent: *mut cg::Float,
        leading: *mut cg::Float,
    ) -> f64;

    fn CTLineGetBoundsWithOptions(line: &Line, options: LineBoundsOpts) -> cg::Rect;
    fn CTLineGetTrailingWhitespaceWidth(line: &Line) -> f64;
    fn CTLineGetStringIndexForPosition(line: &Line, position: cg::Point) -> cf::Index;
    fn CTLineEnumerateCaretOffsets(line: &Line, block: *mut std::ffi::c_void);

}

#[cfg(test)]
mod tests {
    use crate::{cf, cg, ct};

    #[test]
    fn basics() {
        let str = cf::String::from_str("test");
        let astr = cf::AttrString::new(&str);
        let line = ct::Line::with_attr_string(&astr);

        assert_eq!(line.glyph_count(), 4);

        let runs = line.glyph_runs();
        assert_eq!(runs.len(), 1);

        let range = line.string_range();
        assert_eq!(range, cf::Range::new(0, 4));

        let mut leading: cg::Float = 0.0;
        let mut ascent: cg::Float = 0.0;
        let mut descent: cg::Float = 0.0;

        let width = line.typographic_bounds(&mut ascent, &mut descent, &mut leading);

        assert_eq!(ascent + descent, 12.0);
        assert!(width > 0.0);

        let bounds = line.bounds(Default::default());

        assert_eq!(bounds.size.width, width);
        assert_eq!(line.trailing_whitspace(), 0.0);

        let mut offsets = Vec::new();

        let mut block =
            |offset: f64, _char_index: cf::Index, _leading_edge: bool, _stop: *mut bool| {
                offsets.push(offset);
            };

        line.enumerate_caret_offsets(&mut block);
        assert_eq!(offsets.len(), 8);

        line.show();
    }
}
