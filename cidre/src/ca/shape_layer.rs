use crate::{arc, ca, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CAShapeLayerFillRule")]
    pub FillRule(ns::String)
);

impl FillRule {
    #[doc(alias = "kCAFillRuleNonZero")]
    #[inline]
    pub fn non_zero() -> &'static Self {
        unsafe { kCAFillRuleNonZero }
    }

    #[doc(alias = "kCAFillRuleEvenOdd")]
    #[inline]
    pub fn even_odd() -> &'static Self {
        unsafe { kCAFillRuleEvenOdd }
    }
}

define_obj_type!(
    #[doc(alias = "CAShapeLayerLineJoin")]
    pub LineJoin(ns::String)
);

impl LineJoin {
    #[doc(alias = "kCALineJoinMiter")]
    #[inline]
    pub fn miter() -> &'static Self {
        unsafe { kCALineJoinMiter }
    }

    #[doc(alias = "kCALineJoinRound")]
    #[inline]
    pub fn round() -> &'static Self {
        unsafe { kCALineJoinRound }
    }

    #[doc(alias = "kCALineJoinBevel")]
    #[inline]
    pub fn bevel() -> &'static Self {
        unsafe { kCALineJoinBevel }
    }
}

define_obj_type!(
    #[doc(alias = "CAShapeLayerLineCap")]
    pub LineCap(ns::String)
);

impl LineCap {
    #[doc(alias = "kCALineCapButt")]
    #[inline]
    pub fn butt() -> &'static Self {
        unsafe { kCALineCapButt }
    }

    #[doc(alias = "kCALineCapRound")]
    #[inline]
    pub fn round() -> &'static Self {
        unsafe { kCALineCapRound }
    }

    #[doc(alias = "kCALineCapSquare")]
    #[inline]
    pub fn square() -> &'static Self {
        unsafe { kCALineCapSquare }
    }
}

define_obj_type!(
    /// Draws a cubic Bezier spline in its coordinate space, composited
    /// between the layer's background and its first sublayer.
    #[doc(alias = "CAShapeLayer")]
    pub ShapeLayer(ca::Layer),
    sym CAShapeLayer
);

impl ShapeLayer {
    /// The path defining the shape to be rendered. Animatable.
    #[objc::msg_send(path)]
    pub fn path(&self) -> Option<&cg::Path>;

    #[objc::msg_send(setPath:)]
    pub fn set_path(&mut self, val: Option<&cg::Path>);

    /// The color to fill the path, or `None` for no fill. Defaults to opaque black.
    #[objc::msg_send(fillColor)]
    pub fn fill_color(&self) -> Option<&cg::Color>;

    #[objc::msg_send(setFillColor:)]
    pub fn set_fill_color(&mut self, val: Option<&cg::Color>);

    #[objc::msg_send(fillRule)]
    pub fn fill_rule(&self) -> arc::R<FillRule>;

    #[objc::msg_send(setFillRule:)]
    pub fn set_fill_rule(&mut self, val: &FillRule);

    /// The color to stroke the path, or `None` for no stroke. Defaults to `None`.
    #[objc::msg_send(strokeColor)]
    pub fn stroke_color(&self) -> Option<&cg::Color>;

    #[objc::msg_send(setStrokeColor:)]
    pub fn set_stroke_color(&mut self, val: Option<&cg::Color>);

    /// The start of the stroked part of the path in `0..=1`. Defaults to 0.
    #[objc::msg_send(strokeStart)]
    pub fn stroke_start(&self) -> cg::Float;

    #[objc::msg_send(setStrokeStart:)]
    pub fn set_stroke_start(&mut self, val: cg::Float);

    /// The end of the stroked part of the path in `0..=1`. Defaults to 1.
    #[objc::msg_send(strokeEnd)]
    pub fn stroke_end(&self) -> cg::Float;

    #[objc::msg_send(setStrokeEnd:)]
    pub fn set_stroke_end(&mut self, val: cg::Float);

    #[objc::msg_send(lineWidth)]
    pub fn line_width(&self) -> cg::Float;

    #[objc::msg_send(setLineWidth:)]
    pub fn set_line_width(&mut self, val: cg::Float);

    #[objc::msg_send(miterLimit)]
    pub fn miter_limit(&self) -> cg::Float;

    #[objc::msg_send(setMiterLimit:)]
    pub fn set_miter_limit(&mut self, val: cg::Float);

    #[objc::msg_send(lineCap)]
    pub fn line_cap(&self) -> arc::R<LineCap>;

    #[objc::msg_send(setLineCap:)]
    pub fn set_line_cap(&mut self, val: &LineCap);

    #[objc::msg_send(lineJoin)]
    pub fn line_join(&self) -> arc::R<LineJoin>;

    #[objc::msg_send(setLineJoin:)]
    pub fn set_line_join(&mut self, val: &LineJoin);

    #[objc::msg_send(lineDashPhase)]
    pub fn line_dash_phase(&self) -> cg::Float;

    #[objc::msg_send(setLineDashPhase:)]
    pub fn set_line_dash_phase(&mut self, val: cg::Float);

    /// The dash pattern: lengths of the painted and unpainted segments, alternating.
    /// `None` draws a solid line.
    #[objc::msg_send(lineDashPattern)]
    pub fn line_dash_pattern(&self) -> Option<arc::R<ns::Array<ns::Number>>>;

    #[objc::msg_send(setLineDashPattern:)]
    pub fn set_line_dash_pattern(&mut self, val: Option<&ns::Array<ns::Number>>);
}

unsafe extern "C" {
    static kCAFillRuleNonZero: &'static FillRule;
    static kCAFillRuleEvenOdd: &'static FillRule;

    static kCALineJoinMiter: &'static LineJoin;
    static kCALineJoinRound: &'static LineJoin;
    static kCALineJoinBevel: &'static LineJoin;

    static kCALineCapButt: &'static LineCap;
    static kCALineCapRound: &'static LineCap;
    static kCALineCapSquare: &'static LineCap;
}

#[cfg(test)]
mod tests {
    use crate::{ca, cg, ns};

    #[test]
    fn basics() {
        let mut layer = ca::ShapeLayer::new();
        assert!(layer.path().is_none());
        assert!(layer.stroke_color().is_none());

        let mut path = cg::PathMut::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 0.0);
        layer.set_path(Some(&path));
        assert!(layer.path().is_some());

        layer.set_line_width(3.0);
        assert_eq!(layer.line_width(), 3.0);
        layer.set_line_cap(ca::LineCap::round());
        assert!(layer.line_cap().is_equal(ca::LineCap::round()));
        layer.set_fill_rule(ca::FillRule::even_odd());
        assert!(layer.fill_rule().is_equal(ca::FillRule::even_odd()));

        let dashes = ns::Array::from_slice(&[
            ns::Number::with_f64(1.0).as_ref(),
            ns::Number::with_f64(4.0).as_ref(),
        ]);
        layer.set_line_dash_pattern(Some(&dashes));
        assert_eq!(layer.line_dash_pattern().unwrap().len(), 2);
        layer.set_stroke_color(Some(&cg::Color::generic_gray(0.0, 1.0)));
        assert!(layer.stroke_color().is_some());
    }
}
