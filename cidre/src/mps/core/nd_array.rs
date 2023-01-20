use crate::{define_obj_type, ns, objc};

define_obj_type!(NDArray(ns::Id));

impl NDArray {
    /// Copy bytes from NDArray into buffer
    /// The dimensionality and size of the copy region is given by the size of the NDArray
    /// For subregions, use a NDArray view.
    #[doc(alias = "readBytes:strideBytes:")]
    #[objc::msg_send2(readBytes:strideBytes:)]
    pub fn read_bytes(&self, bytes: *mut u8, stride_bytes: *mut isize);
}
