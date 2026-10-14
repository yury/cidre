use crate::{arc, cg, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIImageView")]
    pub ImageView(ui::View),
    sym UIImageView
);

impl arc::A<ImageView> {
    #[objc::msg_send(initWithImage:)]
    pub fn init_with_image(self, image: Option<&ui::Image>) -> arc::R<ImageView>;

    #[objc::msg_send(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<ImageView>;
}

impl ImageView {
    #[inline]
    pub fn with_image(image: Option<&ui::Image>) -> arc::R<Self> {
        Self::alloc().init_with_image(image)
    }

    #[inline]
    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ui::Image>);

    #[objc::msg_send(preferredSymbolConfiguration)]
    pub fn preferred_symbol_cfg(&self) -> Option<arc::R<ui::ImageSymbolCfg>>;

    #[objc::msg_send(setPreferredSymbolConfiguration:)]
    pub fn set_preferred_symbol_cfg(&mut self, val: Option<&ui::ImageSymbolCfg>);
}

#[cfg(test)]
mod tests {
    use crate::ui;

    #[test]
    fn basics() {
        let image = ui::Image::with_sys_name(crate::ns::str!(c"power"));
        let mut view = ui::ImageView::with_image(image.as_deref());
        assert!(view.image().is_some());
        view.set_image(None);
        assert!(view.image().is_none());
    }
}
