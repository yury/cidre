use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSImageView")]
    pub ImageView(ns::Control),
    NS_IMAGE_VIEW
);

impl ImageView {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<ImageView>;

    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    /// A non-editable image view sized to `image`, ready for auto layout.
    #[objc::msg_send(imageViewWithImage:)]
    #[objc::available(macos = 10.12)]
    pub fn with_image(image: &ns::Image) -> arc::R<Self>;

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ns::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ns::Image>);

    #[objc::msg_send(imageScaling)]
    pub fn image_scaling(&self) -> ns::ImageScaling;

    #[objc::msg_send(setImageScaling:)]
    pub fn set_image_scaling(&mut self, val: ns::ImageScaling);

    #[objc::msg_send(isEditable)]
    pub fn is_editable(&self) -> bool;

    #[objc::msg_send(setEditable:)]
    pub fn set_editable(&mut self, val: bool);

    #[objc::msg_send(animates)]
    pub fn animates(&self) -> bool;

    #[objc::msg_send(setAnimates:)]
    pub fn set_animates(&mut self, val: bool);

    /// The tint applied to template images.
    #[objc::msg_send(contentTintColor)]
    #[objc::available(macos = 10.14)]
    pub fn content_tint_color(&self) -> Option<arc::R<ns::Color>>;

    #[objc::msg_send(setContentTintColor:)]
    #[objc::available(macos = 10.14)]
    pub fn set_content_tint_color(&mut self, val: Option<&ns::Color>);

    #[objc::msg_send(symbolConfiguration)]
    #[objc::available(macos = 11.0)]
    pub fn symbol_cfg(&self) -> Option<arc::R<ns::ImageSymbolCfg>>;

    #[objc::msg_send(setSymbolConfiguration:)]
    #[objc::available(macos = 11.0)]
    pub fn set_symbol_cfg(&mut self, val: Option<&ns::ImageSymbolCfg>);
}

unsafe extern "C" {
    static NS_IMAGE_VIEW: &'static objc::Class<ImageView>;
}
