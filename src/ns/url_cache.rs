use crate::{arc, define_obj_type, ns};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum StoragePolicy {
    Allowed,
    InMemoryOnly,
    NotAllowed,
}

define_obj_type!(CachedURLResponse(ns::Id));
define_obj_type!(URLCache(ns::Id));

impl URLCache {
    /// ```
    /// use cidre::ns;
    ///
    /// let cache = ns::URLCache::shared();
    ///
    /// assert_eq!(512_000, cache.memory_capacity());
    /// assert_eq!(20_000_000, cache.disk_capacity());
    ///
    /// assert_eq!(0, cache.current_memory_usage());
    ///
    /// ```
    pub fn shared() -> &'static Self {
        unsafe { NSURLCache_sharedURLCache() }
    }

    pub fn with_capacity(
        mem_capacity: usize,
        disk_capacity: usize,
        directory_url: Option<&ns::URL>,
    ) -> arc::R<Self> {
        unsafe {
            NSURLCache_initWithMemoryCapacity_diskCapacity_directoryURL(
                mem_capacity,
                disk_capacity,
                directory_url,
            )
        }
    }

    pub fn memory_capacity(&self) -> usize {
        unsafe { rsel_memoryCapacity(self) }
    }

    pub fn set_memory_capacity(&self, value: usize) {
        unsafe { wsel_setMemoryCapacity(self, value) }
    }

    pub fn disk_capacity(&self) -> usize {
        unsafe { rsel_diskCapacity(self) }
    }

    pub fn set_disk_capacity(&self, value: usize) {
        unsafe { wsel_setDiskCapacity(self, value) }
    }

    pub fn current_memory_usage(&self) -> usize {
        unsafe { rsel_currentMemoryUsage(self) }
    }

    pub fn current_disk_usage(&self) -> usize {
        unsafe { rsel_currentDiskUsage(self) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURLCache_sharedURLCache() -> &'static URLCache;

    fn NSURLCache_initWithMemoryCapacity_diskCapacity_directoryURL(
        mem_capacity: usize,
        disk_capacity: usize,
        directory_url: Option<&ns::URL>,
    ) -> arc::R<URLCache>;

    fn rsel_memoryCapacity(id: &ns::Id) -> usize;
    fn wsel_setMemoryCapacity(id: &ns::Id, value: usize);

    fn rsel_diskCapacity(id: &ns::Id) -> usize;
    fn wsel_setDiskCapacity(id: &ns::Id, value: usize);

    fn rsel_currentMemoryUsage(id: &ns::Id) -> usize;
    fn rsel_currentDiskUsage(id: &ns::Id) -> usize;

}
