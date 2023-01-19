use std::ops::{Index, IndexMut};

use crate::{arc, define_obj_type, mtl, ns};

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
#[repr(usize)]
pub enum LoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

/// Types of actions performed for an attachment at the end of a rendering pass.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum StoreAction {
    /// The GPU has permission to discard the rendered contents of the attachment
    /// at the end of the render pass, replacing them with arbitrary data.
    ///
    /// Use this option when you need the attachment’s contents during the render pass
    /// but not afterwards. Some GPUs may still store the contents back to the texture,
    /// but you can’t rely on that behavior. You must assume that GPU discarded
    /// the texture’s contents.
    DontCare = 0,
    /// The GPU stores the rendered contents to the texture.
    Store = 1,

    /// The GPU resolves the multisampled data to one sample per pixel and stores the data
    /// to the resolve texture, discarding the multisample data afterwards.
    ///
    /// Use this option when you need to resolve the multisample attachment’s contents
    /// at the end of the render pass but don’t need the multisample data afterwards.
    /// Some GPUs may still store the multisample data back to the texture, but you
    /// can’t rely on that behavior. You must assume that GPU discarded the multisample
    /// texture’s contents.
    MultisampleResolve = 2,

    /// The GPU stores the multisample data to the multisample texture, resolves the data
    /// to a sample per pixel, and stores the data to the resolve texture.
    StoreAndMultisampleResolve = 3,

    /// The app will specify the store action when it encodes the render pass.
    ///
    /// Use this action only if you can’t determine the store action when you
    /// are creating the render pass descriptor. You must specify a store action
    /// before you finish encoding commands into the render command encoder.
    /// Refer to the `mtl::RenderCommandEncoder` and `mtl::ParallelRenderCommandEncoder`
    /// protocol references for further information
    Unknown = 4,

    /// The GPU stores depth data in a sample-position–agnostic representation.
    ///
    /// You can only set this action on a MTLRenderPassDepthAttachmentDescriptor object.
    ///
    /// Set this action when you need to read the depth data in a subsequent render pass
    /// or blit operation that is unaware of the programmable sample positions used to generate the data.
    ///
    /// If you specify this action, Metal may decompress the depth render target
    /// and store the resulting data in its decompressed form. If you don't change
    /// programmable sample positions in a subsequent render pass, use `StoreAction::Store`
    /// instead to improve performance.
    CustomSampleDepthStore = 5,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum StoreActionOptions {
    None = 0,
    CustomSamplePositions = 1 << 0,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl ClearColor {
    #[inline]
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub const fn red() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    pub const fn green() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    pub const fn blue() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }
}

define_obj_type!(Descriptor(ns::Id));

define_obj_type!(AttachmentDescriptor(ns::Id));
define_obj_type!(ColorAttachmentDescriptor(AttachmentDescriptor));
define_obj_type!(DepthAttachmentDescriptor(AttachmentDescriptor));
define_obj_type!(StencilAttachmentDescriptor(AttachmentDescriptor));

impl Descriptor {
    #[inline]
    pub fn new() -> arc::R<Descriptor> {
        unsafe { MTLRenderPassDescriptor_new() }
    }

