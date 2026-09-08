use crate::{arc, cg, ci, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CIContextOption")]
    pub ContextOpt(ns::String)
);

impl ContextOpt {
    #[doc(alias = "kCIContextAllowLowPower")]
    #[inline]
    pub fn allow_low_power() -> &'static Self {
        unsafe { kCIContextAllowLowPower }
    }

    #[doc(alias = "kCIContextHighQualityDownsample")]
    #[inline]
    pub fn high_quality_downsample() -> &'static Self {
        unsafe { kCIContextHighQualityDownsample }
    }

    #[doc(alias = "kCIContextCacheIntermediates")]
    #[inline]
    pub fn cache_intermediates() -> &'static Self {
        unsafe { kCIContextCacheIntermediates }
    }
}

define_obj_type!(
    /// An evaluation context for rendering image processing results and performing image analysis.
    #[doc(alias = "CIContext")]
    pub Context(ns::Id),
    CI_CONTEXT
);

impl Context {
    #[objc::init(initWithOptions:)]
    pub fn init_with_opts(
        self,
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<Context>>;

    #[inline]
    pub fn with_opts(options: Option<&ns::Dictionary<ns::String, ns::Id>>) -> Option<arc::R<Self>> {
        Self::alloc().init_with_opts(options)
    }

    #[objc::msg_send(writePNGRepresentationOfImage:toURL:format:colorSpace:options:error:)]
    pub unsafe fn write_png_to_url_format_colorspace_opts_err<'ear>(
        &self,
        image: &ci::Image,
        url: &ns::Url,
        format: ci::Format,
        color_space: &cg::ColorSpace,
        options: &ns::Dictionary<ns::String, ns::Id>,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn write_png_to_url<'ear>(
        &self,
        image: &ci::Image,
        url: &ns::Url,
        format: ci::Format,
        color_space: &cg::ColorSpace,
        options: &ns::Dictionary<ns::String, ns::Id>,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe {
            self.write_png_to_url_format_colorspace_opts_err(
                image,
                url,
                format,
                color_space,
                options,
                err,
            )
        })
    }
}

/// contextWithMTLDevice
#[cfg(feature = "mtl")]
impl Context {
    #[objc::msg_send(contextWithMTLDevice:)]
    pub fn with_mtl_device(device: &crate::mtl::Device) -> arc::R<Self>;

    #[objc::msg_send(contextWithMTLDevice:options:)]
    pub fn with_mtl_device_opts(
        device: &crate::mtl::Device,
        opts: Option<&ns::Dictionary<ContextOpt, ns::Id>>,
    ) -> arc::R<Self>;
}

unsafe extern "C" {
    static CI_CONTEXT: &'static objc::Class<Context>;
}

unsafe extern "C" {
    static kCIContextAllowLowPower: &'static ContextOpt;
    static kCIContextHighQualityDownsample: &'static ContextOpt;
    static kCIContextCacheIntermediates: &'static ContextOpt;
}

#[cfg(test)]
mod tests {
    use crate::{cg, ci, mtl, ns, objc::ar_pool};

    #[test]
    fn basics() {
        let device = mtl::Device::sys_default().unwrap();
        let _ctx = ci::Context::with_mtl_device(&device);
        let opts = ns::Dictionary::with_keys_values(
            &[ci::ContextOpt::high_quality_downsample()],
            &[ns::Number::with_bool(true).as_id_ref()],
        );
        let _ctx = ci::Context::with_mtl_device(&device);
        let _ctx = ci::Context::with_mtl_device_opts(&device, Some(opts.as_ref()));
    }

    #[test]
    fn error_autorelease() {
        ar_pool(|| {
            let black = ci::Image::black();
            let ctx = ci::Context::with_opts(None).unwrap();
            let url = ns::Url::with_string(ns::str!(c"url")).unwrap();
            let opts = ns::Dictionary::new();
            let _err = ctx
                .write_png_to_url(
                    &black,
                    &url,
                    ci::Format::argb8(),
                    cg::ColorSpace::device_cmyk().unwrap().as_ref(),
                    &opts,
                )
                .expect_err("should fail");
        })
    }
}
