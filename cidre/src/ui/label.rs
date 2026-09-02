use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UILabel")]
    pub Label(ui::View),
    UI_LABEL
);

impl arc::A<Label> {
    #[objc::msg_send(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<Label>;
}

impl Label {
    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(text)]
    pub fn text(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setText:)]
    pub fn set_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(font)]
    pub fn font(&self) -> arc::R<ui::Font>;

    #[objc::msg_send(setFont:)]
    pub fn set_font(&mut self, val: Option<&ui::Font>);

    #[objc::msg_send(textColor)]
    pub fn text_color(&self) -> arc::R<ui::Color>;

    #[objc::msg_send(setTextColor:)]
    pub fn set_text_color(&mut self, val: Option<&ui::Color>);

    /// 0 means unlimited.
    #[objc::msg_send(numberOfLines)]
    pub fn number_of_lines(&self) -> ns::Integer;

    #[objc::msg_send(setNumberOfLines:)]
    pub fn set_number_of_lines(&mut self, val: ns::Integer);

    #[objc::msg_send(adjustsFontSizeToFitWidth)]
    pub fn adjusts_font_size_to_fit_width(&self) -> bool;

    #[objc::msg_send(setAdjustsFontSizeToFitWidth:)]
    pub fn set_adjusts_font_size_to_fit_width(&mut self, val: bool);

    /// `NSTextAlignment`: 0 left, 1 center, 2 right, 3 justified, 4 natural.
    #[objc::msg_send(textAlignment)]
    pub fn text_alignment(&self) -> ns::Integer;

    #[objc::msg_send(setTextAlignment:)]
    pub fn set_text_alignment(&mut self, val: ns::Integer);
}

unsafe extern "C" {
    static UI_LABEL: &'static objc::Class<Label>;
}
