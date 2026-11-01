use crate::{arc, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIVisualEffectView")]
    pub VisualEffectView(ui::View),
    sym UIVisualEffectView
);

impl VisualEffectView {
    #[objc::init(initWithEffect:)]
    pub fn init_with_effect(self, effect: Option<&ui::VisualEffect>) -> arc::R<VisualEffectView>;

    pub fn with_effect(effect: Option<&ui::VisualEffect>) -> arc::R<Self> {
        Self::alloc().init_with_effect(effect)
    }

    #[objc::msg_send(contentView)]
    pub fn content_view(&self) -> arc::R<ui::View>;
}
