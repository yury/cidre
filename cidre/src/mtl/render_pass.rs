use crate::{arc, define_obj_type, mtl, ns, objc};

#[doc(alias = "MTLLoadAction")]
#[derive(Debug, Default, PartialEq, Copy, Clone, Eq)]
#[repr(usize)]
pub enum LoadAction {
    #[default]
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

/// Types of actions performed for an attachment at the end of a rendering pass.
#[doc(alias = "MTLStoreAction")]
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum StoreAction {
    /// The GPU has permission to discard the rendered contents of the attachment
    /// at the end of the render pass, replacing them with arbitrary data.
    ///
    /// Use this option when you need the attachment’s contents during the render pass
    /// but not afterwards. Some GPUs may still store the contents back to the texture,
    /// but you can’t rely on that behavior. You must assume that GPU discarded
    /// the texture’s contents.
    #[default]
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

#[doc(alias = "MTLStoreActionOptions")]
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum StoreActionOpts {
    #[default]
    None = 0,
    CustomSamplePositions = 1 << 0,
}

#[doc(alias = "MTLClearColor")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct ClearColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl ClearColor {
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub const fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    pub const fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    pub const fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    pub const fn clear() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub const fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub const fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }
}

define_obj_type!(
    #[doc(alias = "MTLRenderPassStencilAttachmentDescriptor")]
    pub StencilAttachDesc(AttachDesc)
);

impl StencilAttachDesc {
    /// The clear stencil value to be used if the load_action property is mtl::LoadAction::Clear
    #[objc::msg_send(clearStencil)]
    pub fn clear_stencil(&self) -> u32;

    #[objc::msg_send(setClearStencil:)]
    pub fn set_clear_stencil(&mut self, val: u32);

    #[objc::msg_send(stencilResolveFilter)]
    pub fn stencil_resolve_filter(&self) -> MultisampleDepthResolveFilter;

    #[objc::msg_send(setStencilResolveFilter:)]
    pub fn set_stencil_resolve_filter(&mut self, val: MultisampleDepthResolveFilter);
}

define_obj_type!(
    #[doc(alias = "MTLRenderPassDescriptor")]
    pub Desc(ns::Id), MTL_RENDER_PASS_DESCRIPTOR
);

impl Desc {
    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches(&self) -> arc::R<ColorAttachDescArray>;

    #[objc::msg_send(depthAttachment)]
    pub fn depth_attach(&self) -> arc::R<DepthAttachDesc>;

    #[objc::msg_send(setDepthAttachment:)]
    pub fn set_depth_attach(&mut self, val: Option<&DepthAttachDesc>);

    #[objc::msg_send(stencilAttachment)]
    pub fn stencil_attach(&self) -> arc::R<StencilAttachDesc>;

    #[objc::msg_send(setStencilAttachment:)]
    pub fn set_stencil_attach_option(&mut self, val: Option<&StencilAttachDesc>);

    #[objc::msg_send(setStencilAttachment:)]
    pub fn set_stencil_attach(&mut self, val: &StencilAttachDesc);

    #[objc::msg_send(tileWidth)]
    pub fn tile_width(&self) -> usize;

    #[objc::msg_send(setTileWidth:)]
    pub fn set_tile_width(&mut self, val: usize);

    #[objc::msg_send(tileHeight)]
    pub fn tile_height(&self) -> usize;

    #[objc::msg_send(setTileHeight:)]
    pub fn set_tile_height(&mut self, val: usize);

    #[objc::msg_send(renderTargetWidth)]
    pub fn render_target_width(&self) -> usize;

    #[objc::msg_send(setRenderTargetWidth:)]
    pub fn set_render_target_width(&mut self, val: usize);

    #[objc::msg_send(renderTargetHeight)]
    pub fn render_target_height(&self) -> usize;

    #[objc::msg_send(setRenderTargetHeight:)]
    pub fn set_render_target_height(&mut self, val: usize);

    #[objc::msg_send(defaultRasterSampleCount)]
    pub fn default_raster_sample_count(&self) -> usize;

    #[objc::msg_send(setDefaultRasterSampleCount:)]
    pub fn set_default_raster_sample_count(&mut self, val: usize);

    #[objc::msg_send(imageblockSampleLength)]
    pub fn imageblock_sample_len(&self);

    #[objc::msg_send(setImageblockSampleLength:)]
    pub fn set_imageblock_sample_len(&self, val: usize);
}

