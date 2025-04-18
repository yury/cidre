use crate::{arc, cf, define_cf_type};

define_cf_type!(
    #[doc(alias = "CMMemoryPool")]
    MemPool(cf::Type)
);

unsafe impl Send for MemPool {}

/// Memory pool for optimizing repeated large block allocation.
///
/// cm::MemoryPool is a memory allocation service that holds onto a pool of
/// recently deallocated memory so as to speed up subsequent allocations of the same size.  
/// It's intended for cases where large memory blocks need to be repeatedly allocated --
/// for example, the compressed data output by a video encoder.
///
/// All of its allocations are on the granularity of page sizes; it does not suballocate
/// memory within pages, so it is a poor choice for allocating tiny blocks.
/// For example, it's appropriate to use as the blockAllocator argument to
/// cm::BlockBufferCreateWithMemoryBlock, but not the structureAllocator argument --
/// use kCFAllocatorDefault instead.
/// When you no longer need to allocate memory from the pool, call cm::MemoryPoolInvalidate
/// and CFRelease. Calling CMMemoryPoolInvalidate tells the pool to stop holding onto
/// memory for reuse.  Note that the pool's cf::Allocator can outlive the pool, owing
/// to the way that CoreFoundation is designed: cf::Allocators are themselves CF objects,
/// and every object allocated with a cf::Allocator implicitly retains the cf::Allocator
/// until it is finalized.  After the cm::MemoryPool is invalidated or finalized,
/// its cf::Allocator allocates and deallocates with no pooling behavior.
///
/// cm::MemoryPool deallocates memory if it has not been recycled in 0.5 second,
/// so that short-term peak usage does not cause persistent bloat.
/// (This period may be overridden by specifying kCMMemoryPoolOption_AgeOutPeriod.)
/// Such "aging out" is done during the pool's cf::Allocator::allocate and
/// cf::Allocator::deallocate methods.
impl MemPool {
    ///```
    /// use cidre::cm;
    /// let mut pool = cm::MemPool::new();
    /// let allocator = pool.pool_allocator();
    /// pool.flush();
    ///````
    #[doc(alias = "CMMemoryPoolCreate")]
    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::with_opts(None)
    }

    #[doc(alias = "CMMemoryPoolCreate")]
    #[inline]
    pub fn with_opts(options: Option<&cf::Dictionary>) -> arc::R<MemPool> {
        unsafe { CMMemoryPoolCreate(options) }
    }

    #[inline]
    pub fn with_age(duration: std::time::Duration) -> arc::R<Self> {
        let opts = cf::Dictionary::with_keys_values(
            &[keys::age_out_period()],
            &[&cf::Number::from_f64(duration.as_secs_f64())],
        );

        Self::with_opts(opts.as_deref())
    }

    #[doc(alias = "CMMemoryPoolGetAllocator")]
    #[inline]
    pub fn pool_allocator(&self) -> &cf::Allocator {
        unsafe { CMMemoryPoolGetAllocator(self) }
    }

    /// Deallocates all memory the pool was holding for recycling.
    #[doc(alias = "CMMemoryPoolFlush")]
    #[inline]
    pub fn flush(&mut self) {
        unsafe { CMMemoryPoolFlush(self) }
    }

    /// Stops the pool from recycling.
    #[doc(alias = "CMMemoryPoolInvalidate")]
    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { CMMemoryPoolInvalidate(self) }
    }
}

pub mod keys {
    use crate::cf;

    #[doc(alias = "kCMMemoryPoolOption_AgeOutPeriod")]
    #[inline]
    pub fn age_out_period() -> &'static cf::String {
        unsafe { kCMMemoryPoolOption_AgeOutPeriod }
    }

    unsafe extern "C" {
        static kCMMemoryPoolOption_AgeOutPeriod: &'static cf::String;
    }
}

unsafe extern "C-unwind" {
    fn CMMemoryPoolCreate(options: Option<&cf::Dictionary>) -> arc::R<MemPool>;
    fn CMMemoryPoolGetAllocator(pool: &MemPool) -> &cf::Allocator;
    fn CMMemoryPoolFlush(pool: &MemPool);
    fn CMMemoryPoolInvalidate(pool: &MemPool);
}
