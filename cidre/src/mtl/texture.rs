use crate::{arc, cf, define_mtl, define_obj_type, define_opts, mtl, ns, objc};

#[cfg(feature = "io")]
use crate::io;

/// Describes the dimensionality of each image, and if multiple images are arranged into an array or cube.
#[doc(alias = "MTLTextureType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum Type {
    #[doc(alias = "MTLTextureType1D")]
    _1d = 0,

    #[doc(alias = "MTLTextureType1DArray")]
    _1dArray = 1,

    #[doc(alias = "MTLTextureType2D")]
    _2d = 2,

    #[doc(alias = "MTLTextureType2DArray")]
    _2dArray = 3,

    #[doc(alias = "MTLTextureType2DMultisample")]
    _2dMultisample = 4,

    #[doc(alias = "MTLTextureTypeCube")]
    Cube = 5,

    #[doc(alias = "MTLTextureTypeCubeArray")]
    CubeArray = 6,

    #[doc(alias = "MTLTextureType3D")]
    _3d = 7,

    #[doc(alias = "MTLTextureType2DMultisampleArray")]
    _2dMultisampleArray = 8,

    #[doc(alias = "MTLTextureTypeTextureBuffer")]
    TextureBuffer = 9,
}

#[doc(alias = "MTLTextureSwizzle")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum Swizzle {
    #[doc(alias = "MTLTextureSwizzleZero")]
    Zero = 0,

    #[doc(alias = "MTLTextureSwizzleOne")]
    One = 1,

    #[doc(alias = "MTLTextureSwizzleRed")]
    Red = 2,

    #[doc(alias = "MTLTextureSwizzleGreen")]
    Green = 3,

    #[doc(alias = "MTLTextureSwizzleBlue")]
    Blue = 4,

    #[doc(alias = "MTLTextureSwizzleAlpha")]
    Alpha = 5,
}

#[doc(alias = "MTLTextureSwizzleChannels")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct SwizzleChannels {
    pub red: Swizzle,
    pub green: Swizzle,
    pub blue: Swizzle,
    pub alpha: Swizzle,
}

impl Default for SwizzleChannels {
    #[inline]
    fn default() -> SwizzleChannels {
        SwizzleChannels {
            red: Swizzle::Red,
            green: Swizzle::Green,
            blue: Swizzle::Blue,
            alpha: Swizzle::Alpha,
        }
    }
}

define_obj_type!(
    #[doc(alias = "MTLSharedTextureHandle")]
    pub SharedTextureHandle(ns::Id)
);

impl SharedTextureHandle {
    define_mtl!(device, label);
}

define_opts!(
    #[doc(alias = "MTLTextureUsage")]
    pub Usage(usize)
);

impl Usage {
    #[doc(alias = "MTLTextureUsageUnknown")]
    pub const UNKNOWN: Self = Self(0x0000);

    #[doc(alias = "MTLTextureUsageShaderRead")]
    pub const SHADER_READ: Self = Self(0x0001);

    #[doc(alias = "MTLTextureUsageShaderWrite")]
    pub const SHADER_WRITE: Self = Self(0x0002);

    #[doc(alias = "MTLTextureUsageRenderTarget")]
    pub const RENDER_TARGET: Self = Self(0x0004);

    #[doc(alias = "MTLTextureUsagePixelFormatView")]
    pub const PIXEL_FORMAT_VIEW: Self = Self(0x0010);

    #[doc(alias = "MTLTextureUsageShaderAtomic")]
    pub const SHADER_ATOMIC: Self = Self(0x0020);

    pub fn to_cf_number(&self) -> arc::R<cf::Number> {
        cf::Number::from_i64(self.0 as _)
    }
}

#[doc(alias = "MTLTextureCompressionType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum Compression {
    #[doc(alias = "MTLTextureCompressionTypeLossless")]
    Lossless = 0,

    #[doc(alias = "MTLTextureCompressionTypeLossy")]
    Lossy = 1,
}

