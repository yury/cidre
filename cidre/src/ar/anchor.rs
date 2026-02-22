use crate::{arc, define_cls, define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARAnchor")]
    /// Physical location/orientation in world space tracked by ARKit.
    pub Anchor(ns::Id)
);

impl arc::A<Anchor> {
    /// Initializes an anchor with `transform`.
    #[cfg(target_arch = "aarch64")]
    pub fn init_with_transform(self, transform: simd::f32x4x4) -> arc::R<Anchor> {
        let mut out: *mut Anchor = unsafe { std::mem::transmute(self) };

        unsafe {
            core::arch::asm!(
                "bl \"_objc_msgSend$initWithTransform:\"",
                inlateout("x0") out,
                in("q0") transform.0.0,
                in("q1") transform.0.1,
                in("q2") transform.0.2,
                in("q3") transform.0.3,
                clobber_abi("C"),
            );

            std::mem::transmute(out)
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn init_with_transform(self, transform: simd::f32x4x4) -> arc::R<Anchor> {
        let _ = (self, transform);
        unimplemented!()
    }

    /// Initializes an anchor with `name` and `transform`.
    #[cfg(target_arch = "aarch64")]
    pub fn init_with_name_transform(
        self,
        name: &ns::String,
        transform: simd::f32x4x4,
    ) -> arc::R<Anchor> {
        let mut out: *mut Anchor = unsafe { std::mem::transmute(self) };

        unsafe {
            core::arch::asm!(
                "bl \"_objc_msgSend$initWithName:transform:\"",
                inlateout("x0") out,
                in("x2") name as *const ns::String,
                in("q0") transform.0.0,
                in("q1") transform.0.1,
                in("q2") transform.0.2,
                in("q3") transform.0.3,
                clobber_abi("C"),
            );

            std::mem::transmute(out)
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn init_with_name_transform(
        self,
        name: &ns::String,
        transform: simd::f32x4x4,
    ) -> arc::R<Anchor> {
        let _ = (name, transform);
        let _ = self;
        unimplemented!()
    }
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
    #[cfg(target_arch = "aarch64")]
    #[inline]
    pub fn transform(&self) -> simd::f32x4x4 {
        let q0: std::arch::aarch64::float32x4_t;
        let q1: std::arch::aarch64::float32x4_t;
        let q2: std::arch::aarch64::float32x4_t;
        let q3: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$transform",
                in("x0") self as *const Anchor,
                lateout("q0") q0,
                lateout("q1") q1,
                lateout("q2") q2,
                lateout("q3") q3,
                clobber_abi("C"),
            );
        }
        simd::f32x4x4(std::arch::aarch64::float32x4x4_t(q0, q1, q2, q3))
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn transform(&self) -> simd::f32x4x4 {
        unimplemented!()
    }
}

#[link(name = "ar", kind = "static")]
unsafe extern "C" {
    static AR_ANCHOR: &'static objc::Class<Anchor>;
}
