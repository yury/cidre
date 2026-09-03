use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIPanGestureRecognizer")]
    pub PanGestureRecognizer(ui::GestureRecognizer),
    UI_PAN_GESTURE_RECOGNIZER
);

impl PanGestureRecognizer {
    #[objc::init(initWithTarget:action:)]
    pub fn init_with_target_action(
        self,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<PanGestureRecognizer>;

    pub fn with_target_action(target: Option<&ns::Id>, action: Option<&objc::Sel>) -> arc::R<Self> {
        Self::alloc().init_with_target_action(target, action)
    }

    #[objc::msg_send(minimumNumberOfTouches)]
    pub fn min_number_of_touches(&self) -> usize;

    #[objc::msg_send(setMinimumNumberOfTouches:)]
    pub fn set_min_number_of_touches(&mut self, val: usize);

    #[objc::msg_send(maximumNumberOfTouches)]
    pub fn max_number_of_touches(&self) -> usize;

    #[objc::msg_send(setMaximumNumberOfTouches:)]
    pub fn set_max_number_of_touches(&mut self, val: usize);

    /// Translation in the coordinate system of the specified view.
    #[objc::msg_send(translationInView:)]
    pub fn translation_in_view(&self, view: Option<&ui::View>) -> cg::Point;

    #[objc::msg_send(setTranslation:inView:)]
    pub fn set_translation_in_view(&mut self, translation: cg::Point, view: Option<&ui::View>);

    /// Velocity of the pan in points/second in the coordinate system of the specified view.
    #[objc::msg_send(velocityInView:)]
    pub fn velocity_in_view(&self, view: Option<&ui::View>) -> cg::Point;
}

unsafe extern "C" {
    static UI_PAN_GESTURE_RECOGNIZER: &'static objc::Class<PanGestureRecognizer>;
}
