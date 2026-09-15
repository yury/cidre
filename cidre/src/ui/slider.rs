use crate::{arc, cg, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UISlider")]
    pub Slider(ui::Control),
    UI_SLIDER
);

impl Slider {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<Slider>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(value)]
    pub fn value(&self) -> f32;

    #[objc::msg_send(setValue:)]
    pub fn set_value(&mut self, val: f32);

    #[objc::msg_send(setValue:animated:)]
    pub fn set_value_animated(&mut self, val: f32, animated: bool);

    #[objc::msg_send(minimumValue)]
    pub fn min_value(&self) -> f32;

    #[objc::msg_send(setMinimumValue:)]
    pub fn set_min_value(&mut self, val: f32);

    #[objc::msg_send(maximumValue)]
    pub fn max_value(&self) -> f32;

    #[objc::msg_send(setMaximumValue:)]
    pub fn set_max_value(&mut self, val: f32);

    #[objc::msg_send(isContinuous)]
    pub fn is_continuous(&self) -> bool;

    #[objc::msg_send(setContinuous:)]
    pub fn set_continuous(&mut self, val: bool);

    #[objc::msg_send(minimumValueImage)]
    pub fn min_value_image(&self) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(setMinimumValueImage:)]
    pub fn set_min_value_image(&mut self, val: Option<&ui::Image>);

    #[objc::msg_send(maximumValueImage)]
    pub fn max_value_image(&self) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(setMaximumValueImage:)]
    pub fn set_max_value_image(&mut self, val: Option<&ui::Image>);

    #[objc::msg_send(minimumTrackTintColor)]
    pub fn min_track_tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setMinimumTrackTintColor:)]
    pub fn set_min_track_tint_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(maximumTrackTintColor)]
    pub fn max_track_tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setMaximumTrackTintColor:)]
    pub fn set_max_track_tint_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(thumbTintColor)]
    pub fn thumb_tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setThumbTintColor:)]
    pub fn set_thumb_tint_color(&mut self, val: Option<&ui::Color>);
}

unsafe extern "C" {
    static UI_SLIDER: &'static objc::Class<Slider>;
}
