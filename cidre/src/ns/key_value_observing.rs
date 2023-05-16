use crate::{define_obj_type, define_options, ns};

define_options!(KVOOptions(usize));
impl KVOOptions {
    pub const NEW: Self = Self(0x01);
    pub const OLD: Self = Self(0x02);
    pub const INITIAL: Self = Self(0x04);
    pub const PRIOR: Self = Self(0x08);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum KVChange {
    Setting = 1,
    Insertion = 2,
    Removal = 3,
    Replacement = 4,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum KVSetMutationKind {
    Union = 1,
    Minus = 2,
    Intersect = 3,
    Set = 4,
}

define_obj_type!(KVChangeKey(ns::String));
impl KVChangeKey {
    pub fn kind() -> &'static Self {
        unsafe { NSKeyValueChangeKindKey }
    }

    pub fn new() -> &'static Self {
        unsafe { NSKeyValueChangeNewKey }
    }

    pub fn old() -> &'static Self {
        unsafe { NSKeyValueChangeOldKey }
    }

    pub fn indexes() -> &'static Self {
        unsafe { NSKeyValueChangeIndexesKey }
    }

    pub fn is_prior_key() -> &'static Self {
        unsafe { NSKeyValueChangeNotificationIsPriorKey }
    }
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    static NSKeyValueChangeKindKey: &'static KVChangeKey;
    static NSKeyValueChangeNewKey: &'static KVChangeKey;
    static NSKeyValueChangeOldKey: &'static KVChangeKey;
    static NSKeyValueChangeIndexesKey: &'static KVChangeKey;
    static NSKeyValueChangeNotificationIsPriorKey: &'static KVChangeKey;
}
