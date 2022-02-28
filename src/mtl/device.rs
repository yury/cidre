use crate::{
    cf::{self, Retained},
    define_obj_type,
    ns::Id,
};

use super::{texture, CommandQueue, Size};

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
    pub fn default<'a>() -> Option<Retained<'a, Device>> {
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
        unsafe { rsel_name(self) }
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
    /// let queue = device.new_command_queue().unwrap();
    ///
    /// queue.as_type_ref().show();
    ///
    #[inline]
    pub fn new_command_queue(&self) -> Option<Retained<CommandQueue>> {
        unsafe { rsel_newCommandQueue(self) }
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let queue = device.new_command_queue_with_max_command_buffer_count(1).unwrap();
    ///
    /// queue.as_type_ref().show();
    ///
    #[inline]
    pub fn new_command_queue_with_max_command_buffer_count<'a>(
        &self,
        max_command_buffer_count: usize,
    ) -> Option<Retained<'a, CommandQueue>> {
        unsafe { rsel_newCommandQueueWithMaxCommandBufferCount(self, max_command_buffer_count) }
    }

    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let td = mtl::TextureDescriptor::new_2d_with_pixel_format(mtl::PixelFormat::A8Unorm, 100, 200, false);
    ///
    /// let t = device.new_texture_with_descriptor(&td).unwrap();
    ///
    /// ```
    #[inline]
    pub fn new_texture_with_descriptor<'a>(
        &self,
        descriptor: &texture::Descriptor,
    ) -> Option<Retained<texture::Texture>> {
        unsafe { rsel_newTextureWithDescriptor(self, descriptor) }
    }
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    fn MTLCreateSystemDefaultDevice<'a>() -> Option<Retained<'a, Device>>;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_name(id: &Device) -> &cf::String;
    fn rsel_registryID(id: &Device) -> u64;
    fn rsel_maxThreadsPerThreadgroup(id: &Device) -> Size;
    fn rsel_hasUnifiedMemory(id: &Device) -> bool;
    fn rsel_readWriteTextureSupport(id: &Device) -> ReadWriteTextureTier;
    fn rsel_argumentBuffersSupport(id: &Device) -> ArgumentBuffersTier;
    fn rsel_newCommandQueue(id: &Device) -> Option<Retained<CommandQueue>>;
    fn rsel_newCommandQueueWithMaxCommandBufferCount<'a>(
        id: &Device,
        maxCommandBufferCount: usize,
    ) -> Option<Retained<'a, CommandQueue>>;

    fn rsel_newTextureWithDescriptor<'a>(
        id: &Device,
        descriptor: &texture::Descriptor,
    ) -> Option<Retained<'a, texture::Texture>>;
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
