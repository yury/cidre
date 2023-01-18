use crate::{define_mtl, define_obj_type, ns};

define_obj_type!(Event(ns::Id));

impl Event {
    define_mtl!(device, label, set_label);
}

define_obj_type!(SharedEventHandle(ns::Id));

impl SharedEventHandle {
    define_mtl!(label);
}

define_obj_type!(SharedEvent(Event));

impl SharedEvent {}
