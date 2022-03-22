use crate::{cf::ArrayOf, define_obj_type, objc::Id};

use super::{argument::Argument, Function, PixelFormat};

#[repr(usize)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    SourceAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
    Source1Color = 15,
    OneMinusSource1Color = 16,
    Source1Alpha = 17,
    OneMinusSource1Alpha = 18,
}

#[repr(usize)]
pub enum BlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

#[repr(usize)]
pub enum ColorWriteMask {
    None = 0,
    Red = 0x1 << 3,
    Green = 0x1 << 2,
    Blue = 0x1 << 1,
    Alpha = 0x1 << 0,
    All = 0xf,
}

#[repr(usize)]
pub enum PrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

#[repr(usize)]
pub enum TessellationPartitionMode {
    Pow2 = 0,
    Integer = 1,
    FractionalOdd = 2,
    FractionalEven = 3,
}

#[repr(usize)]
pub enum TessellationFactorStepFunction {
    Constant = 0,
    PerPatch = 1,
    PerInstance = 2,
    PerPatchAndPerInstance = 3,
}

#[repr(usize)]
pub enum TessellationFactorFormat {
    MTLTessellationFactorFormatHalf = 0,
}

#[repr(usize)]
pub enum TessellationControlPointIndexType {
    None = 0,
    UInt16 = 1,
    UInt32 = 2,
}

define_obj_type!(RenderPipelineColorAttachmentDescriptor(Id));

impl RenderPipelineColorAttachmentDescriptor {
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { rsel_pixelFormat(self) }
    }

    #[inline]
    pub fn set_pixel_format(&mut self, value: PixelFormat) {
        unsafe { wsel_setPixelFormat(self, value) }
    }

    #[inline]
    pub fn blening_enabled(&self) -> bool {
        unsafe { rsel_isBlendingEnabled(self) }
    }

    #[inline]
    pub fn set_blening_enabled(&mut self, value: bool) {
        unsafe { wsel_setBlendingEnabled(self, value) }
    }

    #[inline]
    pub fn src_rgb_blend_factor(&self) -> BlendFactor {
        unsafe { rsel_sourceRGBBlendFactor(self) }
    }

    #[inline]
    pub fn set_src_rgb_blend_factor(&mut self, value: BlendFactor) {
        unsafe { wsel_setSourceRGBBlendFactor(self, value) }
    }

    #[inline]
    pub fn dest_rgb_blend_factor(&self) -> BlendFactor {
        unsafe { rsel_destinationRGBBlendFactor(self) }
    }

    #[inline]
    pub fn set_dest_rgb_blend_factor(&mut self, value: BlendFactor) {
        unsafe { wsel_setDestinationRGBBlendFactor(self, value) }
    }

    #[inline]
    pub fn rgb_blend_operation(&self) -> BlendOperation {
        unsafe { rsel_rgbBlendOperation(self) }
    }

    #[inline]
    pub fn set_rgb_blend_operation(&mut self, value: BlendOperation) {
        unsafe { wsel_setRgbBlendOperation(self, value) }
    }

    #[inline]
    pub fn src_alpha_blend_factor(&self) -> BlendFactor {
        unsafe { rsel_sourceAlphaBlendFactor(self) }
    }

    #[inline]
    pub fn set_src_alpha_blend_factor(&mut self, value: BlendFactor) {
        unsafe { wsel_setSourceAlphaBlendFactor(self, value) }
    }

    #[inline]
    pub fn dest_alpha_blend_factor(&self) -> BlendFactor {
        unsafe { rsel_destinationAlphaBlendFactor(self) }
    }

    #[inline]
    pub fn set_dest_alpha_blend_factor(&mut self, value: BlendFactor) {
        unsafe { wsel_setDestinationAlphaBlendFactor(self, value) }
    }

    #[inline]
    pub fn alpha_blend_operation(&self) -> BlendOperation {
        unsafe { rsel_alphaBlendOperation(self) }
    }

    #[inline]
    pub fn alpha_rgb_blend_operation(&mut self, value: BlendOperation) {
        unsafe { wsel_setAlphaBlendOperation(self, value) }
    }

    #[inline]
    pub fn write_mask(&self) -> ColorWriteMask {
        unsafe { render_pipeline_rsel_writeMask(self) }
    }

    #[inline]
    pub fn set_write_mask(&mut self, value: ColorWriteMask) {
        unsafe { render_pipeline_wsel_setWriteMask(self, value) }
    }
}
#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_pixelFormat(id: &Id) -> PixelFormat;
    fn wsel_setPixelFormat(id: &mut Id, value: PixelFormat);

    fn rsel_isBlendingEnabled(id: &Id) -> bool;
    fn wsel_setBlendingEnabled(id: &mut Id, value: bool);

    fn rsel_sourceRGBBlendFactor(id: &Id) -> BlendFactor;
    fn wsel_setSourceRGBBlendFactor(id: &mut Id, value: BlendFactor);

    fn rsel_destinationRGBBlendFactor(id: &Id) -> BlendFactor;
    fn wsel_setDestinationRGBBlendFactor(id: &mut Id, value: BlendFactor);

    fn rsel_rgbBlendOperation(id: &Id) -> BlendOperation;
    fn wsel_setRgbBlendOperation(id: &mut Id, value: BlendOperation);

    fn rsel_sourceAlphaBlendFactor(id: &Id) -> BlendFactor;
    fn wsel_setSourceAlphaBlendFactor(id: &mut Id, value: BlendFactor);

    fn rsel_destinationAlphaBlendFactor(id: &Id) -> BlendFactor;
    fn wsel_setDestinationAlphaBlendFactor(id: &mut Id, value: BlendFactor);

    fn rsel_alphaBlendOperation(id: &Id) -> BlendOperation;
    fn wsel_setAlphaBlendOperation(id: &mut Id, value: BlendOperation);

    fn render_pipeline_rsel_writeMask(id: &Id) -> ColorWriteMask;
    fn render_pipeline_wsel_setWriteMask(id: &mut Id, value: ColorWriteMask);
}

