use std::ffi::c_void;

use crate::{arc, cf, cg, define_cf_type};

#[cfg(feature = "blocks")]
use crate::blocks;

#[cfg(feature = "blocks")]
#[doc(alias = "CGPathApplyBlock")]
pub type ApplyBlock<Attr> = blocks::Block<fn(&cg::PathElement), Attr>;

pub type PathApplierFn<T> = extern "C" fn(info: *mut T, element: *mut Element);

#[doc(alias = "CGLineJoin")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[doc(alias = "CGLineCap")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[doc(alias = "CGPathElementType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ElementType {
    MoveToPoint,
    AddLineToPoint,
    AddQuadCurveToPoint,
    AddCurveToPoint,
    CloseSubpath,
}

#[doc(alias = "CGPathElement")]
#[derive(Debug)]
#[repr(C)]
pub struct Element {
    pub type_: ElementType,
    points: *mut cg::Point,
}

impl Element {
    #[inline]
    pub fn points(&self) -> &[cg::Point] {
        use ElementType::*;
        let len = match self.type_ {
            MoveToPoint | AddLineToPoint => 1,
            AddQuadCurveToPoint => 2,
            AddCurveToPoint => 3,
            CloseSubpath => return &[],
        };
        unsafe { std::slice::from_raw_parts(self.points, len) }
    }

    #[inline]
    pub fn points_mut(&mut self) -> &mut [cg::Point] {
        use ElementType::*;
        let len = match self.type_ {
            MoveToPoint | AddLineToPoint => 1,
            AddQuadCurveToPoint => 2,
            AddCurveToPoint => 3,
            CloseSubpath => return &mut [],
        };
        unsafe { std::slice::from_raw_parts_mut(self.points, len) }
    }
}

define_cf_type!(
    #[doc(alias = "CGPathRef")]
    Path(cf::Type)
);

