use crate::{arc, cf, cv, define_cf_type, define_opts, os};

define_cf_type!(
    #[doc(alias = "CVPixelBufferPoolRef")]
    PixelBufPool(cf::Type)
);

impl PixelBufPool {
    #[inline]
    pub fn new(
        pool_attributes: Option<&cf::Dictionary>,
        pixel_buf_attributes: Option<&cf::Dictionary>,
    ) -> os::Result<arc::R<Self>> {
        unsafe {
            let mut pool_out = None;
            Self::create_in(pool_attributes, pixel_buf_attributes, &mut pool_out, None)
                .to_result_unchecked(pool_out)
        }
    }

    #[doc(alias = "CVPixelBufferPoolCreate")]
    #[inline]
    pub unsafe fn create_in(
        pool_attributes: Option<&cf::Dictionary>,
        pixel_buf_attributes: Option<&cf::Dictionary>,
        pool_out: *mut Option<arc::R<PixelBufPool>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        CVPixelBufferPoolCreate(allocator, pool_attributes, pixel_buf_attributes, pool_out)
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
        pixel_buffer_out: *mut Option<arc::R<cv::PixelBuf>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        CVPixelBufferPoolCreatePixelBuffer(allocator, self, pixel_buffer_out)
    }

    #[inline]
    pub fn pixel_buf(&self) -> os::Result<arc::R<cv::PixelBuf>> {
        unsafe { os::result_unchecked(|res| self.create_pixel_buffer_in(res, None)) }
    }

    #[doc(alias = "CVPixelBufferPoolCreatePixelBufferWithAuxAttributes")]
    #[inline]
    pub unsafe fn create_pixel_buf_with_aux_attributes_in(
        &self,
        allocator: Option<&cf::Allocator>,
        aux_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: *mut Option<arc::R<cv::PixelBuf>>,
    ) -> cv::Return {
        CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
            allocator,
            self,
            aux_attributes,
            pixel_buffer_out,
        )
    }

    #[inline]
    pub fn pixel_buf_with_aux_attrs(
        &self,
        aux_attributes: Option<&cf::Dictionary>,
    ) -> os::Result<arc::R<cv::PixelBuf>> {
        unsafe {
            os::result_unchecked(|res| {
                self.create_pixel_buf_with_aux_attributes_in(None, aux_attributes, res)
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
        pool_attributes: Option<&cf::Dictionary>,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pool_out: *mut Option<arc::R<PixelBufPool>>,
    ) -> cv::Return;

    fn CVPixelBufferPoolGetAttributes(pool: &PixelBufPool) -> Option<&cf::Dictionary>;
    fn CVPixelBufferPoolGetPixelBufferAttributes(pool: &PixelBufPool) -> Option<&cf::Dictionary>;

    fn CVPixelBufferPoolCreatePixelBuffer(
        allocator: Option<&cf::Allocator>,
        pixel_buffer_pool: &PixelBufPool,
        pixel_buffer_out: *mut Option<arc::R<cv::PixelBuf>>,
    ) -> cv::Return;

    fn CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
        allocator: Option<&cf::Allocator>,
        pixel_buffer_pool: &PixelBufPool,
        aux_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: *mut Option<arc::R<cv::PixelBuf>>,
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

pub mod aux_attribute_keys {
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

define_opts!(pub FlushFlags(u64));

impl FlushFlags {
    pub const NONE: Self = Self(0);
    pub const EXCESS_BUFFERS: Self = Self(1);
}
