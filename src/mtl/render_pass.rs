use std::ops::Index;

use crate::cf::Retained;
use crate::define_obj_type;

use crate::ns::Id;

use super::Texture;

#[repr(usize)]
pub enum LoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[repr(usize)]
pub enum StoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
    StoreAndMultisampleResolve = 3,
    Unknown = 4,
    CustomSampleDepthStore = 5,
}

#[repr(usize)]
pub enum StoreActionOptions {
    None = 0,
    CustomSamplePositions = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
}

define_obj_type!(Descriptor(Id));

define_obj_type!(AttachmentDescriptor(Id));
define_obj_type!(ColorAttachmentDescriptor(AttachmentDescriptor));
define_obj_type!(DepthAttachmentDescriptor(AttachmentDescriptor));
define_obj_type!(StencilAttachmentDescriptor(AttachmentDescriptor));

impl Descriptor {
    #[inline]
    pub fn new<'new>() -> Retained<'new, Descriptor> {
        unsafe { MTLRenderPassDescriptor_new() }
    }

    #[inline]
    pub fn default<'autoreleased>() -> &'autoreleased Descriptor {
        unsafe { MTLRenderPassDescriptor_renderPassDescriptor() }
    }

    #[inline]
    pub fn color_attachments(&self) -> &ColorAttachmentDescriptorArray {
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
        unsafe {
            rsel_tileWidth(self)
        }
    }

    #[inline]
    pub fn set_title_width(&mut self, value: usize) {
        unsafe {
            wsel_setTileWidth(self, value)
        }
    }
    
    #[inline]
    pub fn title_height(&self) -> usize {
        unsafe {
            rsel_tileHeight(self)
        }
    }

    #[inline]
    pub fn set_title_height(&mut self, value: usize) {
        unsafe {
            wsel_setTileHeight(self, value)
        }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLRenderPassDescriptor_new<'new>() -> Retained<'new, Descriptor>;
    fn MTLRenderPassDescriptor_renderPassDescriptor<'autoreleased>() -> &'autoreleased Descriptor;

    fn rsel_colorAttachments(id: &Id) -> &ColorAttachmentDescriptorArray;

    fn rsel_depthAttachment(id: &Id) -> &DepthAttachmentDescriptor;
    fn wsel_setDepthAttachment(id: &mut Id, value: Option<&DepthAttachmentDescriptor>);

    fn rsel_stencilAttachment(id: &Id) -> &StencilAttachmentDescriptor;
    fn wsel_setStencilAttachment(id: &mut Id, value: Option<&StencilAttachmentDescriptor>);

    fn rsel_tileWidth(id: &Id) -> usize;
    fn wsel_setTileWidth(id: &mut Id, value: usize);
    fn rsel_tileHeight(id: &Id) -> usize;
    fn wsel_setTileHeight(id: &mut Id, value: usize);
}

define_obj_type!(ColorAttachmentDescriptorArray(Id));

impl ColorAttachmentDescriptorArray {
    pub fn get_at(&self, index: usize) -> &ColorAttachmentDescriptor {
        unsafe { MTLRenderPassColorAttachmentDescriptorArray_objectAtIndexedSubscript(self, index) }
    }

    pub fn set_at(&mut self, index: usize, value: &ColorAttachmentDescriptor) {
        unsafe {
            MTLRenderPassColorAttachmentDescriptorArray_setObjectAtIndexedSubscript(
                self,
                Some(value),
                index,
            )
        }
    }

    pub fn reset_at(&mut self, index: usize) {
        unsafe {
            MTLRenderPassColorAttachmentDescriptorArray_setObjectAtIndexedSubscript(
                self, None, index,
            )
        }
    }
}

impl Index<usize> for ColorAttachmentDescriptorArray {
    type Output = ColorAttachmentDescriptor;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_at(index)
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLRenderPassColorAttachmentDescriptorArray_objectAtIndexedSubscript(
        id: &Id,
        index: usize,
    ) -> &ColorAttachmentDescriptor;

