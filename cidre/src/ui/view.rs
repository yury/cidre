#[cfg(feature = "ca")]
use crate::ca;
use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIView")]
    pub View(ns::Id), UI_VIEW
);

impl View {
    #[cfg(feature = "ca")]
    #[objc::msg_send(layer)]
    pub fn layer(&self) -> &ca::Layer;

    #[objc::msg_send(backgroundColor)]
    pub fn background_color(&self) -> Option<&ui::Color>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_background_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    pub fn set_hidden(&self, val: bool);
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
