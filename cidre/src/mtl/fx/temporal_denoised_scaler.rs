use crate::{api, arc, define_obj_type, mtl, mtl4, ns, objc, simd};

use super::FrameInterpolatableScaler;

define_obj_type!(
    #[doc(alias = "MTLFXTemporalDenoisedScalerDescriptor")]
    pub TemporalDenoisedScalerDesc(ns::Id),
    MTLFX_TEMPORAL_DENOISED_SCALER_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 18.0, tvos = 18.0)]
);

impl ns::Copying for TemporalDenoisedScalerDesc {}

impl TemporalDenoisedScalerDesc {
    #[objc::msg_send(colorTextureFormat)]
    pub fn color_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setColorTextureFormat:)]
    pub fn set_color_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(depthTextureFormat)]
    pub fn depth_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setDepthTextureFormat:)]
    pub fn set_depth_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(motionTextureFormat)]
    pub fn motion_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setMotionTextureFormat:)]
    pub fn set_motion_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(diffuseAlbedoTextureFormat)]
    pub fn diffuse_albedo_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setDiffuseAlbedoTextureFormat:)]
    pub fn set_diffuse_albedo_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(specularAlbedoTextureFormat)]
    pub fn specular_albedo_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setSpecularAlbedoTextureFormat:)]
    pub fn set_specular_albedo_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(normalTextureFormat)]
    pub fn normal_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setNormalTextureFormat:)]
    pub fn set_normal_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(roughnessTextureFormat)]
    pub fn roughness_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setRoughnessTextureFormat:)]
    pub fn set_roughness_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(specularHitDistanceTextureFormat)]
    pub fn specular_hit_distance_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setSpecularHitDistanceTextureFormat:)]
    pub fn set_specular_hit_distance_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(denoiseStrengthMaskTextureFormat)]
    pub fn denoise_strength_mask_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setDenoiseStrengthMaskTextureFormat:)]
    pub fn set_denoise_strength_mask_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(transparencyOverlayTextureFormat)]
    pub fn transparency_overlay_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setTransparencyOverlayTextureFormat:)]
    pub fn set_transparency_overlay_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(outputTextureFormat)]
    pub fn output_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setOutputTextureFormat:)]
    pub fn set_output_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(inputWidth)]
    pub fn input_width(&self) -> usize;

    #[objc::msg_send(setInputWidth:)]
    pub fn set_input_width(&mut self, val: usize);

    #[objc::msg_send(inputHeight)]
    pub fn input_height(&self) -> usize;

    #[objc::msg_send(setInputHeight:)]
    pub fn set_input_height(&mut self, val: usize);

    #[objc::msg_send(outputWidth)]
    pub fn output_width(&self) -> usize;

    #[objc::msg_send(setOutputWidth:)]
    pub fn set_output_width(&mut self, val: usize);

    #[objc::msg_send(outputHeight)]
    pub fn output_height(&self) -> usize;

    #[objc::msg_send(setOutputHeight:)]
    pub fn set_output_height(&mut self, val: usize);

    #[objc::msg_send(requiresSynchronousInitialization)]
    pub fn requires_synchronous_initialization(&self) -> bool;

    #[objc::msg_send(setRequiresSynchronousInitialization:)]
    pub fn set_requires_synchronous_initialization(&mut self, val: bool);

    #[objc::msg_send(isAutoExposureEnabled)]
    pub fn is_auto_exposure_enabled(&self) -> bool;

    #[objc::msg_send(setAutoExposureEnabled:)]
    pub fn set_auto_exposure_enabled(&mut self, val: bool);

    #[objc::msg_send(isReactiveMaskTextureEnabled)]
    pub fn is_reactive_mask_texture_enabled(&self) -> bool;

    #[objc::msg_send(setReactiveMaskTextureEnabled:)]
    pub fn set_reactive_mask_texture_enabled(&mut self, val: bool);

    #[objc::msg_send(reactiveMaskTextureFormat)]
    pub fn reactive_mask_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setReactiveMaskTextureFormat:)]
    pub fn set_reactive_mask_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(isSpecularHitDistanceTextureEnabled)]
    pub fn is_specular_hit_distance_texture_enabled(&self) -> bool;

    #[objc::msg_send(setSpecularHitDistanceTextureEnabled:)]
    pub fn set_specular_hit_distance_texture_enabled(&mut self, val: bool);

    #[objc::msg_send(isDenoiseStrengthMaskTextureEnabled)]
    pub fn is_denoise_strength_mask_texture_enabled(&self) -> bool;

    #[objc::msg_send(setDenoiseStrengthMaskTextureEnabled:)]
    pub fn set_denoise_strength_mask_texture_enabled(&mut self, val: bool);

    #[objc::msg_send(isTransparencyOverlayTextureEnabled)]
    pub fn is_transparency_overlay_texture_enabled(&self) -> bool;

    #[objc::msg_send(setTransparencyOverlayTextureEnabled:)]
    pub fn set_transparency_overlay_texture_enabled(&mut self, val: bool);

    #[objc::msg_send(newTemporalDenoisedScalerWithDevice:)]
    pub fn new_temporal_denoised_scaler(
        &self,
        device: &mtl::Device,
    ) -> Option<arc::R<TemporalDenoisedScaler>>;

    #[objc::msg_send(newTemporalDenoisedScalerWithDevice:compiler:)]
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0)]
    pub fn new_temporal_denoised_scaler_with_compiler(
        &self,
        device: &mtl::Device,
        compiler: &mtl4::Compiler,
    ) -> Option<arc::R<mtl4::fx::TemporalDenoisedScaler>>;

    #[objc::msg_send(supportedInputContentMinScaleForDevice:)]
    pub fn supported_input_content_min_scale(device: &mtl::Device) -> f32;

    #[objc::msg_send(supportedInputContentMaxScaleForDevice:)]
    pub fn supported_input_content_max_scale(device: &mtl::Device) -> f32;

    #[objc::msg_send(supportsMetal4FX:)]
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0)]
    pub fn supports_metal4_fx(device: &mtl::Device) -> bool;

    #[objc::msg_send(supportsDevice:)]
    pub fn supports_device(device: &mtl::Device) -> bool;
}

