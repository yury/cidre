use crate::{arc, define_obj_type, ns, objc, ui};

#[objc::protocol(UIProtocol)]
pub trait Interaction: objc::Obj {
    #[objc::msg_send(view)]
    fn view(&self) -> arc::R<ui::View>;

    #[objc::msg_send(willMoveToView:)]
    fn will_move_to_view(&self, view: Option<&ui::View>);

    #[objc::msg_send(didMoveToView:)]
    fn did_move_to_view(&self, view: Option<&ui::View>);
}

define_obj_type!(
    pub AnyInteraction(ns::Id)
);

impl Interaction for AnyInteraction {}

/// Interactions
impl ui::View {
    #[objc::msg_send(addInteraction:)]
    pub fn add_interaction<T: Interaction>(&mut self, val: &T);

    #[objc::msg_send(removeInteraction:)]
    pub fn remove_interaction<T: Interaction>(&mut self, val: &T);

    #[objc::msg_send(interactions)]
    pub fn interactions(&self) -> arc::R<ns::Array<AnyInteraction>>;
}
