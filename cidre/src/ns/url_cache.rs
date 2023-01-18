use crate::{
    arc, define_obj_type, ns,
    objc::{msg_send, Obj},
};

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
    /// ```no_run
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

    #[inline]
    pub fn memory_capacity(&self) -> usize {
        unsafe { self.call0(msg_send::memory_capacity) }
    }

    #[inline]
    pub fn set_memory_capacity(&self, value: usize) {
        unsafe { self.call1(msg_send::set_memory_capacity, value) }
    }

    #[inline]
    pub fn disk_capacity(&self) -> usize {
        unsafe { self.call0(msg_send::disk_capacity) }
    }

    #[inline]
    pub fn set_disk_capacity(&self, value: usize) {
        unsafe { self.call1(msg_send::set_disk_capacity, value) }
    }

    #[inline]
    pub fn current_memory_usage(&self) -> usize {
        unsafe { self.call0(msg_send::current_memory_usage) }
    }

    #[inline]
    pub fn current_disk_usage(&self) -> usize {
        unsafe { self.call0(msg_send::current_disk_usage) }
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

}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let cache = ns::URLCache::shared();

        assert_eq!(512_000, cache.memory_capacity());
        assert_eq!(20_000_000, cache.disk_capacity());

        assert_eq!(0, cache.current_memory_usage());
    }
}
