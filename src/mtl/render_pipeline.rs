use std::ops::{Index, IndexMut};

use crate::{arc, cf, define_mtl, define_obj_type, msg_send, ns};

use super::{argument::Argument, Function, PixelFormat};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum BlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum ColorWriteMask {
    None = 0,
    Red = 0x1 << 3,
    Green = 0x1 << 2,
    Blue = 0x1 << 1,
    Alpha = 0x1 << 0,
    All = 0xf,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum PrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationPartitionMode {
    Pow2 = 0,
    Integer = 1,
    FractionalOdd = 2,
    FractionalEven = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationFactorStepFunction {
    Constant = 0,
    PerPatch = 1,
    PerInstance = 2,
    PerPatchAndPerInstance = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationFactorFormat {
    MTLTessellationFactorFormatHalf = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationControlPointIndexType {
    None = 0,
    UInt16 = 1,
    UInt32 = 2,
}

define_obj_type!(ColorAttachmentDescriptor(ns::Id));

impl ColorAttachmentDescriptor {
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
    fn rsel_pixelFormat(id: &ns::Id) -> PixelFormat;
    fn wsel_setPixelFormat(id: &mut ns::Id, value: PixelFormat);

    fn rsel_isBlendingEnabled(id: &ns::Id) -> bool;
    fn wsel_setBlendingEnabled(id: &mut ns::Id, value: bool);

    fn rsel_sourceRGBBlendFactor(id: &ns::Id) -> BlendFactor;
    fn wsel_setSourceRGBBlendFactor(id: &mut ns::Id, value: BlendFactor);

    fn rsel_destinationRGBBlendFactor(id: &ns::Id) -> BlendFactor;
    fn wsel_setDestinationRGBBlendFactor(id: &mut ns::Id, value: BlendFactor);

    fn rsel_rgbBlendOperation(id: &ns::Id) -> BlendOperation;
    fn wsel_setRgbBlendOperation(id: &mut ns::Id, value: BlendOperation);

    fn rsel_sourceAlphaBlendFactor(id: &ns::Id) -> BlendFactor;
    fn wsel_setSourceAlphaBlendFactor(id: &mut ns::Id, value: BlendFactor);

    fn rsel_destinationAlphaBlendFactor(id: &ns::Id) -> BlendFactor;
    fn wsel_setDestinationAlphaBlendFactor(id: &mut ns::Id, value: BlendFactor);

    fn rsel_alphaBlendOperation(id: &ns::Id) -> BlendOperation;
    fn wsel_setAlphaBlendOperation(id: &mut ns::Id, value: BlendOperation);

    fn render_pipeline_rsel_writeMask(id: &ns::Id) -> ColorWriteMask;
    fn render_pipeline_wsel_setWriteMask(id: &mut ns::Id, value: ColorWriteMask);
}

define_obj_type!(Reflection(ns::Id));

impl Reflection {
    #[inline]
    pub fn vertex_arguments(&self) -> Option<&cf::ArrayOf<Argument>> {
        unsafe { rsel_vertexArguments(self) }
    }
    #[inline]
    pub fn fragment_arguments(&self) -> Option<&cf::ArrayOf<Argument>> {
        unsafe { rsel_fragmentArguments(self) }
    }
    #[inline]
    pub fn tile_arguments(&self) -> Option<&cf::ArrayOf<Argument>> {
        unsafe { rsel_tileArguments(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_vertexArguments(id: &ns::Id) -> Option<&cf::ArrayOf<Argument>>;
    fn rsel_fragmentArguments(id: &ns::Id) -> Option<&cf::ArrayOf<Argument>>;
    fn rsel_tileArguments(id: &ns::Id) -> Option<&cf::ArrayOf<Argument>>;
}

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    define_mtl!(reset);

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let mut desc = mtl::RenderPipelineDescriptor::new();
    ///
    /// assert!(desc.vertex_function().is_none());
    /// assert!(desc.fragment_function().is_none());
    ///
    /// desc.reset();
    /// ```
    pub fn new() -> arc::R<Descriptor> {
        unsafe { MTLRenderPipelineDescriptor_new() }
    }

    #[inline]
    pub fn vertex_function(&self) -> Option<&Function> {
        unsafe { rsel_vertexFunction(self) }
    }

    #[inline]
    pub fn set_vertex_function(&mut self, value: Option<&Function>) {
        unsafe { wsel_setVertexFunction(self, value) }
    }

    #[inline]
    pub fn set_vertex_fn(&mut self, value: &Function) {
        self.set_vertex_function(Some(value))
    }

    #[inline]
    pub fn fragment_function(&self) -> Option<&Function> {
        unsafe { rsel_fragmentFunction(self) }
    }

    #[inline]
    pub fn set_fragment_function(&mut self, value: Option<&Function>) {
        unsafe { wsel_setFragmentFunction(self, value) }
    }

    #[inline]
    pub fn set_fragment_fn(&mut self, value: &Function) {
        self.set_fragment_function(Some(value))
    }

    #[inline]
    pub fn color_attachments(&self) -> &ColorAttachmentDescriptorArray {
        unsafe { rsel_colorAttachments(self) }
    }

    #[inline]
    pub fn color_attachments_mut(&mut self) -> &mut ColorAttachmentDescriptorArray {
        unsafe { rsel_colorAttachments(self) }
    }

    #[inline]
    pub fn raster_sample_count(&self) -> usize {
        unsafe { rsel_rasterSampleCount(self) }
    }

    pub fn set_raster_sample_count(&mut self, value: usize) {
        unsafe { wsel_setRasterSampleCount(self, value) }
    }
}
#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLRenderPipelineDescriptor_new() -> arc::R<Descriptor>;
    fn rsel_vertexFunction(id: &ns::Id) -> Option<&Function>;
    fn wsel_setVertexFunction(id: &mut ns::Id, value: Option<&Function>);

    fn rsel_fragmentFunction(id: &ns::Id) -> Option<&Function>;
    fn wsel_setFragmentFunction(id: &mut ns::Id, value: Option<&Function>);
    fn rsel_colorAttachments(id: &ns::Id) -> &mut ColorAttachmentDescriptorArray;

    //rwsel(, id, rasterSampleCount, setRasterSampleCount, NSUInteger)
    fn rsel_rasterSampleCount(id: &ns::Id) -> usize;
    fn wsel_setRasterSampleCount(id: &ns::Id, value: usize);
}

define_obj_type!(FunctionsDescriptor(ns::Id));

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(device, label, gpu_resouce_id);

    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        unsafe { rsel_maxTotalThreadsPerThreadgroup(self) }
    }

    pub fn threadgourp_size_matches_tile_size(&self) -> bool {
        unsafe { rsel_threadgroupSizeMatchesTileSize(self) }
    }

    pub fn imageblock_sample_length(&self) -> usize {
        unsafe { rsel_imageblockSampleLength(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_maxTotalThreadsPerThreadgroup(id: &ns::Id) -> usize;
    fn rsel_threadgroupSizeMatchesTileSize(id: &ns::Id) -> bool;
    fn rsel_imageblockSampleLength(id: &ns::Id) -> usize;
}

define_obj_type!(ColorAttachmentDescriptorArray(ns::Id));

impl ColorAttachmentDescriptorArray {
    #[inline]
    pub fn get_at(&self, index: usize) -> &ColorAttachmentDescriptor {
        unsafe {
            MTLRenderPipelineColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(
                self, index,
            )
        }
    }

    #[inline]
    pub fn get_mut_at(&mut self, index: usize) -> &mut ColorAttachmentDescriptor {
        unsafe {
            MTLRenderPipelineColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(
                self, index,
            )
        }
    }

    #[inline]
    pub fn set_at(&mut self, index: usize, value: &ColorAttachmentDescriptor) {
        unsafe {
            MTLRenderPipelineColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
                self,
                Some(value),
                index,
            )
        }
    }

    #[inline]
    pub fn reset_at(&mut self, index: usize) {
        unsafe {
            MTLRenderPipelineColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
                self, None, index,
            )
        }
    }
}

impl Index<usize> for ColorAttachmentDescriptorArray {
    type Output = ColorAttachmentDescriptor;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get_at(index)
    }
}

impl IndexMut<usize> for ColorAttachmentDescriptorArray {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut_at(index)
    }
}

extern "C" {

    fn MTLRenderPipelineColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(
        id: &ns::Id,
        index: usize,
    ) -> &mut ColorAttachmentDescriptor;

    fn MTLRenderPipelineColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
        id: &mut ns::Id,
        value: Option<&ColorAttachmentDescriptor>,
        index: usize,
    );
}

define_obj_type!(TileRenderPipelineColorAttachmentDescriptor(ns::Id));
define_obj_type!(TileRenderPipelineColorAttachmentDescriptorArray(ns::Id));

define_obj_type!(TileRenderPipelineDescriptor(ns::Id));

impl TileRenderPipelineDescriptor {
    define_mtl!(reset);
}

define_obj_type!(MeshRenderPipelineDescriptor(ns::Id));

impl MeshRenderPipelineDescriptor {
    define_mtl!(reset, label, set_label);
}
