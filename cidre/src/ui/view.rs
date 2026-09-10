#[cfg(feature = "blocks")]
use crate::blocks;
#[cfg(feature = "ca")]
use crate::ca;
use crate::{arc, cg, define_obj_type, define_opts, ns, objc, ui};

define_opts!(
    #[doc(alias = "UIViewAnimationOptions")]
    pub ViewAnimationOpts(usize)
);

impl ViewAnimationOpts {
    /// Animate contents (applies to transitions only).
    pub const LAYOUT_SUBVIEWS: Self = Self(1 << 0);
    /// Turn on user interaction while animating.
    pub const ALLOW_USER_INTERACTION: Self = Self(1 << 1);
    /// Start all views from current value, not initial value.
    pub const BEGIN_FROM_CURRENT_STATE: Self = Self(1 << 2);
    /// Repeat animation indefinitely.
    pub const REPEAT: Self = Self(1 << 3);
    /// If repeat, run animation back and forth.
    pub const AUTOREVERSE: Self = Self(1 << 4);
    /// Ignore nested duration.
    pub const OVERRIDE_INHERITED_DURATION: Self = Self(1 << 5);
    /// Ignore nested curve.
    pub const OVERRIDE_INHERITED_CURVE: Self = Self(1 << 6);
    /// Animate contents (applies to transitions only).
    pub const ALLOW_ANIMATED_CONTENT: Self = Self(1 << 7);
    /// Flip to/from hidden state instead of adding/removing.
    pub const SHOW_HIDE_TRANSITION_VIEWS: Self = Self(1 << 8);
    /// Do not inherit any options or animation type.
    pub const OVERRIDE_INHERITED_OPTIONS: Self = Self(1 << 9);

    pub const CURVE_EASE_IN_OUT: Self = Self(0 << 16);
    pub const CURVE_EASE_IN: Self = Self(1 << 16);
    pub const CURVE_EASE_OUT: Self = Self(2 << 16);
    pub const CURVE_LINEAR: Self = Self(3 << 16);

    pub const TRANSITION_NONE: Self = Self(0 << 20);
    pub const TRANSITION_FLIP_FROM_LEFT: Self = Self(1 << 20);
    pub const TRANSITION_FLIP_FROM_RIGHT: Self = Self(2 << 20);
    pub const TRANSITION_CURL_UP: Self = Self(3 << 20);
    pub const TRANSITION_CURL_DOWN: Self = Self(4 << 20);
    pub const TRANSITION_CROSS_DISSOLVE: Self = Self(5 << 20);
    pub const TRANSITION_FLIP_FROM_TOP: Self = Self(6 << 20);
    pub const TRANSITION_FLIP_FROM_BOTTOM: Self = Self(7 << 20);

    pub const PREFERRED_FRAMES_PER_SECOND_DEFAULT: Self = Self(0 << 24);
    pub const PREFERRED_FRAMES_PER_SECOND_60: Self = Self(3 << 24);
    pub const PREFERRED_FRAMES_PER_SECOND_30: Self = Self(7 << 24);
}

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

    #[objc::msg_send(addGestureRecognizer:)]
    pub fn add_gesture_recognizer(&mut self, val: &ui::GestureRecognizer);

    #[objc::msg_send(removeGestureRecognizer:)]
    pub fn remove_gesture_recognizer(&mut self, val: &ui::GestureRecognizer);

    #[objc::msg_send(gestureRecognizers)]
    pub fn gesture_recognizers(&self) -> Option<arc::R<ns::Array<ui::GestureRecognizer>>>;

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

    #[objc::msg_send(tag)]
    pub fn tag(&self) -> ns::Integer;

    #[objc::msg_send(setTag:)]
    pub fn set_tag(&mut self, val: ns::Integer);

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

/// UIViewAnimationWithBlocks
#[cfg(feature = "blocks")]
impl View {
    #[objc::msg_send(animateWithDuration:delay:options:animations:completion:)]
    pub fn animate_with_duration_delay_opts_ch(
        duration: ns::TimeInterval,
        delay: ns::TimeInterval,
        opts: ViewAnimationOpts,
        animations: &mut blocks::EscBlock<fn()>,
        completion: Option<&mut blocks::EscBlock<fn(bool)>>,
    );

    #[objc::msg_send(animateWithDuration:animations:completion:)]
    pub fn animate_with_duration_ch(
        duration: ns::TimeInterval,
        animations: &mut blocks::EscBlock<fn()>,
        completion: Option<&mut blocks::EscBlock<fn(bool)>>,
    );

