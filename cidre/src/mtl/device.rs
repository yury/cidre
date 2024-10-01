use crate::{api, arc, blocks, define_obj_type, define_opts, mtl, ns, objc};

#[cfg(feature = "io")]
use crate::io;

use super::{event::SharedEvent, Buf, CmdQueue, Event, Fence, Lib, Size};

define_opts!(
    #[doc(alias = "MTLPipelineOption")]
    pub PipelineOpt(usize)
);

impl PipelineOpt {
    #[doc(alias = "MTLPipelineOptionNone")]
    pub const NONE: Self = Self(0);

    #[doc(alias = "MTLPipelineOptionBindingInfo")]
    pub const BINDING_INFO: Self = Self(1 << 0);

    #[doc(alias = "MTLPipelineOptionBufferTypeInfo")]
    pub const BUFFER_TYPE_INFO: Self = Self(1 << 1);

    #[doc(alias = "MTLPipelineOptionFailOnBinaryArchiveMiss")]
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1 << 2);
}

#[doc(alias = "MTLReadWriteTextureTier")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum ReadWriteTextureTier {
    #[doc(alias = "MTLReadWriteTextureTierNone")]
    None = 0,

    #[doc(alias = "MTLReadWriteTextureTier1")]
    _1 = 1,

    #[doc(alias = "MTLReadWriteTextureTier2")]
    _2 = 2,
}

#[doc(alias = "MTLArgumentBuffersTier")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum ArgBufsTier {
    #[doc(alias = "MTLArgumentBuffersTier1")]
    _1 = 0,

    #[doc(alias = "MTLArgumentBuffersTier2")]
    _2 = 1,
}

#[doc(alias = "MTLSparsePageSize")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(isize)]
pub enum SparsePageSize {
    #[doc(alias = "MTLSparsePageSize16")]
    _16 = 101,

    #[doc(alias = "MTLSparsePageSize64")]
    _64 = 102,

    #[doc(alias = "MTLSparsePageSize256")]
    _256 = 103,
}

#[doc(alias = "MTLSizeAndAlign")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(C)]
pub struct SizeAlign {
    pub size: usize,
    pub align: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum GpuFamily {
    Apple1 = 1001,
    Apple2 = 1002,
    Apple3 = 1003,
    Apple4 = 1004,
    Apple5 = 1005,
    Apple6 = 1006,
    Apple7 = 1007,
    Apple8 = 1008,
    Apple9 = 1009,

    Mac2 = 2002,

    Common1 = 3001,
    Common2 = 3002,
    Common3 = 3003,

    Metal3 = 5001,
}

define_obj_type!(
    #[doc(alias = "MTLDevice")]
    pub Device(ns::Id)
);

impl Device {
    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::sys_default().unwrap();
    /// ```
    #[inline]
    pub fn sys_default() -> Option<arc::R<Device>> {
        unsafe { MTLCreateSystemDefaultDevice() }
    }

    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(registryID)]
    pub fn registry_id(&self) -> u64;

    #[objc::msg_send(maxThreadsPerThreadgroup)]
    pub fn max_threads_per_threadgroup(&self) -> Size;

    #[objc::msg_send(hasUnifiedMemory)]
    pub fn has_unified_memory(&self) -> bool;

    #[objc::msg_send(readWriteTextureSupport)]
    pub fn read_write_texture_support(&self) -> ReadWriteTextureTier;

    #[objc::msg_send(argumentBuffersSupport)]
    pub fn argument_bufs_support(&self) -> ArgBufsTier;

    #[objc::msg_send(newCommandQueue)]
    pub fn new_cmd_queue(&self) -> Option<arc::R<CmdQueue>>;

    #[objc::msg_send(newCommandQueueWithMaxCommandBufferCount:)]
    pub fn new_cmd_queue_max_cmd_buf_count(
        &self,
        max_cmd_buf_count: usize,
    ) -> Option<arc::R<CmdQueue>>;

    #[objc::msg_send(newTextureWithDescriptor:)]
    pub fn new_texture(&self, descriptor: &mtl::TextureDesc) -> Option<arc::R<mtl::Texture>>;

