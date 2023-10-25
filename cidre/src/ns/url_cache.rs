use crate::{
    arc, define_cls, define_obj_type, ns,
    objc::{self, Class},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum StoragePolicy {
    Allowed,
    InMemoryOnly,
    NotAllowed,
}

define_obj_type!(CachedUrlResponse(ns::Id));
define_obj_type!(UrlCache(ns::Id));

impl arc::A<UrlCache> {
    #[objc::msg_send(initWithMemoryCapacity:diskCapacity:directoryURL:)]
    pub fn init_with_capacity(
        self,
        mem_capacity: usize,
        disk_capacity: usize,
        directory_url: Option<&ns::Url>,
    ) -> arc::R<UrlCache>;
}

impl UrlCache {
    define_cls!(NS_URL_CACHE);
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

    pub fn with_capacity(
        mem_capacity: usize,
        disk_capacity: usize,
        directory_url: Option<&ns::Url>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_capacity(mem_capacity, disk_capacity, directory_url)
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
    static NS_URL_CACHE: &'static Class<UrlCache>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let cache = ns::UrlCache::shared();

        assert_eq!(512_000, cache.memory_capacity());
        assert_eq!(20_000_000, cache.disk_capacity());

        assert_eq!(0, cache.current_memory_usage());
    }
}
