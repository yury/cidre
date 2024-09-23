use crate::{api, arc, av, define_obj_type, dispatch, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "AVCaptureSlider")]
    pub Slider(av::CaptureControl)
);

impl arc::A<Slider> {
    #[objc::msg_send(initWithLocalizedTitle:symbolName:minValue:maxValue:)]
    pub unsafe fn init_with_range_throws(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        min_val: f32,
        max_val: f32,
    ) -> arc::R<Slider>;

    pub fn init_with_range<'ear>(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        min_val: f32,
        max_val: f32,
    ) -> ns::ExResult<'ear, arc::R<Slider>> {
        ns::try_catch(|| unsafe {
            self.init_with_range_throws(localized_title, symbol_name, min_val, max_val)
        })
    }

    #[objc::msg_send(initWithLocalizedTitle:symbolName:minValue:maxValue:step:)]
    pub unsafe fn init_with_range_step_throws(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        min_val: f32,
        max_val: f32,
        step: f32,
    ) -> arc::R<Slider>;

    pub fn init_with_range_step<'ear>(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        min_val: f32,
        max_val: f32,
        step: f32,
    ) -> ns::ExResult<'ear, arc::R<Slider>> {
        ns::try_catch(|| unsafe {
            self.init_with_range_step_throws(localized_title, symbol_name, min_val, max_val, step)
        })
    }

    #[objc::msg_send(initWithLocalizedTitle:symbolName:values:)]
    pub unsafe fn init_with_vals_throws(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        vals: &ns::Array<ns::Number>,
    ) -> arc::R<Slider>;

    pub fn init_with_vals<'ear>(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        vals: &ns::Array<ns::Number>,
    ) -> ns::ExResult<'ear, arc::R<Slider>> {
        ns::try_catch(|| unsafe { self.init_with_vals_throws(localized_title, symbol_name, vals) })
    }
}

impl Slider {
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    crate::define_cls!(AV_CAPTURE_SLIDER);

    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn with_range<'ear>(
        localized_title: &ns::String,
        symbol_name: &ns::String,
        min_val: f32,
        max_val: f32,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        Self::alloc().init_with_range(localized_title, symbol_name, min_val, max_val)
    }

    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn with_range_step<'ear>(
        localized_title: &ns::String,
        symbol_name: &ns::String,
        min_val: f32,
        max_val: f32,
        step: f32,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        Self::alloc().init_with_range_step(localized_title, symbol_name, min_val, max_val, step)
    }

    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn with_vals<'ear>(
        localized_title: &ns::String,
        symbol_name: &ns::String,
        vals: &ns::Array<ns::Number>,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        Self::alloc().init_with_vals(localized_title, symbol_name, vals)
    }

    #[objc::msg_send(value)]
    pub unsafe fn value_throws(&self) -> f32;

    /// The current value of the slider.
    pub fn value(&self) -> ns::ExResult<f32> {
        ns::try_catch(|| unsafe { self.value_throws() })
    }

    #[objc::msg_send(setValue:)]
    pub unsafe fn set_value_throws(&mut self, val: f32);

    pub fn set_value(&mut self, val: f32) -> ns::ExResult {
        ns::try_catch(|| unsafe { self.set_value_throws(val) })
    }

    #[objc::msg_send(localizedValueFormat)]
    pub fn localized_value_format(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLocalizedValueFormat:)]
    pub fn set_localized_value_format(&mut self, val: Option<&ns::String>);

    /// Values in this array may receive unique visual representations or behaviors.
    #[objc::msg_send(prominentValues)]
    pub fn prominent_values(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(setProminentValues:)]
    pub fn set_prominent_values(&mut self, val: &ns::Array<ns::Number>);

    #[objc::msg_send(localizedTitle)]
    pub fn localized_title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(symbolName)]
    pub fn symbol_name(&self) -> arc::R<ns::String>;

    ///  A string that identifies the slider.
    #[objc::msg_send(accessibilityIdentifier)]
    pub fn accessibility_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setAccessibilityIdentifier:)]
    pub fn set_accessibility_id(&mut self, val: Option<&ns::String>);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setActionQueue:action:)]
    pub fn set_queue_action(
        &mut self,
        queue: &dispatch::Queue,
        action: &mut blocks::EscBlock<fn(new_value: f32)>,
    );
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_SLIDER: &'static objc::Class<Slider>;
}
