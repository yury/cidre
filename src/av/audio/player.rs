use std::ffi::c_void;

use crate::{av, define_obj_type, msg_send, ns};

define_obj_type!(Player(ns::Id));

impl Player {}
