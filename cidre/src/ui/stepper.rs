use crate::{arc, cg, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIStepper")]
    pub Stepper(ui::Control),
    UI_STEPPER
);

impl Stepper {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<Stepper>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(isContinuous)]
    pub fn is_continuous(&self) -> bool;

    #[objc::msg_send(setContinuous:)]
    pub fn set_continuous(&mut self, val: bool);

    #[objc::msg_send(autorepeat)]
    pub fn autorepeat(&self) -> bool;

    #[objc::msg_send(setAutorepeat:)]
    pub fn set_autorepeat(&mut self, val: bool);

    #[objc::msg_send(wraps)]
    pub fn wraps(&self) -> bool;

    #[objc::msg_send(setWraps:)]
    pub fn set_wraps(&mut self, val: bool);

    #[objc::msg_send(value)]
    pub fn value(&self) -> f64;

    #[objc::msg_send(setValue:)]
    pub fn set_value(&mut self, val: f64);

    #[objc::msg_send(minimumValue)]
    pub fn min_value(&self) -> f64;

    #[objc::msg_send(setMinimumValue:)]
    pub fn set_min_value(&mut self, val: f64);

    #[objc::msg_send(maximumValue)]
    pub fn max_value(&self) -> f64;

    #[objc::msg_send(setMaximumValue:)]
    pub fn set_max_value(&mut self, val: f64);

    #[objc::msg_send(stepValue)]
    pub fn step_value(&self) -> f64;

    #[objc::msg_send(setStepValue:)]
    pub fn set_step_value(&mut self, val: f64);
}

unsafe extern "C" {
    static UI_STEPPER: &'static objc::Class<Stepper>;
}
