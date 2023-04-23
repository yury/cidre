use std::{ffi::c_void, intrinsics::transmute};

use crate::{arc, blocks, define_obj_type, define_options, io, mtl, ns, objc};

use super::{event::SharedEvent, Buf, CmdQueue, Event, Fence, Lib, Size};

define_options!(PipelineOption(usize));

impl PipelineOption {
    pub const NONE: Self = Self(0);
    pub const ARGUMENT_INFO: Self = Self(1 << 0);
    pub const BUFFER_TYPE_INFO: Self = Self(1 << 1);
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1 << 2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum ReadWriteTextureTier {
    None = 0,
    _1 = 1,
    _2 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum ArgumentBuffersTier {
    _1 = 0,
    _2 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(isize)]
pub enum SparsePageSize {
    _16 = 101,
    _64 = 102,
    _256 = 103,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(C)]
pub struct SizeAndAlign {
    pub size: usize,
    pub align: usize,
}

define_obj_type!(Device(ns::Id));

impl Device {
    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    /// ```
    #[inline]
    pub fn default() -> Option<arc::R<Device>> {
        unsafe { MTLCreateSystemDefaultDevice() }
    }

    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let name = device.name();
    /// ```
    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(registryID)]
    pub fn registry_id(&self) -> u64;

    #[objc::msg_send(maxThreadsPerThreadgroup)]
    pub fn max_threads_per_threadgroup(&self) -> Size;

    #[objc::msg_send(hasUnifiedMemory)]
    pub fn has_unified_memory(&self) -> bool;

    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let tier = device.read_write_texture_support();
    ///
    /// assert_ne!(tier, mtl::ReadWriteTextureTier::None);
    #[objc::msg_send(readWriteTextureSupport)]
    pub fn read_write_texture_support(&self) -> ReadWriteTextureTier;

    /// Example
    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let tier = device.argument_buffers_support();
    ///
    /// assert_ne!(tier, mtl::ArgumentBuffersTier::_1);
    #[objc::msg_send(argumentBuffersSupport)]
    pub fn argument_buffers_support(&self) -> ArgumentBuffersTier;

    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let queue = device.command_queue().unwrap();
    ///
    /// queue.as_type_ref().show();
    ///
    #[objc::msg_send(newCommandQueue)]
    pub fn new_cmd_queue_ar(&self) -> Option<&'ar CmdQueue>;

    #[objc::rar_retain()]
    pub fn new_cmd_queue(&self) -> Option<arc::R<CmdQueue>>;

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let queue = device.command_queue_with_max_command_buffer_count(1).unwrap();
    ///
    /// queue.as_type_ref().show();
    ///
    #[objc::msg_send(newCommandQueueWithMaxCommandBufferCount:)]
    pub fn new_cmd_queue_max_cmd_buf_count_ar(
        &self,
        max_cmd_buf_count: usize,
    ) -> Option<&'ar CmdQueue>;

    #[objc::rar_retain()]
    pub fn new_cmd_queue_max_cmd_buf_count(
        &self,
        max_cmd_buf_count: usize,
    ) -> Option<arc::R<CmdQueue>>;

    #[objc::msg_send(newTextureWithDescriptor:)]
    pub fn new_texture_ar(&self, descriptor: &mtl::TextureDescriptor) -> Option<&'ar mtl::Texture>;

    #[objc::rar_retain()]
    pub fn new_texture(&self, descriptor: &mtl::TextureDescriptor) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(newTextureWithDescriptor:iosurface:plane:)]
    pub fn new_texture_with_surface_ar(
        &self,
        descriptor: &mtl::TextureDescriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<&'ar mtl::Texture>;

    #[objc::rar_retain()]
    pub fn new_texture_with_surface(
        &self,
        descriptor: &mtl::TextureDescriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(newDefaultLibrary)]
    pub fn new_default_lib_ar(&self) -> Option<&'ar Lib>;

    #[objc::rar_retain()]
    pub fn new_default_lib(&self) -> Option<arc::R<Lib>>;

    #[objc::msg_send(newLibraryWithSource:options:error:)]
    pub fn new_lib_with_src_err_ar(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
        error: &mut Option<&ns::Error>,
    ) -> Option<&'ar Lib>;

    #[objc::rar_retain()]
    pub fn new_lib_with_src_err(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<Lib>>;

    #[inline]
    pub fn new_lib_with_src<'a>(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
    ) -> Result<arc::R<Lib>, &'a ns::Error> {
        let mut error = None;
        let res = Self::new_lib_with_src_err(self, source, options, &mut error);

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }

