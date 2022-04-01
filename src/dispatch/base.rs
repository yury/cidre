use std::ffi::c_void;

pub type Function = extern "C" fn(*mut c_void);