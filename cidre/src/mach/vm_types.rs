use std::ffi::{c_int, c_uint};

use super::Port;

pub type Integer = c_int;
pub type Natural = c_uint;

pub type Size = usize;
pub type Offset = usize;
pub type Addr = usize;

pub type Map = Port;
pub type MapRead = Port;
pub type MapInspect = Port;
