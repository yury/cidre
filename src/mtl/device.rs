use std::{ffi::c_void, intrinsics::transmute};

use crate::{
    cf, define_obj_type, define_options, io, msg_send, mtl, ns, objc::block::CompletionHandlerAB,
};

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
    pub fn default() -> Option<cf::Retained<Device>> {
        unsafe { MTLCreateSystemDefaultDevice() }
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let name = device.name();
    ///
    /// name.show_str();
    /// ```
    #[inline]
    pub fn name(&self) -> &cf::String {
        msg_send!("common", self, sel_name)
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let registry_id = device.registry_id();
    ///
    /// assert_ne!(0, registry_id);
    #[inline]
    pub fn registry_id(&self) -> u64 {
        unsafe { rsel_registryID(self) }
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let size = device.max_threads_per_threadgroup();
    ///
    /// assert_ne!(0, size.width);
    /// assert_ne!(0, size.height);
    /// assert_ne!(0, size.depth);
    #[inline]
    pub fn max_threads_per_threadgroup(&self) -> Size {
        unsafe { rsel_maxThreadsPerThreadgroup(self) }
    }

    #[inline]
    pub fn has_unified_memory(&self) -> bool {
        unsafe { rsel_hasUnifiedMemory(self) }
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let tier = device.read_write_texture_support();
    ///
    /// assert_ne!(tier, mtl::ReadWriteTextureTier::None);
    #[inline]
    pub fn read_write_texture_support(&self) -> ReadWriteTextureTier {
        unsafe { rsel_readWriteTextureSupport(self) }
    }

    /// Example
    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let tier = device.argument_buffers_support();
    ///
    /// assert_ne!(tier, mtl::ArgumentBuffersTier::_1);
    #[inline]
    pub fn argument_buffers_support(&self) -> ArgumentBuffersTier {
        unsafe { rsel_argumentBuffersSupport(self) }
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let queue = device.command_queue().unwrap();
    ///
    /// queue.as_type_ref().show();
    ///
    #[inline]
    pub fn command_queue(&self) -> Option<cf::Retained<CommandQueue>> {
        msg_send!("mtl", self, sel_newCommandQueue)
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let queue = device.command_queue_with_max_command_buffer_count(1).unwrap();
    ///
    /// queue.as_type_ref().show();
    ///
    #[inline]
    pub fn command_queue_with_max_command_buffer_count(
        &self,
        max_command_buffer_count: usize,
    ) -> Option<cf::Retained<CommandQueue>> {
        unsafe { rsel_newCommandQueueWithMaxCommandBufferCount(self, max_command_buffer_count) }
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let td = mtl::TextureDescriptor::new_2d_with_pixel_format(mtl::PixelFormat::A8Unorm, 100, 200, false);
    ///
    /// let t = device.texture_with_descriptor(&td).unwrap();
    ///
    /// ```
    #[inline]
    pub fn texture_with_descriptor(
        &self,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<cf::Retained<mtl::Texture>> {
        unsafe { rsel_newTextureWithDescriptor(self, descriptor) }
    }

    #[inline]
    pub fn texture_with_surface(
        &self,
        descriptor: &mtl::TextureDescriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<cf::Retained<mtl::Texture>> {
        unsafe { rsel_newTextureWithDescriptor_iosurface_plane(self, descriptor, surface, plane) }
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// assert!(device.default_library().is_none());
    /// ```
    #[inline]
    pub fn default_library(&self) -> Option<cf::Retained<Library>> {
        unsafe { rsel_newDefaultLibrary(self) }
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let source = cf::String::from_str("void function_a() {}");
    /// let options = None;
    /// let mut err = None;
    /// let lib = device.library_with_source_and_error(&source, options, &mut err).unwrap();
    ///
    /// ```
    #[inline]
    pub fn library_with_source_and_error(
        &self,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
        error: &mut Option<&cf::Error>,
    ) -> Option<cf::Retained<Library>> {
        unsafe { rsel_newLibraryWithSource_options_error(self, source, options, error) }
    }

    #[inline]
    pub fn library_with_source<'a>(
        &self,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
    ) -> Result<cf::Retained<Library>, &'a cf::Error> {
        let mut error = None;
        let res = Self::library_with_source_and_error(self, source, options, &mut error);

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }

    pub fn library_with_source_options_completion<T>(
        &self,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
        completion: T,
    ) where
        T: FnOnce(Option<&mtl::library::Library>, Option<&cf::error::Error>),
        T: Send + 'static,
    {
        unsafe {
            sel_newLibraryWithSource_options_completionHandler(
                self,
                source,
                options,
                completion.into_raw(),
            )
        }
    }

    #[inline]
    pub fn compute_pipeline_state_with_function_error(
        &self,
        function: &mtl::Function,
        error: &mut Option<&cf::Error>,
    ) -> Option<cf::Retained<mtl::ComputePipelineState>> {
        unsafe { rsel_newComputePipelineStateWithFunction_error(self, function, error) }
    }

    #[inline]
    pub unsafe fn render_pipeline_state_with_descriptor_error(
        &self,
        descriptor: &mtl::RenderPipelineDescriptor,
        error: &mut Option<&cf::Error>,
    ) -> Option<cf::Retained<mtl::RenderPipelineState>> {
        rsel_newRenderPipelineStateWithDescriptor_error(self, descriptor, error)
    }

    #[inline]
    pub fn render_pipeline_state_with_descriptor<'a>(
        &self,
        descriptor: &mtl::RenderPipelineDescriptor,
    ) -> Result<cf::Retained<mtl::RenderPipelineState>, &'a cf::Error> {
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
    ) -> Result<cf::Retained<mtl::ComputePipelineState>, &'ar cf::Error> {
        let mut error = None;
        let res = self.compute_pipeline_state_with_function_error(function, &mut error);

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }

    #[inline]
    pub fn buffer_with_length_and_options(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<cf::Retained<Buffer>> {
        unsafe { rsel_newBufferWithLength_options(self, length, options) }
    }

    #[inline]
    pub fn buffer_with_bytes_length_and_options(
        &self,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<cf::Retained<Buffer>> {
        unsafe { rsel_newBufferWithBytes_length_options(self, bytes, length, options) }
    }

    pub fn buffer_with_slice<T: Sized>(
        &self,
        slice: &[T],
        options: mtl::ResourceOptions,
    ) -> Option<cf::Retained<Buffer>> {
        self.buffer_with_bytes_length_and_options(
            slice.as_ptr() as _,
            std::mem::size_of::<T>() * slice.len(),
            options,
        )
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let mut fence = device.fence().unwrap();
    /// let label = cf::String::from_str("nice");
    /// fence.set_label(Some(&label));
    /// ```
    #[inline]
    pub fn fence(&self) -> Option<cf::Retained<Fence>> {
        msg_send!("mtl", self, sel_newFence)
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let mut event = device.event().unwrap();
    /// let label = cf::String::from_str("nice");
    /// event.set_label(Some(&label));
    /// ```
    #[inline]
    pub fn event(&self) -> Option<cf::Retained<Event>> {
        unsafe { rsel_newEvent(self) }
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let mut event = device.shared_event().unwrap();
    /// let label = cf::String::from_str("nice");
    /// event.set_label(Some(&label));
    /// ```
    #[inline]
    pub fn shared_event(&self) -> Option<cf::Retained<SharedEvent>> {
        unsafe { rsel_newSharedEvent(self) }
    }

    /// ```
    /// use cidre::{mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// assert!(device.max_buffer_length() > 10);
    /// ```
    #[inline]
    pub fn max_buffer_length(&self) -> usize {
        unsafe { rsel_maxBufferLength(self) }
    }

    #[inline]
    pub fn heap_texture_size_and_align(&self, descriptor: &mtl::TextureDescriptor) -> SizeAndAlign {
        unsafe { rsel_heapTextureSizeAndAlignWithDescriptor(self, descriptor) }
    }

    #[inline]
    pub fn heap_buffer_size_and_align(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> SizeAndAlign {
        unsafe { rsel_heapBufferSizeAndAlignWithLength(self, length, options) }
    }

    #[inline]
    pub fn new_heap_with_descriptor(
        &self,
        descriptor: &mtl::HeapDescriptor,
    ) -> Option<cf::Retained<mtl::Heap>> {
        unsafe { rsel_newHeapWithDescriptor(self, descriptor) }
    }
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> Option<cf::Retained<Device>>;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    // static sel_newCommandQueue: &'static Sel;

    fn sel_newLibraryWithSource_options_completionHandler(
        id: &Device,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
        rb_bloc: *const c_void,
    );
    fn rsel_registryID(id: &Device) -> u64;
    fn rsel_maxThreadsPerThreadgroup(id: &Device) -> Size;
    fn rsel_hasUnifiedMemory(id: &Device) -> bool;
    fn rsel_readWriteTextureSupport(id: &Device) -> ReadWriteTextureTier;
    fn rsel_argumentBuffersSupport(id: &Device) -> ArgumentBuffersTier;
    // fn rsel_newCommandQueue<'create>(id: &Device) -> Option<Retained<'create, CommandQueue>>;
    fn rsel_newCommandQueueWithMaxCommandBufferCount(
        id: &Device,
        maxCommandBufferCount: usize,
    ) -> Option<cf::Retained<CommandQueue>>;

    // reuse in Heap
    fn rsel_newTextureWithDescriptor(
        id: &ns::Id,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<cf::Retained<mtl::Texture>>;

    fn rsel_newTextureWithDescriptor_iosurface_plane(
        id: &Device,
        descriptor: &mtl::TextureDescriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<cf::Retained<mtl::Texture>>;

    fn rsel_newDefaultLibrary(id: &Device) -> Option<cf::Retained<Library>>;

    fn rsel_newLibraryWithSource_options_error(
        id: &Device,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
        error: &mut Option<&cf::Error>,
    ) -> Option<cf::Retained<Library>>;

    // NS_RETURNS_RETAINED
    // rsel_ab(, id, newComputePipelineStateWithFunction, id<MTLFunction>, error, NSError * _Nullable * _Nullable, id<MTLComputePipelineState> _Nullable)
    fn rsel_newComputePipelineStateWithFunction_error<'a>(
        id: &Device,
        function: &mtl::Function,
        error: &mut Option<&'a cf::Error>,
    ) -> Option<cf::Retained<mtl::ComputePipelineState>>;

    fn rsel_newRenderPipelineStateWithDescriptor_error<'a>(
        id: &Device,
        descriptor: &mtl::RenderPipelineDescriptor,
        error: &mut Option<&'a cf::Error>,
    ) -> Option<cf::Retained<mtl::RenderPipelineState>>;

    // reuse this in Heap
    fn rsel_newBufferWithLength_options(
        id: &ns::Id,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<cf::Retained<Buffer>>;

    fn rsel_newBufferWithBytes_length_options(
        id: &Device,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<cf::Retained<Buffer>>;

    // fn rsel_newFence<'create>(id: &Device) -> Option<Retained<'create, Fence>>;
    fn rsel_newEvent(id: &Device) -> Option<cf::Retained<Event>>;

    fn rsel_maxBufferLength(id: &Device) -> usize;

    fn rsel_newSharedEvent(id: &Device) -> Option<cf::Retained<SharedEvent>>;

    fn rsel_heapTextureSizeAndAlignWithDescriptor(
        id: &Device,
        descriptor: &mtl::TextureDescriptor,
    ) -> SizeAndAlign;

    fn rsel_heapBufferSizeAndAlignWithLength(
        id: &Device,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> SizeAndAlign;

    fn rsel_newHeapWithDescriptor(
        id: &Device,
        descriptor: &mtl::HeapDescriptor,
    ) -> Option<cf::Retained<mtl::Heap>>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn it_works() {
        let device = unsafe { mtl::Device::default().unwrap_unchecked() };

        let n = device.name();
        n.show_str()
    }
}
