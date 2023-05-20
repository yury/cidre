use crate::{arc, cf, cg, define_cf_type};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ElementType {
    MoveToPoint,
    AddLineToPoint,
    AddQuadCurveToPoint,
    AddCurveToPoint,
    CloseSubpath,
}

define_cf_type!(Path(cf::Type));
impl Path {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGPathGetTypeID() }
    }

    #[inline]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { CGPathCreateCopy(self) }
    }

    #[inline]
    pub fn copy_mut(&self) -> arc::R<PathMut> {
        unsafe { CGPathCreateMutableCopy(self) }
    }

    #[inline]
    pub fn copy_transforming_path(&self, transform: Option<&cg::AffineTransform>) -> arc::R<Self> {
        unsafe { CGPathCreateCopyByTransformingPath(self, transform) }
    }

    #[inline]
    pub fn copy_mut_transforming_path(
        &self,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<PathMut> {
        unsafe { CGPathCreateMutableCopyByTransformingPath(self, transform) }
    }

    #[inline]
    pub fn copy_dashing_path(
        &self,
        transform: Option<&cg::AffineTransform>,
        phase: cg::Float,
        lengths: &[cg::Float],
    ) -> arc::R<Self> {
        unsafe {
            CGPathCreateCopyByDashingPath(self, transform, phase, lengths.as_ptr(), lengths.len())
        }
    }

    #[inline]
    pub fn copy_stroking_path(
        &self,
        transform: Option<&cg::AffineTransform>,
        line_width: cg::Float,
        line_cap: cg::LineCap,
        line_join: cg::LineJoin,
        miter_limit: cg::Float,
    ) -> arc::R<Self> {
        unsafe {
            CGPathCreateCopyByStrokingPath(
                self,
                transform,
                line_width,
                line_cap,
                line_join,
                miter_limit,
            )
        }
    }

    #[inline]
    pub fn equal(&self, other: &Path) -> bool {
        unsafe { CGPathEqualToPath(self, other) }
    }

    #[inline]
    pub fn with_rect(rect: cg::Rect, transform: Option<&cg::AffineTransform>) -> arc::R<Self> {
        unsafe { CGPathCreateWithRect(rect, transform) }
    }

    #[inline]
    pub fn with_ellipse_in_rect(
        rect: cg::Rect,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateWithEllipseInRect(rect, transform) }
    }

    #[inline]
    pub fn with_rounded_rect(
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateWithRoundedRect(rect, corner_width, corner_height, transform) }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

define_cf_type!(PathMut(Path));
impl PathMut {
    pub fn new() -> arc::R<Self> {
        unsafe { CGPathCreateMutable() }
    }

    pub fn add_rounded_rect(
        &mut self,
        transform: Option<&cg::AffineTransform>,
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
    ) {
        unsafe { CGPathAddRoundedRect(self, transform, rect, corner_width, corner_height) }
    }

    #[inline]
    pub fn move_to_point(&mut self, m: Option<&cg::AffineTransform>, x: cg::Float, y: cg::Float) {
        unsafe { CGPathMoveToPoint(self, m, x, y) }
    }

    #[inline]
    pub fn add_line_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
    ) {
        unsafe { CGPathAddLineToPoint(self, m, x, y) }
    }

    pub fn add_quad_curve_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        cpx: cg::Float,
        cpy: cg::Float,
        x: cg::Float,
        y: cg::Float,
    ) {
        unsafe { CGPathAddQuadCurveToPoint(self, m, cpx, cpy, x, y) }
    }

    pub fn add_curve_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        cp1x: cg::Float,
        cp1y: cg::Float,
        cp2x: cg::Float,
        cp2y: cg::Float,
        x: cg::Float,
        y: cg::Float,
    ) {
        unsafe { CGPathAddCurveToPoint(self, m, cp1x, cp1y, cp2x, cp2y, x, y) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGPathGetTypeID() -> cf::TypeId;
    fn CGPathCreateMutable() -> arc::R<PathMut>;
    fn CGPathCreateCopy(path: &Path) -> arc::R<Path>;
    fn CGPathCreateMutableCopy(path: &Path) -> arc::R<PathMut>;
    fn CGPathCreateCopyByTransformingPath(
        path: &Path,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Path>;
    fn CGPathCreateMutableCopyByTransformingPath(
        path: &Path,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<PathMut>;
    fn CGPathCreateWithRect(
        rect: cg::Rect,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Path>;
    fn CGPathCreateWithEllipseInRect(
        rect: cg::Rect,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Path>;

    fn CGPathCreateWithRoundedRect(
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Path>;

    fn CGPathAddRoundedRect(
        path: &mut PathMut,
        transform: Option<&cg::AffineTransform>,
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
    );

    fn CGPathCreateCopyByDashingPath(
        path: &Path,
        transform: Option<&cg::AffineTransform>,
        phase: cg::Float,
        lengths: *const cg::Float,
        count: usize,
    ) -> arc::R<Path>;

    fn CGPathCreateCopyByStrokingPath(
        path: &Path,
        transform: Option<&cg::AffineTransform>,
        line_width: cg::Float,
        line_cap: cg::LineCap,
        line_join: cg::LineJoin,
        miter_limit: cg::Float,
    ) -> arc::R<Path>;

    fn CGPathEqualToPath(path1: &Path, path2: &Path) -> bool;

    fn CGPathMoveToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
    );

    fn CGPathAddLineToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
    );

    fn CGPathAddQuadCurveToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        cpx: cg::Float,
        cpy: cg::Float,
        x: cg::Float,
        y: cg::Float,
    );

    fn CGPathAddCurveToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        cp1x: cg::Float,
        cp1y: cg::Float,
        cp2x: cg::Float,
        cp2y: cg::Float,
        x: cg::Float,
        y: cg::Float,
    );

}

#[cfg(test)]
mod tests {
    use crate::cg;

    #[test]
    fn basics() {
        let path = cg::Path::with_rect(cg::Rect::zero(), None);
        path.show();
        let path = cg::Path::with_ellipse_in_rect(cg::Rect::zero(), None);
        path.show();

        let path = path.copy_stroking_path(
            None,
            5.0f64,
            cg::LineCap::Round,
            cg::LineJoin::Round,
            0.0f64,
        );
        path.show();
    }
}
