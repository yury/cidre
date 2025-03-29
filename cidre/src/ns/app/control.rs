use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSControl")]
    pub Control(ns::View)
);

impl Control {
    #[objc::msg_send(target)]
    pub fn target(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setTarget:)]
    pub fn set_target(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(action)]
    pub fn action(&self) -> *const objc::Sel;

    #[objc::msg_send(setAction:)]
    pub fn set_action(&mut self, val: *const objc::Sel);

    #[objc::msg_send(tag)]
    pub fn tag(&self) -> isize;

    #[objc::msg_send(setTag:)]
    pub fn set_tag(&mut self, val: isize);

    #[objc::msg_send(ignoresMultiClick)]
    pub fn ignores_multi_click(&self) -> bool;

    #[objc::msg_send(setIgnoresMultiClick:)]
    pub fn set_ignores_multi_click(&mut self, val: bool);

    #[objc::msg_send(isContinuous)]
    pub fn is_continuous(&self) -> bool;

    #[objc::msg_send(setContinuous:)]
    pub fn set_continuous(&mut self, val: bool);

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(refusesFirstResponder)]
    pub fn refuses_first_responder(&self) -> bool;

    #[objc::msg_send(setRefusesFirstResponder:)]
    pub fn set_refuses_first_responder(&mut self, val: bool);

    #[objc::msg_send(isHighlighted)]
    pub fn is_highlighted(&self) -> bool;

    #[objc::msg_send(setHighlighted:)]
    pub fn set_highlighted(&mut self, val: bool);

    #[objc::msg_send(controlSize)]
    pub fn control_size(&self) -> ns::ControlSize;

    #[objc::msg_send(setControlSize:)]
    pub fn set_control_size(&mut self, val: ns::ControlSize);

    #[objc::msg_send(formatter)]
    pub fn formatter(&self) -> Option<arc::R<ns::Formatter>>;

    #[objc::msg_send(setFormatter:)]
    pub fn set_formatter(&mut self, val: Option<&ns::Formatter>);

    #[objc::msg_send(objectValue)]
    pub fn obj_value(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setObjectValue:)]
    pub fn set_obj_value(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(stringValue)]
    pub fn string_value(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setStringValue:)]
    pub fn set_string_value(&mut self, val: &ns::String);

    #[objc::msg_send(attributedStringValue)]
    pub fn attr_string_value(&self) -> arc::R<ns::AttrString>;

    #[objc::msg_send(setAttributedStringValue:)]
    pub fn set_attr_string_value(&mut self, val: &ns::AttrString);

    #[objc::msg_send(intValue)]
    pub fn c_int_value(&self) -> std::ffi::c_int;

    #[objc::msg_send(setIntValue:)]
    pub fn set_c_int_value(&mut self, val: std::ffi::c_int);

    #[objc::msg_send(integerValue)]
    pub fn integer_value(&self) -> ns::Integer;

    #[objc::msg_send(setIntegerValue:)]
    pub fn set_integer_value(&mut self, val: ns::Integer);

    #[objc::msg_send(floatValue)]
    pub fn f32_value(&self) -> f32;

    #[objc::msg_send(setFloatValue:)]
    pub fn set_f32_value(&mut self, val: f32);

    #[objc::msg_send(doubleValue)]
    pub fn f64_value(&self) -> f64;

    #[objc::msg_send(setDoubleValue:)]
    pub fn set_f64_value(&mut self, val: f64);

    #[objc::msg_send(sizeThatFits:)]
    pub fn size_that_fits(&self, size: ns::Size) -> ns::Size;

    #[objc::msg_send(sizeToFit)]
    pub fn size_to_fit(&mut self);

    #[objc::msg_send(sendActionOn:)]
    pub fn send_action_on(&mut self, mask: ns::EventMask) -> ns::Integer;

    #[objc::msg_send(sendAction:to:)]
    pub fn send_action_to(&mut self, action: *const objc::Sel, target: Option<&ns::Id>) -> bool;

    #[objc::msg_send(takeIntValueFrom:)]
    pub fn take_c_int_value_from(&mut self, sender: &ns::Id);

    #[objc::msg_send(takeFloatValueFrom:)]
    pub fn take_f32_value_from(&mut self, sender: &ns::Id);

    #[objc::msg_send(takeDoubleValueFrom:)]
    pub fn take_f64_value_from(&mut self, sender: &ns::Id);

    #[objc::msg_send(takeStringValueFrom:)]
    pub fn take_string_value_from(&mut self, sender: &ns::Id);

    #[objc::msg_send(takeObjectValueFrom:)]
    pub fn take_obj_value_from(&mut self, sender: &ns::Id);

    #[objc::msg_send(takeIntegerValueFrom:)]
    pub fn take_integer_value_from(&mut self, sender: &ns::Id);

    #[objc::msg_send(mouseDown:)]
    pub fn mouse_down(&mut self, event: &ns::Event);

    #[objc::msg_send(performClick:)]
    pub fn perform_click(&mut self, sender: Option<&ns::Id>);

    // #[objc::msg_send(font)]
    // pub fn font(&self) -> Option<arc::R<ns::Font>>;

    // #[objc::msg_send(setFont:)]
    // pub fn set_font(&mut self, val: Option<&ns::Font>);

    // #[objc::msg_send(lineBreakMode)]
    // pub fn line_break_mode(&self) -> ns::LineBreakMode;

    // #[objc::msg_send(setLineBreakMode:)]
    // pub fn set_line_break_mode(&mut self, val: ns::LineBreakMode);
}
