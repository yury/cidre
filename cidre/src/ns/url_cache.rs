use crate::{
    arc, define_obj_type, ns,
    objc::{self, Class},
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
    #[objc::cls_msg_send(sharedURLCache)]
    pub fn shared() -> &'static Self;

    pub fn cls() -> &'static Class<Self> {
        unsafe { NS_URL_CACHE }
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

    #[objc::msg_send(memoryCapacity)]
    pub fn memory_capacity(&self) -> usize;

    #[objc::msg_send(setMemoryCapacity:)]
    pub fn set_memory_capacity(&self, value: usize);

    #[objc::msg_send(diskCapacity)]
    pub fn disk_capacity(&self) -> usize;

    #[objc::msg_send(setDiskCapacity:)]
    pub fn set_disk_capacity(&self, value: usize);

    #[objc::msg_send(currentMemoryUsage)]
    pub fn current_memory_usage(&self) -> usize;

    #[objc::msg_send(currentDiskUsage)]
    pub fn current_disk_usage(&self) -> usize;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_URL_CACHE: &'static Class<URLCache>;

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
