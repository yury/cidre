use crate::{arc, blocks, cf, define_cls, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTKTextureLoader")]
    pub TextureLoader(ns::Id)
);

define_obj_type!(
  #[doc(alias = "MTKTextureLoaderOption")]
  pub TextureLoaderOpt(ns::String)
);

impl TextureLoaderOpt {
    /// If the boolean value specified with this string is true, the resulting Metal texture will have
    /// been created with mipmaps whose contents are undefined. It is the responsibility of the caller to
    /// fill out the contents of the mipmap data unless MTLTextureLoaderOptionGenerateMipmaps is specified.
    /// If the file being loaded contains data for mipmaps (such as in a PVR or KTX file) this option does
    ///  not need to be specified. In those cases the mipmap memory will be allocated and the image data loaded.
    #[doc(alias = "MTKTextureLoaderOptionAllocateMipmaps")]
    #[inline]
    pub fn allocate_mipmaps() -> &'static Self {
        unsafe { MTKTextureLoaderOptionAllocateMipmaps }
    }

    /// If the boolean value specified with this string is true, the resulting Metal texture
    /// will be created with mipmaps. If the file being loaded contains data for mipmaps (such as in a PVR or KTX file),
    /// specifying this option will overwrite the existing mipmap data in the loaded texture.
    /// This option can only be used if the pixel format of the texture is color filterable and color renderable.
    /// This option implies MTKTextureLoaderOptionAllocateMipmaps. Specifying this option will cause the mtk::TextureLoader
    /// to submit work to the GPU on behalf of the caller.
    #[doc(alias = "MTKTextureLoaderOptionGenerateMipmaps")]
    #[inline]
    pub fn generate_mipmaps() -> &'static Self {
        unsafe { MTKTextureLoaderOptionGenerateMipmaps }
    }

    /// If the boolean value specified with this string is true, the texture will be created with an sRGB pixel format
    /// regardless of whether the image file specifies that the data has already been gamma corrected.
    /// Likewise, if false, the texture will be created with a non-sRGB pixel format regardless of whether
    /// the image file specifies that the data has been gamma corrected. To use the sRGB information specified
    /// in the file, do not specify this in the options dictionary.
    ///
    /// When deploying to OS's prior to macOS 10.15 / iOS 13.0, this option is ignored for loading KTX textures.
    #[doc(alias = "MTKTextureLoaderOptionSRGB")]
    #[inline]
    pub fn srgb() -> &'static Self {
        unsafe { MTKTextureLoaderOptionSRGB }
    }

    /// Identifier to be used with an ns::Number specifying the mtl::TextureUsage flags
    ///
    /// The resulting Metal texture will be created with the mtl::TextureUsage flags indicated
    /// by the nsNumber associated with this string.
    #[doc(alias = "MTKTextureLoaderOptionTextureUsage")]
    pub fn texture_usage() -> &'static Self {
        unsafe { MTKTextureLoaderOptionTextureUsage }
    }

    /// Identifier to be used with an ns::Number specifying the mtl::CpuCacheMode
    ///
    /// The resulting Metal texture will be created with the mtl::CpuCacheMode indicated
    /// by the ns::Number associated with this string.
    #[doc(alias = "MTKTextureLoaderOptionTextureCPUCacheMode")]
    pub fn texture_cpu_cache_mode() -> &'static Self {
        unsafe { MTKTextureLoaderOptionTextureCPUCacheMode }
    }

    /// Identifier to be used with an ns::Number specifying the mtl::StorageMode
    ///
    /// The resulting Metal texture will be created with the mtl::StorageMode indicated
    /// by the ns::Number associated with this string. If this option is omitted, the texture will be created
    /// with the default storage mode for Metal textures: mtl::StorageModeShared on iOS, and mtl::StorageModeManaged on OS X.
    /// Specifying this option with mtl::TextureStorageModePrivate cause the mtk::TextureLoader to submit work to the GPU
    /// on behalf of the caller.
    #[doc(alias = "MTKTextureLoaderOptionTextureStorageMode")]
    pub fn texture_storage_mode() -> &'static Self {
        unsafe { MTKTextureLoaderOptionTextureStorageMode }
    }

    /// dentifier to be used in an options ns::Dictionary with an mkt::TextureLoaderOrigin ns::String specifying whether to flip textures vertically
    #[doc(alias = "MTKTextureLoaderOptionOrigin")]
    pub fn origin() -> &'static Self {
        unsafe { MTKTextureLoaderOptionOrigin }
    }

    /// Identifier specifying that the texture should be loaded as an array texture when possible.
    ///
    /// Type is an ns::Number with a boolean value.
    #[doc(alias = "MTKTextureLoaderOptionLoadAsArray")]
    pub fn load_as_array() -> &'static Self {
        unsafe { MTKTextureLoaderOptionLoadAsArray }
    }
}