define_obj_type!(
    #[doc(alias = "MTLFXTemporalDenoisedScalerBase")]
    pub TemporalDenoisedScalerBase(FrameInterpolatableScaler)
);

impl TemporalDenoisedScalerBase {
    #[objc::msg_send(colorTextureUsage)]
    pub fn color_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(depthTextureUsage)]
    pub fn depth_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(motionTextureUsage)]
    pub fn motion_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(reactiveTextureUsage)]
    pub fn reactive_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(diffuseAlbedoTextureUsage)]
    pub fn diffuse_albedo_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(specularAlbedoTextureUsage)]
    pub fn specular_albedo_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(normalTextureUsage)]
    pub fn normal_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(roughnessTextureUsage)]
    pub fn roughness_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(specularHitDistanceTextureUsage)]
    pub fn specular_hit_distance_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(denoiseStrengthMaskTextureUsage)]
    pub fn denoise_strength_mask_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(transparencyOverlayTextureUsage)]
    pub fn transparency_overlay_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(outputTextureUsage)]
    pub fn output_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(colorTexture)]
    pub fn color_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setColorTexture:)]
    pub fn set_color_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(depthTexture)]
    pub fn depth_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setDepthTexture:)]
    pub fn set_depth_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(motionTexture)]
    pub fn motion_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setMotionTexture:)]
    pub fn set_motion_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(diffuseAlbedoTexture)]
    pub fn diffuse_albedo_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setDiffuseAlbedoTexture:)]
    pub fn set_diffuse_albedo_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(specularAlbedoTexture)]
    pub fn specular_albedo_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setSpecularAlbedoTexture:)]
    pub fn set_specular_albedo_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(normalTexture)]
    pub fn normal_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setNormalTexture:)]
    pub fn set_normal_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(roughnessTexture)]
    pub fn roughness_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setRoughnessTexture:)]
    pub fn set_roughness_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(specularHitDistanceTexture)]
    pub fn specular_hit_distance_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setSpecularHitDistanceTexture:)]
    pub fn set_specular_hit_distance_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(denoiseStrengthMaskTexture)]
    pub fn denoise_strength_mask_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setDenoiseStrengthMaskTexture:)]
    pub fn set_denoise_strength_mask_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(transparencyOverlayTexture)]
    pub fn transparency_overlay_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setTransparencyOverlayTexture:)]
    pub fn set_transparency_overlay_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(outputTexture)]
    pub fn output_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setOutputTexture:)]
    pub fn set_output_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(exposureTexture)]
    pub fn exposure_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setExposureTexture:)]
    pub fn set_exposure_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(preExposure)]
    pub fn pre_exposure(&self) -> f32;

    #[objc::msg_send(setPreExposure:)]
    pub fn set_pre_exposure(&mut self, val: f32);

    #[objc::msg_send(reactiveMaskTexture)]
    pub fn reactive_mask_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setReactiveMaskTexture:)]
    pub fn set_reactive_mask_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(jitterOffsetX)]
    pub fn jitter_offset_x(&self) -> f32;

    #[objc::msg_send(setJitterOffsetX:)]
    pub fn set_jitter_offset_x(&mut self, val: f32);

    #[objc::msg_send(jitterOffsetY)]
    pub fn jitter_offset_y(&self) -> f32;

    #[objc::msg_send(setJitterOffsetY:)]
    pub fn set_jitter_offset_y(&mut self, val: f32);

    #[objc::msg_send(motionVectorScaleX)]
    pub fn motion_vector_scale_x(&self) -> f32;

    #[objc::msg_send(setMotionVectorScaleX:)]
    pub fn set_motion_vector_scale_x(&mut self, val: f32);

    #[objc::msg_send(motionVectorScaleY)]
    pub fn motion_vector_scale_y(&self) -> f32;

    #[objc::msg_send(setMotionVectorScaleY:)]
    pub fn set_motion_vector_scale_y(&mut self, val: f32);

    #[objc::msg_send(shouldResetHistory)]
    pub fn should_reset_history(&self) -> bool;

    #[objc::msg_send(setShouldResetHistory:)]
    pub fn set_should_reset_history(&mut self, val: bool);

    #[objc::msg_send(isDepthReversed)]
    pub fn is_depth_reversed(&self) -> bool;

    #[objc::msg_send(setDepthReversed:)]
    pub fn set_depth_reversed(&mut self, val: bool);

    #[objc::msg_send(colorTextureFormat)]
    pub fn color_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(depthTextureFormat)]
    pub fn depth_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(motionTextureFormat)]
    pub fn motion_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(diffuseAlbedoTextureFormat)]
    pub fn diffuse_albedo_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(specularAlbedoTextureFormat)]
    pub fn specular_albedo_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(normalTextureFormat)]
    pub fn normal_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(roughnessTextureFormat)]
    pub fn roughness_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(specularHitDistanceTextureFormat)]
    pub fn specular_hit_distance_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(denoiseStrengthMaskTextureFormat)]
    pub fn denoise_strength_mask_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(transparencyOverlayTextureFormat)]
    pub fn transparency_overlay_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(reactiveMaskTextureFormat)]
    pub fn reactive_mask_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(outputTextureFormat)]
    pub fn output_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(inputWidth)]
    pub fn input_width(&self) -> usize;

    #[objc::msg_send(inputHeight)]
    pub fn input_height(&self) -> usize;

    #[objc::msg_send(outputWidth)]
    pub fn output_width(&self) -> usize;

    #[objc::msg_send(outputHeight)]
    pub fn output_height(&self) -> usize;

    #[objc::msg_send(inputContentMinScale)]
    pub fn input_content_min_scale(&self) -> f32;

    #[objc::msg_send(inputContentMaxScale)]
    pub fn input_content_max_scale(&self) -> f32;

    #[cfg(target_arch = "aarch64")]
    #[inline]
    pub fn world_to_view_matrix(&self) -> simd::f32x4x4 {
        let q0: std::arch::aarch64::float32x4_t;
        let q1: std::arch::aarch64::float32x4_t;
        let q2: std::arch::aarch64::float32x4_t;
        let q3: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$worldToViewMatrix",
                in("x0") self,
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
    pub fn world_to_view_matrix(&self) -> simd::f32x4x4 {
        unimplemented!()
    }

    #[cfg(target_arch = "aarch64")]
    #[inline]
    pub fn set_world_to_view_matrix(&mut self, val: simd::f32x4x4) {
        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$setWorldToViewMatrix:",
                in("x0") self,
                in("q0") val.0.0,
                in("q1") val.0.1,
                in("q2") val.0.2,
                in("q3") val.0.3,
                clobber_abi("C"),
            );
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn set_world_to_view_matrix(&mut self, val: simd::f32x4x4) {
        let _ = (self, val);
        unimplemented!()
    }

    #[cfg(target_arch = "aarch64")]
    #[inline]
    pub fn view_to_clip_matrix(&self) -> simd::f32x4x4 {
        let q0: std::arch::aarch64::float32x4_t;
        let q1: std::arch::aarch64::float32x4_t;
        let q2: std::arch::aarch64::float32x4_t;
        let q3: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$viewToClipMatrix",
                in("x0") self,
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
    pub fn view_to_clip_matrix(&self) -> simd::f32x4x4 {
        unimplemented!()
    }

    #[cfg(target_arch = "aarch64")]
    #[inline]
    pub fn set_view_to_clip_matrix(&mut self, val: simd::f32x4x4) {
        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$setViewToClipMatrix:",
                in("x0") self,
                in("q0") val.0.0,
                in("q1") val.0.1,
                in("q2") val.0.2,
                in("q3") val.0.3,
                clobber_abi("C"),
            );
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn set_view_to_clip_matrix(&mut self, val: simd::f32x4x4) {
        let _ = (self, val);
        unimplemented!()
    }

    #[objc::msg_send(fence)]
    pub fn fence(&self) -> Option<arc::R<mtl::Fence>>;

    #[objc::msg_send(setFence:)]
    pub fn set_fence(&mut self, val: Option<&mtl::Fence>);
}

define_obj_type!(
    #[doc(alias = "MTLFXTemporalDenoisedScaler")]
    pub TemporalDenoisedScaler(TemporalDenoisedScalerBase)
);

impl TemporalDenoisedScaler {
    #[objc::msg_send(encodeToCommandBuffer:)]
    #[objc::available(macos = 26.0, ios = 18.0)]
    pub fn encode_to_cmd_buf(&self, command_buffer: &mtl::CmdBuf);
}

unsafe extern "C" {
    static MTLFX_TEMPORAL_DENOISED_SCALER_DESCRIPTOR:
        &'static objc::Class<TemporalDenoisedScalerDesc>;
}
