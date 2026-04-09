use crate::{api, arc, define_obj_type, mtl, mtl4, ns, objc};

#[doc(alias = "MTLFXSpatialScalerColorProcessingMode")]
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum SpatialScalerColorProcessingMode {
    #[default]
    #[doc(alias = "MTLFXSpatialScalerColorProcessingModePerceptual")]
    Perceptual = 0,
    #[doc(alias = "MTLFXSpatialScalerColorProcessingModeLinear")]
    Linear = 1,
    #[doc(alias = "MTLFXSpatialScalerColorProcessingModeHDR")]
    Hdr = 2,
}

define_obj_type!(
    #[doc(alias = "MTLFXSpatialScalerDescriptor")]
    pub SpatialScaleDesc(ns::Id),
    MTLFX_SPATIAL_SCALE_DESCRIPTOR,
    #[api::available(macos = 13.0, ios = 16.0)]
);

impl ns::Copying for SpatialScaleDesc {}

impl SpatialScaleDesc {
    #[objc::msg_send(colorTextureFormat)]
    pub fn color_texture_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setColorTextureFormat:)]
    pub fn set_color_texture_format(&mut self, val: mtl::PixelFormat);

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

    #[objc::msg_send(colorProcessingMode)]
    pub fn color_processing_mode(&self) -> SpatialScalerColorProcessingMode;

    #[objc::msg_send(setColorProcessingMode:)]
    pub fn set_color_processing_mode(&mut self, val: SpatialScalerColorProcessingMode);

    #[objc::msg_send(newSpatialScalerWithDevice:)]
    pub fn new_spatial_scaler(&self, device: &mtl::Device) -> Option<arc::R<SpatialScaler>>;

    #[objc::msg_send(newSpatialScalerWithDevice:compiler:)]
    #[api::available(macos = 26.0, ios = 26.0)]
    pub fn new_spatial_scaler_with_compiler(
        &self,
        device: &mtl::Device,
        compiler: &mtl4::Compiler,
    ) -> Option<arc::R<mtl4::fx::SpatialScaler>>;

    #[objc::msg_send(supportsMetal4FX:)]
    #[api::available(macos = 26.0, ios = 26.0)]
    pub fn supports_metal4_fx(device: &mtl::Device) -> bool;

    #[objc::msg_send(supportsDevice:)]
    pub fn supports_device(device: &mtl::Device) -> bool;
}

define_obj_type!(
    #[doc(alias = "MTLFXSpatialScalerBase")]
    pub SpatialScalerBase(ns::Id)
);

impl SpatialScalerBase {
    #[objc::msg_send(colorTextureUsage)]
    pub fn color_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(outputTextureUsage)]
    pub fn output_texture_usage(&self) -> mtl::TextureUsage;

    #[objc::msg_send(inputContentWidth)]
    pub fn input_content_width(&self) -> usize;

    #[objc::msg_send(setInputContentWidth:)]
    pub fn set_input_content_width(&mut self, val: usize);

    #[objc::msg_send(inputContentHeight)]
    pub fn input_content_height(&self) -> usize;

    #[objc::msg_send(setInputContentHeight:)]
    pub fn set_input_content_height(&mut self, val: usize);

    #[objc::msg_send(colorTexture)]
    pub fn color_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setColorTexture:)]
    pub fn set_color_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(outputTexture)]
    pub fn output_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setOutputTexture:)]
    pub fn set_output_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(colorTextureFormat)]
    pub fn color_texture_format(&self) -> mtl::PixelFormat;

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

    #[objc::msg_send(colorProcessingMode)]
    pub fn color_processing_mode(&self) -> SpatialScalerColorProcessingMode;

    #[objc::msg_send(fence)]
    pub fn fence(&self) -> Option<arc::R<mtl::Fence>>;

    #[objc::msg_send(setFence:)]
    pub fn set_fence(&mut self, val: Option<&mtl::Fence>);
}

define_obj_type!(
    #[doc(alias = "MTLFXSpatialScaler")]
    pub SpatialScaler(SpatialScalerBase)
);

impl SpatialScaler {
    #[objc::msg_send(encodeToCommandBuffer:)]
    pub fn encode_to_cmd_buf(&self, command_buffer: &mtl::CmdBuf);
}

unsafe extern "C" {
    static MTLFX_SPATIAL_SCALE_DESCRIPTOR: &'static objc::Class<SpatialScaleDesc>;
}
