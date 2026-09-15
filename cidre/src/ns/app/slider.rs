use crate::{arc, define_obj_type, ns, objc};

#[doc(alias = "NSSliderType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum SliderType {
    Linear = 0,
    Circular = 1,
}

#[doc(alias = "NSTickMarkPosition")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TickMarkPosition {
    /// Below a horizontal slider, left of a vertical one.
    Below = 0,
    /// Above a horizontal slider, right of a vertical one.
    Above = 1,
}

define_obj_type!(
    #[doc(alias = "NSSlider")]
    pub Slider(ns::Control),
    NS_SLIDER
);

impl Slider {
    #[objc::msg_send(sliderWithTarget:action:)]
    pub fn with_target_action(target: Option<&ns::Id>, action: *const objc::Sel) -> arc::R<Self>;

    #[objc::msg_send(sliderWithValue:minValue:maxValue:target:action:)]
    pub fn with_value_min_max_target_action(
        value: f64,
        min_value: f64,
        max_value: f64,
        target: Option<&ns::Id>,
        action: *const objc::Sel,
    ) -> arc::R<Self>;

    #[objc::msg_send(sliderType)]
    pub fn slider_type(&self) -> SliderType;

    #[objc::msg_send(setSliderType:)]
    pub fn set_slider_type(&mut self, val: SliderType);

    #[objc::msg_send(minValue)]
    pub fn min_value(&self) -> f64;

    #[objc::msg_send(setMinValue:)]
    pub fn set_min_value(&mut self, val: f64);

    #[objc::msg_send(maxValue)]
    pub fn max_value(&self) -> f64;

    #[objc::msg_send(setMaxValue:)]
    pub fn set_max_value(&mut self, val: f64);

    #[objc::msg_send(altIncrementValue)]
    pub fn alt_increment_value(&self) -> f64;

    #[objc::msg_send(setAltIncrementValue:)]
    pub fn set_alt_increment_value(&mut self, val: f64);

    #[objc::msg_send(isVertical)]
    pub fn is_vertical(&self) -> bool;

    #[objc::msg_send(setVertical:)]
    pub fn set_vertical(&mut self, val: bool);

    #[objc::msg_send(trackFillColor)]
    pub fn track_fill_color(&self) -> Option<arc::R<ns::Color>>;

    #[objc::msg_send(setTrackFillColor:)]
    pub fn set_track_fill_color(&mut self, val: Option<&ns::Color>);

    #[objc::msg_send(numberOfTickMarks)]
    pub fn num_tick_marks(&self) -> isize;

    #[objc::msg_send(setNumberOfTickMarks:)]
    pub fn set_num_tick_marks(&mut self, val: isize);

    #[objc::msg_send(tickMarkPosition)]
    pub fn tick_mark_position(&self) -> TickMarkPosition;

    #[objc::msg_send(setTickMarkPosition:)]
    pub fn set_tick_mark_position(&mut self, val: TickMarkPosition);

    #[objc::msg_send(allowsTickMarkValuesOnly)]
    pub fn allows_tick_mark_values_only(&self) -> bool;

    #[objc::msg_send(setAllowsTickMarkValuesOnly:)]
    pub fn set_allows_tick_mark_values_only(&mut self, val: bool);

    #[objc::msg_send(closestTickMarkValueToValue:)]
    pub fn closest_tick_mark_value_to_value(&self, val: f64) -> f64;
}

unsafe extern "C" {
    static NS_SLIDER: &'static objc::Class<Slider>;
}
