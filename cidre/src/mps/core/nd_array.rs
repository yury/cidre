use crate::{define_obj_type, msg_send, ns};

define_obj_type!(NDArray(ns::Id));

impl NDArray {
    /// Copy bytes from NDArray into buffer
    /// The dimensionality and size of the copy region is given by the size of the NDArray
    /// For subregions, use a NDArray view.
    #[doc(alias = "readBytes:strideBytes:")]
    #[inline]
    pub fn read_bytes(&self, bytes: *mut u8, stride_bytes: *mut isize) {
        msg_send!("mps", self, sel_readBytes_strideBytes, bytes, stride_bytes)
    }
}
