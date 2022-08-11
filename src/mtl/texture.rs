use crate::ns::Id;
use crate::{
    cf::{self, Retained},
    define_obj_type,
};
use crate::{define_mtl, io, msg_send};

use super::{resource, Device, PixelFormat, Resource, ResourceID};

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

define_obj_type!(SharedTextureHandle(Id));

impl SharedTextureHandle {
    pub fn device(&self) -> &Device {
        msg_send!("mtl", self, sel_device)
    }

    pub fn label(&self) -> Option<&cf::String> {
        msg_send!("common", self, sel_label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Usage(usize);

impl Usage {
    pub const UNKNOWN: Self = Self(0x0000);
    pub const SHADER_READ: Self = Self(0x0001);
    pub const SHADER_WRITE: Self = Self(0x0002);
    pub const RENDER_TARGET: Self = Self(0x0004);
    pub const PIXEL_FROMAT_VIEW: Self = Self(0x0010);

    pub fn to_cf_number(&self) -> Retained<cf::Number> {
        cf::Number::from_i64(self.0 as _)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum CompressionType {
    Lossless = 0,
    TypeLossy = 1,
}

define_obj_type!(Descriptor(Id));

impl Descriptor {
    define_mtl!(storage_mode, set_storage_mode);

    /// ```rust
    /// use cidre::mtl;
    ///
    /// let mut td = mtl::TextureDescriptor::new_2d_with_pixel_format(mtl::PixelFormat::A8Unorm, 100, 200, false);
    ///
    /// assert_eq!(td.texture_type(), mtl::TextureType::_2D);
    /// assert_eq!(td.pixel_format(), mtl::PixelFormat::A8Unorm);
    /// assert_eq!(td.width(), 100);
    /// assert_eq!(td.height(), 200);
    /// assert_eq!(td.depth(), 1);
    /// assert_eq!(td.mipmap_level_count(), 1);
    /// assert_eq!(td.sample_count(), 1);
    /// assert_eq!(td.array_len(), 1);
    ///
    /// td.set_width(200);
    /// assert_eq!(td.width(), 200);

    /// td.set_height(300);
    /// assert_eq!(td.height(), 300);
    ///
    /// ```
    #[inline]
    pub fn new_2d_with_pixel_format(
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> Retained<Descriptor> {
        unsafe {
            MTLTextureDescriptor_texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                pixel_format,
                width,
                height,
                mipmapped,
            )
        }
    }

    /// ```rust
    /// use cidre::mtl;
    ///
    /// let td = mtl::TextureDescriptor::new_cube_with_pixel_format(mtl::PixelFormat::A8Unorm, 100, false);
    ///
    /// assert_eq!(td.texture_type(), mtl::TextureType::Cube);
    ///
    /// ```
    #[inline]
    pub fn new_cube_with_pixel_format(
        pixel_format: PixelFormat,
        size: usize,
        mipmapped: bool,
    ) -> Retained<Descriptor> {
        unsafe {
            MTLTextureDescriptor_textureCubeDescriptorWithPixelFormat_size_mipmapped(
                pixel_format,
                size,
                mipmapped,
            )
        }
    }

    #[inline]
    pub fn with_resource_options(
        pixel_format: PixelFormat,
        width: usize,
        resource_options: crate::mtl::resource::Options,
        usage: Usage,
    ) -> Retained<Descriptor> {
        unsafe {
            MTLTextureDescriptor_texture2DDescriptorWithPixelFormat_width_resourceOptions_usage(
                pixel_format,
                width,
                resource_options,
                usage,
            )
        }
    }

    #[inline]
    pub fn texture_type(&self) -> Type {
        unsafe { rsel_textureType(self) }
    }

    #[inline]
    pub fn set_texture_type(&mut self, value: Type) {
        unsafe { wsel_textureType(self, value) }
    }

    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { rsel_pixelFormat(self) }
    }

    #[inline]
    pub fn set_pixel_format(&mut self, value: PixelFormat) {
        unsafe { wsel_setPixelFormat(self, value) }
    }

    define_mtl!(width, set_width, height, set_height, depth, set_depth);

    #[inline]
    pub fn mipmap_level_count(&self) -> usize {
        unsafe { rsel_mipmapLevelCount(self) }
    }

    #[inline]
    pub fn set_mipmap_level_count(&mut self, value: usize) {
        unsafe { wsel_setMipmapLevelCount(self, value) }
    }

    #[inline]
    pub fn sample_count(&self) -> usize {
        unsafe { MTLTextureDescriptor_rsel_sampleCount(self) }
    }

    #[inline]
    pub fn set_sample_count(&mut self, value: usize) {
        unsafe { MTLTextureDescriptor_wsel_setSampleCount(self, value) }
    }

    #[inline]
    pub fn array_len(&self) -> usize {
        unsafe { rsel_arrayLength(self) }
    }

    #[inline]
    pub fn set_array_len(&mut self, value: usize) {
        unsafe { wsel_setArrayLength(self, value) }
    }

    #[inline]
    pub fn reource_options(&self) -> resource::Options {
        unsafe { rsel_resourceOptions(self) }
    }

    #[inline]
    pub fn set_resource_options(&mut self, value: resource::Options) {
        unsafe { wsel_setResourceOptions(self, value) }
    }

    #[inline]
    pub fn cpu_cache_mode(&self) -> resource::CPUCacheMode {
        unsafe { rsel_cpuCacheMode(self) }
    }

    #[inline]
    pub fn set_cpu_cache_mode(&mut self, value: resource::CPUCacheMode) {
        unsafe { wsel_setCpuCacheMode(self, value) }
    }

    #[inline]
    pub fn usage(&self) -> Usage {
        unsafe { rsel_usage(self) }
    }

    #[inline]
    pub fn set_usage(&mut self, value: Usage) {
        unsafe { wsel_setUsage(self, value) }
    }

    #[inline]
    pub fn allow_gpu_optimized_contents(&self) -> bool {
        unsafe { rsel_allowGPUOptimizedContents(self) }
    }

    #[inline]
    pub fn set_allow_gpu_optimized_contents(&mut self, value: bool) {
        unsafe { wsel_setAllowGPUOptimizedContents(self, value) }
    }

    // #[inline]
    // pub fn compression_type(&self) -> CompressionType {
    //     unsafe { MTLTextureDescriptor_rsel_compressionType(self) }
    // }

    // #[inline]
    // pub fn set_compression_type(&mut self, value: CompressionType) {
    //     unsafe { MTLTextureDescriptor_wsel_setCompressionType(self, value) }
    // }

    #[inline]
    pub fn swizzle(&self) -> SwizzleChannels {
        unsafe { rsel_swizzle(self) }
    }

    #[inline]
    pub fn set_swizzle(&mut self, value: SwizzleChannels) {
        unsafe { wsel_setSwizzle(self, value) }
    }
}

define_obj_type!(Texture(Resource));

/// ```
/// use cidre::mtl;
///
/// let device = mtl::Device::default().unwrap();
///
/// let td = mtl::TextureDescriptor::new_2d_with_pixel_format(mtl::PixelFormat::A8Unorm, 100, 200, false);
///
/// let t = device.texture_with_descriptor(&td).unwrap();
///
/// assert_eq!(t.width(), 100);
/// assert_eq!(t.height(), 200);
/// assert_eq!(t.depth(), 1);
///
/// ```
impl Texture {
    define_mtl!(width, height, depth);

    /// ```rust
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let td = mtl::TextureDescriptor::new_2d_with_pixel_format(mtl::PixelFormat::A8Unorm, 100, 200, false);
    ///
    /// let t = device.texture_with_descriptor(&td).unwrap();
    ///
    /// assert!(t.parent_texture().is_none());
    /// assert!(t.io_surface().is_none());
    /// assert_eq!(t.texture_type(), mtl::texture::Type::_2D);
    ///
    /// let tv = t.texture_view_with_pixel_format(mtl::PixelFormat::A8Unorm).unwrap();
    ///
    /// assert!(tv.parent_texture().is_some());
    /// assert_eq!(tv.width(), 100);
    /// assert_eq!(tv.height(), 200);
    ///
    /// ```
    #[inline]
    pub fn parent_texture(&self) -> Option<&Texture> {
        unsafe { rsel_parentTexture(self) }
    }

    #[inline]
    pub fn texture_view_with_pixel_format(
        &self,
        pixel_format: PixelFormat,
    ) -> Option<Retained<Texture>> {
        unsafe { rsel_newTextureViewWithPixelFormat(self, pixel_format) }
    }

    #[inline]
    pub fn io_surface(&self) -> Option<&io::Surface> {
        unsafe { rsel_iosurface(self) }
    }

    #[inline]
    pub fn io_surface_plane(&self) -> usize {
        unsafe { rsel_iosurfacePlane(self) }
    }

    #[inline]
    pub fn texture_type(&self) -> Type {
        unsafe { rsel_textureType(self) }
    }

    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { rsel_pixelFormat(self) }
    }

    #[inline]
    pub fn gpu_resouce_id(&self) -> ResourceID {
        msg_send!("mtl", self, sel_gpuResourceID)
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLTextureDescriptor_texture2DDescriptorWithPixelFormat_width_height_mipmapped(
        pixel_format: PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> Retained<Descriptor>;
    fn MTLTextureDescriptor_textureCubeDescriptorWithPixelFormat_size_mipmapped(
        pixel_format: PixelFormat,
        size: usize,
        mipmapped: bool,
    ) -> Retained<Descriptor>;
    fn MTLTextureDescriptor_texture2DDescriptorWithPixelFormat_width_resourceOptions_usage(
        pixel_format: PixelFormat,
        width: usize,
        resource_options: crate::mtl::resource::Options,
        usage: Usage,
    ) -> Retained<Descriptor>;

    fn rsel_textureType(id: &Id) -> Type;
    fn wsel_textureType(id: &mut Id, value: Type);

    fn rsel_pixelFormat(id: &Id) -> PixelFormat;
    fn wsel_setPixelFormat(id: &mut Id, value: PixelFormat);

    fn rsel_mipmapLevelCount(id: &Id) -> usize;
    fn wsel_setMipmapLevelCount(id: &mut Id, value: usize);

    fn MTLTextureDescriptor_rsel_sampleCount(id: &Id) -> usize;
    fn MTLTextureDescriptor_wsel_setSampleCount(id: &mut Id, value: usize);

    fn rsel_arrayLength(id: &Id) -> usize;
    fn wsel_setArrayLength(id: &mut Id, value: usize);

    fn rsel_resourceOptions(id: &Id) -> resource::Options;
    fn wsel_setResourceOptions(id: &mut Id, value: resource::Options);

    fn rsel_cpuCacheMode(id: &Id) -> resource::CPUCacheMode;
    fn wsel_setCpuCacheMode(id: &mut Id, value: resource::CPUCacheMode);

    fn rsel_usage(id: &Id) -> Usage;
    fn wsel_setUsage(id: &mut Id, value: Usage);

    fn rsel_allowGPUOptimizedContents(id: &Id) -> bool;
    fn wsel_setAllowGPUOptimizedContents(id: &mut Id, value: bool);

    // fn MTLTextureDescriptor_rsel_compressionType(id: &Id) -> CompressionType;
    // fn MTLTextureDescriptor_wsel_setCompressionType(id: &Id, value: CompressionType);

    fn rsel_swizzle(id: &Id) -> SwizzleChannels;
    fn wsel_setSwizzle(id: &mut Id, value: SwizzleChannels);

    fn rsel_parentTexture(id: &Texture) -> Option<&Texture>;

    fn rsel_newTextureViewWithPixelFormat(
        id: &Texture,
        pixel_format: PixelFormat,
    ) -> Option<Retained<Texture>>;

    fn rsel_iosurface(id: &Texture) -> Option<&io::Surface>;
    fn rsel_iosurfacePlane(id: &Texture) -> usize;
}
