use crate::{
    cf::{self, Retained},
    cv, define_cf_type,
};

define_cf_type!(PixelBufferPool(cf::Type));

impl PixelBufferPool {
    pub fn new<'a>(
        pool_attributes: Option<&cf::Dictionary>,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
    ) -> Result<Retained<'a, Self>, cv::Return> {
        let mut pool_out = None;
        unsafe {
            Self::create(
                None,
                pool_attributes,
                pixel_buffer_attributes,
                &mut pool_out,
            )
            .to_result(pool_out)
        }
    }

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

    pub fn attributes(&self) -> Option<&cf::Dictionary> {
        unsafe { CVPixelBufferPoolGetAttributes(self) }
    }

    pub fn pixel_buffer_attributes(&self) -> Option<&cf::Dictionary> {
        unsafe { CVPixelBufferPoolGetPixelBufferAttributes(self) }
    }

    pub unsafe fn create_pixel_buffer<'a>(
        &self,
        allocator: Option<&cf::Allocator>,
        pixel_buffer_out: &mut Option<Retained<'a, cv::PixelBuffer>>,
    ) -> cv::Return {
        CVPixelBufferPoolCreatePixelBuffer(allocator, self, pixel_buffer_out)
    }

    pub fn pixel_buffer<'a>(&self) -> Result<Retained<'a, cv::PixelBuffer>, cv::Return> {
        unsafe {
            let mut pixel_buffer_out = None;
            self.create_pixel_buffer(None, &mut pixel_buffer_out)
                .to_result(pixel_buffer_out)
        }
    }

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
}

pub mod keys {
    use crate::cf;

    pub fn minimum_buffer_count() -> &'static cf::String {
        unsafe { kCVPixelBufferPoolMinimumBufferCountKey }
    }

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

    pub fn allocation_threashold() -> &'static cf::String {
        unsafe { kCVPixelBufferPoolAllocationThresholdKey }
    }

    extern "C" {
        static kCVPixelBufferPoolAllocationThresholdKey: &'static cf::String;
    }
}

pub mod notifications {
    use crate::cf;
    pub fn free_buffer_notification() -> &'static cf::NotificationName {
        unsafe { kCVPixelBufferPoolFreeBufferNotification }
    }

    extern "C" {
        static kCVPixelBufferPoolFreeBufferNotification: &'static cf::NotificationName;
    }
}
