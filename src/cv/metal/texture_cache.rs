use crate::{cf, cv, define_cf_type, mtl};

define_cf_type!(TextureCache(cf::Type));

impl TextureCache {
    #[inline]
    pub unsafe fn create_cache<'a>(
        allocator: Option<&cf::Allocator>,
        cache_attributes: Option<&cf::Dictionary>,
        metal_device: &mtl::Device,
        texture_attributes: Option<&cf::Dictionary>,
        cache_out: &mut Option<cf::Retained<'a, TextureCache>>,
    ) -> cv::Return {
        CVMetalTextureCacheCreate(
            allocator,
            cache_attributes,
            metal_device,
            texture_attributes,
            cache_out,
        )
    }

    #[inline]
    pub fn create<'a>(
        cache_attributes: Option<&cf::Dictionary>,
        metal_device: &mtl::Device,
        texture_attributes: Option<&cf::Dictionary>,
    ) -> Result<cf::Retained<'a, TextureCache>, cv::Return> {
        unsafe {
            let mut cache_out = None;
            Self::create_cache(
                None,
                cache_attributes,
                metal_device,
                texture_attributes,
                &mut cache_out,
            )
            .to_result(cache_out)
        }
    }

    #[inline]
    pub unsafe fn create_texture<'a>(
        &self,
        allocator: Option<&cf::Allocator>,
        source_image: &cv::ImageBuffer,
        texture_attributes: Option<&cf::Dictionary>,
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        plane_index: usize,
        texture_out: &mut Option<cf::Retained<'a, cv::MetalTexture>>,
    ) -> cv::Return {
        CVMetalTextureCacheCreateTextureFromImage(
            allocator,
            self,
            source_image,
            texture_attributes,
            pixel_format,
            width,
            height,
            plane_index,
            texture_out,
        )
    }

    #[inline]
    pub fn texture<'a>(
        &self,
        source_image: &cv::ImageBuffer,
        texture_attributes: Option<&cf::Dictionary>,
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        plane_index: usize,
    ) -> Result<cf::Retained<'a, cv::MetalTexture>, cv::Return> {
        unsafe {
            let mut texture_out = None;
            self.create_texture(
                None,
                source_image,
                texture_attributes,
                pixel_format,
                width,
                height,
                plane_index,
                &mut texture_out,
            )
            .to_result(texture_out)
        }
    }

    /// Performs internal housekeeping/recycling operations
    ///
    /// This call must be made periodically to give the texture cache a chance to do internal housekeeping operations.
    pub fn flush(&self) {
        unsafe { CVMetalTextureCacheFlush(self, 0) }
    }
}

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {
    fn CVMetalTextureCacheCreate<'a>(
        allocator: Option<&cf::Allocator>,
        cache_attributes: Option<&cf::Dictionary>,
        metal_device: &mtl::Device,
        texture_attributes: Option<&cf::Dictionary>,
        cache_out: &mut Option<cf::Retained<'a, TextureCache>>,
    ) -> cv::Return;

    fn CVMetalTextureCacheCreateTextureFromImage<'a>(
        allocator: Option<&cf::Allocator>,
        texture_cache: &TextureCache,
        source_image: &cv::ImageBuffer,
        texture_attributes: Option<&cf::Dictionary>,
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        plane_index: usize,
        texture_out: &mut Option<cf::Retained<'a, cv::MetalTexture>>,
    ) -> cv::Return;

    fn CVMetalTextureCacheFlush(texture_cache: &TextureCache, options: usize);
}

pub mod keys {
    use crate::cf;

    /// By default, textures will age out after one second.  Setting a maximum
    /// texture age of zero will disable the age-out mechanism completely.
    /// TextureCache::flush() can be used to force eviction in either case.
    pub fn maximum_texture_age() -> &'static cf::String {
        unsafe { kCVMetalTextureCacheMaximumTextureAgeKey }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        static kCVMetalTextureCacheMaximumTextureAgeKey: &'static cf::String;
    }
}
