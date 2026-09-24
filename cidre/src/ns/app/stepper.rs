use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSStepper")]
    pub Stepper(ns::Control),
    NS_STEPPER
);

impl Stepper {
    #[objc::msg_send(minValue)]
    pub fn min_value(&self) -> f64;

    #[objc::msg_send(setMinValue:)]
    pub fn set_min_value(&mut self, val: f64);

    #[objc::msg_send(maxValue)]
    pub fn max_value(&self) -> f64;

    #[objc::msg_send(setMaxValue:)]
    pub fn set_max_value(&mut self, val: f64);

    /// How much a click changes the value.
    #[objc::msg_send(increment)]
    pub fn increment(&self) -> f64;

    #[objc::msg_send(setIncrement:)]
    pub fn set_increment(&mut self, val: f64);

    /// Whether holding an arrow keeps stepping.
    #[objc::msg_send(autorepeat)]
    pub fn autorepeat(&self) -> bool;

    #[objc::msg_send(setAutorepeat:)]
    pub fn set_autorepeat(&mut self, val: bool);

    /// Whether stepping past one end goes on from the other.
    #[objc::msg_send(valueWraps)]
    pub fn value_wraps(&self) -> bool;

    #[objc::msg_send(setValueWraps:)]
    pub fn set_value_wraps(&mut self, val: bool);
}

impl Stepper {
    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        let mut stepper = Self::new();
        stepper.set_frame(frame);
        stepper
    }
}

unsafe extern "C" {
    static NS_STEPPER: &'static objc::Class<Stepper>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut stepper = ns::Stepper::new();
        stepper.set_min_value(-10.0);
        stepper.set_max_value(10.0);
        stepper.set_increment(2.5);
        stepper.set_autorepeat(true);
        assert_eq!(stepper.increment(), 2.5);
        assert!(stepper.autorepeat());
        assert_eq!(stepper.min_value(), -10.0);
    }
}
