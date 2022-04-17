use crate::{define_obj_type, objc::Id};

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    Complete,
    Idle,
    Blank,
    Suspended,
    Started,
    Stopped,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum OutputType {
    Screen,
}

define_obj_type!(ContentFilter(Id));
define_obj_type!(Configuration(Id));
define_obj_type!(Stream(Id));