    #[cfg(feature = "io")]
    #[objc::msg_send(newTextureWithDescriptor:iosurface:plane:)]
    pub fn new_texture_with_surf(
        &self,
        descriptor: &mtl::TextureDesc,
        surf: &io::Surf,
        plane: usize,
    ) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(newDefaultLibrary)]
    pub fn new_default_lib(&self) -> Option<arc::R<Lib>>;

    #[objc::msg_send(newLibraryWithSource:options:error:)]
    pub unsafe fn new_lib_with_src_err<'ear>(
        &self,
        src: &ns::String,
        opts: Option<&mtl::CompileOpts>,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Lib>>;

    #[inline]
    pub fn new_lib_with_src_blocking<'ear>(
        &self,
        src: &ns::String,
        opts: Option<&mtl::CompileOpts>,
    ) -> Result<arc::R<Lib>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::new_lib_with_src_err(self, src, opts, err) })
    }

    #[cfg(feature = "async")]
    pub async fn new_lib_with_src_opts(
        &self,
        src: &ns::String,
        opts: Option<&mtl::CompileOpts>,
    ) -> Result<arc::R<mtl::Lib>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        self.new_lib_with_src_ch(src, opts, &mut block);
        future.await
    }

    #[objc::msg_send(newLibraryWithSource:options:completionHandler:)]
    pub fn new_lib_with_src_ch(
        &self,
        src: &ns::String,
        ops: Option<&mtl::CompileOpts>,
        ch: &mut blocks::ResultCompletionHandler<mtl::Lib>,
    );

    #[objc::msg_send(newLibraryWithStitchedDescriptor:error:)]
    pub unsafe fn new_lib_with_stitched_desc_err<'ear>(
        &self,
        desc: mtl::FnStitchedLibDesc,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl::Lib>>;

    pub fn new_lib_with_stitched_desc_blocking<'ear>(
        &self,
        desc: mtl::FnStitchedLibDesc,
    ) -> Result<arc::R<mtl::Lib>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { self.new_lib_with_stitched_desc_err(desc, err) })
    }

    #[objc::msg_send(newLibraryWithStitchedDescriptor:completionHandler:)]
    pub fn new_lib_with_stitched_desc_ch(
        &self,
        desc: mtl::FnStitchedLibDesc,
        ch: &mut blocks::ResultCompletionHandler<mtl::Lib>,
    );

    #[cfg(feature = "async")]
    pub async fn new_lib_with_stitched_desc(
        &self,
        desc: mtl::FnStitchedLibDesc,
    ) -> Result<arc::R<mtl::Lib>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        self.new_lib_with_stitched_desc_ch(desc, &mut block);
        future.await
    }

    #[objc::msg_send(newComputePipelineStateWithFunction:error:)]
    pub unsafe fn new_compute_ps_with_fn_err<'ear>(
        &self,
        function: &mtl::Fn,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl::ComputePipelineState>>;

    #[objc::msg_send(newRenderPipelineStateWithDescriptor:error:)]
    pub unsafe fn new_render_ps_err<'ear>(
        &self,
        desc: &mtl::RenderPipelineDesc,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl::RenderPipelineState>>;

    #[inline]
    pub fn new_render_ps<'ear>(
        &self,
        desc: &mtl::RenderPipelineDesc,
    ) -> Result<arc::R<mtl::RenderPipelineState>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::new_render_ps_err(self, desc, err) })
    }

    #[inline]
    pub fn new_compute_ps_with_fn<'ear>(
        &self,
        function: &mtl::Fn,
    ) -> Result<arc::R<mtl::ComputePipelineState>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { self.new_compute_ps_with_fn_err(function, err) })
    }

    #[objc::msg_send(newComputePipelineStateWithDescriptor:options:reflection:error:)]
    pub unsafe fn new_compute_ps_err<'ear, 'rar>(
        &self,
        desc: &mtl::ComputePipelineDesc,
        opts: mtl::PipelineOpt,
        reflection: *mut Option<&'rar mtl::ComputePipelineReflection>,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl::ComputePipelineState>>;

    #[inline]
    pub fn new_compute_ps<'ear>(
        &self,
        desc: &mtl::ComputePipelineDesc,
        opts: mtl::PipelineOpt,
    ) -> Result<arc::R<mtl::ComputePipelineState>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { self.new_compute_ps_err(desc, opts, std::ptr::null_mut(), err) })
    }

    #[objc::msg_send(newRenderPipelineStateWithTileDescriptor:options:reflection:error:)]
    pub unsafe fn new_tile_render_ps_err<'ear, 'rar>(
        &self,
        desc: &mtl::TileRenderPipelineDesc,
        opts: mtl::PipelineOpt,
        reflection: *mut Option<&'rar mtl::RenderPipelineReflection>,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl::RenderPipelineState>>;

    pub fn new_tile_render_ps<'ear>(
        &self,
        desc: &mtl::TileRenderPipelineDesc,
        opts: mtl::PipelineOpt,
    ) -> Result<arc::R<mtl::RenderPipelineState>, &'ear ns::Error> {
        ns::if_none(|err| unsafe {
            self.new_tile_render_ps_err(desc, opts, std::ptr::null_mut(), err)
        })
    }

    #[objc::msg_send(newBufferWithLength:options:)]
    pub fn new_buf(&self, len: usize, options: mtl::ResOpts) -> Option<arc::R<mtl::Buf>>;

    pub fn new_buf_of<T: Sized>(&self, len: usize, opts: mtl::ResOpts) -> Option<arc::R<mtl::Buf>> {
        self.new_buf(std::mem::size_of::<T>() * len, opts)
    }

    #[objc::msg_send(newBufferWithBytes:length:options:)]
    pub fn new_buf_with_bytes(
        &self,
        bytes: *const u8,
        len: usize,
        opts: mtl::ResOpts,
    ) -> Option<arc::R<Buf>>;

    #[inline]
    pub fn new_buf_with_slice<T: Sized>(
        &self,
        slice: &[T],
        opts: mtl::ResOpts,
    ) -> Option<arc::R<mtl::Buf>> {
        self.new_buf_with_bytes(slice.as_ptr() as _, std::mem::size_of_val(slice), opts)
    }

    #[inline]
    pub fn new_buf_from_vec<T: Sized>(
        &self,
        vec: Vec<T>,
        opts: mtl::ResOpts,
    ) -> Option<arc::R<mtl::Buf>> {
        self.new_buf_with_slice(&vec, opts)
    }

    #[objc::msg_send(newDepthStencilStateWithDescriptor:)]
    pub fn new_depth_stencil_state(
        &self,
        descr: &mtl::DepthStencilDesc,
    ) -> Option<arc::R<mtl::DepthStencilState>>;

    #[objc::msg_send(newFence)]
    pub fn new_fence(&self) -> Option<arc::R<Fence>>;

    /// Creates a new indirect command buffer with the given descriptor and count.
    #[objc::msg_send(newIndirectCommandBufferWithDescriptor:maxCommandCount:options:)]
    #[api::available(macos = 10.14, ios = 12.0)]
    pub fn new_indirect_cmd_buf(
        &self,
        desc: &mtl::IndirectCmdBufDesc,
        max_command_count: usize,
        options: mtl::ResOpts,
    ) -> Option<arc::R<mtl::IndirectCmdBuf>>;

    /// New single-device non-shareable Metal event object.
    #[objc::msg_send(newEvent)]
    pub fn new_event(&self) -> Option<arc::R<Event>>;

    /// New shareable multi-device event.
    #[objc::msg_send(newSharedEvent)]
    pub fn new_shared_event(&self) -> Option<arc::R<SharedEvent>>;

    #[objc::msg_send(maxBufferLength)]
    pub fn max_buf_len(&self) -> usize;

    /// Returns the size and alignment, in bytes, of a texture if you create it from a heap.
    #[objc::msg_send(heapTextureSizeAndAlignWithDescriptor:)]
    pub fn heap_texture_size_and_align(&self, descriptor: &mtl::TextureDesc) -> SizeAlign;

    /// Returns the size and alignment, in bytes, of a buffer if you create it from a heap.
    #[objc::msg_send(heapBufferSizeAndAlignWithLength:options:)]
    pub fn heap_buf_size_and_align(&self, len: usize, opts: mtl::ResOpts) -> SizeAlign;

    #[objc::msg_send(newHeapWithDescriptor:)]
    pub fn new_heap_desc(&self, descriptor: &mtl::HeapDesc) -> Option<arc::R<mtl::Heap>>;

    /// The maximum threadgroup memory available to a compute kernel, in bytes.
    #[objc::msg_send(maxThreadgroupMemoryLength)]
    pub fn max_threadgroup_memory_len(&self) -> usize;

    /// The maximum number of unique argument buffer samplers per app.
    ///
    /// This limit is only applicable to samplers that have their supportArgumentBuffers
    /// property set to true. A `mtl::SamplerState` object is considered unique if the
    /// configuration of its originating `mtl::SamplerDescriptor` properties is unique.
    /// For example, two samplers with equal min_filter values but different mag_filter values
    /// are considered unique.
    #[objc::msg_send(maxArgumentBufferSamplerCount)]
    pub fn max_argument_buf_sampler_count(&self) -> usize;

    /// Query device support for using ray tracing from compute pipelines.
    #[objc::msg_send(supportsRaytracing)]
    pub fn supports_raytracing(&self) -> bool;

    /// Query device support for using function pointers from compute pipelines.
    #[objc::msg_send(supportsFunctionPointers)]
    #[api::available(macos = 11.0, ios = 14.0)]
    pub fn supports_fn_pointers(&self) -> bool;

    /// Query device support for using function pointers from render pipeline stages.
    #[objc::msg_send(supportsFunctionPointersFromRender)]
    pub fn supports_fn_pointers_from_render(&self) -> bool;

    /// Query device support for using ray tracing from render pipeline stages.
    #[objc::msg_send(supportsRaytracingFromRender)]
    pub fn supports_raytracing_from_render(&self) -> bool;

    /// Query device support for using ray tracing primitive motion blur.
    #[objc::msg_send(supportsPrimitiveMotionBlur)]
    #[api::available(macos = 11.0, ios = 14.0)]
    pub fn supports_primitive_motion_blur(&self) -> bool;

    #[objc::msg_send(supportsFamily:)]
    #[api::available(macos = 10.15, ios = 13.0)]
    pub fn supports_family(&self, val: GpuFamily) -> bool;

    /// Returns the minimum alignment required for offset and rowBytes when creating a linear texture.
    /// An error is thrown for queries with invalid pixel formats (depth, stencil, or compressed formats).
    #[objc::msg_send(minimumLinearTextureAlignmentForPixelFormat:)]
    pub unsafe fn min_linear_texture_alignment_for_pixel_format_throws(
        &self,
        format: mtl::PixelFormat,
    ) -> usize;

    pub fn min_linear_texture_alignment_for_pixel_format<'ear>(
        &self,
        format: mtl::PixelFormat,
    ) -> Result<usize, &'ear ns::Exception> {
        ns::try_catch(|| unsafe {
            self.min_linear_texture_alignment_for_pixel_format_throws(format)
        })
    }

    #[objc::msg_send(minimumTextureBufferAlignmentForPixelFormat:)]
    pub fn min_texture_buffer_alignment_for_pixel_format(&self, format: mtl::PixelFormat) -> usize;

    #[objc::msg_send(newResidencySetWithDescriptor:error:)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub unsafe fn new_residency_set_err<'ear>(
        &self,
        desc: &mtl::ResidencySetDesc,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl::ResidencySet>>;

    /// Creates a new residency set with a descriptor.
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn new_residency_set<'ear>(
        &self,
        desc: &mtl::ResidencySetDesc,
    ) -> Result<arc::R<mtl::ResidencySet>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { self.new_residency_set_err(desc, err) })
    }

    /// Returns an array of all the Metal device instances in the system.
    #[doc(alias = "MTLCopyAllDevices")]
    #[api::available(
        macos = 10.11,
        ios = 18.0,
        maccatalyst = 13.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn all_devices() -> arc::R<ns::Array<mtl::Device>> {
        unsafe { MTLCopyAllDevices() }
    }
}

