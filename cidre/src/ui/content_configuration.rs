use crate::{arc, define_obj_type, ns, objc, ui};

#[objc::protocol(UIContentConfiguration)]
pub trait ContentConfiguration: objc::Obj {
    #[objc::msg_send(makeContentView)]
    fn make_content_view(&self) -> arc::R<ui::View>;

    #[objc::msg_send(updatedConfigurationForState:)]
    fn updated_for_state(
        &self,
        state: &ui::AnyConfigurationState,
    ) -> arc::R<AnyContentConfiguration>;
}

define_obj_type!(pub AnyContentConfiguration(ns::Id));

impl ContentConfiguration for AnyContentConfiguration {}
