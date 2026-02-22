use crate::{arc, cg, define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARLightEstimate")]
    /// Light estimate for the current AR scene.
    pub LightEstimate(ns::Id)
);

impl LightEstimate {
    /// Ambient intensity of the scene lighting.
    ///
    /// In a well-lit environment this value is usually near `1000`.
    #[objc::msg_send(ambientIntensity)]
    pub fn ambient_intensity(&self) -> cg::Float;

    /// Ambient color temperature in Kelvin.
    #[objc::msg_send(ambientColorTemperature)]
    pub fn ambient_color_temperature(&self) -> cg::Float;
}

define_obj_type!(
    #[doc(alias = "ARDirectionalLightEstimate")]
    /// Directional light estimate with coefficients and primary light.
    pub DirectionalLightEstimate(LightEstimate)
);

impl DirectionalLightEstimate {
    /// Second-degree spherical harmonics coefficients.
    ///
    /// The data contains 27 `f32` values in three non-interleaved RGB sets.
    #[objc::msg_send(sphericalHarmonicsCoefficients)]
    pub fn spherical_harmonics_coefficients(&self) -> arc::R<ns::Data>;

    /// Primary direction of light.
    #[doc(alias = "primaryLightDirection")]
    #[cfg(target_arch = "aarch64")]
    pub fn primary_light_direction(&self) -> simd::f32x4 {
        let q0: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$primaryLightDirection",
                in("x0") self as *const DirectionalLightEstimate,
                lateout("q0") q0,
                clobber_abi("C"),
            );
        }

        simd::f32x4(q0)
    }

    #[cfg(not(target_arch = "aarch64"))]
    #[doc(alias = "primaryLightDirection")]
    pub fn primary_light_direction(&self) -> simd::f32x4 {
        unimplemented!()
    }

    /// Intensity of light in the primary direction.
    #[objc::msg_send(primaryLightIntensity)]
    pub fn primary_light_intensity(&self) -> cg::Float;
}