define_obj_type!(
    /// An object that you use to configure new Metal texture objects.
    #[doc(alias = "MTLTextureDescriptor")]
    pub Desc(ns::Id),
    MTL_TEXTURE_DESCRIPTOR
);

impl Desc {
    #[objc::msg_send(texture2DDescriptorWithPixelFormat:width:height:mipmapped:)]
    pub fn new_2d(
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> arc::R<Desc>;

    /// ```
    /// use cidre::mtl;
    ///
    /// let td = mtl::TextureDesc::new_cube(mtl::PixelFormat::A8UNorm, 100, false);
    ///
    /// assert_eq!(td.texture_type(), mtl::TextureType::Cube);
    ///
    /// ```
    #[objc::msg_send(textureCubeDescriptorWithPixelFormat:size:mipmapped:)]
    pub fn new_cube(pixel_format: mtl::PixelFormat, size: usize, mipmapped: bool) -> arc::R<Desc>;

    #[objc::msg_send(textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:)]
    pub fn new_buff(
        pixel_format: mtl::PixelFormat,
        width: usize,
        res_opts: mtl::resource::Opts,
        usage: Usage,
    ) -> arc::R<Desc>;

    #[objc::msg_send(textureType)]
    pub fn texture_type(&self) -> Type;

    #[objc::msg_send(setTextureType:)]
    pub fn set_texture_type(&mut self, val: Type);

    #[inline]
    pub fn with_texture_type(&mut self, val: Type) -> &mut Self {
        self.set_texture_type(val);
        self
    }

    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, val: mtl::PixelFormat);

    // #[inline]
    // pub fn with_pixel_format(&mut self, val: mtl::PixelFormat) -> &mut Self {
    //     self.set_pixel_format(val);
    //     self
    // }

    define_mtl!(
        width,
        set_width,
        height,
        set_height,
        depth,
        set_depth,
        res_opts,
        set_res_opts,
        cpu_cache_mode,
        set_cpu_cache_mode,
        storage_mode,
        set_storage_mode,
        hazard_tracking_mode,
        set_hazard_tracking_mode
    );

    #[objc::msg_send(mipmapLevelCount)]
    pub fn mipmap_level_count(&self) -> usize;

    #[objc::msg_send(setMipmapLevelCount:)]
    pub fn set_mipmap_level_count(&mut self, val: usize);

    #[objc::msg_send(sampleCount)]
    pub fn sample_count(&self) -> usize;

    #[objc::msg_send(setSampleCount:)]
    pub fn set_sample_count(&mut self, val: usize);

    #[objc::msg_send(arrayLength)]
    pub fn array_len(&self) -> usize;

    #[objc::msg_send(setArrayLength:)]
    pub fn set_array_len(&mut self, val: usize);

    #[objc::msg_send(usage)]
    pub fn usage(&self) -> Usage;

    #[objc::msg_send(setUsage:)]
    pub fn set_usage(&mut self, val: Usage);

    /// Allow GPU-optimization for the contents of this texture. The default value is true.
    #[objc::msg_send(allowGPUOptimizedContents)]
    pub fn allow_gpu_optimized_contents(&self) -> bool;

    #[objc::msg_send(setAllowGPUOptimizedContents:)]
    pub fn set_allow_gpu_optimized_contents(&mut self, val: bool);

    #[objc::msg_send(compressionType)]
    pub fn compression_type(&self) -> Compression;

    #[objc::msg_send(setCompressionType:)]
    pub fn set_compression_type(&mut self, val: Compression);

    #[objc::msg_send(swizzle)]
    pub fn swizzle(&self) -> SwizzleChannels;

    #[objc::msg_send(setSwizzle:)]
    pub fn set_swizzle(&mut self, val: SwizzleChannels);
}

define_obj_type!(
    /// A resource that holds formatted image data.
    #[doc(alias = "MTLTexture")]
    pub Texture(mtl::Res)
);

impl Texture {
    define_mtl!(width, height, depth, gpu_resource_id);

