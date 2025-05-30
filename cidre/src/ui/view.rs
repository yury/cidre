#[cfg(feature = "ca")]
use crate::ca;
use crate::{arc, cg, define_obj_type, define_opts, ns, objc, ui};

define_opts!(
    #[doc(alias = "UIViewAutoresizing")]
    pub ViewAutoresizing(usize)
);

impl ViewAutoresizing {
    pub const NONE: Self = Self(0);
    pub const FLEX_LEFT_MARGIN: Self = Self(1 << 0);
    pub const FLEX_WIDTH: Self = Self(1 << 1);
    pub const FLEX_RIGHT_MARGIN: Self = Self(1 << 2);
    pub const FLEX_TOP_MARGIN: Self = Self(1 << 3);
    pub const FLEX_HEIGHT: Self = Self(1 << 4);
    pub const FLEX_BOTTOM_MARGIN: Self = Self(1 << 5);
}

define_obj_type!(
    #[doc(alias = "UIView")]
    pub View(ns::Id), UI_VIEW
);

impl View {
    #[cfg(feature = "ca")]
    #[objc::msg_send(layer)]
    pub fn layer(&self) -> arc::R<ca::Layer>;

    #[objc::msg_send(backgroundColor)]
    pub fn background_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_background_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    pub fn set_hidden(&self, val: bool);
}

/// UIViewGeometry
impl View {
    /// Animatable. do not use frame if view is transformed since it will not correctly
    /// reflect the actual location of the view. use bounds + center instead.
    #[objc::msg_send(frame)]
    pub fn frame(&self) -> cg::Rect;

    #[objc::msg_send(setFrame:)]
    pub fn set_frame(&mut self, val: cg::Rect);

    /// Use bounds/center and not frame if non-identity transform.
    ///
    /// default bounds is zero origin, frame size. animatable
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, val: cg::Rect);

    /// Center is center of frame, relative to anchorPoint. animatable
    #[objc::msg_send(center)]
    pub fn center(&self) -> cg::Point;

    #[objc::msg_send(setCenter:)]
    pub fn set_center(&self, val: cg::Point);

    #[objc::msg_send(transform)]
    pub fn transform(&self) -> cg::AffineTransform;

    #[objc::msg_send(setTransform:)]
    pub fn set_transform(&mut self, val: cg::AffineTransform);

    #[objc::msg_send(transform3D)]
    pub fn transform_3d(&self) -> ca::Transform3d;

    #[objc::msg_send(setTransform3D:)]
    pub fn set_transform_3d(&mut self, val: ca::Transform3d);

    #[objc::msg_send(contentScaleFactor)]
    pub fn content_scale_factor(&self) -> cg::Float;

    #[objc::msg_send(setContentScaleFactor:)]
    pub fn set_content_scale_factor(&mut self, val: cg::Float);

    #[objc::msg_send(anchorPoint)]
    pub fn anchor_point(&self) -> cg::Point;

    #[objc::msg_send(setAnchorPoint:)]
    pub fn set_anchor_point(&mut self, val: cg::Point);

    #[objc::msg_send(isMultipleTouchEnabled)]
    pub fn is_multiple_touch_enabled(&self) -> bool;

    #[objc::msg_send(setMultipleTouchEnabled:)]
    pub fn set_multiple_touch_enabled(&mut self, val: bool);

    #[objc::msg_send(isExclusiveTouch)]
    pub fn is_exclusive_touch(&self) -> bool;

    #[objc::msg_send(setExclusiveTouch:)]
    pub fn set_exclusive_touch(&mut self, val: bool);

    // #[objc::msg_send(pointInside:withEvent:)]
    // pub fn point_inside_with_event(&self, val: cg::Point, event: Option<&ui::Event>);

    #[objc::msg_send(convertPoint:toView:)]
    pub fn convert_point_to_view(&self, val: cg::Point, to_view: Option<&ui::View>) -> cg::Point;

    #[objc::msg_send(convertPoint:fromView:)]
    pub fn convert_point_from_view(
        &self,
        val: cg::Point,
        from_view: Option<&ui::View>,
    ) -> cg::Point;

    #[objc::msg_send(convertRect:toView:)]
    pub fn convert_rect_to_view(&self, val: cg::Rect, to_view: Option<&ui::View>) -> cg::Rect;

    #[objc::msg_send(convertRect:fromView:)]
    pub fn convert_rect_from_view(&self, val: cg::Rect, from_view: Option<&ui::View>) -> cg::Rect;

    /// Default is YES. if set, subviews are adjusted according to their autoresizingMask if self.bounds changes
    #[objc::msg_send(autoresizesSubviews)]
    pub fn autoresizes_subviews(&self) -> bool;

    #[objc::msg_send(setAutoresizesSubviews:)]
    pub fn set_autoresizes_subviews(&mut self, val: bool);

    #[objc::msg_send(autoresizingMask)]
    pub fn autoresizing_mask(&self) -> ui::ViewAutoresizing;

    #[objc::msg_send(setAutoresizingMask:)]
    pub fn set_autoresizing_mask(&mut self, val: ui::ViewAutoresizing);

    #[objc::msg_send(sizeThatFits)]
    pub fn size_that_fits(&self) -> cg::Size;

    #[objc::msg_send(sizeToFit)]
    pub fn size_to_fit(&mut self);
}