define_obj_type!(
    #[doc(alias = "MTKTextureLoaderCubeLayout")]
    pub TextureLoaderCubeLayout(ns::String)
);

impl TextureLoaderCubeLayout {
    #[doc(alias = "MTKTextureLoaderCubeLayoutVertical")]
    #[inline]
    pub fn vertical() -> &'static Self {
        unsafe { MTKTextureLoaderCubeLayoutVertical }
    }
}

define_obj_type!(
    #[doc(alias = "MTKTextureLoaderOrigin")]
    pub TextureLoaderOrigin(ns::String)
);

impl TextureLoaderOrigin {
    #[doc(alias = "MTKTextureLoaderOriginTopLeft")]
    #[inline]
    pub fn top_left() -> &'static Self {
        unsafe { MTKTextureLoaderOriginTopLeft }
    }

    #[doc(alias = "MTKTextureLoaderOriginBottomLeft")]
    #[inline]
    pub fn bottom_left() -> &'static Self {
        unsafe { MTKTextureLoaderOriginBottomLeft }
    }

    #[doc(alias = "MTKTextureLoaderOriginFlippedVertically")]
    #[inline]
    pub fn flipped_vertically() -> &'static Self {
        unsafe { MTKTextureLoaderOriginFlippedVertically }
    }
}

#[doc(alias = "MTKTextureLoaderCallback")]
pub type TextureLoaderCb = blocks::ResultCompletionHandler<mtl::Texture>;

#[doc(alias = "MTKTextureLoaderArrayCallback")]
pub type TextureLoaderArrayCb = blocks::ResultCompletionHandler<ns::Array<mtl::Texture>>;

impl arc::A<TextureLoader> {
    #[objc::msg_send(initWithDevice:)]
    pub fn init_with_device(self, device: &mtl::Device) -> arc::R<TextureLoader>;
}

impl TextureLoader {
    define_cls!(MTK_TEXTURE_LOADER);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    pub fn with_device(device: &mtl::Device) -> arc::R<Self> {
        Self::alloc().init_with_device(device)
    }

    /// Synchronously create a Metal texture and load image data from the file at URL
    #[objc::msg_send(newTextureWithContentsOfURL:options:error:)]
    pub fn new_texture_with_url_err<'ear>(
        &self,
        url: &ns::Url,
        options: Option<&ns::Dictionary<TextureLoaderOpt, ns::Id>>,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl::Texture>>;

    /// Synchronously create a Metal texture and load image data from the file at URL
    pub fn new_texture_with_url<'ear>(
        &self,
        url: &ns::Url,
        options: Option<&ns::Dictionary<TextureLoaderOpt, ns::Id>>,
    ) -> ns::Result<arc::R<mtl::Texture>> {
        ns::if_none(|err| self.new_texture_with_url_err(url, options, err))
    }

    #[objc::msg_send(newTexturesWithContentsOfURLs:options:error:)]
    pub fn new_textures_with_urls_err<'ear>(
        &self,
        urls: &ns::Array<ns::Url>,
        options: Option<&ns::Dictionary<TextureLoaderOpt, ns::Id>>,
        error: *mut Option<&'ear ns::Error>,
    ) -> arc::R<ns::Array<OptionTexture>>;

    pub fn new_textures_with_urls<'ear>(
        &self,
        urls: &ns::Array<ns::Url>,
        options: Option<&ns::Dictionary<TextureLoaderOpt, ns::Id>>,
    ) -> (arc::R<ns::Array<OptionTexture>>, Option<&'ear ns::Error>) {
        let mut err = None;
        let textures = self.new_textures_with_urls_err(urls, options, &mut err);
        (textures, err)
    }

    /// The key used to retrieve an error string from an error object’s user_info() dictionary.
    #[doc(alias = "MTKTextureLoaderErrorKey")]
    #[inline]
    pub fn error_key() -> &'static ns::String {
        unsafe { MTKTextureLoaderErrorKey }
    }
}