#[link(name = "Metal", kind = "framework")]
#[api::weak]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> Option<arc::R<Device>>;

    #[api::available(
        macos = 10.11,
        ios = 18.0,
        maccatalyst = 13.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    fn MTLCopyAllDevices() -> arc::R<ns::Array<mtl::Device>>;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let device = mtl::Device::sys_default().unwrap();

        assert!(device.supports_raytracing());
        assert!(device.supports_fn_pointers());
        assert!(device.supports_fn_pointers_from_render());
        assert!(device.supports_raytracing_from_render());
        assert!(device.supports_primitive_motion_blur());

        let mut fence = device.new_fence().unwrap();
        fence.set_label(Some(ns::str!(c"nice")));

        let mut event = device.new_shared_event().unwrap();
        event.set_label(Some(ns::str!(c"nice")));

        let _name = device.name();

        assert!(device.max_buf_len() > 10);

        let registry_id = device.registry_id();

        assert_ne!(0, registry_id);
        let size = device.max_threads_per_threadgroup();

        assert_ne!(0, size.width);
        assert_ne!(0, size.height);
        assert_ne!(0, size.depth);

        assert!(device.has_unified_memory());

        let tier = device.read_write_texture_support();
        assert_ne!(tier, mtl::ReadWriteTextureTier::None);

        let tier = device.argument_bufs_support();
        assert_ne!(tier, mtl::ArgBufsTier::_1);

        assert!(device.new_default_lib().is_none());

        let td = mtl::TextureDesc::new_2d(mtl::PixelFormat::A8UNorm, 100, 200, false);

        let _t = device.new_texture(&td).unwrap();

        assert!(device.max_buf_len() > 10);

        let _queue = device.new_cmd_queue().unwrap();

        let source = ns::str!(c"void function_a() {}");
        let options = None;
        let _lib = device.new_lib_with_src_blocking(source, options).unwrap();
    }
}
