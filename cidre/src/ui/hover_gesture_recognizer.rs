use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIHoverGestureRecognizer")]
    pub HoverGestureRecognizer(ui::GestureRecognizer),
    sym UIHoverGestureRecognizer
);

impl HoverGestureRecognizer {
    #[objc::init(initWithTarget:action:)]
    pub fn init_with_target_action(
        self,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<HoverGestureRecognizer>;

    pub fn with_target_action(target: Option<&ns::Id>, action: Option<&objc::Sel>) -> arc::R<Self> {
        Self::alloc().init_with_target_action(target, action)
    }
}
