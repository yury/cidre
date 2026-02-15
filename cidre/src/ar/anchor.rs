use crate::{arc, define_cls, define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARAnchor")]
    /// Physical location/orientation in world space tracked by ARKit.
    pub Anchor(ns::Id)
);

impl arc::A<Anchor> {
    /// Initializes an anchor with `transform`.
    #[objc::msg_send(initWithTransform:)]
    pub fn init_with_transform(self, transform: simd::f32x4x4) -> arc::R<Anchor>;

    /// Initializes an anchor with `name` and `transform`.
    #[objc::msg_send(initWithName:transform:)]
    pub fn init_with_name_transform(
        self,
        name: &ns::String,
        transform: simd::f32x4x4,
    ) -> arc::R<Anchor>;
}

impl Anchor {
    define_cls!(AR_ANCHOR);

    /// Creates an anchor with `transform`.
    #[inline]
    pub fn with_transform(transform: simd::f32x4x4) -> arc::R<Self> {
        Self::alloc().init_with_transform(transform)
    }

    /// Creates an anchor with `name` and `transform`.
    #[inline]
    #[objc::available(ios = 12.0)]
    pub fn with_name_transform(name: &ns::String, transform: simd::f32x4x4) -> arc::R<Self> {
        Self::alloc().init_with_name_transform(name, transform)
    }

    /// Unique identifier of the anchor.
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::Uuid>;

    /// Optional name associated with the anchor.
    #[objc::msg_send(name)]
    #[objc::available(ios = 12.0)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    /// Identifier of the session that owns this anchor.
    #[objc::msg_send(sessionIdentifier)]
    #[objc::available(ios = 13.0)]
    pub fn session_id(&self) -> Option<arc::R<ns::Uuid>>;

    /// Anchor transform in world coordinates.
    #[objc::msg_send(transform)]
    pub fn transform(&self) -> simd::f32x4x4;
}

#[link(name = "ar", kind = "static")]
unsafe extern "C" {
    static AR_ANCHOR: &'static objc::Class<Anchor>;
}