    pub async fn new_lib_with_src_options(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
    ) -> Result<arc::R<mtl::Lib>, arc::R<ns::Error>> {
        let (future, block) = blocks::result();
        self.new_lib_with_src_options_completion(source, options, block.escape());
        future.await
    }

    #[objc::msg_send(newLibraryWithSource:options:completionHandler:)]
    pub fn new_lib_with_src_options_completion<'ar, F>(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
        completion: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(Option<&'ar mtl::library::Lib>, Option<&'ar ns::Error>) + 'static;

    #[objc::msg_send(newComputePipelineStateWithFunction:error:)]
    pub fn new_compute_ps_with_fn_err_ar<'a>(
        &self,
        function: &mtl::Fn,
        error: &mut Option<&'a ns::Error>,
    ) -> Option<&'ar mtl::ComputePipelineState>;

    #[objc::rar_retain()]
    pub fn new_compute_ps_with_fn_err<'a>(
        &self,
        function: &mtl::Fn,
        error: &mut Option<&'a ns::Error>,
    ) -> Option<arc::R<mtl::ComputePipelineState>>;

    #[objc::msg_send(newRenderPipelineStateWithDescriptor:error:)]
    pub unsafe fn new_render_ps_err_ar(
        &self,
        descriptor: &mtl::RenderPipelineDescriptor,
        error: &mut Option<&ns::Error>,
    ) -> Option<&'ar mtl::RenderPipelineState>;

    #[objc::rar_retain]
    pub unsafe fn new_render_ps_err(
        &self,
        descriptor: &mtl::RenderPipelineDescriptor,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<mtl::RenderPipelineState>>;

    #[inline]
    pub fn new_render_ps<'a>(
        &self,
        descriptor: &mtl::RenderPipelineDescriptor,
    ) -> Result<arc::R<mtl::RenderPipelineState>, &'a ns::Error> {
        let mut error = None;
        unsafe {
            let res = Self::new_render_ps_err(self, descriptor, &mut error);
            if res.is_some() {
                Ok(transmute(res))
            } else {
                Err(transmute(error))
            }
        }
    }

    #[inline]
    pub fn new_compute_ps_with_fn<'ar>(
        &self,
        function: &mtl::Fn,
    ) -> Result<arc::R<mtl::ComputePipelineState>, &'ar ns::Error> {
        let mut error = None;
        let res = self.new_compute_ps_with_fn_err(function, &mut error);

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }

    #[objc::msg_send(newBufferWithLength:options:)]
    pub fn new_buf_len_opts_ar(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<&'ar mtl::Buf>;

    #[objc::rar_retain()]
    pub fn new_buf_len_opts(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<mtl::Buf>>;

    #[objc::msg_send(newBufferWithBytes:length:options:)]
    pub fn new_buf_bytes_len_opts_ar(
        &self,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<&'ar Buf>;

    #[objc::rar_retain()]
    pub fn new_buf_bytes_len_opts(
        &self,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<Buf>>;

    #[inline]
    pub fn new_buf_slice_ar<'ar, T: Sized>(
        &self,
        slice: &[T],
        options: mtl::ResourceOptions,
    ) -> Option<&'ar mtl::Buf> {
        self.new_buf_bytes_len_opts_ar(
            slice.as_ptr() as _,
            std::mem::size_of::<T>() * slice.len(),
            options,
        )
    }

    #[inline]
    pub fn new_buf_slice<T: Sized>(
        &self,
        slice: &[T],
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<mtl::Buf>> {
        arc::rar_retain_option(self.new_buf_slice_ar(slice, options))
    }

    #[objc::msg_send(newFence)]
    pub fn new_fence_ar(&self) -> Option<&'ar Fence>;

    #[objc::rar_retain()]
    pub fn new_fence(&self) -> Option<arc::R<Fence>>;

    #[objc::msg_send(newEvent)]
    pub fn new_event_ar(&self) -> Option<&'ar Event>;

    #[objc::rar_retain()]
    pub fn new_event(&self) -> Option<arc::R<Event>>;

    #[objc::msg_send(newSharedEvent)]
    pub fn new_shared_event_ar(&self) -> Option<&'ar SharedEvent>;

    #[objc::rar_retain()]
    pub fn new_shared_event(&self) -> Option<arc::R<SharedEvent>>;

    #[objc::msg_send(maxBufferLength)]
    pub fn max_buffer_length(&self) -> usize;

    /// Returns the size and alignment, in bytes, of a texture if you create it from a heap.
    #[objc::msg_send(heapTextureSizeAndAlignWithDescriptor:)]
    pub fn heap_texture_size_and_align(&self, descriptor: &mtl::TextureDescriptor) -> SizeAndAlign;

    /// Returns the size and alignment, in bytes, of a buffer if you create it from a heap.
    #[objc::msg_send(heapBufferSizeAndAlignWithLength:options:)]
    pub fn heap_buffer_size_and_align(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> SizeAndAlign;

    #[objc::msg_send(newHeapWithDescriptor:)]
    pub fn new_heap_desc_ar(&self, descriptor: &mtl::HeapDescriptor) -> Option<&'ar mtl::Heap>;

    #[objc::rar_retain()]
    pub fn new_heap_desc(&self, descriptor: &mtl::HeapDescriptor) -> Option<arc::R<mtl::Heap>>;

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
    pub fn supports_function_pointers(&self) -> bool;

    /// Query device support for using function pointers from render pipeline stages.
    #[objc::msg_send(supportsFunctionPointersFromRender)]
    pub fn supports_function_pointers_from_render(&self) -> bool;

    /// Query device support for using ray tracing from render pipeline stages.
    #[objc::msg_send(supportsRaytracingFromRender)]
    pub fn supports_raytracing_from_render(&self) -> bool;

    /// Query device support for using ray tracing primitive motion blur.
    #[objc::msg_send(supportsPrimitiveMotionBlur)]
    pub fn supports_primitive_motion_blur(&self) -> bool;
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> Option<arc::R<Device>>;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics1() {
        let device = mtl::Device::default().unwrap();

        assert!(device.supports_raytracing());
        assert!(device.supports_function_pointers());
        assert!(device.supports_function_pointers_from_render());
        assert!(device.supports_raytracing_from_render());
        assert!(device.supports_primitive_motion_blur());

        let mut fence = device.new_fence().unwrap();
        let label = ns::String::with_str("nice");
        fence.set_label(Some(&label));

        let mut event = device.new_shared_event().unwrap();
        let label = ns::String::with_str("nice");
        event.set_label(Some(&label));

        let _name = device.name();

        assert!(device.max_buffer_length() > 10);

        let registry_id = device.registry_id();

        assert_ne!(0, registry_id);
        let size = device.max_threads_per_threadgroup();

        assert_ne!(0, size.width);
        assert_ne!(0, size.height);
        assert_ne!(0, size.depth);

        assert!(device.has_unified_memory());

        let tier = device.read_write_texture_support();
        assert_ne!(tier, mtl::ReadWriteTextureTier::None);

        let tier = device.argument_buffers_support();
        assert_ne!(tier, mtl::ArgumentBuffersTier::_1);

        assert!(device.new_default_lib_ar().is_none());
        assert!(device.new_default_lib().is_none());

        let td = mtl::TextureDescriptor::new_2d_with_pixel_format(
            mtl::PixelFormat::A8Unorm,
            100,
            200,
            false,
        );

        let _t = device.new_texture(&td).unwrap();
        let _t = device.new_texture_ar(&td).unwrap();

        assert!(device.max_buffer_length() > 10);

        let _queue = device.new_cmd_queue_ar().unwrap();
        let _queue = device.new_cmd_queue().unwrap();

        let source = ns::String::with_str("void function_a() {}");
        let options = None;
        let mut err = None;
        let _lib = device
            .new_lib_with_src_err(&source, options, &mut err)
            .unwrap();
    }
}
