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
        let transform_base = &transform as *const simd::f32x4x4 as *const simd::f32x4;
        let transform_c0 = transform_base;
        let transform_c1 = unsafe { transform_base.add(1) };
        let transform_c2 = unsafe { transform_base.add(2) };
        let transform_c3 = unsafe { transform_base.add(3) };

        let mut out: *mut Anchor = unsafe { std::mem::transmute(self) };

        unsafe {
            core::arch::asm!(
                "ldr q0, [x23]",
                "ldr q1, [x24]",
                "ldr q2, [x25]",
                "ldr q3, [x26]",
                "bl \"_objc_msgSend$initWithTransform:\"",
                inlateout("x0") out,
                in("x23") transform_c0,
                in("x24") transform_c1,
                in("x25") transform_c2,
                in("x26") transform_c3,
                lateout("x23") _,
                lateout("x24") _,
                lateout("x25") _,
                lateout("x26") _,
                clobber_abi("C"),
            );

            std::mem::transmute(out)
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn init_with_transform(self, transform: simd::f32x4x4) -> arc::R<Anchor> {
        unimplemented!()
    }

    /// Initializes an anchor with `name` and `transform`.
    #[cfg(target_arch = "aarch64")]
    pub fn init_with_name_transform(
        self,
        name: &ns::String,
        transform: simd::f32x4x4,
    ) -> arc::R<Anchor> {
        let transform_base = &transform as *const simd::f32x4x4 as *const simd::f32x4;
        let transform_c0 = transform_base;
        let transform_c1 = unsafe { transform_base.add(1) };
        let transform_c2 = unsafe { transform_base.add(2) };
        let transform_c3 = unsafe { transform_base.add(3) };

        let mut out: *mut Anchor = unsafe { std::mem::transmute(self) };

        unsafe {
            core::arch::asm!(
                "ldr q0, [x23]",
                "ldr q1, [x24]",
                "ldr q2, [x25]",
                "ldr q3, [x26]",
                "bl \"_objc_msgSend$initWithName:transform:\"",
                inlateout("x0") out,
                in("x2") name as *const ns::String,
                in("x23") transform_c0,
                in("x24") transform_c1,
                in("x25") transform_c2,
                in("x26") transform_c3,
                lateout("x23") _,
                lateout("x24") _,
                lateout("x25") _,
                lateout("x26") _,
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
    pub fn transform(&self) -> simd::f32x4x4 {
        let mut out = std::mem::MaybeUninit::<simd::f32x4x4>::uninit();

        unsafe {
            let out_base = out.as_mut_ptr() as *mut simd::f32x4;
            let out_c0 = out_base;
            let out_c1 = out_base.add(1);
            let out_c2 = out_base.add(2);
            let out_c3 = out_base.add(3);

            core::arch::asm!(
                "bl _objc_msgSend$transform",
                "str q0, [x23]",
                "str q1, [x24]",
                "str q2, [x25]",
                "str q3, [x26]",
                in("x0") self as *const Anchor,
                in("x23") out_c0,
                in("x24") out_c1,
                in("x25") out_c2,
                in("x26") out_c3,
                lateout("x23") _,
                lateout("x24") _,
                lateout("x25") _,
                lateout("x26") _,
                clobber_abi("C"),
            );

            out.assume_init()
        }
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
