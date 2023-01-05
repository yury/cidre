use crate::{arc, cf, cg, ci, define_obj_type, ns};

define_obj_type!(Context(ns::Id));

impl Context {
    #[inline]
    pub fn with_options(options: Option<&cf::Dictionary>) -> Option<arc::R<Self>> {
        unsafe { CIContext_contextWithOptions(options) }
    }

    #[inline]
    pub fn new() -> Option<arc::R<Self>> {
        Self::with_options(None)
    }

    pub fn write_png_to_url<'ar>(
        &self,
        image: &ci::Image,
        url: &cf::URL,
        format: ci::Format,
        color_space: &cg::ColorSpace,
        options: &cf::Dictionary,
    ) -> Result<(), &'ar cf::Error> {
        unsafe {
            let mut error = None;
            let res = rsel_writePNGRepresentationOfImage_toURL_format_colorSpace_options_error(
                self,
                image,
                url,
                format,
                color_space,
                options,
                &mut error,
            );

            if res {
                Ok(())
            } else {
                Err(error.unwrap())
            }
        }
    }
}

#[link(name = "ci", kind = "static")]
extern "C" {
    fn CIContext_contextWithOptions(options: Option<&cf::Dictionary>) -> Option<arc::R<Context>>;

    fn rsel_writePNGRepresentationOfImage_toURL_format_colorSpace_options_error<'ar>(
        context: &ns::Id,
        image: &ci::Image,
        url: &cf::URL,
        format: ci::Format,
        color_space: &cg::ColorSpace,
        options: &cf::Dictionary,
        error: &mut Option<&'ar cf::Error>,
    ) -> bool;
}
