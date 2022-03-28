use crate::{define_mtl, define_obj_type, objc::Id};

define_obj_type!(Event(Id));

impl Event {
    define_mtl!(device, label, set_label);
}

define_obj_type!(SharedEventHandle(Id));

impl SharedEventHandle {
    define_mtl!(label);
}

define_obj_type!(SharedEvent(Event));

impl SharedEvent {

}

extern "C" {
    // - (MTLSharedEventHandle *)newSharedEventHandle;
}
