use crate::{arc, cg, define_obj_type, ns, objc};

#[cfg(any(target_os = "ios", target_os = "tvos"))]
#[cfg(feature = "ui")]
use crate::ui::Image;

#[cfg(target_os = "macos")]
#[cfg(feature = "app")]
use crate::ns::Image;

define_obj_type!(
    #[doc(alias = "NSTextAttachment")]
    pub TextAttachment(ns::Id),
    NS_TEXT_ATTACHMENT
);

impl arc::A<TextAttachment> {
    #[objc::msg_send(initWithData:ofType:)]
    pub fn init_with_data(
        self,
        data: Option<&ns::Data>,
        type_: Option<&ns::String>,
    ) -> arc::R<TextAttachment>;
}

impl TextAttachment {
    #[inline]
    pub fn with_data(data: Option<&ns::Data>, type_: Option<&ns::String>) -> arc::R<Self> {
        Self::alloc().init_with_data(data, type_)
    }

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    #[cfg(feature = "ui")]
    #[objc::msg_send(textAttachmentWithImage:)]
    pub fn with_image(image: &Image) -> arc::R<Self>;

    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    #[objc::msg_send(setBounds:)]
    pub fn set_bounds(&mut self, val: cg::Rect);

    #[objc::msg_send(fileType)]
    pub fn file_fype(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setFileType:)]
    pub fn set_file_type(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&Image>);

    #[objc::msg_send(textAttachmentViewProviderClassForFileType:)]
    pub fn text_attachment_view_provider_class<'a>(
        file_type: &ns::String,
    ) -> Option<&'a objc::Class<ns::Id>>;

    #[objc::msg_send(registerTextAttachmentViewProviderClass:forFileType:)]
    pub fn register_view_provider_class(cls: &objc::Class<ns::Id>, file_type: &ns::String);

    #[objc::msg_send(allowsTextAttachmentView)]
    pub fn allows_text_attachment_view(&self) -> bool;

    #[objc::msg_send(setAllowsTextAttachmentView:)]
    pub fn set_allows_text_attachment_view(&mut self, val: bool);

    #[objc::msg_send(usesTextAttachmentView)]
    pub fn uses_text_attachment_view(&self) -> bool;
}

impl ns::AttrString {
    #[objc::msg_send(attributedStringWithAttachment:)]
    pub fn with_attachment(attachment: &ns::TextAttachment) -> arc::R<Self>;
}

#[cfg(target_os = "macos")]
#[cfg(feature = "app")]
extern "C" {
    static NS_TEXT_ATTACHMENT: &'static objc::Class<TextAttachment>;
}

#[cfg(any(target_os = "ios", target_os = "tvos"))]
#[cfg(feature = "ui")]
extern "C" {
    static NS_TEXT_ATTACHMENT: &'static objc::Class<TextAttachment>;
}
