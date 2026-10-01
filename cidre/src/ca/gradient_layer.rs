use crate::{arc, ca, cf, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CAGradientLayerType")]
    pub GradientLayerType(ns::String)
);

impl GradientLayerType {
    #[doc(alias = "kCAGradientLayerAxial")]
    #[inline]
    pub fn axial() -> &'static Self {
        unsafe { kCAGradientLayerAxial }
    }

    #[doc(alias = "kCAGradientLayerRadial")]
    #[inline]
    pub fn radial() -> &'static Self {
        unsafe { kCAGradientLayerRadial }
    }

    #[doc(alias = "kCAGradientLayerConic")]
    #[inline]
    pub fn conic() -> &'static Self {
        unsafe { kCAGradientLayerConic }
    }
}

define_obj_type!(
    /// Draws a color gradient over its background color, filling the shape
    /// of the layer (including rounded corners).
    #[doc(alias = "CAGradientLayer")]
    pub GradientLayer(ca::Layer),
    sym CAGradientLayer
);

impl GradientLayer {
    /// The colors of the gradient stops.
    #[objc::msg_send(colors)]
    pub fn colors(&self) -> Option<&cf::ArrayOf<cg::Color>>;

    #[objc::msg_send(setColors:)]
    pub fn set_colors(&mut self, val: Option<&cf::ArrayOf<cg::Color>>);

    /// The locations of the gradient stops in `0..=1`, monotonically increasing.
    /// `None` spreads the stops uniformly.
    #[objc::msg_send(locations)]
    pub fn locations(&self) -> Option<arc::R<ns::Array<ns::Number>>>;

    #[objc::msg_send(setLocations:)]
    pub fn set_locations(&mut self, val: Option<&ns::Array<ns::Number>>);

    /// The start point in the unit coordinate space of the layer. Defaults to `(0.5, 0)`.
    #[objc::msg_send(startPoint)]
    pub fn start_point(&self) -> cg::Point;

    #[objc::msg_send(setStartPoint:)]
    pub fn set_start_point(&mut self, val: cg::Point);

    /// The end point in the unit coordinate space of the layer. Defaults to `(0.5, 1)`.
    #[objc::msg_send(endPoint)]
    pub fn end_point(&self) -> cg::Point;

    #[objc::msg_send(setEndPoint:)]
    pub fn set_end_point(&mut self, val: cg::Point);

    #[objc::msg_send(type)]
    pub fn type_(&self) -> arc::R<GradientLayerType>;

    #[objc::msg_send(setType:)]
    pub fn set_type(&mut self, val: &GradientLayerType);
}

unsafe extern "C" {
    static kCAGradientLayerAxial: &'static GradientLayerType;
    static kCAGradientLayerRadial: &'static GradientLayerType;
    static kCAGradientLayerConic: &'static GradientLayerType;
}

#[cfg(test)]
mod tests {
    use crate::{ca, cf, cg, ns};

    #[test]
    fn basics() {
        let mut layer = ca::GradientLayer::new();
        assert!(layer.colors().is_none());
        let red = cg::Color::generic_rgba(1.0, 0.0, 0.0, 1.0);
        let blue = cg::Color::generic_rgba(0.0, 0.0, 1.0, 1.0);
        let colors = cf::ArrayOf::from_slice(&[red.as_ref(), blue.as_ref()]);
        layer.set_colors(Some(&colors));
        assert_eq!(layer.colors().unwrap().len(), 2);
        let locations = ns::Array::from_slice(&[
            ns::Number::with_f64(0.0).as_ref(),
            ns::Number::with_f64(1.0).as_ref(),
        ]);
        layer.set_locations(Some(&locations));
        assert_eq!(layer.locations().unwrap().len(), 2);
        layer.set_start_point(cg::Point::new(0.0, 0.5));
        assert_eq!(layer.start_point(), cg::Point::new(0.0, 0.5));
        layer.set_type(ca::GradientLayerType::radial());
        assert!(layer.type_().is_equal(ca::GradientLayerType::radial()));
    }
}
