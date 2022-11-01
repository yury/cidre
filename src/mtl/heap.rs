use crate::cf::Retained;
use crate::{define_mtl, define_obj_type, msg_send, mtl, objc};

use crate::ns::Id;

#[derive(Debug, Eq, PartialEq)]
#[repr(isize)]
pub enum Type {
    Automatic = 0,
    Placement = 1,
    Sparce = 2,
}

define_obj_type!(Descriptor(Id));

impl Descriptor {
    define_mtl!(
        storage_mode,
        set_storage_mode,
        cpu_cache_mode,
        set_cpu_cache_mode,
        hazard_tracking_mode,
        set_hazard_tracking_mode,
        resource_options,
        set_resource_options
    );

    /// ```
    /// use cidre::mtl;
    ///
    /// let mut desc = mtl::HeapDescriptor::new();
    /// assert_eq!(0, desc.size());
    /// desc.set_size(1024);
    /// assert_eq!(1024, desc.size());
    ///
    /// let device = mtl::Device::default().unwrap();
    /// let heap = device.new_heap_with_descriptor(&desc).unwrap();
    /// assert!(heap.size() >= 1024);
    /// ```
    pub fn new() -> Retained<Self> {
        unsafe { MTLHeapDescriptor_new() }
    }

    pub fn size(&self) -> usize {
        msg_send!("common", self, sel_size)
    }

    pub fn set_size(&mut self, value: usize) {
        msg_send!("common", self, sel_setSize, value)
    }
}

define_obj_type!(Heap(Id));

impl Heap {
    define_mtl!(
        device,
        label,
        set_label,
        hazard_tracking_mode,
        resource_options
    );

    pub fn size(&self) -> usize {
        msg_send!("common", self, sel_size)
    }

    #[inline]
    pub fn buffer_with_length_and_options(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<Retained<mtl::Buffer>> {
        unsafe { rsel_newBufferWithLength_options(self, length, options) }
    }

    #[inline]
    pub fn texture_with_descriptor(
        &self,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<Retained<mtl::Texture>> {
        unsafe { rsel_newTextureWithDescriptor(self, descriptor) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {

    fn MTLHeapDescriptor_new() -> Retained<Descriptor>;

    fn rsel_newBufferWithLength_options(
        id: &objc::Id,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<Retained<mtl::Buffer>>;

    fn rsel_newTextureWithDescriptor(
        id: &objc::Id,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<Retained<mtl::Texture>>;
}
