use std::{ffi::c_void, intrinsics::transmute};

use crate::{arc, blocks, define_obj_type, define_options, io, mtl, ns, objc};

use super::{event::SharedEvent, Buffer, CommandQueue, Event, Fence, Library, Size};

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
    #[objc::msg_send2(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send2(registryID)]
    pub fn registry_id(&self) -> u64;

    #[objc::msg_send2(maxThreadsPerThreadgroup)]
    pub fn max_threads_per_threadgroup(&self) -> Size;

    #[objc::msg_send2(hasUnifiedMemory)]
    pub fn has_unified_memory(&self) -> bool;

    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let tier = device.read_write_texture_support();
    ///
    /// assert_ne!(tier, mtl::ReadWriteTextureTier::None);
    #[objc::msg_send2(readWriteTextureSupport)]
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
    #[objc::msg_send2(argumentBuffersSupport)]
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
    #[objc::msg_send2(newCommandQueue)]
    pub fn command_queue_ar(&self) -> Option<arc::Rar<CommandQueue>>;

    #[objc::rar_retain()]
    pub fn command_queue(&self) -> Option<arc::R<CommandQueue>>;

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let queue = device.command_queue_with_max_command_buffer_count(1).unwrap();
    ///
    /// queue.as_type_ref().show();
    ///
    #[objc::msg_send2(newCommandQueueWithMaxCommandBufferCount:)]
    pub fn command_queue_with_max_command_buffer_count_ar(
        &self,
        max_command_buffer_count: usize,
    ) -> Option<arc::Rar<CommandQueue>>;

    #[objc::rar_retain()]
    pub fn command_queue_with_max_command_buffer_count(
        &self,
        max_command_buffer_count: usize,
    ) -> Option<arc::R<CommandQueue>>;

    #[objc::msg_send2(newTextureWithDescriptor:)]
    pub fn texture_with_descriptor_ar(
        &self,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<arc::Rar<mtl::Texture>>;

    #[objc::rar_retain()]
    pub fn texture_with_descriptor(
        &self,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<arc::R<mtl::Texture>>;

    #[inline]
    pub fn texture_with_surface(
        &self,
        descriptor: &mtl::TextureDescriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<arc::R<mtl::Texture>> {
        unsafe { rsel_newTextureWithDescriptor_iosurface_plane(self, descriptor, surface, plane) }
    }

    #[objc::msg_send2(newDefaultLibrary)]
    pub fn default_library_ar(&self) -> Option<arc::Rar<Library>>;

    #[objc::rar_retain()]
    pub fn default_library(&self) -> Option<arc::R<Library>>;

    /// ```
    /// use cidre::{ns, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let source = ns::String::with_str("void function_a() {}");
    /// let options = None;
    /// let mut err = None;
    /// let lib = device.library_with_source_and_error(&source, options, &mut err).unwrap();
    ///
    /// ```
    #[inline]
    pub fn library_with_source_and_error(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<Library>> {
        unsafe { rsel_newLibraryWithSource_options_error(self, source, options, error) }
    }

    #[inline]
    pub fn library_with_source<'a>(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
    ) -> Result<arc::R<Library>, &'a ns::Error> {
        let mut error = None;
        let res = Self::library_with_source_and_error(self, source, options, &mut error);

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }

    pub async fn library_with_source_options(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
    ) -> Result<arc::R<mtl::Library>, arc::R<ns::Error>> {
        let (future, block) = blocks::result();
        self.library_with_source_options_completion(source, options, block.escape());
        future.await
    }

    pub fn library_with_source_options_completion<'ar, F>(
        &self,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
        completion: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(Option<&'ar mtl::library::Library>, Option<&'ar ns::Error>) + 'static,
    {
        unsafe {
            wsel_newLibraryWithSource_options_completionHandler(
                self,
                source,
                options,
                completion.as_ptr(),
            )
        }
    }

    #[inline]
    pub fn compute_pipeline_state_with_function_error(
        &self,
        function: &mtl::Function,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<mtl::ComputePipelineState>> {
        unsafe { rsel_newComputePipelineStateWithFunction_error(self, function, error) }
    }

    #[inline]
    pub unsafe fn render_pipeline_state_with_descriptor_error(
        &self,
        descriptor: &mtl::RenderPipelineDescriptor,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<mtl::RenderPipelineState>> {
        rsel_newRenderPipelineStateWithDescriptor_error(self, descriptor, error)
    }

    #[inline]
    pub fn render_pipeline_state_with_descriptor<'a>(
        &self,
        descriptor: &mtl::RenderPipelineDescriptor,
    ) -> Result<arc::R<mtl::RenderPipelineState>, &'a ns::Error> {
        let mut error = None;
        unsafe {
            let res =
                Self::render_pipeline_state_with_descriptor_error(self, descriptor, &mut error);
            if res.is_some() {
                Ok(transmute(res))
            } else {
                Err(transmute(error))
            }
        }
    }

    #[inline]
    pub fn compute_pipeline_state_with_function<'ar>(
        &self,
        function: &mtl::Function,
    ) -> Result<arc::R<mtl::ComputePipelineState>, &'ar ns::Error> {
        let mut error = None;
        let res = self.compute_pipeline_state_with_function_error(function, &mut error);

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }

    #[objc::msg_send2(newBufferWithLength:options:)]
    pub fn buffer_with_length_and_options_ar(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::Rar<mtl::Buffer>>;

    #[objc::rar_retain()]
    pub fn buffer_with_length_and_options(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<mtl::Buffer>>;

    #[objc::msg_send2(newBufferWithBytes:length:options:)]
    pub fn buffer_with_bytes_length_and_options_ar(
        &self,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::Rar<Buffer>>;

    #[objc::rar_retain()]
    pub fn buffer_with_bytes_length_and_options(
        &self,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<Buffer>>;

    #[inline]
    pub fn buffer_with_slice_ar<T: Sized>(
        &self,
        slice: &[T],
        options: mtl::ResourceOptions,
    ) -> Option<arc::Rar<Buffer>> {
        self.buffer_with_bytes_length_and_options_ar(
            slice.as_ptr() as _,
            std::mem::size_of::<T>() * slice.len(),
            options,
        )
    }

    #[inline]
    pub fn buffer_with_slice<T: Sized>(
        &self,
        slice: &[T],
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<Buffer>> {
        arc::Rar::option_retain(self.buffer_with_slice_ar(slice, options))
    }

    /// ```no_run
    /// use cidre::{ns, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let mut fence = device.fence().unwrap();
    /// let label = ns::String::with_str("nice");
    /// fence.set_label(Some(&label));
    /// ```
    #[objc::msg_send2(newFence)]
    pub fn fence_ar(&self) -> Option<arc::Rar<Fence>>;

    #[objc::rar_retain()]
    pub fn fence(&self) -> Option<arc::R<Fence>>;

    /// ```no_run
    /// use cidre::{ns, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let mut event = device.event().unwrap();
    /// let label = ns::String::with_str("nice");
    /// event.set_label(Some(&label));
    /// ```
    #[objc::msg_send2(newEvent)]
    pub fn event_ar(&self) -> Option<arc::Rar<Event>>;

    #[objc::rar_retain()]
    pub fn event(&self) -> Option<arc::R<Event>>;

    /// ```no_run
    /// use cidre::{ns, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let mut event = device.shared_event().unwrap();
    /// let label = ns::String::with_str("nice");
    /// event.set_label(Some(&label));
    /// ```
    #[objc::msg_send2(newSharedEvent)]
    pub fn shared_event_ar(&self) -> Option<arc::Rar<SharedEvent>>;

    #[objc::rar_retain()]
    pub fn shared_event(&self) -> Option<arc::R<SharedEvent>>;

    /// ```no_run
    /// use cidre::{mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// assert!(device.max_buffer_length() > 10);
    /// ```
    #[objc::msg_send2(maxBufferLength)]
    pub fn max_buffer_length(&self) -> usize;

    /// Returns the size and alignment, in bytes, of a texture if you create it from a heap.
    #[objc::msg_send2(heapTextureSizeAndAlignWithDescriptor:)]
    pub fn heap_texture_size_and_align(&self, descriptor: &mtl::TextureDescriptor) -> SizeAndAlign;

    /// Returns the size and alignment, in bytes, of a buffer if you create it from a heap.
    #[objc::msg_send2(heapBufferSizeAndAlignWithLength:options:)]
    pub fn heap_buffer_size_and_align(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> SizeAndAlign;

    #[objc::msg_send2(newHeapWithDescriptor:)]
    pub fn new_heap_with_descriptor_ar(
        &self,
        descriptor: &mtl::HeapDescriptor,
    ) -> Option<arc::Rar<mtl::Heap>>;

    #[objc::rar_retain()]
    pub fn new_heap_with_descriptor(
        &self,
        descriptor: &mtl::HeapDescriptor,
    ) -> Option<arc::R<mtl::Heap>>;
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> Option<arc::R<Device>>;
}

#[link(name = "mtl", kind = "static")]
extern "C" {

    fn wsel_newLibraryWithSource_options_completionHandler(
        id: &Device,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
        block: *mut c_void,
    );

    fn rsel_newTextureWithDescriptor_iosurface_plane(
        id: &Device,
        descriptor: &mtl::TextureDescriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<arc::R<mtl::Texture>>;

    fn rsel_newLibraryWithSource_options_error(
        id: &Device,
        source: &ns::String,
        options: Option<&mtl::CompileOptions>,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<Library>>;

    fn rsel_newComputePipelineStateWithFunction_error<'a>(
        id: &Device,
        function: &mtl::Function,
        error: &mut Option<&'a ns::Error>,
    ) -> Option<arc::R<mtl::ComputePipelineState>>;

    fn rsel_newRenderPipelineStateWithDescriptor_error<'a>(
        id: &Device,
        descriptor: &mtl::RenderPipelineDescriptor,
        error: &mut Option<&'a ns::Error>,
    ) -> Option<arc::R<mtl::RenderPipelineState>>;

}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics1() {
        let device = mtl::Device::default().unwrap();

        let mut fence = device.fence().unwrap();
        let label = ns::String::with_str("nice");
        fence.set_label(Some(&label));

        let mut event = device.shared_event().unwrap();
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

        assert!(device.default_library_ar().is_none());
        assert!(device.default_library().is_none());

        let td = mtl::TextureDescriptor::new_2d_with_pixel_format(
            mtl::PixelFormat::A8Unorm,
            100,
            200,
            false,
        );

        let _t = device.texture_with_descriptor(&td).unwrap();
        let _t = device.texture_with_descriptor_ar(&td).unwrap();

        assert!(device.max_buffer_length() > 10);

        let _queue = device.command_queue_ar().unwrap();
        let _queue = device.command_queue().unwrap();
    }
}
