use crate::ns::{self, Id};
use crate::{define_mtl_device_and_label, define_obj_type};

define_obj_type!(ArgumentEncoder(Id));

impl ArgumentEncoder {
    define_mtl_device_and_label!();

    pub fn encoded_length(&self) -> ns::UInteger {
        unsafe { rsel_encodedLength(self) }
    }
}

extern "C" {
    // rsel(, id, encodedLength, NSUInteger)
    fn rsel_encodedLength(id: &Id) -> ns::UInteger;
}
