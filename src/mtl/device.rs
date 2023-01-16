use std::{ffi::c_void, intrinsics::transmute};

use crate::{
    arc, blocks, define_obj_type, define_options, io, msg_send, mtl, ns,
    objc::{msg_send, Obj},
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
    #[inline]
    pub fn name(&self) -> &ns::String {
        unsafe { self.call0(msg_send::name) }
    }

    #[inline]
    pub fn registry_id(&self) -> u64 {
        unsafe { self.call0(crate::mtl::msg_send::registry_id) }
    }

    #[inline]
    pub fn max_threads_per_threadgroup(&self) -> Size {
        unsafe { self.call0(crate::mtl::msg_send::max_threads_per_threadgroup) }
    }

    #[inline]
    pub fn has_unified_memory(&self) -> bool {
        unsafe { self.call0(crate::mtl::msg_send::has_unified_memory) }
    }

    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let tier = device.read_write_texture_support();
    ///
    /// assert_ne!(tier, mtl::ReadWriteTextureTier::None);
    #[inline]
    pub fn read_write_texture_support(&self) -> ReadWriteTextureTier {
        unsafe { self.call0(crate::mtl::msg_send::read_write_texture_support) }
    }

    /// Example
    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let tier = device.argument_buffers_support();
    ///
    /// assert_ne!(tier, mtl::ArgumentBuffersTier::_1);
    #[inline]
    pub fn argument_buffers_support(&self) -> ArgumentBuffersTier {
        unsafe { self.call0(crate::mtl::msg_send::argument_buffers_support) }
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
    pub fn command_queue(&self) -> Option<arc::R<CommandQueue>> {
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
    ) -> Option<arc::R<CommandQueue>> {
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
    ) -> Option<arc::R<mtl::Texture>> {
        unsafe { rsel_newTextureWithDescriptor(self, descriptor) }
    }

    #[inline]
    pub fn texture_with_surface(
        &self,
        descriptor: &mtl::TextureDescriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<arc::R<mtl::Texture>> {
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
    pub fn default_library(&self) -> Option<arc::R<Library>> {
        unsafe { rsel_newDefaultLibrary(self) }
    }

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

    #[inline]
    pub fn buffer_with_length_and_options(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<Buffer>> {
        unsafe { rsel_newBufferWithLength_options(self, length, options) }
    }

    #[inline]
    pub fn buffer_with_bytes_length_and_options(
        &self,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<Buffer>> {
        unsafe { rsel_newBufferWithBytes_length_options(self, bytes, length, options) }
    }

    pub fn buffer_with_slice<T: Sized>(
        &self,
        slice: &[T],
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<Buffer>> {
        self.buffer_with_bytes_length_and_options(
            slice.as_ptr() as _,
            std::mem::size_of::<T>() * slice.len(),
            options,
        )
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
    #[inline]
    pub fn fence(&self) -> Option<arc::R<Fence>> {
        msg_send!("mtl", self, sel_newFence)
    }

    /// ```no_run
    /// use cidre::{ns, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let mut event = device.event().unwrap();
    /// let label = ns::String::with_str("nice");
    /// event.set_label(Some(&label));
    /// ```
    #[inline]
    pub fn event(&self) -> Option<arc::R<Event>> {
        unsafe { rsel_newEvent(self) }
    }

    /// ```no_run
    /// use cidre::{ns, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let mut event = device.shared_event().unwrap();
    /// let label = ns::String::with_str("nice");
    /// event.set_label(Some(&label));
    /// ```
    #[inline]
    pub fn shared_event(&self) -> Option<arc::R<SharedEvent>> {
        unsafe { rsel_newSharedEvent(self) }
    }

    /// ```no_run
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
    ) -> Option<arc::R<mtl::Heap>> {
        unsafe { rsel_newHeapWithDescriptor(self, descriptor) }
    }
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
    fn rsel_newCommandQueueWithMaxCommandBufferCount(
        id: &Device,
        maxCommandBufferCount: usize,
    ) -> Option<arc::R<CommandQueue>>;

    // reuse in Heap
    fn rsel_newTextureWithDescriptor(
        id: &ns::Id,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<arc::R<mtl::Texture>>;

    fn rsel_newTextureWithDescriptor_iosurface_plane(
        id: &Device,
        descriptor: &mtl::TextureDescriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<arc::R<mtl::Texture>>;

    fn rsel_newDefaultLibrary(id: &Device) -> Option<arc::R<Library>>;

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

    // reuse this in Heap
    fn rsel_newBufferWithLength_options(
        id: &ns::Id,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<Buffer>>;

    fn rsel_newBufferWithBytes_length_options(
        id: &Device,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<Buffer>>;

    fn rsel_newEvent(id: &Device) -> Option<arc::R<Event>>;

    fn rsel_maxBufferLength(id: &Device) -> usize;

    fn rsel_newSharedEvent(id: &Device) -> Option<arc::R<SharedEvent>>;

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
    ) -> Option<arc::R<mtl::Heap>>;
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
    }

    #[test]
    fn basics2() {
        let device = mtl::Device::default().unwrap();

        let mut event = device.shared_event().unwrap();
        let label = ns::String::with_str("nice");
        event.set_label(Some(&label));
    }

    #[test]
    fn basics3() {
        let device = mtl::Device::default().unwrap();

        let name = device.name();
        assert!(device.max_buffer_length() > 10);
    }

    #[test]
    fn basics4() {
        let device = mtl::Device::default().unwrap();
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
    }
}
