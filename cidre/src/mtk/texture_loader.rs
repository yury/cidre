use crate::{arc, blocks, define_cls, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTKTextureLoader")]
    pub TextureLoader(ns::Id)
);

define_obj_type!(
  #[doc(alias = "MTKTextureLoaderOption")]
  pub TextureLoaderOpt(ns::String)
);

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

    #[objc::msg_send(newTextureWithContentsOfURL:options:error:)]
    pub fn new_texture_with_url_err<'ear>(
        &self,
        url: &ns::Url,
        options: Option<&ns::Dictionary<TextureLoaderOpt, ns::Id>>,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl::Texture>>;

    pub fn new_texture_with_url<'ear>(
        &self,
        url: &ns::Url,
        options: Option<&ns::Dictionary<TextureLoaderOpt, ns::Id>>,
    ) -> ns::Result<arc::R<mtl::Texture>> {
        ns::if_none(|err| self.new_texture_with_url_err(url, options, err))
    }
}

#[link(name = "mtk", kind = "static")]
extern "C" {
    static MTK_TEXTURE_LOADER: &'static objc::Class<TextureLoader>;
}

#[cfg(test)]
mod tests {
    use crate::{mtk, mtl, ns};

    #[test]
    fn basics() {
        let device = mtl::Device::sys_default().unwrap();
        let loader = mtk::TextureLoader::with_device(&device);

        let url = ns::Url::with_fs_path_str("unknown.png", false);
        let _err = loader
            .new_texture_with_url(&url, None)
            .expect_err("Should be err");
    }
}
