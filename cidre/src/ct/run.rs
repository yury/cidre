use crate::{cf, cg, define_cf_type, define_options};

define_options!(Status(u32));

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
        let range = cf::Range::new(index as isize, buf.len() as isize);
        unsafe { CTRunGetGlyphs(self, range, buf.as_mut_ptr()) }
    }

    #[inline]
    pub fn positions_ptr(&self) -> *const cg::Point {
        unsafe { CTRunGetPositionsPtr(self) }
    }

    #[inline]
    pub fn copy_positions(&self, index: usize, buf: &mut [cg::Point]) {
        let range = cf::Range::new(index as isize, buf.len() as isize);
        unsafe { CTRunGetPositions(self, range, buf.as_mut_ptr()) }
    }

    #[inline]
    pub fn advances_ptr(&self) -> *const cg::Size {
        unsafe { CTRunGetAdvancesPtr(self) }
    }

    #[inline]
    pub fn copy_advances(&self, index: usize, buf: &mut [cg::Size]) {
        let range = cf::Range::new(index as isize, buf.len() as isize);
        unsafe { CTRunGetAdvances(self, range, buf.as_mut_ptr()) }
    }

    #[inline]
    pub fn string_indices_ptr(&self) -> *const cf::Index {
        unsafe { CTRunGetStringIndicesPtr(self) }
    }

    #[inline]
    pub fn copy_string_indices(&self, index: usize, buf: &mut [cf::Index]) {
        let range = cf::Range::new(index as isize, buf.len() as isize);
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
        ascent: *mut cg::Float,
        descent: *mut cg::Float,
        leading: *mut cg::Float,
    ) -> f64 {
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
        advances_buf: *mut cg::Size,
        origins_buf: *mut cg::Point,
    ) {
        unsafe { CTRunGetBaseAdvancesAndOrigins(self, range, advances_buf, origins_buf) }
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "C" {
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
        ascent: *mut cg::Float,
        descent: *mut cg::Float,
        leading: *mut cg::Float,
    ) -> f64;

    fn CTRunGetTextMatrix(run: &Run) -> cg::AffineTransform;

    fn CTRunGetBaseAdvancesAndOrigins(
        run: &Run,
        range: cf::Range,
        advances_buf: *mut cg::Size,
        origins_buf: *mut cg::Point,
    );

}
