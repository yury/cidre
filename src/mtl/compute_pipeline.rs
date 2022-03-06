use crate::{define_mtl, define_obj_type};

use crate::ns::Id;

define_obj_type!(Descriptor(Id));
define_obj_type!(State(Id));

impl State {
    define_mtl!(device, get label);
}
