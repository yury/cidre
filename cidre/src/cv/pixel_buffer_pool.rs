use crate::{arc, cf, cv, define_cf_type, define_opts, os};

define_cf_type!(
    #[doc(alias = "CVPixelBufferPoolRef")]
    PixelBufPool(cf::Type)
);

impl PixelBufPool {
    #[doc(alias = "CVPixelBufferPoolCreate")]
    #[inline]
    pub fn new(
        pool_attrs: Option<&cf::Dictionary>,
        pixel_buf_attrs: Option<&cf::Dictionary>,
    ) -> os::Result<arc::R<Self>> {
        unsafe {
            os::result_unchecked(|pool_out| {
                Self::create_in(pool_attrs, pixel_buf_attrs, pool_out, None)
            })
        }
    }

    #[doc(alias = "CVPixelBufferPoolCreate")]
    #[inline]
    pub unsafe fn create_in(
        pool_attrs: Option<&cf::Dictionary>,
        pixel_buf_attrs: Option<&cf::Dictionary>,
        pool_out: *mut Option<arc::R<PixelBufPool>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        CVPixelBufferPoolCreate(allocator, pool_attrs, pixel_buf_attrs, pool_out)
    }

    #[doc(alias = "CVPixelBufferPoolGetAttribute")]
    #[inline]
    pub fn attrs(&self) -> Option<&cf::Dictionary> {
        unsafe { CVPixelBufferPoolGetAttributes(self) }
    }

    #[doc(alias = "CVPixelBufferPoolGetPixelBufferAttributes")]
    #[inline]
    pub fn pixel_buf_attrs(&self) -> Option<&cf::Dictionary> {
        unsafe { CVPixelBufferPoolGetPixelBufferAttributes(self) }
    }

    #[inline]
    pub unsafe fn create_pixel_buffer_in(
        &self,
        pixel_buf_out: *mut Option<arc::R<cv::PixelBuf>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        CVPixelBufferPoolCreatePixelBuffer(allocator, self, pixel_buf_out)
    }

    #[inline]
    pub fn pixel_buf(&self) -> os::Result<arc::R<cv::PixelBuf>> {
        unsafe {
            os::result_unchecked(|pixel_buf_out| self.create_pixel_buffer_in(pixel_buf_out, None))
        }
    }

    #[doc(alias = "CVPixelBufferPoolCreatePixelBufferWithAuxAttributes")]
    #[inline]
    pub unsafe fn create_pixel_buf_with_aux_attributes_in(
        &self,
        allocator: Option<&cf::Allocator>,
        aux_attrs: Option<&cf::Dictionary>,
        pixel_buf_out: *mut Option<arc::R<cv::PixelBuf>>,
    ) -> cv::Return {
        CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
            allocator,
            self,
            aux_attrs,
            pixel_buf_out,
        )
    }

    #[inline]
    pub fn pixel_buf_with_aux_attrs(
        &self,
        aux_attrs: Option<&cf::Dictionary>,
    ) -> os::Result<arc::R<cv::PixelBuf>> {
        unsafe {
            os::result_unchecked(|pixel_buf_out| {
                self.create_pixel_buf_with_aux_attributes_in(None, aux_attrs, pixel_buf_out)
            })
        }
    }

    #[inline]
    pub fn flush(&self, options: FlushFlags) {
        unsafe { CVPixelBufferPoolFlush(self, options) }
    }
}

extern "C-unwind" {
    fn CVPixelBufferPoolCreate(
        allocator: Option<&cf::Allocator>,
        pool_attrs: Option<&cf::Dictionary>,
        pixel_buf_attrs: Option<&cf::Dictionary>,
        pool_out: *mut Option<arc::R<PixelBufPool>>,
    ) -> cv::Return;

    fn CVPixelBufferPoolGetAttributes(pool: &PixelBufPool) -> Option<&cf::Dictionary>;
    fn CVPixelBufferPoolGetPixelBufferAttributes(pool: &PixelBufPool) -> Option<&cf::Dictionary>;

    fn CVPixelBufferPoolCreatePixelBuffer(
        allocator: Option<&cf::Allocator>,
        pixel_buf_pool: &PixelBufPool,
        pixel_buf_out: *mut Option<arc::R<cv::PixelBuf>>,
    ) -> cv::Return;

    fn CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
        allocator: Option<&cf::Allocator>,
        pixel_buf_pool: &PixelBufPool,
        aux_attrs: Option<&cf::Dictionary>,
        pixel_buf_out: *mut Option<arc::R<cv::PixelBuf>>,
    ) -> cv::Return;

    fn CVPixelBufferPoolFlush(pool: &PixelBufPool, options: FlushFlags);

}

pub mod keys {
    use crate::cf;

    #[doc(alias = "kCVPixelBufferPoolMinimumBufferCountKey")]
    #[inline]
    pub fn minimum_buffer_count() -> &'static cf::String {
        unsafe { kCVPixelBufferPoolMinimumBufferCountKey }
    }

    #[doc(alias = "kCVPixelBufferPoolMaximumBufferAgeKey")]
    #[inline]
    pub fn maximum_buffer_age() -> &'static cf::String {
        unsafe { kCVPixelBufferPoolMaximumBufferAgeKey }
    }

    extern "C" {
        static kCVPixelBufferPoolMinimumBufferCountKey: &'static cf::String;
        static kCVPixelBufferPoolMaximumBufferAgeKey: &'static cf::String;
    }
}

pub mod aux_attr_keys {
    use crate::cf;

    #[doc(alias = "kCVPixelBufferPoolAllocationThresholdKey")]
    #[inline]
    pub fn allocation_threashold() -> &'static cf::String {
        unsafe { kCVPixelBufferPoolAllocationThresholdKey }
    }

    extern "C" {
        static kCVPixelBufferPoolAllocationThresholdKey: &'static cf::String;
    }
}

pub mod notifications {
    use crate::cf;

    #[doc(alias = "kCVPixelBufferPoolFreeBufferNotification")]
    #[inline]
    pub fn free_buf() -> &'static cf::NotificationName {
        unsafe { kCVPixelBufferPoolFreeBufferNotification }
    }

    extern "C" {
        static kCVPixelBufferPoolFreeBufferNotification: &'static cf::NotificationName;
    }
}

define_opts!(
    #[doc(alias = "CVPixelBufferPoolFlushFlags")]
    pub FlushFlags(u64)
);

impl FlushFlags {
    pub const NONE: Self = Self(0);

    /// This flag will cause CVPixelBufferPoolFlush to flush all unused buffers regardless of age.
    #[doc(alias = "kCVPixelBufferPoolFlushExcessBuffers")]
    pub const EXCESS_BUFFERS: Self = Self(1);
}
