use crate::{arc, cf, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSImage")]
    pub Image(ns::Id),
    NS_IMAGE
);

impl arc::A<Image> {
    #[objc::msg_send(initWithContentsOfFile:)]
    fn init_with_contents_of_file(self, path: &ns::String) -> Option<arc::R<Image>>;

    #[objc::msg_send(initWithContentsOfURL:)]
    fn init_with_contents_of_url(self, url: &ns::Url) -> Option<arc::R<Image>>;

    #[objc::msg_send(initWithData:)]
    fn init_with_data(self, data: &ns::Data) -> Option<arc::R<Image>>;
}

impl Image {
    #[inline]
    pub fn with_contents_of_file(path: &ns::String) -> Option<arc::R<Self>> {
        Self::alloc().init_with_contents_of_file(path)
    }

    #[inline]
    pub fn with_contents_of_url(url: &ns::Url) -> Option<arc::R<Self>> {
        Self::alloc().init_with_contents_of_url(url)
    }

    #[inline]
    pub fn with_data(data: &ns::Data) -> Option<arc::R<Self>> {
        Self::alloc().init_with_data(data)
    }

    #[objc::cls_msg_send(imageWithSystemSymbolName:accessibilityDescription:)]
    pub fn with_sys_symbol_name_ar(
        name: &ns::String,
        accessibility_description: Option<&ns::String>,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_sys_symbol_name(
        name: &ns::String,
        accessibility_description: Option<&ns::String>,
    ) -> Option<arc::R<Self>>;

    pub fn with_sys_symbol_str(name: &str) -> Option<arc::R<Self>> {
        let str = unsafe { cf::String::from_str_no_copy(name) };
        Self::with_sys_symbol_name(str.as_ns(), None)
    }

    #[objc::msg_send(size)]
    pub fn size(&self) -> ns::Size;

    #[objc::msg_send(setSize:)]
    pub fn set_size(&mut self, val: ns::Size);
}

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_IMAGE: &'static objc::Class<Image>;
}

#[cfg(test)]
mod tests {
    use crate::{cg, ci, ns};

    #[test]
    fn basics() {
        let img = ci::Image::yellow().cropped_to(cg::Rect::new(0.0, 0.0, 100.0, 100.0));
        let ctx = ci::Context::new();
        let tmp_path = ns::Url::with_fs_path_str("/tmp/cidre-test-image.png", false);
        let opts = ns::Dictionary::new();
        ctx.write_png_to_url(
            &img,
            &tmp_path,
            ci::Format::rgba8(),
            &cg::ColorSpace::device_rgb().unwrap(),
            &opts,
        )
        .expect("Written");

        let mut img = ns::Image::with_contents_of_url(&tmp_path).unwrap();
        let size = img.size();
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 100.0);

        img.set_size(cg::Size::new(200.0, 200.0));
        let size = img.size();
        assert_eq!(size.width, 200.0);
        assert_eq!(size.height, 200.0);
    }

    #[test]
    fn sys_symbols() {
        let name = ns::String::with_str("rectangle");
        let _img = ns::Image::with_sys_symbol_name(&name, None).unwrap();
        let _img = ns::Image::with_sys_symbol_str("rectangle").unwrap();
    }
}
