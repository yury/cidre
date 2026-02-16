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
    #[objc::msg_send(primaryLightDirection)]
    pub fn primary_light_direction(&self) -> simd::f32x3;

    /// Intensity of light in the primary direction.
    #[objc::msg_send(primaryLightIntensity)]
    pub fn primary_light_intensity(&self) -> cg::Float;
}
