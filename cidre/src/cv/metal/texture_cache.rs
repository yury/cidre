use crate::{arc, cf, cv, define_cf_type, mtl, os};

define_cf_type!(
    #[doc(alias = "CVMetalTextureCacheRef")]
    TextureCache(cf::Type)
);

impl TextureCache {
    #[doc(alias = "CVMetalTextureCacheCreate")]
    #[inline]
    pub unsafe fn create_in(
        cache_attrs: Option<&cf::Dictionary>,
        metal_device: &mtl::Device,
        texture_attrs: Option<&cf::Dictionary>,
        cache_out: *mut Option<arc::R<TextureCache>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        CVMetalTextureCacheCreate(
            allocator,
            cache_attrs,
            metal_device,
            texture_attrs,
            cache_out,
        )
    }

    #[doc(alias = "CVMetalTextureCacheCreate")]
    #[inline]
    pub fn create(
        cache_attrs: Option<&cf::Dictionary>,
        metal_device: &mtl::Device,
        texture_attrs: Option<&cf::Dictionary>,
    ) -> os::Result<arc::R<TextureCache>> {
        unsafe {
            os::result_unchecked(|res| {
                Self::create_in(cache_attrs, metal_device, texture_attrs, res, None)
            })
        }
    }

    #[doc(alias = "CVMetalTextureCacheCreateTextureFromImage")]
    #[inline]
    pub unsafe fn create_texture_in(
        &self,
        source_image: &cv::ImageBuf,
        texture_attrs: Option<&cf::Dictionary>,
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        plane_index: usize,
        texture_out: *mut Option<arc::R<cv::MetalTexture>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        CVMetalTextureCacheCreateTextureFromImage(
            allocator,
            self,
            source_image,
            texture_attrs,
            pixel_format,
            width,
            height,
            plane_index,
            texture_out,
        )
    }

    #[doc(alias = "CVMetalTextureCacheCreateTextureFromImage")]
    #[inline]
    pub fn texture(
        &self,
        source_image: &cv::ImageBuf,
        texture_attributes: Option<&cf::Dictionary>,
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        plane_index: usize,
    ) -> os::Result<arc::R<cv::MetalTexture>> {
        unsafe {
            os::result_unchecked(|res| {
                self.create_texture_in(
                    source_image,
                    texture_attributes,
                    pixel_format,
                    width,
                    height,
                    plane_index,
                    res,
                    None,
                )
            })
        }
    }

    /// Performs internal housekeeping/recycling operations
    ///
    /// This call must be made periodically to give the texture cache a chance to do internal housekeeping operations.
    #[doc(alias = "CVMetalTextureCacheFlush")]
    #[inline]
    pub fn flush(&mut self) {
        unsafe { CVMetalTextureCacheFlush(self, 0) }
    }
}

#[link(name = "CoreVideo", kind = "framework")]
extern "C-unwind" {
    fn CVMetalTextureCacheCreate(
        allocator: Option<&cf::Allocator>,
        cache_attrs: Option<&cf::Dictionary>,
        metal_device: &mtl::Device,
        texture_attrs: Option<&cf::Dictionary>,
        cache_out: *mut Option<arc::R<TextureCache>>,
    ) -> cv::Return;

    fn CVMetalTextureCacheCreateTextureFromImage(
        allocator: Option<&cf::Allocator>,
        texture_cache: &TextureCache,
        source_image: &cv::ImageBuf,
        texture_attrs: Option<&cf::Dictionary>,
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        plane_index: usize,
        texture_out: *mut Option<arc::R<cv::MetalTexture>>,
    ) -> cv::Return;

    fn CVMetalTextureCacheFlush(texture_cache: &mut TextureCache, options: usize);
}

pub mod keys {
    use crate::cf;

    /// By default, textures will age out after one second.  Setting a maximum
    /// texture age of zero will disable the age-out mechanism completely.
    /// TextureCache::flush() can be used to force eviction in either case.
    #[inline]
    pub fn maximum_texture_age() -> &'static cf::String {
        unsafe { kCVMetalTextureCacheMaximumTextureAgeKey }
    }

    /// Property that can be placed on a cv::MetalTextureCache to instruct the mtl::TextureUsage of the created mtl::Texture.
    #[doc(alias = "kCVMetalTextureUsage")]
    #[inline]
    pub fn texture_usage() -> &'static cf::String {
        unsafe { kCVMetalTextureUsage }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        static kCVMetalTextureCacheMaximumTextureAgeKey: &'static cf::String;
        static kCVMetalTextureUsage: &'static cf::String;
    }
}