impl Path {
    #[doc(alias = "CGPathGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGPathGetTypeID() }
    }

    #[doc(alias = "CGPathCreateCopy")]
    #[inline]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { CGPathCreateCopy(self) }
    }

    #[doc(alias = "CGPathCreateMutableCopy")]
    #[inline]
    pub fn copy_mut(&self) -> arc::R<PathMut> {
        unsafe { CGPathCreateMutableCopy(self) }
    }

    #[doc(alias = "CGPathCreateCopyByTransformingPath")]
    #[inline]
    pub fn transforming_path(&self, transform: Option<&cg::AffineTransform>) -> arc::R<Self> {
        unsafe { CGPathCreateCopyByTransformingPath(self, transform) }
    }

    #[doc(alias = "CGPathCreateMutableCopyByTransformingPath")]
    #[inline]
    pub fn transforming_path_mut(
        &self,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<PathMut> {
        unsafe { CGPathCreateMutableCopyByTransformingPath(self, transform) }
    }

    #[doc(alias = "CGPathCreateCopyByDashingPath")]
    #[inline]
    pub fn dashing_path(
        &self,
        transform: Option<&cg::AffineTransform>,
        phase: cg::Float,
        lengths: &[cg::Float],
    ) -> arc::R<Self> {
        unsafe {
            CGPathCreateCopyByDashingPath(self, transform, phase, lengths.as_ptr(), lengths.len())
        }
    }

    #[doc(alias = "CGPathCreateCopyByStrokingPath")]
    #[inline]
    pub fn stroking_path(
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

    #[doc(alias = "CGPathEqualToPath")]
    #[inline]
    pub fn equal(&self, other: &Path) -> bool {
        unsafe { CGPathEqualToPath(self, other) }
    }

    #[doc(alias = "CGPathCreateWithRect")]
    #[inline]
    pub fn with_rect(rect: cg::Rect, transform: Option<&cg::AffineTransform>) -> arc::R<Self> {
        unsafe { CGPathCreateWithRect(rect, transform) }
    }

    #[doc(alias = "CGPathCreateWithEllipseInRect")]
    #[inline]
    pub fn with_ellipse_in_rect(
        rect: cg::Rect,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateWithEllipseInRect(rect, transform) }
    }

    #[doc(alias = "CGPathCreateWithRoundedRect")]
    #[inline]
    pub fn with_rounded_rect(
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateWithRoundedRect(rect, corner_width, corner_height, transform) }
    }

    #[doc(alias = "CGPathIsEmpty")]
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { CGPathIsEmpty(self) }
    }

    #[doc(alias = "CGPathIsRect")]
    #[inline]
    pub fn is_rect(&self) -> bool {
        unsafe { CGPathIsRect(self) }
    }

    #[doc(alias = "CGPathGetCurrentPoint")]
    #[inline]
    pub fn current_point(&self) -> cg::Point {
        unsafe { CGPathGetCurrentPoint(self) }
    }

    #[doc(alias = "CGPathGetBoundingBox")]
    #[inline]
    pub fn bounding_box(&self) -> cg::Rect {
        unsafe { CGPathGetBoundingBox(self) }
    }

    #[doc(alias = "CGPathGetPathBoundingBox")]
    #[inline]
    pub fn path_bounding_box(&self) -> cg::Rect {
        unsafe { CGPathGetPathBoundingBox(self) }
    }

    #[doc(alias = "CGPathContainsPoint")]
    #[inline]
    pub fn contains_point(
        &self,
        m: Option<&cg::AffineTransform>,
        point: cg::Point,
        eo_fill: bool,
    ) -> bool {
        unsafe { CGPathContainsPoint(self, m, point, eo_fill) }
    }

    #[inline]
    pub fn apply<T: Sized>(&self, info: &mut T, function: PathApplierFn<T>) {
        unsafe {
            CGPathApply(
                self,
                info as *mut T as *mut c_void,
                std::mem::transmute(function),
            )
        }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn apply_block(&self, block: &mut ApplyBlock<blocks::NoEsc>) {
        unsafe { CGPathApplyWithBlock(self, block) }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn apply_mut(&self, mut f: impl FnMut(&cg::PathElement)) {
        let mut block = unsafe { ApplyBlock::stack1(&mut f) };
        self.apply_block(&mut block);
    }

    #[inline]
    pub fn normalizing(&self, even_odd_fill_rule: bool) -> arc::R<Self> {
        unsafe { CGPathCreateCopyByNormalizing(self, even_odd_fill_rule) }
    }

    #[inline]
    pub fn unioning_path(
        &self,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateCopyByUnioningPath(self, mask_path, even_odd_fill_rule) }
    }

    #[inline]
    pub fn intersecting_path(
        &self,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateCopyByIntersectingPath(self, mask_path, even_odd_fill_rule) }
    }

    #[inline]
    pub fn subtructing_path(
        &self,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateCopyBySubtractingPath(self, mask_path, even_odd_fill_rule) }
    }

    #[inline]
    pub fn symmetric_diff_of_path(
        &self,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateCopyBySymmetricDifferenceOfPath(self, mask_path, even_odd_fill_rule) }
    }

    #[inline]
    pub fn line_by_substructing_path(
        &self,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateCopyOfLineBySubtractingPath(self, mask_path, even_odd_fill_rule) }
    }
    #[inline]
    pub fn line_by_intersecting_path(
        &self,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateCopyOfLineByIntersectingPath(self, mask_path, even_odd_fill_rule) }
    }

    #[inline]
    pub fn separate_components(&self, even_odd_fill_rule: bool) -> arc::R<cf::ArrayOf<Path>> {
        unsafe { CGPathCreateSeparateComponents(self, even_odd_fill_rule) }
    }

    #[inline]
    pub fn flattering(&self, flattening_threshold: cg::Float) -> arc::R<Self> {
        unsafe { CGPathCreateCopyByFlattening(self, flattening_threshold) }
    }

    #[inline]
    pub fn intersects_path(&self, other: &Path, even_odd_fill_rule: bool) -> bool {
        unsafe { CGPathIntersectsPath(self, other, even_odd_fill_rule) }
    }
}

impl PartialEq for Path {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

define_cf_type!(PathMut(Path));
impl PathMut {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { CGPathCreateMutable() }
    }

