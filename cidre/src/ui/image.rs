use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIImage")]
    pub Image(ns::Id),
    UI_IMAGE
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
}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_IMAGE: &'static objc::Class<Image>;
}
