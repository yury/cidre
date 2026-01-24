use crate::{define_obj_type, ns};

#[doc(alias = "UIDropOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum DropOperation {
    Cancel,
    Forbidden,
    Copy,
    Move,
}

define_obj_type!(
    #[doc(alias = "UIDropProposal")]
    pub DropProposal(ns::Id)
);
