use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIContentUnavailableTextProperties")]
    pub ContentUnavailableTextProps(ns::Id)
);

impl ContentUnavailableTextProps {
    #[objc::msg_send(font)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn font(&self) -> arc::R<ui::Font>;

    #[objc::msg_send(setFont:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_font(&mut self, val: &ui::Font);

    #[objc::msg_send(color)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn color(&self) -> arc::R<ui::Color>;

    #[objc::msg_send(setColor:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_color(&mut self, val: &ui::Color);

    #[objc::msg_send(setNumberOfLines:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_number_of_lines(&mut self, val: ns::Integer);

    #[objc::msg_send(setLineBreakMode:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_line_break_mode(&mut self, val: ui::LineBreakMode);
}
