use crate::{arc, cf, define_cls, define_mtl, define_obj_type, define_options, io, mtl, ns, objc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum Type {
    _1D = 0,
    _1DArray = 1,
    _2D = 2,
    _2DArray = 3,
    _2DMultisample = 4,
    Cube = 5,
    CubeArray,
    _3D = 7,
    _2DMultisampleArray = 8,
    TextureBuffer = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum Swizzle {
    Zero = 0,
    One = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    Alpha = 5,
}

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

define_obj_type!(SharedTextureHandle(ns::Id));

impl SharedTextureHandle {
    define_mtl!(device, label);
}

define_options!(Usage(usize));

impl Usage {
    pub const UNKNOWN: Self = Self(0x0000);
    pub const SHADER_READ: Self = Self(0x0001);
    pub const SHADER_WRITE: Self = Self(0x0002);
    pub const RENDER_TARGET: Self = Self(0x0004);
    pub const PIXEL_FROMAT_VIEW: Self = Self(0x0010);

    pub fn to_cf_number(&self) -> arc::R<cf::Number> {
        cf::Number::from_i64(self.0 as _)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum CompressionType {
    Lossless = 0,
    Lossy = 1,
}

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    define_cls!(MTL_TEXTURE_DESCRIPTOR);
    define_mtl!(storage_mode, set_storage_mode);

    #[objc::cls_msg_send(texture2DDescriptorWithPixelFormat:width:height:mipmapped:)]
    pub fn new_2d_with_pixel_format_ar(
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> &'ar Descriptor;

    #[objc::cls_rar_retain()]
    pub fn new_2d_with_pixel_format(
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> arc::R<Descriptor>;

    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let td = mtl::TextureDescriptor::new_cube_with_pixel_format(mtl::PixelFormat::A8Unorm, 100, false);
    ///
    /// assert_eq!(td.texture_type(), mtl::TextureType::Cube);
    ///
    /// ```
    #[objc::cls_msg_send(textureCubeDescriptorWithPixelFormat:size:mipmapped:)]
    pub fn new_cube_with_pixel_format_ar(
        pixel_format: mtl::PixelFormat,
        size: usize,
        mipmapped: bool,
    ) -> &'ar Descriptor;

    #[objc::cls_rar_retain()]
    pub fn new_cube_with_pixel_format(
        pixel_format: mtl::PixelFormat,
        size: usize,
        mipmapped: bool,
    ) -> arc::R<Descriptor>;

    #[objc::cls_msg_send(texture2DDescriptorWithPixelFormat:width:resourceOptions:usage:)]
    pub fn new_2d_with_resource_options_ar(
        pixel_format: mtl::PixelFormat,
        width: usize,
        resource_options: mtl::resource::Options,
        usage: Usage,
    ) -> &'ar Descriptor;

    #[objc::cls_rar_retain()]
    pub fn new_2d_with_resource_options(
        pixel_format: mtl::PixelFormat,
        width: usize,
        resource_options: mtl::resource::Options,
        usage: Usage,
    ) -> arc::R<Descriptor>;

    #[objc::msg_send(textureType)]
    pub fn texture_type(&self) -> Type;

    #[objc::msg_send(setTextureType:)]
    pub fn set_texture_type(&mut self, value: Type);

    #[inline]
    pub fn with_texture_type(&mut self, value: Type) -> &mut Self {
        self.set_texture_type(value);
        self
    }

    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, value: mtl::PixelFormat);

    #[inline]
    pub fn with_pixel_format(&mut self, value: mtl::PixelFormat) -> &mut Self {
        self.set_pixel_format(value);
        self
    }

    define_mtl!(
        width,
        set_width,
        height,
        set_height,
        depth,
        set_depth,
        resource_options,
        set_resource_options,
        cpu_cache_mode,
        set_cpu_cache_mode
    );

    #[objc::msg_send(mipmapLevelCount)]
    pub fn mipmap_level_count(&self) -> usize;

    #[objc::msg_send(setMipmapLevelCount:)]
    pub fn set_mipmap_level_count(&mut self, value: usize);

    #[objc::msg_send(sampleCount)]
    pub fn sample_count(&self) -> usize;

    #[objc::msg_send(setSampleCount:)]
    pub fn set_sample_count(&mut self, value: usize);

    #[objc::msg_send(arrayLength)]
    pub fn array_len(&self) -> usize;

    #[objc::msg_send(setArrayLength:)]
    pub fn set_array_len(&mut self, value: usize);

    #[objc::msg_send(usage)]
    pub fn usage(&self) -> Usage;

    #[objc::msg_send(setUsage:)]
    pub fn set_usage(&mut self, value: Usage);

    /// Allow GPU-optimization for the contents of this texture. The default value is true.
    #[objc::msg_send(allowGPUOptimizedContents)]
    pub fn allow_gpu_optimized_contents(&self) -> bool;

    #[objc::msg_send(setAllowGPUOptimizedContents:)]
    pub fn set_allow_gpu_optimized_contents(&mut self, value: bool);

    #[objc::msg_send(compressionType)]
    pub fn compression_type(&self) -> CompressionType;

    #[objc::msg_send(setCompressionType:)]
    pub fn set_compression_type(&mut self, value: CompressionType);

    #[objc::msg_send(swizzle)]
    pub fn swizzle(&self) -> SwizzleChannels;

    #[objc::msg_send(setSwizzle:)]
    pub fn set_swizzle(&mut self, value: SwizzleChannels);
}

define_obj_type!(Texture(mtl::Resource));

impl Texture {
    define_mtl!(width, height, depth, gpu_resouce_id);

    #[objc::msg_send(parentTexture)]
    pub fn parent_texture(&self) -> Option<&Texture>;

    #[objc::msg_send(newTextureViewWithPixelFormat:)]
    pub fn texture_view_with_pixel_format_ar(
        &self,
        pixel_format: mtl::PixelFormat,
    ) -> Option<&'ar Texture>;

    #[objc::rar_retain()]
    pub fn texture_view_with_pixel_format(
        &self,
        pixel_format: mtl::PixelFormat,
    ) -> Option<arc::R<Texture>>;

    #[objc::msg_send(iosurface)]
    pub fn io_surface(&self) -> Option<&io::Surface>;

    #[objc::msg_send(iosurfacePlane)]
    pub fn io_surface_plane(&self) -> usize;

    #[objc::msg_send(textureType)]
    pub fn texture_type(&self) -> Type;

    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_TEXTURE_DESCRIPTOR: &'static objc::Class<Descriptor>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics1() {
        let mut td = mtl::TextureDescriptor::new_2d_with_pixel_format(
            mtl::PixelFormat::A8Unorm,
            100,
            200,
            false,
        );

        assert_eq!(td.texture_type(), mtl::TextureType::_2D);
        assert_eq!(td.pixel_format(), mtl::PixelFormat::A8Unorm);
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
        let td = mtl::TextureDescriptor::new_cube_with_pixel_format(
            mtl::PixelFormat::A8Unorm,
            100,
            false,
        );

        assert_eq!(td.texture_type(), mtl::TextureType::Cube);
    }

    #[test]
    fn basics3() {
        let device = mtl::Device::default().unwrap();

        let td = mtl::TextureDescriptor::new_2d_with_pixel_format(
            mtl::PixelFormat::A8Unorm,
            100,
            200,
            false,
        );

        let t = device.new_texture(&td).unwrap();

        assert_eq!(t.width(), 100);
        assert_eq!(t.height(), 200);
        assert_eq!(t.depth(), 1);
    }

    #[test]
    fn basics4() {
        let device = mtl::Device::default().unwrap();

        let td = mtl::TextureDescriptor::new_2d_with_pixel_format(
            mtl::PixelFormat::A8Unorm,
            100,
            200,
            false,
        );

        let t = device.new_texture(&td).unwrap();

        assert!(t.parent_texture().is_none());
        assert!(t.io_surface().is_none());
        assert_eq!(t.texture_type(), mtl::texture::Type::_2D);
        assert!(t.io_surface().is_none());
        assert_eq!(t.io_surface_plane(), 0);

        let tv = t
            .texture_view_with_pixel_format(mtl::PixelFormat::A8Unorm)
            .unwrap();

        assert!(tv.parent_texture().is_some());
        assert_eq!(tv.width(), 100);
        assert_eq!(tv.height(), 200);
    }
}
