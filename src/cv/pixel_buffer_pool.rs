use crate::{
    cf::{self, Retained},
    cv, define_cf_type, define_options,
};

define_cf_type!(PixelBufferPool(cf::Type));

impl PixelBufferPool {
    #[inline]
    pub fn new<'a>(
        pool_attributes: Option<&cf::Dictionary>,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
    ) -> Result<Retained<'a, Self>, cv::Return> {
        unsafe {
            let mut pool_out = None;
            Self::create(
                None,
                pool_attributes,
                pixel_buffer_attributes,
                &mut pool_out,
            )
            .to_result(pool_out)
        }
    }

    #[inline]
    pub unsafe fn create<'a>(
        allocator: Option<&cf::Allocator>,
        pool_attributes: Option<&cf::Dictionary>,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pool_out: &mut Option<Retained<'a, PixelBufferPool>>,
    ) -> cv::Return {
        CVPixelBufferPoolCreate(
            allocator,
            pool_attributes,
            pixel_buffer_attributes,
            pool_out,
        )
    }

    #[inline]
    pub fn attributes(&self) -> Option<&cf::Dictionary> {
        unsafe { CVPixelBufferPoolGetAttributes(self) }
    }

    #[inline]
    pub fn pixel_buffer_attributes(&self) -> Option<&cf::Dictionary> {
        unsafe { CVPixelBufferPoolGetPixelBufferAttributes(self) }
    }

    #[inline]
    pub unsafe fn create_pixel_buffer<'a>(
        &self,
        allocator: Option<&cf::Allocator>,
        pixel_buffer_out: &mut Option<Retained<'a, cv::PixelBuffer>>,
    ) -> cv::Return {
        CVPixelBufferPoolCreatePixelBuffer(allocator, self, pixel_buffer_out)
    }

    #[inline]
    pub fn pixel_buffer<'a>(&self) -> Result<Retained<'a, cv::PixelBuffer>, cv::Return> {
        unsafe {
            let mut pixel_buffer_out = None;
            self.create_pixel_buffer(None, &mut pixel_buffer_out)
                .to_result(pixel_buffer_out)
        }
    }

    #[inline]
    pub unsafe fn create_pixel_buffer_with_aux_attributes<'a>(
        &self,
        allocator: Option<&cf::Allocator>,
        aux_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: &mut Option<Retained<'a, cv::PixelBuffer>>,
    ) -> cv::Return {
        CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
            allocator,
            self,
            aux_attributes,
            pixel_buffer_out,
        )
    }

    #[inline]
    pub fn pixel_buffer_with_aux_attributes<'a>(
        &self,
        aux_attributes: Option<&cf::Dictionary>,
    ) -> Result<Retained<'a, cv::PixelBuffer>, cv::Return> {
        unsafe {
            let mut pixel_buffer_out = None;
            self.create_pixel_buffer_with_aux_attributes(
                None,
                aux_attributes,
                &mut pixel_buffer_out,
            )
            .to_result(pixel_buffer_out)
        }
    }

    #[inline]
    pub fn flush(&self, options: FlushFlags) {
        unsafe { CVPixelBufferPoolFlush(self, options) }
    }
}

extern "C" {
    fn CVPixelBufferPoolCreate<'a>(
        allocator: Option<&cf::Allocator>,
        pool_attributes: Option<&cf::Dictionary>,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pool_out: &mut Option<Retained<'a, PixelBufferPool>>,
    ) -> cv::Return;

    fn CVPixelBufferPoolGetAttributes(pool: &PixelBufferPool) -> Option<&cf::Dictionary>;
    fn CVPixelBufferPoolGetPixelBufferAttributes(pool: &PixelBufferPool)
        -> Option<&cf::Dictionary>;

    fn CVPixelBufferPoolCreatePixelBuffer<'a>(
        allocator: Option<&cf::Allocator>,
        pixel_buffer_pool: &PixelBufferPool,
        pixel_buffer_out: &mut Option<Retained<'a, cv::PixelBuffer>>,
    ) -> cv::Return;

    fn CVPixelBufferPoolCreatePixelBufferWithAuxAttributes<'a>(
        allocator: Option<&cf::Allocator>,
        pixel_buffer_pool: &PixelBufferPool,
        aux_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: &mut Option<Retained<'a, cv::PixelBuffer>>,
    ) -> cv::Return;

    fn CVPixelBufferPoolFlush(pool: &PixelBufferPool, options: FlushFlags);

}

pub mod keys {
    use crate::cf;

    #[inline]
    pub fn minimum_buffer_count() -> &'static cf::String {
        unsafe { kCVPixelBufferPoolMinimumBufferCountKey }
    }

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

    #[inline]
    pub fn free_buffer() -> &'static cf::NotificationName {
        unsafe { kCVPixelBufferPoolFreeBufferNotification }
    }

    extern "C" {
        static kCVPixelBufferPoolFreeBufferNotification: &'static cf::NotificationName;
    }
}

define_options!(FlushFlags(u64));

impl FlushFlags {
    pub const NONE: Self = Self(0);
    pub const EXCESS_BUFFERS: Self = Self(1);
}
