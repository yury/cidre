use crate::{arc, cg, ci, define_obj_type, ns, objc};

define_obj_type!(
    /// An evaluation context for rendering image processing results and performing image analysis.
    pub Context(ns::Id),
    CI_CONTEXT
);

impl arc::A<Context> {
    #[objc::msg_send(initWithOptions:)]
    pub fn init_with_options(
        self,
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<Context>>;
}

impl Context {
    #[inline]
    pub fn with_options(
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_options(options)
    }

    #[objc::msg_send(writePNGRepresentationOfImage:toURL:format:colorSpace:options:error:)]
    fn write_png_to_url_format_colorspace_options_err<'ar>(
        &self,
        image: &ci::Image,
        url: &ns::Url,
        format: ci::Format,
        color_space: &cg::ColorSpace,
        options: &ns::Dictionary<ns::String, ns::Id>,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    #[inline]
    pub fn write_png_to_url<'ear>(
        &self,
        image: &ci::Image,
        url: &ns::Url,
        format: ci::Format,
        color_space: &cg::ColorSpace,
        options: &ns::Dictionary<ns::String, ns::Id>,
    ) -> Result<(), &'ear ns::Error> {
        let mut error = None;
        if self.write_png_to_url_format_colorspace_options_err(
            image,
            url,
            format,
            color_space,
            options,
            &mut error,
        ) {
            Ok(())
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }
}

#[link(name = "ci", kind = "static")]
extern "C" {
    static CI_CONTEXT: &'static objc::Class<Context>;
}
