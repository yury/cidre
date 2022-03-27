use crate::{define_mtl, define_obj_type, objc::Id};

define_obj_type!(CounterSampleBuffer(Id));

impl CounterSampleBuffer {
    define_mtl!(device, get label);
}
