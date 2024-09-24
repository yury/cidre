use crate::{arc, cg, ci, define_obj_type, ns, objc};

define_obj_type!(
    /// An evaluation context for rendering image processing results and performing image analysis.
    #[doc(alias = "CIContext")]
    pub Context(ns::Id),
    CI_CONTEXT
);

impl arc::A<Context> {
    #[objc::msg_send(initWithOptions:)]
    pub fn init_with_opts(
        self,
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<Context>>;
}

impl Context {
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

#[link(name = "ci", kind = "static")]
extern "C" {
    static CI_CONTEXT: &'static objc::Class<Context>;
}

#[cfg(test)]
mod tests {
    use crate::{cg, ci, ns, objc::ar_pool};

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