    #[inline]
    pub fn default<'ar>() -> &'ar mut Descriptor {
        unsafe { MTLRenderPassDescriptor_renderPassDescriptor() }
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
    pub fn depth_attachment(&self) -> &DepthAttachmentDescriptor {
        unsafe { rsel_depthAttachment(self) }
    }

    #[inline]
    pub fn set_depth_attachemnt(&mut self, value: &DepthAttachmentDescriptor) {
        unsafe { wsel_setDepthAttachment(self, Some(value)) }
    }

    #[inline]
    pub fn reset_depth_attachemnt(&mut self) {
        unsafe { wsel_setDepthAttachment(self, None) }
    }

    #[inline]
    pub fn stencil_attachment(&self) -> &StencilAttachmentDescriptor {
        unsafe { rsel_stencilAttachment(self) }
    }

    #[inline]
    pub fn set_stencil_attachemnt(&mut self, value: &StencilAttachmentDescriptor) {
        unsafe { wsel_setStencilAttachment(self, Some(value)) }
    }

    #[inline]
    pub fn reset_stencil_attachemnt(&mut self) {
        unsafe { wsel_setStencilAttachment(self, None) }
    }

    #[inline]
    pub fn title_width(&self) -> usize {
        unsafe { rsel_tileWidth(self) }
    }

    #[inline]
    pub fn set_title_width(&mut self, value: usize) {
        unsafe { wsel_setTileWidth(self, value) }
    }

    #[inline]
    pub fn title_height(&self) -> usize {
        unsafe { rsel_tileHeight(self) }
    }

    #[inline]
    pub fn set_title_height(&mut self, value: usize) {
        unsafe { wsel_setTileHeight(self, value) }
    }

    #[inline]
    pub fn render_target_width(&self) -> usize {
        unsafe { rsel_renderTargetWidth(self) }
    }

    #[inline]
    pub fn set_render_target_width(&mut self, value: usize) {
        unsafe { wsel_setRenderTargetWidth(self, value) }
    }

    #[inline]
    pub fn render_target_height(&self) -> usize {
        unsafe { rsel_renderTargetHeight(self) }
    }

    #[inline]
    pub fn set_render_target_height(&mut self, value: usize) {
        unsafe { wsel_setRenderTargetHeight(self, value) }
    }

    #[inline]
    pub fn default_raster_sample_count(&self) -> usize {
        unsafe { rsel_defaultRasterSampleCount(self) }
    }

    #[inline]
    pub fn set_default_raster_sample_count(&mut self, value: usize) {
        unsafe { wsel_setSetDefaultRasterSampleCount(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLRenderPassDescriptor_new() -> arc::R<Descriptor>;
    fn MTLRenderPassDescriptor_renderPassDescriptor<'ar>() -> &'ar mut Descriptor;

    fn rsel_colorAttachments(id: &ns::Id) -> &mut ColorAttachmentDescriptorArray;

    fn rsel_depthAttachment(id: &ns::Id) -> &DepthAttachmentDescriptor;
    fn wsel_setDepthAttachment(id: &mut ns::Id, value: Option<&DepthAttachmentDescriptor>);

    fn rsel_stencilAttachment(id: &ns::Id) -> &StencilAttachmentDescriptor;
    fn wsel_setStencilAttachment(id: &mut ns::Id, value: Option<&StencilAttachmentDescriptor>);

    fn rsel_tileWidth(id: &ns::Id) -> usize;
    fn wsel_setTileWidth(id: &mut ns::Id, value: usize);
    fn rsel_tileHeight(id: &ns::Id) -> usize;
    fn wsel_setTileHeight(id: &mut ns::Id, value: usize);

    fn rsel_defaultRasterSampleCount(id: &ns::Id) -> usize;
    fn wsel_setSetDefaultRasterSampleCount(id: &mut ns::Id, value: usize);

    fn rsel_renderTargetWidth(id: &ns::Id) -> usize;
    fn wsel_setRenderTargetWidth(id: &mut ns::Id, value: usize);
    fn rsel_renderTargetHeight(id: &ns::Id) -> usize;
    fn wsel_setRenderTargetHeight(id: &mut ns::Id, value: usize);

}

define_obj_type!(ColorAttachmentDescriptorArray(ns::Id));

impl ColorAttachmentDescriptorArray {
    #[inline]
    pub fn get_at(&self, index: usize) -> &ColorAttachmentDescriptor {
        unsafe {
            MTLRenderPassColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(self, index)
        }
    }

    #[inline]
    pub fn get_mut_at(&mut self, index: usize) -> &mut ColorAttachmentDescriptor {
        unsafe {
            MTLRenderPassColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(self, index)
        }
    }

    #[inline]
    pub fn set_at(&mut self, index: usize, value: &ColorAttachmentDescriptor) {
        unsafe {
            MTLRenderPassColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
                self,
                Some(value),
                index,
            )
        }
    }

    #[inline]
    pub fn reset_at(&mut self, index: usize) {
        unsafe {
            MTLRenderPassColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
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

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLRenderPassColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(
        id: &ns::Id,
        index: usize,
    ) -> &mut ColorAttachmentDescriptor;

    fn MTLRenderPassColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
        id: &mut ns::Id,
        value: Option<&ColorAttachmentDescriptor>,
        index: usize,
    );
}

impl AttachmentDescriptor {
    #[inline]
    pub fn texture(&self) -> Option<&mtl::Texture> {
        unsafe { rsel_texture(self) }
    }

    #[inline]
    pub fn set_texture(&mut self, value: &mtl::Texture) {
        unsafe { wsel_setTexture(self, Some(value)) }
    }

    #[inline]
    pub fn remove_texture(&mut self) {
        unsafe { wsel_setTexture(self, None) }
    }

    #[inline]
    pub fn level(&self) -> usize {
        unsafe { rsel_level(self) }
    }

    #[inline]
    pub fn set_level(&mut self, value: usize) {
        unsafe { wsel_setLevel(self, value) }
    }

    #[inline]
    pub fn slice(&self) -> usize {
        unsafe { rsel_slice(self) }
    }

    #[inline]
    pub fn set_slice(&mut self, value: usize) {
        unsafe { wsel_setSlice(self, value) }
    }

    #[inline]
    pub fn depth_plane(&self) -> usize {
        unsafe { rsel_depthPlane(self) }
    }

    #[inline]
    pub fn set_depth_plane(&mut self, value: usize) {
        unsafe { wsel_setDepthPlane(self, value) }
    }

    #[inline]
    pub fn resolve_texture(&self) -> Option<&mtl::Texture> {
        unsafe { rsel_resolveTexture(self) }
    }

    #[inline]
    pub fn set_resolve_texture(&mut self, value: Option<&mtl::Texture>) {
        unsafe { wsel_setResolveTexture(self, value) }
    }

    #[inline]
    pub fn resolve_level(&self) -> usize {
        unsafe { rsel_resolveLevel(self) }
    }

    #[inline]
    pub fn set_resolve_level(&mut self, value: usize) {
        unsafe { wsel_setResolveLevel(self, value) }
    }

    #[inline]
    pub fn resolve_slice(&self) -> usize {
        unsafe { rsel_resolveSlice(self) }
    }

    #[inline]
    pub fn set_resolve_slice(&mut self, value: usize) {
        unsafe { wsel_setResolveSlice(self, value) }
    }

    #[inline]
    pub fn resolve_depth_plane(&self) -> usize {
        unsafe { rsel_resolveDepthPlane(self) }
    }

    #[inline]
    pub fn set_resolve_depth_plane(&mut self, value: usize) {
        unsafe { wsel_setResolveDepthPlane(self, value) }
    }

    #[inline]
    pub fn load_action(&self) -> LoadAction {
        unsafe { rsel_loadAction(self) }
    }

    #[inline]
    pub fn set_load_action(&mut self, value: LoadAction) {
        unsafe { wsel_setLoadAction(self, value) }
    }

    #[inline]
    pub fn store_action(&self) -> StoreAction {
        unsafe { rsel_storeAction(self) }
    }

    #[inline]
    pub fn set_store_action(&mut self, value: StoreAction) {
        unsafe { wsel_setStoreAction(self, value) }
    }

    #[inline]
    pub fn store_action_options(&self) -> StoreActionOptions {
        unsafe { rsel_storeActionOptions(self) }
    }

    #[inline]
    pub fn set_store_action_options(&mut self, value: StoreActionOptions) {
        unsafe { wsel_setStoreActionOptions(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_texture(id: &ns::Id) -> Option<&mtl::Texture>;
    fn wsel_setTexture(id: &mut ns::Id, value: Option<&mtl::Texture>);

    fn rsel_level(id: &ns::Id) -> usize;
    fn wsel_setLevel(id: &mut ns::Id, value: usize);

    fn rsel_slice(id: &ns::Id) -> usize;
    fn wsel_setSlice(id: &mut ns::Id, value: usize);

    fn rsel_depthPlane(id: &ns::Id) -> usize;
    fn wsel_setDepthPlane(id: &mut ns::Id, value: usize);

    fn rsel_resolveTexture(id: &ns::Id) -> Option<&mtl::Texture>;
    fn wsel_setResolveTexture(id: &mut ns::Id, value: Option<&mtl::Texture>);

    fn rsel_resolveLevel(id: &ns::Id) -> usize;
    fn wsel_setResolveLevel(id: &mut ns::Id, value: usize);

    fn rsel_resolveSlice(id: &ns::Id) -> usize;
    fn wsel_setResolveSlice(id: &mut ns::Id, value: usize);

    fn rsel_resolveDepthPlane(id: &ns::Id) -> usize;
    fn wsel_setResolveDepthPlane(id: &mut ns::Id, value: usize);

    fn rsel_loadAction(id: &ns::Id) -> LoadAction;
    fn wsel_setLoadAction(id: &mut ns::Id, value: LoadAction);

    fn rsel_storeAction(id: &ns::Id) -> StoreAction;
    fn wsel_setStoreAction(id: &mut ns::Id, value: StoreAction);

    fn rsel_storeActionOptions(id: &ns::Id) -> StoreActionOptions;
    fn wsel_setStoreActionOptions(id: &mut ns::Id, value: StoreActionOptions);
}

impl ColorAttachmentDescriptor {
    #[inline]
    pub fn clear_color(&self) -> ClearColor {
        unsafe { rsel_clearColor(self) }
    }

    #[inline]
    pub fn set_clear_color(&mut self, value: ClearColor) {
        unsafe { wsel_setClearColor(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_clearColor(id: &ns::Id) -> ClearColor;
    fn wsel_setClearColor(id: &mut ns::Id, value: ClearColor);
}

#[derive(Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum MultisampleDepthResolveFilter {
    Sample0 = 0,
    Min = 1,
    Max = 2,
}

impl DepthAttachmentDescriptor {
    #[inline]
    pub fn clear_depth(&self) -> f64 {
        unsafe { rsel_clearDepth(self) }
    }

    #[inline]
    pub fn set_clear_depth(&mut self, value: f64) {
        unsafe { wsel_setClearDepth(self, value) }
    }

    #[inline]
    pub fn depth_resolve_filter(&self) -> MultisampleDepthResolveFilter {
        unsafe { rsel_depthResolveFilter(self) }
    }

    #[inline]
    pub fn set_depth_resolve_filter(&mut self, value: MultisampleDepthResolveFilter) {
        unsafe { wsel_setDepthResolveFilter(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_clearDepth(id: &ns::Id) -> f64;
    fn wsel_setClearDepth(id: &mut ns::Id, value: f64);

    fn rsel_depthResolveFilter(id: &ns::Id) -> MultisampleDepthResolveFilter;
    fn wsel_setDepthResolveFilter(id: &mut ns::Id, value: MultisampleDepthResolveFilter);
}