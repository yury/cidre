use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSButton")]
    pub Button(ns::Control),
    NS_BUTTON
);

impl Button {
    #[objc::msg_send(buttonWithTitle:image:target:action:)]
    pub fn with_title_image(
        title: &ns::String,
        image: &ns::Image,
        target: Option<&ns::Id>,
        action: *const objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(buttonWithTitle:target:action:)]
    pub fn with_title(
        title: &ns::String,
        target: Option<&ns::Id>,
        action: *const objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(buttonWithImage:target:action:)]
    pub fn with_image(
        image: &ns::Image,
        target: Option<&ns::Id>,
        action: *const objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(checkboxWithTitle:target:action:)]
    pub fn checkbox(
        title: &ns::String,
        target: Option<&ns::Id>,
        action: *const objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(radioButtonWithTitle:target:action:)]
    pub fn radio_button(
        title: &ns::String,
        target: Option<&ns::Id>,
        action: *const objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(setButtonType:)]
    pub fn set_button_type(&mut self, val: ns::ButtonType);

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);

    #[objc::msg_send(attributedTitle)]
    pub fn attr_title(&self) -> arc::R<ns::AttrString>;

    #[objc::msg_send(setAttributedTitle:)]
    pub fn set_attr_title(&mut self, val: &ns::AttrString);

    #[objc::msg_send(alternateTitle)]
    pub fn alt_title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setAlternateTitle:)]
    pub fn set_alt_title(&mut self, val: &ns::String);

    #[objc::msg_send(attributedAlternateTitle)]
    pub fn attr_alt_title(&self) -> arc::R<ns::AttrString>;

    #[objc::msg_send(setAttributedAlternateTitle:)]
    pub fn set_attr_alt_title(&mut self, val: &ns::AttrString);

    #[objc::msg_send(hasDestructiveAction)]
    pub fn has_destructive_action(&self) -> bool;

    #[objc::msg_send(setHasDestructiveAction:)]
    pub fn set_has_destructive_action(&mut self, val: bool);

    #[objc::msg_send(sound)]
    pub fn sound(&self) -> Option<arc::R<ns::Sound>>;

    #[objc::msg_send(setSound:)]
    pub fn set_sound(&mut self, val: Option<&ns::Sound>);

    #[objc::msg_send(isSpringLoaded)]
    pub fn is_spring_loaded(&self) -> bool;

    #[objc::msg_send(setSpringLoaded:)]
    pub fn set_spring_loaded(&mut self, val: bool);

    #[objc::msg_send(maxAcceleratorLevel)]
    pub fn max_accelerator_level(&self) -> isize;

    #[objc::msg_send(setMaxAcceleratorLevel:)]
    pub fn set_max_accelerator_level(&mut self, val: isize);

    #[objc::msg_send(setPeriodicDelay:interval:)]
    pub fn set_periodic_delay_interval(&mut self, delay: f32, interval: f32);

    #[objc::msg_send(getPeriodicDelay:interval:)]
    pub fn get_periodic_delay_interval(&self, delay: &mut f32, interval: &mut f32);

    #[objc::msg_send(bezelStyle)]
    pub fn bezel_style(&self) -> ns::BezelStyle;

    #[objc::msg_send(setBezelStyle:)]
    pub fn set_bezel_style(&mut self, val: ns::BezelStyle);

    #[objc::msg_send(isBordered)]
    pub fn is_bordered(&self) -> bool;

    #[objc::msg_send(setBordered:)]
    pub fn set_bordered(&mut self, val: bool);

    #[objc::msg_send(isTransparent)]
    pub fn is_transparent(&self) -> bool;

    #[objc::msg_send(setTransparent:)]
    pub fn set_transparent(&mut self, val: bool);

    #[objc::msg_send(showsBorderOnlyWhileMouseInside)]
    pub fn shows_border_only_while_mouse_inside(&self) -> bool;

    #[objc::msg_send(setShowsBorderOnlyWhileMouseInside:)]
    pub fn set_shows_border_only_while_mouse_inside(&mut self, val: bool);

    #[objc::msg_send(bezelColor)]
    pub fn bezel_color(&self) -> Option<arc::R<ns::Color>>;

    #[objc::msg_send(setBezelColor:)]
    pub fn set_bezel_color(&mut self, val: Option<&ns::Color>);

    #[objc::msg_send(contentTintColor)]
    pub fn content_tint_color(&self) -> Option<arc::R<ns::Color>>;

    #[objc::msg_send(setContentTintColor:)]
    pub fn set_content_tint_color(&mut self, val: Option<&ns::Color>);
}

/// Configuring Button Images
impl Button {
    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ns::Image>>;
    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ns::Image>);
    #[objc::msg_send(alternateImage)]
    pub fn alt_image(&self) -> Option<arc::R<ns::Image>>;
    #[objc::msg_send(setAlternateImage:)]
    pub fn set_alt_image(&mut self, val: Option<&ns::Image>);

    #[objc::msg_send(imagePosition)]
    pub fn image_pos(&self) -> ns::CellImagePos;

    #[objc::msg_send(setImagePosition:)]
    pub fn set_image_pos(&mut self, val: ns::CellImagePos);

    #[objc::msg_send(imageScaling)]
    pub fn image_scaling(&self) -> ns::ImageScaling;

    #[objc::msg_send(setImageScaling:)]
    pub fn set_image_scaling(&mut self, val: ns::ImageScaling);

    #[objc::msg_send(imageHugsTitle)]
    pub fn image_hugs_title(&self) -> bool;

    #[objc::msg_send(setImageHugsTitle:)]
    pub fn set_image_hugs_title(&mut self, val: bool);

    #[objc::msg_send(symbolConfiguration)]
    pub fn symbol_cfg(&self) -> Option<arc::R<ns::ImageSymbolCfg>>;

    #[objc::msg_send(setSymbolConfiguration:)]
    pub fn set_symbol_cfg(&mut self, val: Option<&ns::ImageSymbolCfg>);
}

/// Managing Button State
impl Button {
    #[objc::msg_send(state)]
    pub fn state(&self) -> ns::ControlStateValue;

    #[objc::msg_send(setState:)]
    pub fn set_state(&mut self, val: ns::ControlStateValue);

    #[objc::msg_send(allowsMixedState)]
    pub fn allows_mixed_state(&self) -> bool;

    #[objc::msg_send(setAllowsMixedState:)]
    pub fn set_allows_mixed_state(&mut self, val: bool);

    #[objc::msg_send(setNextState)]
    pub fn set_next_state(&mut self);

    #[objc::msg_send(highlight:)]
    pub fn highlight(&mut self, val: bool);
}

/// Handling Keyboard Events
impl Button {
    #[objc::msg_send(keyEquivalent)]
    pub fn key_equivalent(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setKeyEquivalent:)]
    pub fn set_key_equivalent(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(keyEquivalentModifierMask)]
    pub fn key_equivalent_modifier_mask(&self) -> ns::EventModifierFlags;

    #[objc::msg_send(setKeyEquivalentModifierMask:)]
    pub fn set_key_equivalent_modifier_mask(&mut self, val: ns::EventModifierFlags);

    #[objc::msg_send(performKeyEquivalent:)]
    pub fn perform_key_equivalent(&mut self, key: &ns::Event) -> bool;
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_BUTTON: &'static objc::Class<Button>;
}

// #[cfg(test)]
// mod tests {
// use crate::ns;

// #[test]
// fn basics() {
//     let btn = ns::Button::with_title(ns::str!(c"Hello"), None, std::ptr::null());
//     assert_eq!(btn.title().as_ref(), "Hello");
// }
// }
