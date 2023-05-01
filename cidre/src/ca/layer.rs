use crate::{ca, cg, define_obj_type, define_options, ns, objc};

define_obj_type!(ContentsGravity(ns::String));
define_obj_type!(ContentsFormat(ns::String));
define_obj_type!(ContentsFilter(ns::String));
define_obj_type!(CornerCurve(ns::String));

define_options!(AutoresizingMask(u32));
impl AutoresizingMask {
    pub const NOT_SIZABLE: Self = Self(0);
    pub const MIN_X_MARGIN: Self = Self(1 << 0);
    pub const WIDTH_SIZABLE: Self = Self(1 << 1);
    pub const MAX_X_MARGIN: Self = Self(1 << 2);
    pub const MIN_Y_MARGIN: Self = Self(1 << 3);
    pub const HEIGHT_SIZABLE: Self = Self(1 << 4);
    pub const MAX_Y_MARGIN: Self = Self(1 << 5);
}

define_options!(EdgeAntialiasingMask(u32));
impl EdgeAntialiasingMask {
    pub const LEFT: Self = Self(1 << 0); /* Minimum X edge. */
    pub const RIGHT: Self = Self(1 << 1); /* Maximum X edge. */
    pub const BOTTOM: Self = Self(1 << 2); /* Minimum Y edge. */
    pub const TOP: Self = Self(1 << 3); /* Maximum Y edge. */
}

define_options!(CornerMask(usize));
impl CornerMask {
    pub const MIN_X_MIN_Y: Self = Self(1 << 0);
    pub const MAX_X_MIN_Y: Self = Self(1 << 1);
    pub const MIN_X_MAX_Y: Self = Self(1 << 2);
    pub const MAX_X_MAX_Y: Self = Self(1 << 3);
}

define_obj_type!(Layer(ns::Id));
impl Layer {
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, value: cg::Rect);

    #[objc::msg_send(position)]
    pub fn position(&self) -> cg::Point;

    #[objc::msg_send(setPosition:)]
    pub fn set_position(&mut self, value: cg::Point);

    #[objc::msg_send(zPosition)]
    pub fn z_position(&self) -> cg::Float;

    #[objc::msg_send(setZPosition:)]
    pub fn set_z_position(&mut self, value: cg::Float);

    #[objc::msg_send(anchorPoint)]
    pub fn anchor_point(&self) -> cg::Point;

    #[objc::msg_send(setAnchorPoint:)]
    pub fn set_anchor_point(&mut self, value: cg::Point);

    #[objc::msg_send(transform)]
    pub fn transform(&self) -> ca::Transform3D;

    #[objc::msg_send(setTransform:)]
    pub fn set_transform(&mut self, value: ca::Transform3D);

    #[objc::msg_send(frame)]
    pub fn frame(&self) -> cg::Rect;

    #[objc::msg_send(setFrame:)]
    pub fn set_frame(&mut self, value: cg::Rect);

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    pub fn set_hidden(&mut self, value: bool);

    #[objc::msg_send(addSublayer:)]
    pub fn add_sublayer(&mut self, layer: &Self);

    #[objc::msg_send(insertSublayer:atIndex:)]
    pub fn insert_sublayer_at(&mut self, layer: &Self, index: u32);
}
