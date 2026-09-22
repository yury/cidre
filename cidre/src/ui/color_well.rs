use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(#[doc(alias = "UIColorWell")] pub ColorWell(ui::Control), UI_COLOR_WELL);
impl ColorWell {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<ColorWell>;
    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }
    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;
    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, value: Option<&ns::String>);
    #[objc::msg_send(supportsAlpha)]
    pub fn supports_alpha(&self) -> bool;
    #[objc::msg_send(setSupportsAlpha:)]
    pub fn set_supports_alpha(&mut self, value: bool);
    #[objc::msg_send(selectedColor)]
    pub fn selected_color(&self) -> Option<arc::R<ui::Color>>;
    #[objc::msg_send(setSelectedColor:)]
    pub fn set_selected_color(&mut self, value: Option<&ui::Color>);
}
unsafe extern "C" {
    static UI_COLOR_WELL: &'static objc::Class<ColorWell>;
}