/// UIViewHierarchy
impl View {
    #[objc::msg_send(superview)]
    pub fn superview(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(subviews)]
    pub fn subviews(&self) -> arc::R<ns::Array<ui::View>>;

    #[objc::msg_send(window)]
    pub fn window(&self) -> Option<arc::R<ui::Window>>;

    #[objc::msg_send(removeFromSuperview)]
    pub fn remove_from_superview(&self);

    #[objc::msg_send(insertSubview:atIndex:)]
    pub fn insert_subview(&self, view: &ui::View, at_index: ns::Integer);

    #[objc::msg_send(exchangeSubviewAtIndex:withSubviewAtIndex:)]
    pub fn exchange_subview_at_index(&self, index_a: ns::Integer, index_b: ns::Integer);

    #[objc::msg_send(addSubview:)]
    pub fn add_subview(&self, view: &ui::View);

    #[objc::msg_send(insertSubview:belowSubview:)]
    pub fn insert_subview_below(&self, view: &ui::View, sibling_view: &ui::View);

    #[objc::msg_send(insertSubview:aboveSubview:)]
    pub fn insert_subview_above(&self, view: &ui::View, sibling_view: &ui::View);

    #[objc::msg_send(bringSubviewToFront:)]
    pub fn bring_subview_to_front(&self, view: &ui::View);

    #[objc::msg_send(sendSubviewToBack:)]
    pub fn send_subview_to_back(&self, view: &ui::View);

    #[objc::msg_send(didAddSubview:)]
    pub fn did_add_subview(&self, view: &ui::View);

    #[objc::msg_send(willRemoveSubview:)]
    pub fn will_remove_subview(&self, view: &ui::View);

    #[objc::msg_send(willMoveToSuperview:)]
    pub fn will_move_to_superview(&self, new_superview: Option<&ui::View>);

    #[objc::msg_send(didMoveToSuperview)]
    pub fn did_move_to_superview(&self);

    #[objc::msg_send(willMoveToWindow:)]
    pub fn will_move_to_window(&self, window: Option<&ui::Window>);

    #[objc::msg_send(didMoveToWindow)]
    pub fn did_move_to_window(&self);

    #[objc::msg_send(isDescendantOfView:)]
    pub fn is_descendant_of_view(&self, view: &ui::View) -> bool;

