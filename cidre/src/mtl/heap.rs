use crate::{arc, define_cls, define_mtl, define_obj_type, mtl, ns, objc};

#[doc(alias = "MTLHeapType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
#[repr(isize)]
pub enum Type {
    /// A heap that automatically places new resource allocations.
    ///
    /// In this mode, resources are placed in the heap automatically.
    /// Automatically placed resources have optimal GPU-specific layout, and may perform better than mtl::HeapType::Placement.
    /// This heap type is recommended when the heap primarily contains temporary write-often resources.
    #[default]
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
    Sparse = 2,
}

define_obj_type!(pub Desc(ns::Id));

impl arc::A<Desc> {
    #[objc::msg_send(init)]
    fn init(self) -> arc::R<Desc>;
}

impl Desc {
    define_cls!(MTL_HEAP_DESCRIPTOR);
    define_mtl!(
        storage_mode,
        set_storage_mode,
        cpu_cache_mode,
        set_cpu_cache_mode,
        hazard_tracking_mode,
        set_hazard_tracking_mode,
        res_opts,
        set_res_opts
    );

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(size)]
    pub fn size(&self) -> usize;

    #[objc::msg_send(setSize:)]
    pub fn set_size(&mut self, val: usize);

    #[objc::msg_send(type)]
    pub fn type_(&self) -> Type;

    #[objc::msg_send(setType:)]
    pub fn set_type(&self, val: Type);

    #[objc::msg_send(sparsePageSize)]
    pub fn sparse_page_size(&self) -> mtl::SparsePageSize;

    #[objc::msg_send(setSparsePageSize:)]
    pub fn set_sparse_page_size(&mut self, val: mtl::SparsePageSize);
}

define_obj_type!(pub Heap(ns::Id));

impl Heap {
    define_mtl!(set_label, hazard_tracking_mode, res_opts);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(size)]
    pub fn size(&self) -> usize;

    #[objc::msg_send(usedSize)]
    pub fn used_size(&self) -> usize;

    #[objc::msg_send(currentAllocatedSize)]
    pub fn current_allocated_size(&self) -> usize;

    #[objc::msg_send(maxAvailableSizeWithAlignment:)]
    pub fn max_available_size_with_alignment(&self, alignment: usize) -> usize;

    /// Create a new buffer backed by heap memory.
    #[objc::msg_send(newBufferWithLength:options:)]
    pub unsafe fn new_buf_throws(
        &self,
        length: usize,
        options: mtl::ResOpts,
    ) -> Option<arc::R<mtl::Buf>>;

    /// Create a new buffer backed by heap memory.
    pub fn new_buf<'ear>(
        &self,
        length: usize,
        options: mtl::ResOpts,
    ) -> Result<Option<arc::R<mtl::Buf>>, &'ear ns::Exception> {
        ns::try_catch(|| unsafe { self.new_buf_throws(length, options) })
    }

    #[objc::msg_send(newTextureWithDescriptor:)]
    pub fn new_texture(&self, descriptor: &mtl::TextureDesc) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(setPurgeableState:)]
    pub fn set_purgeable_state(&mut self, state: mtl::PurgableState);

    #[objc::msg_send(newBufferWithLength:options:offset:)]
    pub fn new_buf_with_offset(
        &self,
        length: usize,
        options: mtl::ResOpts,
        offset: usize,
    ) -> Option<arc::R<mtl::Buf>>;

    #[objc::msg_send(newTextureWithDescriptor:offset:)]
    pub fn new_texture_with_offset(
        &self,
        descriptor: &mtl::TextureDesc,
        offset: usize,
    ) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(newTextureWithDescriptor:offset:)]
    pub fn new_texture_with_desc_offset(
        &self,
        descriptor: &mtl::TextureDesc,
        offset: usize,
    ) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(newAccelerationStructureWithSize:)]
    pub fn new_acceleration_structure_size(
        &self,
        size: usize,
    ) -> Option<arc::R<mtl::AccelerationStruct>>;

    #[objc::msg_send(newAccelerationStructureWithSize:offset:)]
    pub fn new_acceleration_structure_size_offset(
        &self,
        size: usize,
        offset: usize,
    ) -> Option<arc::R<mtl::AccelerationStruct>>;

    #[objc::msg_send(newAccelerationStructureWithDescriptor:)]
    pub fn new_acceleration_structure_with_desc(
        &self,
        descriptor: &mtl::AccelerationStructDesc,
    ) -> Option<arc::R<mtl::AccelerationStruct>>;

    #[objc::msg_send(newAccelerationStructureWithDescriptor:offset:)]
    pub fn new_acceleration_structure_with_desc_offset(
        &self,
        descriptor: &mtl::AccelerationStructDesc,
        offset: usize,
    ) -> Option<arc::R<mtl::AccelerationStruct>>;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> Type;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_HEAP_DESCRIPTOR: &'static objc::Class<Desc>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let mut desc = mtl::HeapDesc::new();
        assert_eq!(0, desc.size());
        desc.set_size(1024);
        assert_eq!(1024, desc.size());

        let device = mtl::Device::sys_default().unwrap();
        let heap = device.new_heap_desc(&desc).unwrap();
        assert!(heap.size() >= 1024);
    }
}
