use crate::{cf, define_cf_type};

define_cf_type!(MemoryPool(cf::Type));

impl MemoryPool {
    ///```rust
    /// use cidre::cm;
    /// let pool = cm::MemoryPool::new();
    /// let allocator = pool.allocator();
    /// pool.flush();
    ///````
    #[inline]
    pub fn new() -> cf::Retained<MemoryPool> {
        Self::create(None)
    }

    #[inline]
    pub fn create(options: Option<&cf::Dictionary>) -> cf::Retained<MemoryPool> {
        unsafe { CMMemoryPoolCreate(options) }
    }

    #[inline]
    pub fn allocator(&self) -> &cf::Allocator {
        unsafe { CMMemoryPoolGetAllocator(self) }
    }

    #[inline]
    pub fn flush(&self) {
        unsafe { CMMemoryPoolFlush(self) }
    }

    #[inline]
    pub fn invalidate(&self) {
        unsafe { CMMemoryPoolInvalidate(self) }
    }
}

pub mod keys {
    use crate::cf;

    pub fn age_out_period() -> &'static cf::String {
        unsafe { kCMMemoryPoolOption_AgeOutPeriod }
    }

    extern "C" {
        static kCMMemoryPoolOption_AgeOutPeriod: &'static cf::String;
    }
}

extern "C" {
    fn CMMemoryPoolCreate(options: Option<&cf::Dictionary>) -> cf::Retained<MemoryPool>;
    fn CMMemoryPoolGetAllocator(pool: &MemoryPool) -> &cf::Allocator;
    fn CMMemoryPoolFlush(pool: &MemoryPool);
    fn CMMemoryPoolInvalidate(pool: &MemoryPool);
}
