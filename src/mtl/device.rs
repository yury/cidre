use std::{ffi::c_void, intrinsics::transmute};

use crate::{
    cf::{self, Retained},
    define_obj_type, io, mtl,
    ns::Id,
    objc::block::CompletionHandlerAB,
};

use super::{texture, Buffer, CommandQueue, Library, Size};

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

define_obj_type!(Device(Id));

impl Device {
    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    /// ```
    #[inline]
    pub fn default<'create>() -> Option<Retained<'create, Device>> {
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
        unsafe { get_rsel_name(self) }
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
    pub fn command_queue<'create>(&self) -> Option<Retained<'create, CommandQueue>> {
        unsafe { rsel_newCommandQueue(self) }
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
    pub fn command_queue_with_max_command_buffer_count<'create>(
        &self,
        max_command_buffer_count: usize,
    ) -> Option<Retained<'create, CommandQueue>> {
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
    pub fn texture_with_descriptor<'create>(
        &self,
        descriptor: &texture::Descriptor,
    ) -> Option<Retained<'create, texture::Texture>> {
        unsafe { rsel_newTextureWithDescriptor(self, descriptor) }
    }

    #[inline]
    pub fn texture_with_surface<'a>(
        &self,
        descriptor: &texture::Descriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<Retained<'a, texture::Texture>> {
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
    pub fn default_library<'create>(&self) -> Option<Retained<'create, Library>> {
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
    pub fn library_with_source_and_error<'create>(
        &self,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
        error: &mut Option<&cf::Error>,
    ) -> Option<Retained<'create, Library>> {
        unsafe { rsel_newLibraryWithSource_options_error(self, source, options, error) }
    }

    #[inline]
    pub fn library_with_source<'create>(
        &self,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
    ) -> Result<Retained<'create, Library>, &'create cf::Error> {
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
    pub fn compute_pipeline_state_with_function_error<'create>(
        &self,
        function: &mtl::Function,
        error: &mut Option<&cf::Error>,
    ) -> Option<Retained<'create, mtl::ComputePipelineState>> {
        unsafe { rsel_newComputePipelineStateWithFunction_error(self, function, error) }
    }

    #[inline]
    pub fn compute_pipeline_state_with_function<'create>(
        &self,
        function: &mtl::Function,
    ) -> Result<Retained<'create, mtl::ComputePipelineState>, &cf::Error> {
        let mut error = None;
        let res = self.compute_pipeline_state_with_function_error(function, &mut error);

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }

    #[inline]
    pub fn buffer_with_length_and_options<'create>(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<Retained<'create, Buffer>> {
        unsafe { rsel_newBufferWithLength_options(self, length, options) }
    }

    #[inline]
    pub fn buffer_with_bytes_length_and_options<'create>(
        &self,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<Retained<'create, Buffer>> {
        unsafe { rsel_newBufferWithBytes_length_options(self, bytes, length, options) }
    }
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice<'create>() -> Option<Retained<'create, Device>>;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn sel_newLibraryWithSource_options_completionHandler(
        id: &Device,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
        rb_bloc: *const c_void,
    );
    fn get_rsel_name(id: &Device) -> &cf::String;
    fn rsel_registryID(id: &Device) -> u64;
    fn rsel_maxThreadsPerThreadgroup(id: &Device) -> Size;
    fn rsel_hasUnifiedMemory(id: &Device) -> bool;
    fn rsel_readWriteTextureSupport(id: &Device) -> ReadWriteTextureTier;
    fn rsel_argumentBuffersSupport(id: &Device) -> ArgumentBuffersTier;
    fn rsel_newCommandQueue<'create>(id: &Device) -> Option<Retained<'create, CommandQueue>>;
    fn rsel_newCommandQueueWithMaxCommandBufferCount<'a>(
        id: &Device,
        maxCommandBufferCount: usize,
    ) -> Option<Retained<'a, CommandQueue>>;

    fn rsel_newTextureWithDescriptor<'create>(
        id: &Device,
        descriptor: &texture::Descriptor,
    ) -> Option<Retained<'create, texture::Texture>>;

    fn rsel_newTextureWithDescriptor_iosurface_plane<'a>(
        id: &Device,
        descriptor: &texture::Descriptor,
        surface: &io::Surface,
        plane: usize,
    ) -> Option<Retained<'a, texture::Texture>>;

    fn rsel_newDefaultLibrary<'a>(id: &Device) -> Option<Retained<'a, Library>>;

    fn rsel_newLibraryWithSource_options_error<'a>(
        id: &Device,
        source: &cf::String,
        options: Option<&mtl::CompileOptions>,
        error: &mut Option<&cf::Error>,
    ) -> Option<Retained<'a, Library>>;

    // NS_RETURNS_RETAINED
    // rsel_ab(, id, newComputePipelineStateWithFunction, id<MTLFunction>, error, NSError * _Nullable * _Nullable, id<MTLComputePipelineState> _Nullable)
    fn rsel_newComputePipelineStateWithFunction_error<'create>(
        id: &Device,
        function: &mtl::Function,
        error: &mut Option<&cf::Error>,
    ) -> Option<Retained<'create, mtl::ComputePipelineState>>;

    fn rsel_newBufferWithLength_options<'create>(
        id: &Device,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<Retained<'create, Buffer>>;
    fn rsel_newBufferWithBytes_length_options<'create>(
        id: &Device,
        bytes: *const c_void,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<Retained<'create, Buffer>>;

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
