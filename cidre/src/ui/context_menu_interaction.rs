use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIContextMenuInteraction")]
    pub ContextMenuInteraction(ns::Id)
);

#[objc::protocol(UIContextMenuInteractionAnimating)]
pub trait ContextMenuInteractionAnimating: objc::Obj {}

define_obj_type!(
    pub AnyContextMenuInteractionAnimating(ns::Id)
);

impl ContextMenuInteractionAnimating for AnyContextMenuInteractionAnimating {}

#[objc::protocol(UIContextMenuInteractionCommitAnimating)]
pub trait ContextMenuInteractionCommitAnimating: objc::Obj {}

define_obj_type!(
    pub AnyContextMenuInteractionCommitAnimating(ns::Id)
);

impl ContextMenuInteractionCommitAnimating for AnyContextMenuInteractionCommitAnimating {}