    #[objc::msg_send(viewWithTag:)]
    pub fn view_with_tag(&self, tag: ns::Integer) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setNeedsLayout)]
    pub fn set_needs_layout(&self);

    #[objc::msg_send(layoutIfNeeded)]
    pub fn layout_if_needed(&self);

    #[objc::msg_send(layoutSubviews)]
    pub fn layout_subviews(&self);

    #[objc::msg_send(layoutMargins)]
    pub fn layout_margins(&self) -> ui::EdgeInsets;

    // pub fn directional_layout_margins(&self) -> ...

    #[objc::msg_send(preservesSuperviewLayoutMargins)]
    pub fn preserves_superview_layout_margins(&self) -> bool;

    #[objc::msg_send(setPreservesSuperviewLayoutMargins:)]
    pub fn set_preserves_superview_layout_margins(&self, val: bool);

    #[objc::msg_send(insetsLayoutMarginsFromSafeArea)]
    pub fn insets_layout_margins_from_safe_area(&self) -> bool;

    #[objc::msg_send(setInsetsLayoutMarginsFromSafeArea:)]
    pub fn set_insets_layout_margins_from_safe_area(&self, val: bool);

    #[objc::msg_send(layoutMarginsDidChange)]
    pub fn layout_margins_did_change(&self);

    #[objc::msg_send(safeAreaInsets)]
    pub fn safe_area_insets(&self) -> ui::EdgeInsets;

    #[objc::msg_send(safeAreaInsetsDidChange)]
    pub fn safe_area_insets_did_change(&self);
}

#[objc::protocol(UICoordinateSpace)]
pub trait CoordinateSpace: objc::Obj {
    #[objc::msg_send(convertPoint:toCoordinateSpace:)]
    fn _convert_point_to_coordinate_space(
        &self,
        point: cg::Point,
        space: &AnyCoordinateSpace,
    ) -> cg::Point;

    #[inline]
    fn convert_point_to_coordinate_space(
        &self,
        point: cg::Point,
        space: &impl CoordinateSpace,
    ) -> cg::Point {
        self._convert_point_to_coordinate_space(point, AnyCoordinateSpace::new(space))
    }

    #[objc::msg_send(convertPoint:fromCoordinateSpace:)]
    fn _convert_point_from_coordinate_space(
        &self,
        point: cg::Point,
        to: &AnyCoordinateSpace,
    ) -> cg::Point;

    #[inline]
    fn convert_point_from_coordinate_space(
        &self,
        point: cg::Point,
        space: &impl CoordinateSpace,
    ) -> cg::Point {
        self._convert_point_from_coordinate_space(point, AnyCoordinateSpace::new(space))
    }

    #[objc::msg_send(convertRect:toCoordinateSpace:)]
    fn _convert_rect_to_coordinate_space(
        &self,
        rect: cg::Rect,
        space: &AnyCoordinateSpace,
    ) -> cg::Rect;

    fn convert_rect_to_coordinate_space(
        &self,
        rect: cg::Rect,
        space: &impl CoordinateSpace,
    ) -> cg::Rect {
        self._convert_rect_to_coordinate_space(rect, AnyCoordinateSpace::new(space))
    }

    #[objc::msg_send(convertRect:fromCoordinateSpace:)]
    fn _convert_rect_from_coordinate_space(
        &self,
        rect: cg::Rect,
        to: &AnyCoordinateSpace,
    ) -> cg::Rect;

    fn convert_rect_from_coordinate_space(
        &self,
        rect: cg::Rect,
        space: &impl CoordinateSpace,
    ) -> cg::Rect {
        self._convert_rect_from_coordinate_space(rect, AnyCoordinateSpace::new(space))
    }

    #[objc::msg_send(bounds)]
    fn bounds(&self) -> cg::Rect;
}

define_obj_type!(
    pub AnyCoordinateSpace(ns::Id)
);

impl CoordinateSpace for AnyCoordinateSpace {}
impl CoordinateSpace for View {}

impl AnyCoordinateSpace {
    #[inline]
    pub const fn new(other: &impl CoordinateSpace) -> &Self {
        unsafe { std::mem::transmute(other) }
    }
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_VIEW: &'static objc::Class<View>;
}