    #[inline]
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

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn move_to<F: Into<f64>>(&mut self, x: F, y: F) {
        self.move_to_point(None, x.into(), y.into())
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn move_to<F: Into<f32>>(&mut self, x: F, y: F) {
        self.move_to_point(None, x.into(), y.into())
    }

    #[inline]
    pub fn line_to_point(&mut self, m: Option<&cg::AffineTransform>, x: cg::Float, y: cg::Float) {
        unsafe { CGPathAddLineToPoint(self, m, x, y) }
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn line_to<F: Into<f64>>(&mut self, x: F, y: F) {
        self.line_to_point(None, x.into(), y.into())
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn line_to<F: Into<f32>>(&mut self, x: F, y: F) {
        self.line_to_point(None, x.into(), y.into())
    }

    #[inline]
    pub fn quad_curve_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        cpx: cg::Float,
        cpy: cg::Float,
        x: cg::Float,
        y: cg::Float,
    ) {
        unsafe { CGPathAddQuadCurveToPoint(self, m, cpx, cpy, x, y) }
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn quad_to<F: Into<f64>>(&mut self, cpx: F, cpy: F, x: F, y: F) {
        self.quad_curve_to_point(None, cpx.into(), cpy.into(), x.into(), y.into())
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn quad_to<F: Into<f32>>(&mut self, cpx: F, cpy: F, x: F, y: F) {
        self.quad_curve_to_point(None, cpx.into(), cpy.into(), x.into(), y.into())
    }

    #[inline]
    pub fn curve_to_point(
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

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn curve_to<F: Into<f64>>(&mut self, cp1x: F, cp1y: F, cp2x: F, cp2y: F, x: F, y: F) {
        self.curve_to_point(
            None,
            cp1x.into(),
            cp1y.into(),
            cp2x.into(),
            cp2y.into(),
            x.into(),
            y.into(),
        )
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn curve_to<F: Into<f32>>(&mut self, cp1x: F, cp1y: F, cp2x: F, cp2y: F, x: F, y: F) {
        self.curve_to_point(
            None,
            cp1x.into(),
            cp1y.into(),
            cp2x.into(),
            cp2y.into(),
            x.into(),
            y.into(),
        )
    }

    #[inline]
    pub fn close_subpath(&mut self) {
        unsafe { CGPathCloseSubpath(self) }
    }

    #[inline]
    pub fn add_rect(&mut self, m: Option<&cg::AffineTransform>, rect: cg::Rect) {
        unsafe { CGPathAddRect(self, m, rect) }
    }

    #[inline]
    pub fn add_rects(&mut self, m: Option<&cg::AffineTransform>, rects: &[cg::Rect]) {
        unsafe { CGPathAddRects(self, m, rects.as_ptr(), rects.len()) }
    }

    #[inline]
    pub fn add_lines(&mut self, m: Option<&cg::AffineTransform>, points: &[cg::Point]) {
        unsafe { CGPathAddLines(self, m, points.as_ptr(), points.len()) }
    }

    #[inline]
    pub fn add_ellipse_in_rect(&mut self, m: Option<&cg::AffineTransform>, rect: cg::Rect) {
        unsafe { CGPathAddEllipseInRect(self, m, rect) }
    }

    #[inline]
    pub fn add_relative_arc(
        &mut self,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
        radius: cg::Float,
        start_angle: cg::Float,
        delta: cg::Float,
    ) {
        unsafe { CGPathAddRelativeArc(self, m, x, y, radius, start_angle, delta) }
    }

    #[inline]
    pub fn add_arc(
        &mut self,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
        radius: cg::Float,
        start_angle: cg::Float,
        end_ange: cg::Float,
        clockwise: bool,
    ) {
        unsafe { CGPathAddArc(self, m, x, y, radius, start_angle, end_ange, clockwise) }
    }

    #[inline]
    pub fn arc_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        x1: cg::Float,
        y1: cg::Float,
        x2: cg::Float,
        y2: cg::Float,
        radius: cg::Float,
    ) {
        unsafe { CGPathAddArcToPoint(self, m, x1, y1, x2, y2, radius) }
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn arc_to<F: Into<f64>>(&mut self, x1: F, y1: F, x2: F, y2: F, radius: F) {
        self.arc_to_point(
            None,
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            radius.into(),
        )
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn arc_to<F: Into<f32>>(&mut self, x1: F, y1: F, x2: F, y2: F, radius: F) {
        self.arc_to_point(
            None,
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            radius.into(),
        )
    }

    #[inline]
    pub fn add_path(&mut self, m: Option<&cg::AffineTransform>, path2: &Path) {
        unsafe { CGPathAddPath(self, m, path2) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C-unwind" {
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

    fn CGPathCloseSubpath(path: &mut PathMut);

    fn CGPathAddRect(path: &mut PathMut, m: Option<&cg::AffineTransform>, rect: cg::Rect);

    fn CGPathAddRects(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        rects: *const cg::Rect,
        count: usize,
    );

    fn CGPathAddLines(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        rects: *const cg::Point,
        count: usize,
    );

    fn CGPathAddEllipseInRect(path: &mut PathMut, m: Option<&cg::AffineTransform>, rect: cg::Rect);

    fn CGPathAddRelativeArc(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
        radius: cg::Float,
        start_angle: cg::Float,
        delta: cg::Float,
    );

    fn CGPathAddArc(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
        radius: cg::Float,
        start_angle: cg::Float,
        end_ange: cg::Float,
        clockwise: bool,
    );

    fn CGPathAddArcToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x1: cg::Float,
        y1: cg::Float,
        x2: cg::Float,
        y2: cg::Float,
        radius: cg::Float,
    );

    fn CGPathAddPath(path1: &mut PathMut, m: Option<&cg::AffineTransform>, path2: &Path);
    fn CGPathIsEmpty(path: &Path) -> bool;
    fn CGPathIsRect(path: &Path) -> bool;
    fn CGPathGetCurrentPoint(path: &Path) -> cg::Point;
    fn CGPathGetBoundingBox(path: &Path) -> cg::Rect;
    fn CGPathGetPathBoundingBox(path: &Path) -> cg::Rect;

    fn CGPathContainsPoint(
        path: &Path,
        m: Option<&cg::AffineTransform>,
        point: cg::Point,
        eo_fill: bool,
    ) -> bool;

    fn CGPathApply(path: &Path, info: *mut c_void, function: PathApplierFn<c_void>);
    #[cfg(feature = "blocks")]
    fn CGPathApplyWithBlock(path: &Path, block: &mut cg::PathApplyBlock<blocks::NoEsc>);
    fn CGPathCreateCopyByNormalizing(path: &Path, even_odd_fill_rule: bool) -> arc::R<Path>;
    fn CGPathCreateCopyByUnioningPath(
        path: &Path,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Path>;
    fn CGPathCreateCopyByIntersectingPath(
        path: &Path,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Path>;
    fn CGPathCreateCopyBySubtractingPath(
        path: &Path,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Path>;
    fn CGPathCreateCopyBySymmetricDifferenceOfPath(
        path: &Path,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Path>;
    fn CGPathCreateCopyOfLineBySubtractingPath(
        path: &Path,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Path>;
    fn CGPathCreateCopyOfLineByIntersectingPath(
        path: &Path,
        mask_path: Option<&Path>,
        even_odd_fill_rule: bool,
    ) -> arc::R<Path>;

    fn CGPathCreateSeparateComponents(
        path: &Path,
        even_odd_fill_rule: bool,
    ) -> arc::R<cf::ArrayOf<Path>>;
    fn CGPathCreateCopyByFlattening(path: &Path, flattening_threshold: cg::Float) -> arc::R<Path>;
    fn CGPathIntersectsPath(path: &Path, other: &Path, even_odd_fill_rule: bool) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::{blocks, cg};

    #[test]
    fn basics() {
        let path = cg::Path::with_rect(cg::Rect::zero(), None);
        path.show();
        let path = cg::Path::with_ellipse_in_rect(cg::Rect::zero(), None);
        path.show();

        let path = path.stroking_path(
            None,
            5.0f64,
            cg::LineCap::Round,
            cg::LineJoin::Round,
            0.0f64,
        );
        path.show();

        let mut elements = vec![];
        let mut block = cg::PathApplyBlock::<blocks::NoEsc>::new1(|element: &cg::PathElement| {
            elements.push(element.type_);
            println!("{:?} {:?}", element, element.points());
        });
        path.apply_block(&mut block);

        assert!(!elements.is_empty());

        let mut path = cg::PathMut::new();
        path.move_to(10, 10);
        path.line_to(10, 20);
        path.quad_to(40, 40, 30, 30);
        path.curve_to(10, 20, 30, -40, 10, 20);
        path.close_subpath();
        path.show();

        elements.clear();
        path.apply_mut(|element| {
            elements.push(element.type_);
        });
        assert!(!elements.is_empty());
    }
}
