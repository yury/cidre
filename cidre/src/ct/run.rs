use crate::{cf, cg, define_cf_type, define_opts};

define_opts!(pub Status(u32));

impl Status {
    /// The run has no special attributes.
    pub const NO_STATUS: Self = Self(0);

    /// When set, the run is right to left.
    pub const RIGHT_TO_LEFT: Self = Self(1 << 0);

    /// When set, the run has been reordered in some way such that
    /// the string indices associated with the glyphs are no longer
    /// strictly increasing (for left to right runs) or decreasing
    /// (for right to left runs).
    pub const NON_MONOTONIC: Self = Self(1 << 1);

    /// When set, the run requires a specific text matrix to be set
    /// in the current CG context for proper drawing.
    pub const HAS_NON_IDENTITY_MATRIX: Self = Self(1 << 2);
}

define_cf_type!(Run(cf::Type));
impl Run {
    #[inline]
    fn range_for_slice(&self, index: usize, len: usize) -> cf::Range {
        let glyph_count = usize::try_from(self.glyph_count()).expect("negative glyph count");
        let end = index.checked_add(len).expect("glyph range overflow");
        assert!(end <= glyph_count, "glyph range out of bounds");
        cf::Range::new(index as _, len as _)
    }

    #[inline]
    fn range_len(&self, range: cf::Range) -> usize {
        let glyph_count = usize::try_from(self.glyph_count()).expect("negative glyph count");
        let index = usize::try_from(range.loc).expect("negative glyph range location");
        let requested_len = usize::try_from(range.len).expect("negative glyph range length");
        assert!(index <= glyph_count, "glyph range out of bounds");

        let len = if requested_len == 0 {
            glyph_count - index
        } else {
            requested_len
        };
        let end = index.checked_add(len).expect("glyph range overflow");
        assert!(end <= glyph_count, "glyph range out of bounds");
        len
    }

    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CTRunGetTypeID() }
    }

    /// The number of glyphs that the run contains. It is totally
    /// possible that this function could return a value of zero,
    /// indicating that there are no glyphs in this run.
    #[inline]
    pub fn glyph_count(&self) -> cf::Index {
        unsafe { CTRunGetGlyphCount(self) }
    }

    #[inline]
    pub fn attributes(&self) -> &cf::DictionaryOf<cf::String, cf::Type> {
        unsafe { CTRunGetAttributes(self) }
    }

    #[inline]
    pub fn status(&self) -> Status {
        unsafe { CTRunGetStatus(self) }
    }

    #[inline]
    pub fn glyphs_ptr(&self) -> *const cg::Glyph {
        unsafe { CTRunGetGlyphsPtr(self) }
    }

    #[inline]
    pub fn glyphs_slice(&self) -> Option<&[cg::Glyph]> {
        let count = self.glyph_count();
        if count <= 0 {
            return None;
        }
        let ptr = self.glyphs_ptr();
        if ptr.is_null() {
            return None;
        }

        unsafe { Some(std::slice::from_raw_parts(ptr, count as usize)) }
    }

    #[inline]
    pub fn copy_glyphs(&self, index: usize, buf: &mut [cg::Glyph]) {
        let range = self.range_for_slice(index, buf.len());
        unsafe { CTRunGetGlyphs(self, range, buf.as_mut_ptr()) }
    }

    #[inline]
    pub fn positions_ptr(&self) -> *const cg::Point {
        unsafe { CTRunGetPositionsPtr(self) }
    }

    #[inline]
    pub fn copy_positions(&self, index: usize, buf: &mut [cg::Point]) {
        let range = self.range_for_slice(index, buf.len());
        unsafe { CTRunGetPositions(self, range, buf.as_mut_ptr()) }
    }

    #[inline]
    pub fn advances_ptr(&self) -> *const cg::Size {
        unsafe { CTRunGetAdvancesPtr(self) }
    }

    #[inline]
    pub fn copy_advances(&self, index: usize, buf: &mut [cg::Size]) {
        let range = self.range_for_slice(index, buf.len());
        unsafe { CTRunGetAdvances(self, range, buf.as_mut_ptr()) }
    }

    #[inline]
    pub fn string_indices_ptr(&self) -> *const cf::Index {
        unsafe { CTRunGetStringIndicesPtr(self) }
    }

    #[inline]
    pub fn copy_string_indices(&self, index: usize, buf: &mut [cf::Index]) {
        let range = self.range_for_slice(index, buf.len());
        unsafe { CTRunGetStringIndices(self, range, buf.as_mut_ptr()) }
    }

    #[inline]
    pub fn string_range(&self) -> cf::Range {
        unsafe { CTRunGetStringRange(self) }
    }

    #[inline]
    pub fn typographic_bounds(
        &self,
        range: cf::Range,
        ascent: Option<&mut cg::Float>,
        descent: Option<&mut cg::Float>,
        leading: Option<&mut cg::Float>,
    ) -> f64 {
        self.range_len(range);
        unsafe { CTRunGetTypographicBounds(self, range, ascent, descent, leading) }
    }

    #[inline]
    pub fn text_matrix(&self) -> cg::AffineTransform {
        unsafe { CTRunGetTextMatrix(self) }
    }

    #[inline]
    pub fn copy_advances_and_origins(
        &self,
        range: cf::Range,
        advances_buf: Option<&mut [cg::Size]>,
        origins_buf: Option<&mut [cg::Point]>,
    ) {
        let len = self.range_len(range);
        if let Some(buf) = &advances_buf {
            assert!(buf.len() >= len, "advance buffer is too small");
        }
        if let Some(buf) = &origins_buf {
            assert!(buf.len() >= len, "origin buffer is too small");
        }

        let advances_ptr = advances_buf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr());
        let origins_ptr = origins_buf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr());
        unsafe { CTRunGetBaseAdvancesAndOrigins(self, range, advances_ptr, origins_ptr) }
    }
}