define_obj_type!(
    #[doc(alias = "MTLRenderPassColorAttachmentDescriptorArray")]
    pub ColorAttachDescArray(ns::Id)
);

impl ColorAttachDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn ar_get_at(&self, index: usize) -> &ColorAttachDesc;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get_at(&self, index: usize) -> arc::R<ColorAttachDesc>;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn ar_get_mut_at(&mut self, index: usize) -> &mut ColorAttachDesc;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set_at(&mut self, object: Option<&ColorAttachDesc>, index: usize);
}

impl std::ops::Index<usize> for ColorAttachDescArray {
    type Output = ColorAttachDesc;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.ar_get_at(index)
    }
}

impl std::ops::IndexMut<usize> for ColorAttachDescArray {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.ar_get_mut_at(index)
    }
}

define_obj_type!(pub AttachDesc(ns::Id));
impl AttachDesc {
    #[objc::msg_send(texture)]
    pub fn texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setTexture:)]
    pub fn set_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(level)]
    pub fn level(&self) -> usize;

    #[objc::msg_send(setLevel:)]
    pub fn set_level(&mut self, val: usize);

    #[objc::msg_send(slice)]
    pub fn slice(&self) -> usize;

    #[objc::msg_send(setSlice:)]
    pub fn set_slice(&mut self, val: usize);

    #[objc::msg_send(depthPlane)]
    pub fn depth_plane(&self) -> usize;

    #[objc::msg_send(setDepthPlane:)]
    pub fn set_depth_plane(&mut self, val: usize);

    #[objc::msg_send(resolveTexture)]
    pub fn resolve_texture(&self) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setResolveTexture:)]
    pub fn set_resolve_texture(&mut self, val: Option<&mtl::Texture>);

    #[objc::msg_send(resolveLevel)]
    pub fn resolve_level(&self) -> usize;

    #[objc::msg_send(setResolveLevel:)]
    pub fn set_resolve_level(&mut self, val: usize);

    #[objc::msg_send(resolveSlice)]
    pub fn resolve_slice(&self) -> usize;

    #[objc::msg_send(setResolveSlice:)]
    pub fn set_resolve_slice(&mut self, val: usize);

    #[objc::msg_send(resolveDepthPlane)]
    pub fn resolve_depth_plane(&self) -> usize;

    #[objc::msg_send(setResolveDepthPlane:)]
    pub fn set_resolve_depth_plane(&mut self, val: usize);

    #[objc::msg_send(loadAction)]
    pub fn load_action(&self) -> LoadAction;

    #[objc::msg_send(setLoadAction:)]
    pub fn set_load_action(&mut self, val: LoadAction);

    #[objc::msg_send(storeAction)]
    pub fn store_action(&self) -> StoreAction;

    #[objc::msg_send(setStoreAction:)]
    pub fn set_store_action(&mut self, val: StoreAction);

    #[objc::msg_send(storeActionOptions)]
    pub fn store_action_options(&self) -> StoreActionOpts;

    #[objc::msg_send(setStoreActionOptions:)]
    pub fn set_store_action_options(&mut self, val: StoreActionOpts);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_RENDER_PASS_DESCRIPTOR: &'static objc::Class<Desc>;
}

define_obj_type!(
    #[doc(alias = "MTLRenderPassColorAttachmentDescriptor")]
    pub ColorAttachDesc(AttachDesc)
);

impl ColorAttachDesc {
    #[objc::msg_send(clearColor)]
    pub fn clear_color(&self) -> ClearColor;

    #[objc::msg_send(setClearColor:)]
    pub fn set_clear_color(&mut self, val: ClearColor);
}

#[doc(alias = "MTLMultisampleStencilResolveFilter")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum MultisampleDepthResolveFilter {
    Sample0 = 0,
    Min = 1,
    Max = 2,
}

define_obj_type!(
    #[doc(alias = "MTLRenderPassDepthAttachmentDescriptor")]
    pub DepthAttachDesc(AttachDesc)
);

impl DepthAttachDesc {
    #[objc::msg_send(clearDepth)]
    pub fn clear_depth(&self) -> f64;

    #[objc::msg_send(setClearDepth:)]
    pub fn set_clear_depth(&mut self, val: f64);

    #[objc::msg_send(depthResolveFilter)]
    pub fn depth_resolve_filter(&self) -> MultisampleDepthResolveFilter;

    #[objc::msg_send(setDepthResolveFilter:)]
    pub fn set_depth_resolve_filter(&mut self, val: MultisampleDepthResolveFilter);
}