    #[objc::msg_send(animateWithDuration:animations:)]
    pub fn animate_with_duration_block(
        duration: ns::TimeInterval,
        animations: &mut blocks::EscBlock<fn()>,
    );

    /// Animates the changes made in `animations` over `duration` seconds.
    pub fn animate(duration: ns::TimeInterval, animations: impl FnMut() + 'static) {
        let mut animations = blocks::EscBlock::new0(animations);
        Self::animate_with_duration_block(duration, &mut animations);
    }

    /// Performs `animations` using a timing curve described by the motion of a spring.
    ///
    /// `damping_ratio`: 1 for no oscillation, closer to 0 for more oscillation.
    /// `velocity`: initial spring velocity, relative to the total animation distance per second.
    #[objc::msg_send(animateWithDuration:delay:usingSpringWithDamping:initialSpringVelocity:options:animations:completion:)]
    #[objc::available(ios = 7.0, tvos = 9.0)]
    pub fn animate_spring_ch(
        duration: ns::TimeInterval,
        delay: ns::TimeInterval,
        damping_ratio: cg::Float,
        velocity: cg::Float,
        opts: ViewAnimationOpts,
        animations: &mut blocks::EscBlock<fn()>,
        completion: Option<&mut blocks::EscBlock<fn(bool)>>,
    );

    /// See [`Self::animate_spring_ch`]. The completion receives whether the animation finished.
    #[objc::available(ios = 7.0, tvos = 9.0)]
    pub fn animate_spring(
        duration: ns::TimeInterval,
        delay: ns::TimeInterval,
        damping_ratio: cg::Float,
        velocity: cg::Float,
        opts: ViewAnimationOpts,
        animations: impl FnMut() + 'static,
        completion: Option<impl FnMut(bool) + 'static>,
    ) {
        let mut animations = blocks::EscBlock::new0(animations);
        let mut completion = completion.map(blocks::EscBlock::new1);
        Self::animate_spring_ch(
            duration,
            delay,
            damping_ratio,
            velocity,
            opts,
            &mut animations,
            completion.as_deref_mut(),
        );
    }
}

/// Auto Layout
impl View {
    /// Whether the view's autoresizing mask is turned into constraints. Default is
    /// true; turn it off for views placed with constraints.
    #[objc::msg_send(translatesAutoresizingMaskIntoConstraints)]
    pub fn translates_autoresizing_mask_into_constraints(&self) -> bool;

    #[objc::msg_send(setTranslatesAutoresizingMaskIntoConstraints:)]
    pub fn set_translates_autoresizing_mask_into_constraints(&mut self, val: bool);

    #[objc::msg_send(safeAreaLayoutGuide)]
    #[objc::available(ios = 11.0, tvos = 11.0)]
    pub fn safe_area_layout_guide(&self) -> arc::R<ui::LayoutGuide>;

    #[objc::msg_send(layoutMarginsGuide)]
    #[objc::available(ios = 9.0, tvos = 9.0)]
    pub fn layout_margins_guide(&self) -> arc::R<ui::LayoutGuide>;

    #[objc::msg_send(addLayoutGuide:)]
    #[objc::available(ios = 9.0, tvos = 9.0)]
    pub fn add_layout_guide(&mut self, guide: &ui::LayoutGuide);

    #[objc::msg_send(removeLayoutGuide:)]
    #[objc::available(ios = 9.0, tvos = 9.0)]
    pub fn remove_layout_guide(&mut self, guide: &ui::LayoutGuide);

    #[objc::msg_send(layoutGuides)]
    #[objc::available(ios = 9.0, tvos = 9.0)]
    pub fn layout_guides(&self) -> arc::R<ns::Array<ui::LayoutGuide>>;

    #[objc::msg_send(constraints)]
    pub fn constraints(&self) -> arc::R<ns::Array<ns::LayoutConstraint>>;

    #[objc::msg_send(addConstraint:)]
    pub fn add_constraint(&mut self, constraint: &ns::LayoutConstraint);

    #[objc::msg_send(removeConstraint:)]
    pub fn remove_constraint(&mut self, constraint: &ns::LayoutConstraint);

    #[objc::msg_send(hasAmbiguousLayout)]
    pub fn has_ambiguous_layout(&self) -> bool;
}

ns::impl_layout_anchors!(View);
ns::impl_baseline_anchors!(View);

impl ns::KvObserverRegistration for View {}

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

unsafe extern "C" {
    static UI_VIEW: &'static objc::Class<View>;
}

impl View {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<View>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }
}
