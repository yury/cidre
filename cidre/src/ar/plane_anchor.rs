use crate::{ar, define_cls, define_obj_type, objc, simd};

#[doc(alias = "ARPlaneAnchorAlignment")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum PlaneAnchorAlignment {
    /// Horizontal plane with respect to gravity.
    Horizontal,
    /// Vertical plane with respect to gravity.
    Vertical,
}

#[doc(alias = "ARPlaneClassificationStatus")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum PlaneClassificationStatus {
    /// Plane classification is currently unavailable.
    NotAvailable = 0,
    /// ARKit has not yet determined classification for this plane.
    Undetermined,
    /// Plane is not any known class.
    Unknown,
    /// Plane has a confident classification.
    Known,
}

#[doc(alias = "ARPlaneClassification")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum PlaneClassification {
    /// Plane does not match known classes.
    None = 0,
    /// Wall surface.
    Wall,
    /// Floor surface.
    Floor,
    /// Ceiling surface.
    Ceiling,
    /// Table surface.
    Table,
    /// Seat surface.
    Seat,
    /// Window surface.
    Window,
    /// Door surface.
    Door,
}

define_obj_type!(
    #[doc(alias = "ARPlaneAnchor")]
    /// Anchor representing a planar surface in the world.
    pub PlaneAnchor(ar::Anchor)
);

impl PlaneAnchor {
    define_cls!(AR_PLANE_ANCHOR);

    /// Whether plane classification is supported on this device.
    #[objc::msg_send(isClassificationSupported)]
    #[objc::available(ios = 12.0)]
    pub fn is_classification_supported() -> bool;

    /// Alignment of the plane.
    #[objc::msg_send(alignment)]
    pub fn alignment(&self) -> PlaneAnchorAlignment;

    /// Center of the plane in anchor coordinates.
    #[objc::msg_send(center)]
    pub fn center(&self) -> simd::f32x3;

    /// Extent of the plane in anchor coordinates.
    #[objc::msg_send(extent)]
    pub fn extent(&self) -> simd::f32x3;

    /// Classification status for this plane.
    #[objc::msg_send(classificationStatus)]
    #[objc::available(ios = 12.0)]
    pub fn classification_status(&self) -> PlaneClassificationStatus;

    /// Classification for this plane.
    #[objc::msg_send(classification)]
    #[objc::available(ios = 12.0)]
    pub fn classification(&self) -> PlaneClassification;
}

#[link(name = "ar", kind = "static")]
unsafe extern "C" {
    static AR_PLANE_ANCHOR: &'static objc::Class<PlaneAnchor>;
}