define_obj_type!(RenderPipelineReflection(Id));

impl RenderPipelineReflection {
    #[inline]
    pub fn vertex_arguments(&self) -> Option<&ArrayOf<Argument>> {
        unsafe { rsel_vertexArguments(self) }
    }
    #[inline]
    pub fn fragment_arguments(&self) -> Option<&ArrayOf<Argument>> {
        unsafe { rsel_fragmentArguments(self) }
    }
    #[inline]
    pub fn tile_arguments(&self) -> Option<&ArrayOf<Argument>> {
        unsafe { rsel_tileArguments(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_vertexArguments(id: &Id) -> Option<&ArrayOf<Argument>>;
    fn rsel_fragmentArguments(id: &Id) -> Option<&ArrayOf<Argument>>;
    fn rsel_tileArguments(id: &Id) -> Option<&ArrayOf<Argument>>;
}

define_obj_type!(RenderPipelineDescriptor(Id));

impl RenderPipelineDescriptor {
    pub fn vertex_function(&self) -> Option<&Function> {
        unsafe { rsel_vertexFunction(self) }
    }

    pub fn set_vertex_function(&mut self, value: Option<&Function>) {
        unsafe { wsel_setVertexFunction(self, value) }
    }

    pub fn fragment_function(&self) -> Option<&Function> {
        unsafe { rsel_fragmentFunction(self) }
    }

    pub fn set_fragment_function(&mut self, value: Option<&Function>) {
        unsafe { wsel_setFragmentFunction(self, value) }
    }

    pub fn reset(&mut self) {
        unsafe { wsel_reset(self) }
    }
}
// @property (nullable, readwrite, nonatomic, strong) id <MTLFunction> vertexFunction;
#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_vertexFunction(id: &Id) -> Option<&Function>;
    fn wsel_setVertexFunction(id: &mut Id, value: Option<&Function>);

    fn rsel_fragmentFunction(id: &Id) -> Option<&Function>;
    fn wsel_setFragmentFunction(id: &mut Id, value: Option<&Function>);

    fn wsel_reset(id: &mut Id);

}

define_obj_type!(FunctionsDescriptor(Id));

define_obj_type!(State(Id));

define_obj_type!(ColorAttachmentDescriptorArray(Id));

define_obj_type!(TileRenderPipelineColorAttachmentDescriptor(Id));
define_obj_type!(TileRenderPipelineColorAttachmentDescriptorArray(Id));

define_obj_type!(TileRenderPipelineDescriptor(Id));

impl TileRenderPipelineDescriptor {
    pub fn reset(&mut self) {
        unsafe { wsel_reset(self) }
    }
}