    /// The texture this texture view was created from, or [`None`] if this is not
    /// a texture view or it was not created from a texture.
    #[objc::msg_send(parentTexture)]
    pub fn parent_texture(&self) -> Option<arc::R<Texture>>;

    #[objc::msg_send(newTextureViewWithPixelFormat:)]
    pub fn new_texture_view_with_pixel_format(
        &self,
        pixel_format: mtl::PixelFormat,
    ) -> Option<arc::R<Texture>>;

    /// The buffer this texture view was created from, or [`None`] if this is not a texture
    /// view or it was not created from a buffer.
    #[objc::msg_send(buffer)]
    pub fn buf(&self) -> Option<arc::R<mtl::Buf>>;

    /// The offset of the buffer this texture view was created from, or 0 if this is not a texture view.
    #[objc::msg_send(bufferOffset)]
    pub fn buf_offset(&self) -> usize;

    /// The 'bytes_per_row' of the buffer this texture view was created from, or 0 if this is not a texture view.
    #[objc::msg_send(bufferBytesPerRow)]
    pub fn buf_bytes_per_row(&self) -> usize;

    #[cfg(feature = "io")]
    #[objc::msg_send(iosurface)]
    pub fn io_surf(&self) -> Option<&io::Surf>;

    #[objc::msg_send(iosurfacePlane)]
    pub fn io_surf_plane(&self) -> usize;

    #[objc::msg_send(textureType)]
    pub fn texture_type(&self) -> Type;

    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(mipmapLevelCount)]
    pub fn mipmap_level_count(&self) -> usize;

    #[objc::msg_send(sampleCount)]
    pub fn sample_count(&self) -> usize;

    #[objc::msg_send(arrayLength)]
    pub fn array_len(&self) -> usize;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_TEXTURE_DESCRIPTOR: &'static objc::Class<Desc>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics1() {
        let mut td = mtl::TextureDesc::new_2d(mtl::PixelFormat::A8UNorm, 100, 200, false);

        assert_eq!(td.texture_type(), mtl::TextureType::_2d);
        assert_eq!(td.pixel_format(), mtl::PixelFormat::A8UNorm);
        assert_eq!(td.width(), 100);
        assert_eq!(td.height(), 200);
        assert_eq!(td.depth(), 1);
        assert_eq!(td.mipmap_level_count(), 1);
        assert_eq!(td.sample_count(), 1);
        assert_eq!(td.array_len(), 1);

        td.set_width(200);
        assert_eq!(td.width(), 200);

        td.set_height(300);
        assert_eq!(td.height(), 300);
    }

    #[test]
    fn basics2() {
        let td = mtl::TextureDesc::new_cube(mtl::PixelFormat::A8UNorm, 100, false);

        assert_eq!(td.texture_type(), mtl::TextureType::Cube);
    }

    #[test]
    fn basics3() {
        let device = mtl::Device::sys_default().unwrap();

        let td = mtl::TextureDesc::new_2d(mtl::PixelFormat::A8UNorm, 100, 200, false);

        let t = device.new_texture(&td).unwrap();

        assert_eq!(t.width(), 100);
        assert_eq!(t.height(), 200);
        assert_eq!(t.depth(), 1);
    }

    #[test]
    fn basics4() {
        let device = mtl::Device::sys_default().unwrap();

        let td = mtl::TextureDesc::new_2d(mtl::PixelFormat::A8UNorm, 100, 200, false);

        let t = device.new_texture(&td).unwrap();

        assert!(t.parent_texture().is_none());
        assert!(t.io_surf().is_none());
        assert_eq!(t.texture_type(), mtl::texture::Type::_2d);
        assert_eq!(t.pixel_format(), mtl::PixelFormat::A8UNorm);
        assert!(t.io_surf().is_none());
        assert_eq!(t.io_surf_plane(), 0);

        let tv = t
            .new_texture_view_with_pixel_format(mtl::PixelFormat::A8UNorm)
            .unwrap();

        assert!(tv.parent_texture().is_some());
        assert_eq!(tv.width(), 100);
        assert_eq!(tv.height(), 200);
    }
}
