use crate::{
    arc, define_mtl, define_obj_type, mtl, ns,
    objc::{self, Class},
};

#[derive(Debug, Eq, PartialEq)]
#[repr(isize)]
pub enum Type {
    Automatic = 0,
    Placement = 1,
    Sparce = 2,
}

define_obj_type!(Descriptor(ns::Id));

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

    /// ```no_run
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
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { MTL_HEAP_DESCRIPTOR.alloc_init() }
    }

    #[objc::msg_send2(size)]
    pub fn size(&self) -> usize;

    #[objc::msg_send2(setSize:)]
    pub fn set_size(&mut self, value: usize);

    #[objc::msg_send2(type)]
    pub fn _type(&self) -> Type;

    #[objc::msg_send2(setType:)]
    pub fn set_type(&self, value: Type);
}

define_obj_type!(Heap(ns::Id));

impl Heap {
    define_mtl!(
        device,
        label,
        set_label,
        hazard_tracking_mode,
        resource_options
    );

    #[objc::msg_send2(size)]
    pub fn size(&self) -> usize;

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

    #[objc::msg_send2(type)]
    pub fn _type(&self) -> Type;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_HEAP_DESCRIPTOR: &'static Class<Descriptor>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let mut desc = mtl::HeapDescriptor::new();
        assert_eq!(0, desc.size());
        desc.set_size(1024);
        assert_eq!(1024, desc.size());

        let device = mtl::Device::default().unwrap();
        let heap = device.new_heap_with_descriptor(&desc).unwrap();
        assert!(heap.size() >= 1024);
        let heap = device.new_heap_with_descriptor_ar(&desc).unwrap();
        assert!(heap.size() >= 1024);
    }
}
