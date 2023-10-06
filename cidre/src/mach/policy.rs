use std::ffi::c_int;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Policy(c_int);
