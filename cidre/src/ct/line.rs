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

    /// Draws the line into `context` at its current text position, honoring the context's text
    /// matrix. Set the position with [`crate::cg::Context::set_text_pos`] first.
    #[doc(alias = "CTLineDraw")]
    #[inline]
    pub fn draw(&self, context: &mut cg::Context) {
        unsafe { CTLineDraw(self, context) }
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

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn enum_caret_offsets_block(
        &self,
        block: &mut blocks::Block<fn(f64, cf::Index, bool, &mut bool), blocks::NoEsc>,
    ) {
        unsafe { CTLineEnumerateCaretOffsets(self, block) }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn enum_caret_offsets(&self, block: impl FnMut(f64, cf::Index, bool, &mut bool)) {
        let mut block =
            blocks::Block::<fn(f64, cf::Index, bool, &mut bool), blocks::NoEsc>::new4(block);
        unsafe { CTLineEnumerateCaretOffsets(self, &mut block) }
    }
}

unsafe extern "C-unwind" {
    fn CTLineGetTypeID() -> cf::TypeId;
    fn CTLineCreateWithAttributedString(attr_string: &cf::AttrString) -> arc::R<Line>;
    fn CTLineDraw(line: &Line, context: &mut cg::Context);

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
    fn CTLineEnumerateCaretOffsets(
        line: &Line,
        block: &mut blocks::Block<fn(f64, cf::Index, bool, &mut bool), blocks::NoEsc>,
    );

}

#[cfg(test)]
mod tests {
    use crate::{cf, cg, ct};

    #[test]
    fn basics() {
        let astr = cf::AttrString::new(cf::str!(c"test"));
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
        line.enum_caret_offsets(|offset, _char_index, _leading_edge, _stop| {
            offsets.push(offset);
        });
        assert_eq!(offsets.len(), 8);

        line.show();
    }

    #[test]
    fn draws_into_bitmap_context() {
        let (w, h) = (64usize, 16usize);
        let bpr = w * 4;
        let mut buf = vec![0u8; bpr * h];

        let space = cg::ColorSpace::device_rgb().unwrap();
        let info = cg::BitmapInfo::with_alpha(cg::ImageAlphaInfo::PremultipliedFirst)
            | cg::BitmapInfo::BYTE_ORDER_32_LITTLE;
        let mut ctx =
            cg::Context::new_bitmap(buf.as_mut_ptr() as *mut _, w, h, 8, bpr, &space, info)
                .expect("bitmap context");

        // A white fill proves the context writes into our buffer.
        ctx.set_rgb_fill_color(1.0, 1.0, 1.0, 1.0);
        ctx.fill_rect(cg::Rect::new(0.0, 0.0, w as cg::Float, h as cg::Float));
        assert!(buf.iter().all(|&b| b == 255), "fill_rect did not paint");

        // Black text over the white fill: some pixels must darken.
        ctx.set_rgb_fill_color(0.0, 0.0, 0.0, 1.0);
        let astr = cf::AttrString::new(cf::str!(c"Hi"));
        let line = ct::Line::with_attr_string(&astr);
        ctx.set_text_pos(2.0, 3.0);
        line.draw(&mut ctx);

        assert!(buf.iter().any(|&b| b < 255), "CTLineDraw painted nothing");
    }
}
