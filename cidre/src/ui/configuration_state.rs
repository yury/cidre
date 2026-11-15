use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIConfigurationState")]
    pub AnyConfigurationState(ns::Id)
);

impl AnyConfigurationState {
    #[objc::msg_send(traitCollection)]
    pub fn trait_collection(&self) -> arc::R<ui::TraitCollection>;
}
