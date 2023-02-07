use crate::{
    arc, define_cls, define_mtl, define_obj_type, mtl, ns,
    objc::{self, Class},
};

#[derive(Debug, Eq, PartialEq)]
#[repr(isize)]
pub enum Type {
    /// A heap that automatically places new resource allocations.
    ///
    /// In this mode, resources are placed in the heap automatically.
    /// Automatically placed resources have optimal GPU-specific layout, and may perform better than mtl::HeapType::Placement.
    /// This heap type is recommended when the heap primarily contains temporary write-often resources.
    Automatic = 0,

    /// The app controls placement of resources on the heap.
    ///
    /// Use placement heaps when you need direct control over memory use and heap fragmentation.
    /// Typically, you use placement heaps for resources you keep for long time periods and rarely change.
    ///
    /// In this mode, the app places resources in the heap.
    /// Manually placed resources allow the app to control memory usage and heap fragmentation directly.
    /// This heap type is recommended when the heap primarily contains persistent write-rarely resources.
    Placement = 1,

    /// The heap contains sparse texture tiles.
    Sparce = 2,
}

define_obj_type!(Descriptor(ns::Id));

impl arc::A<Descriptor> {
    #[objc::msg_send(init)]
    fn init(self) -> arc::R<Descriptor>;
}

impl Descriptor {
    define_cls!(MTL_HEAP_DESCRIPTOR);
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
        Self::alloc().init()
    }

    #[objc::msg_send(size)]
    pub fn size(&self) -> usize;

    #[objc::msg_send(setSize:)]
    pub fn set_size(&mut self, value: usize);

    #[objc::msg_send(type)]
    pub fn _type(&self) -> Type;

    #[objc::msg_send(setType:)]
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

    #[objc::msg_send(size)]
    pub fn size(&self) -> usize;

    #[objc::msg_send(newBufferWithLength:options:)]
    pub fn new_buf_length_and_options_ar(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::Rar<mtl::Buf>>;

    #[objc::rar_retain()]
    pub fn new_buf_length_and_options(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<mtl::Buf>>;

    #[objc::msg_send(newTextureWithDescriptor:)]
    pub fn texture_with_descriptor_ar(
        &self,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<arc::Rar<mtl::Texture>>;

    #[objc::rar_retain()]
    pub fn texture_with_descriptor(
        &self,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(type)]
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