    fn MTLRenderPassColorAttachmentDescriptorArray_setObjectAtIndexedSubscript(
        id: &mut Id,
        value: Option<&ColorAttachmentDescriptor>,
        index: usize,
    );
}

impl AttachmentDescriptor {
    #[inline]
    pub fn texture(&self) -> Option<&Texture> {
        unsafe { rsel_texture(self) }
    }

    #[inline]
    pub fn set_texture(&mut self, value: Option<&Texture>) {
        unsafe { wsel_setTexture(self, value) }
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
    pub fn resolve_texture(&self) -> Option<&Texture> {
        unsafe { rsel_resolveTexture(self) }
    }

    #[inline]
    pub fn set_resolve_texture(&mut self, value: Option<&Texture>) {
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
        unsafe { wsel_setSetStoreActionOptions(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_texture(id: &Id) -> Option<&Texture>;
    fn wsel_setTexture(id: &mut Id, value: Option<&Texture>);

    fn rsel_level(id: &Id) -> usize;
    fn wsel_setLevel(id: &mut Id, value: usize);

    fn rsel_slice(id: &Id) -> usize;
    fn wsel_setSlice(id: &mut Id, value: usize);

    fn rsel_depthPlane(id: &Id) -> usize;
    fn wsel_setDepthPlane(id: &mut Id, value: usize);

    fn rsel_resolveTexture(id: &Id) -> Option<&Texture>;
    fn wsel_setResolveTexture(id: &mut Id, value: Option<&Texture>);

    fn rsel_resolveLevel(id: &Id) -> usize;
    fn wsel_setResolveLevel(id: &mut Id, value: usize);

    fn rsel_resolveSlice(id: &Id) -> usize;
    fn wsel_setResolveSlice(id: &mut Id, value: usize);

    fn rsel_resolveDepthPlane(id: &Id) -> usize;
    fn wsel_setResolveDepthPlane(id: &mut Id, value: usize);

    fn rsel_loadAction(id: &Id) -> LoadAction;
    fn wsel_setLoadAction(id: &mut Id, value: LoadAction);

    fn rsel_storeAction(id: &Id) -> StoreAction;
    fn wsel_setStoreAction(id: &mut Id, value: StoreAction);

    fn rsel_storeActionOptions(id: &Id) -> StoreActionOptions;
    fn wsel_setSetStoreActionOptions(id: &mut Id, value: StoreActionOptions);
}

impl ColorAttachmentDescriptor {
    pub fn clear_color(&self) -> ClearColor {
        unsafe { rsel_clearColor(self) }
    }

    pub fn set_clear_color(&mut self, value: ClearColor) {
        unsafe { wsel_setClearColor(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_clearColor(id: &Id) -> ClearColor;
    fn wsel_setClearColor(id: &mut Id, value: ClearColor);
}

#[repr(usize)]
pub enum MultisampleDepthResolveFilter {
    Sample0 = 0,
    Min = 1,
    Max = 2,
}

impl DepthAttachmentDescriptor {
    pub fn clear_depth(&self) -> f64 {
        unsafe { rsel_clearDepth(self) }
    }

    pub fn set_clear_depth(&mut self, value: f64) {
        unsafe { wsel_setClearDepth(self, value) }
    }

    pub fn depth_resolve_filter(&self) -> MultisampleDepthResolveFilter {
        unsafe { rsel_depthResolveFilter(self) }
    }

    pub fn set_depth_resolve_filter(&mut self, value: MultisampleDepthResolveFilter) {
        unsafe { wsel_setDepthResolveFilter(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_clearDepth(id: &Id) -> f64;
    fn wsel_setClearDepth(id: &mut Id, value: f64);

    fn rsel_depthResolveFilter(id: &Id) -> MultisampleDepthResolveFilter;
    fn wsel_setDepthResolveFilter(id: &mut Id, value: MultisampleDepthResolveFilter);
}
