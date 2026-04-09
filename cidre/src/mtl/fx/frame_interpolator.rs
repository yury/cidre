use super::FrameInterpolatableScaler;
use crate::{api, arc, define_obj_type, mtl, mtl4, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLFXFrameInterpolatorDescriptor")]
    pub FrameInterpolatorDesc(ns::Id),
    MTLFX_FRAME_INTERPOLATOR_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0)]
);

impl ns::Copying for FrameInterpolatorDesc {}

impl FrameInterpolatorDesc {
    #[objc::msg_send(colorTextureFormat)]
    pub fn color_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setColorTextureFormat:)]
    pub fn set_color_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(outputTextureFormat)]
    pub fn output_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setOutputTextureFormat:)]
    pub fn set_output_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(depthTextureFormat)]
    pub fn depth_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setDepthTextureFormat:)]
    pub fn set_depth_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(motionTextureFormat)]
    pub fn motion_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setMotionTextureFormat:)]
    pub fn set_motion_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(uiTextureFormat)]
    pub fn ui_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setUITextureFormat:)]
    pub fn set_ui_texture_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(scaler)]
    pub fn scaler(&self) -> Option<arc::R<FrameInterpolatableScaler>>;

    #[objc::msg_send(setScaler:)]
    pub fn set_scaler(&mut self, val: Option<&FrameInterpolatableScaler>);

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

    #[objc::msg_send(newFrameInterpolatorWithDevice:)]
    pub fn new_frame_interpolator(&self, device: &mtl::Device)
    -> Option<arc::R<FrameInterpolator>>;

    #[objc::msg_send(newFrameInterpolatorWithDevice:compiler:)]
    #[api::available(macos = 26.0, ios = 26.0)]
    pub fn new_frame_interpolator_with_compiler(
        &self,
        device: &mtl::Device,
        compiler: &mtl4::Compiler,
    ) -> Option<arc::R<mtl4::fx::FrameInterpolator>>;

    #[objc::msg_send(supportsMetal4FX:)]
    #[api::available(macos = 26.0, ios = 26.0)]
    pub fn supports_metal4_fx(device: &mtl::Device) -> bool;

    #[objc::msg_send(supportsDevice:)]
    #[api::available(macos = 26.0, ios = 26.0)]
    pub fn supports_device(device: &mtl::Device) -> bool;
}

define_obj_type!(
    #[doc(alias = "MTLFXFrameInterpolatorBase")]
    pub FrameInterpolatorBase(ns::Id)
);

impl FrameInterpolatorBase {
    #[objc::msg_send(colorTextureUsage)]
    pub fn color_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(outputTextureUsage)]
    pub fn output_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(depthTextureUsage)]
    pub fn depth_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(motionTextureUsage)]
    pub fn motion_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(uiTextureUsage)]
    pub fn ui_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(colorTextureFormat)]
    pub fn color_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(depthTextureFormat)]
    pub fn depth_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(motionTextureFormat)]
    pub fn motion_texture_format(&self) -> mtl::PixelFormat;

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

    #[objc::msg_send(uiTextureFormat)]
    pub fn ui_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(colorTexture)]
    pub fn color_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setColorTexture:)]
    pub fn set_color_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(prevColorTexture)]
    pub fn prev_color_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setPrevColorTexture:)]
    pub fn set_prev_color_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(depthTexture)]
    pub fn depth_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setDepthTexture:)]
    pub fn set_depth_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(motionTexture)]
    pub fn motion_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setMotionTexture:)]
    pub fn set_motion_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(motionVectorScaleX)]
    pub fn motion_vector_scale_x(&self) -> f32;

    #[objc::msg_send(setMotionVectorScaleX:)]
    pub fn set_motion_vector_scale_x(&mut self, val: f32);

    #[objc::msg_send(motionVectorScaleY)]
    pub fn motion_vector_scale_y(&self) -> f32;

    #[objc::msg_send(setMotionVectorScaleY:)]
    pub fn set_motion_vector_scale_y(&mut self, val: f32);

    #[objc::msg_send(deltaTime)]
    pub fn delta_time(&self) -> f32;

    #[objc::msg_send(setDeltaTime:)]
    pub fn set_delta_time(&mut self, val: f32);

    #[objc::msg_send(nearPlane)]
    pub fn near_plane(&self) -> f32;

    #[objc::msg_send(setNearPlane:)]
    pub fn set_near_plane(&mut self, val: f32);

    #[objc::msg_send(farPlane)]
    pub fn far_plane(&self) -> f32;

    #[objc::msg_send(setFarPlane:)]
    pub fn set_far_plane(&mut self, val: f32);

    #[objc::msg_send(fieldOfView)]
    pub fn field_of_view(&self) -> f32;

    #[objc::msg_send(setFieldOfView:)]
    pub fn set_field_of_view(&mut self, val: f32);

    #[objc::msg_send(aspectRatio)]
    pub fn aspect_ratio(&self) -> f32;

    #[objc::msg_send(setAspectRatio:)]
    pub fn set_aspect_ratio(&mut self, val: f32);

    #[objc::msg_send(uiTexture)]
    pub fn ui_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setUITexture:)]
    pub fn set_ui_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(jitterOffsetX)]
    pub fn jitter_offset_x(&self) -> f32;

    #[objc::msg_send(setJitterOffsetX:)]
    pub fn set_jitter_offset_x(&mut self, val: f32);

    #[objc::msg_send(jitterOffsetY)]
    pub fn jitter_offset_y(&self) -> f32;

    #[objc::msg_send(setJitterOffsetY:)]
    pub fn set_jitter_offset_y(&mut self, val: f32);

    #[objc::msg_send(isUITextureComposited)]
    pub fn is_ui_texture_composited(&self) -> bool;

    #[objc::msg_send(setIsUITextureComposited:)]
    pub fn set_is_ui_texture_composited(&mut self, val: bool);

    #[objc::msg_send(shouldResetHistory)]
    pub fn should_reset_history(&self) -> bool;

    #[objc::msg_send(setShouldResetHistory:)]
    pub fn set_should_reset_history(&mut self, val: bool);

    #[objc::msg_send(outputTexture)]
    pub fn output_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setOutputTexture:)]
    pub fn set_output_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(fence)]
    pub fn fence(&self) -> Option<arc::R<mtl::Fence>>;

    #[objc::msg_send(setFence:)]
    pub fn set_fence(&mut self, val: Option<&mtl::Fence>);

    #[objc::msg_send(isDepthReversed)]
    pub fn is_depth_reversed(&self) -> bool;

    #[objc::msg_send(setDepthReversed:)]
    pub fn set_depth_reversed(&mut self, val: bool);
}

define_obj_type!(
    #[doc(alias = "MTLFXFrameInterpolator")]
    pub FrameInterpolator(FrameInterpolatorBase)
);

impl FrameInterpolator {
    #[objc::msg_send(encodeToCommandBuffer:)]
    #[objc::available(macos = 26.0, ios = 26.0)]
    pub fn encode_to_cmd_buf(&self, command_buffer: &mtl::CmdBuf);
}

unsafe extern "C" {
    static MTLFX_FRAME_INTERPOLATOR_DESCRIPTOR: &'static objc::Class<FrameInterpolatorDesc>;
}
