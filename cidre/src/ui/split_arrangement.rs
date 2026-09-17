use crate::{api, arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UISplitArrangementDimension")]
    pub SplitArrangementDimension(ns::Id)
);

/// A width or a height of a child in a split arrangement.
impl SplitArrangementDimension {
    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    crate::define_cls!(UI_SPLIT_ARRANGEMENT_DIMENSION);

    #[objc::msg_send(automaticDimension)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn automatic() -> arc::R<Self>;

    #[objc::msg_send(intrinsicDimension)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn intrinsic() -> arc::R<Self>;

    /// A fraction of the arrangement's size along the axis.
    #[objc::msg_send(fractionalDimension:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn fractional(fraction: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(absoluteDimension:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn absolute(val: cg::Float) -> arc::R<Self>;
}

define_obj_type!(
    #[doc(alias = "UISplitArrangementDimensionRange")]
    pub SplitArrangementDimensionRange(ns::Id)
);

/// The minimum, preferred and maximum of a dimension.
impl SplitArrangementDimensionRange {
    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    crate::define_cls!(UI_SPLIT_ARRANGEMENT_DIMENSION_RANGE);

    #[objc::init(init)]
    pub fn init(self) -> arc::R<SplitArrangementDimensionRange>;

    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(minimum)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn min(&self) -> arc::R<SplitArrangementDimension>;

    #[objc::msg_send(setMinimum:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_min(&mut self, val: &SplitArrangementDimension);

    #[objc::msg_send(preferred)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn preferred(&self) -> arc::R<SplitArrangementDimension>;

    #[objc::msg_send(setPreferred:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_preferred(&mut self, val: &SplitArrangementDimension);

    #[objc::msg_send(maximum)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn max(&self) -> arc::R<SplitArrangementDimension>;

    #[objc::msg_send(setMaximum:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_max(&mut self, val: &SplitArrangementDimension);
}

define_obj_type!(
    #[doc(alias = "UISplitArrangementViewProperties")]
    pub SplitArrangementViewProps(ns::Id)
);

/// How a child is sized in a split arrangement.
impl SplitArrangementViewProps {
    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    crate::define_cls!(UI_SPLIT_ARRANGEMENT_VIEW_PROPERTIES);

    #[objc::init(init)]
    pub fn init(self) -> arc::R<SplitArrangementViewProps>;

    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(width)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn width(&self) -> arc::R<SplitArrangementDimensionRange>;

    #[objc::msg_send(setWidth:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_width(&mut self, val: &SplitArrangementDimensionRange);

    #[objc::msg_send(height)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn height(&self) -> arc::R<SplitArrangementDimensionRange>;

    #[objc::msg_send(setHeight:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_height(&mut self, val: &SplitArrangementDimensionRange);

    /// Which child gives way when both cannot have their preferred size.
    #[objc::msg_send(layoutPriority)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn layout_priority(&self) -> cg::Float;

    #[objc::msg_send(setLayoutPriority:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_layout_priority(&mut self, val: cg::Float);
}

define_obj_type!(
    #[doc(alias = "UISplitArrangement")]
    pub SplitArrangement(ui::Arrangement)
);

/// The primary and the secondary child side by side, along the axes.
impl SplitArrangement {
    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    crate::define_cls!(UI_SPLIT_ARRANGEMENT);

    #[objc::msg_send(splitArrangement)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn new() -> arc::R<Self>;

    #[objc::msg_send(axes)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn axes(&self) -> ui::Axis;

    #[objc::msg_send(setAxes:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_axes(&mut self, val: ui::Axis);

    /// The properties of a placement that has none of its own.
    #[objc::msg_send(defaultViewProperties)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn default_view_props(&self) -> arc::R<SplitArrangementViewProps>;

    #[objc::msg_send(setViewProperties:forPlacement:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_view_props_for_placement(
        &mut self,
        props: &SplitArrangementViewProps,
        placement: ui::ArrangementViewPlacement,
    );
}

unsafe extern "C" {
    static UI_SPLIT_ARRANGEMENT_DIMENSION: &'static objc::Class<SplitArrangementDimension>;
    static UI_SPLIT_ARRANGEMENT_DIMENSION_RANGE:
        &'static objc::Class<SplitArrangementDimensionRange>;
    static UI_SPLIT_ARRANGEMENT_VIEW_PROPERTIES: &'static objc::Class<SplitArrangementViewProps>;
    static UI_SPLIT_ARRANGEMENT: &'static objc::Class<SplitArrangement>;
}