define_obj_type!(
    pub OptionTexture(ns::Id)
);

impl OptionTexture {
    pub fn is_none(&self) -> bool {
        unsafe { cf::Null::value().as_type_ptr() == self.as_type_ref().as_type_ptr() }
    }

    pub fn as_texture(&self) -> Option<&mtl::Texture> {
        if self.is_none() {
            None
        } else {
            Some(unsafe { std::mem::transmute(self) })
        }
    }
}

#[link(name = "mtk", kind = "static")]
extern "C" {
    static MTK_TEXTURE_LOADER: &'static objc::Class<TextureLoader>;
}

impl ns::ErrorDomain {
    #[doc(alias = "MTKTextureLoaderErrorDomain")]
    pub fn texture_loader() -> &'static Self {
        unsafe { MTKTextureLoaderErrorDomain }
    }
}

#[link(name = "MetalKit", kind = "framework")]
extern "C" {
    static MTKTextureLoaderErrorDomain: &'static ns::ErrorDomain;
    static MTKTextureLoaderErrorKey: &'static ns::String;

    static MTKTextureLoaderOptionAllocateMipmaps: &'static TextureLoaderOpt;
    static MTKTextureLoaderOptionGenerateMipmaps: &'static TextureLoaderOpt;
    static MTKTextureLoaderOptionSRGB: &'static TextureLoaderOpt;
    static MTKTextureLoaderOptionTextureUsage: &'static TextureLoaderOpt;
    static MTKTextureLoaderOptionTextureCPUCacheMode: &'static TextureLoaderOpt;
    static MTKTextureLoaderOptionTextureStorageMode: &'static TextureLoaderOpt;
    static MTKTextureLoaderOptionOrigin: &'static TextureLoaderOpt;
    static MTKTextureLoaderOptionLoadAsArray: &'static TextureLoaderOpt;

    static MTKTextureLoaderCubeLayoutVertical: &'static TextureLoaderCubeLayout;

    static MTKTextureLoaderOriginTopLeft: &'static TextureLoaderOrigin;
    static MTKTextureLoaderOriginBottomLeft: &'static TextureLoaderOrigin;
    static MTKTextureLoaderOriginFlippedVertically: &'static TextureLoaderOrigin;
}

#[cfg(test)]
mod tests {
    use crate::{mtk, mtl, ns};

    #[test]
    fn basics() {
        let device = mtl::Device::sys_default().unwrap();
        let loader = mtk::TextureLoader::with_device(&device);

        let url = ns::Url::with_fs_path_str("unknown.png", false);
        let err = loader
            .new_texture_with_url(&url, None)
            .expect_err("Should be err");
        assert_eq!(err.domain(), ns::ErrorDomain::texture_loader());
        let user_info = err.user_info();
        let _error = user_info.get(mtk::TextureLoader::error_key()).unwrap();
    }

    #[test]
    fn batch() {
        let device = mtl::Device::sys_default().unwrap();
        let loader = mtk::TextureLoader::with_device(&device);

        let url = ns::Url::with_fs_path_str("unknown.png", false);
        let urls = ns::Array::from_slice_retained(&[url]);

        let (textures, err) = loader.new_textures_with_urls(&urls, None);

        assert_eq!(1, textures.len());
        assert!(err.is_some());
        assert!(textures[0].is_none());
        let texture = textures[0].as_texture();
        assert!(texture.is_none());
    }
}