unsafe extern "C-unwind" {
    fn CTRunGetTypeID() -> cf::TypeId;
    fn CTRunGetGlyphCount(run: &Run) -> cf::Index;
    fn CTRunGetAttributes(run: &Run) -> &cf::DictionaryOf<cf::String, cf::Type>;
    fn CTRunGetStatus(run: &Run) -> Status;
    fn CTRunGetGlyphsPtr(run: &Run) -> *const cg::Glyph;
    fn CTRunGetGlyphs(run: &Run, range: cf::Range, buffer: *mut cg::Glyph);
    fn CTRunGetPositionsPtr(run: &Run) -> *const cg::Point;
    fn CTRunGetPositions(run: &Run, range: cf::Range, buffer: *mut cg::Point);
    fn CTRunGetAdvancesPtr(run: &Run) -> *const cg::Size;
    fn CTRunGetAdvances(run: &Run, range: cf::Range, buffer: *mut cg::Size);
    fn CTRunGetStringIndicesPtr(run: &Run) -> *const cf::Index;
    fn CTRunGetStringIndices(run: &Run, range: cf::Range, buffer: *mut cf::Index);
    fn CTRunGetStringRange(run: &Run) -> cf::Range;
    fn CTRunGetTypographicBounds(
        run: &Run,
        range: cf::Range,
        ascent: Option<&mut cg::Float>,
        descent: Option<&mut cg::Float>,
        leading: Option<&mut cg::Float>,
    ) -> f64;

    fn CTRunGetTextMatrix(run: &Run) -> cg::AffineTransform;

    fn CTRunGetBaseAdvancesAndOrigins(
        run: &Run,
        range: cf::Range,
        advances_buf: *mut cg::Size,
        origins_buf: *mut cg::Point,
    );

}

#[cfg(test)]
mod tests {
    use crate::{cf, cg, ct};

    #[test]
    fn copies_optional_run_outputs() {
        let attr_string = cf::AttrString::new(cf::str!(c"test"));
        let line = ct::Line::with_attr_string(&attr_string);
        let run = &line.glyph_runs()[0];
        let glyph_count = run.glyph_count() as usize;
        let range = cf::Range::new(0, 0);

        let mut advances = vec![cg::Size::zero(); glyph_count];
        let mut origins = vec![cg::Point::zero(); glyph_count];
        run.copy_advances_and_origins(range, Some(&mut advances), Some(&mut origins));
        assert!(advances.iter().any(|advance| advance.width > 0.0));

        let mut ascent = 0.0;
        let mut descent = 0.0;
        let width = run.typographic_bounds(range, Some(&mut ascent), Some(&mut descent), None);
        assert!(width > 0.0);
        assert!(ascent + descent > 0.0);
        assert_eq!(width, run.typographic_bounds(range, None, None, None));
    }
}
