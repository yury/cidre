use std::ffi::c_void;

use crate::cf::Retained;
use crate::define_obj_type;

use crate::dispatch::Object;
use crate::mtl::Function;

use super::{Queue, Time};

define_obj_type!(Group(Object));

impl Group {
    #[inline]
    pub fn new<'a>() -> Retained<'a, Self> {
        unsafe { dispatch_group_create() }
    }

    #[inline]
    pub fn wait(&self, timeout: Time) -> isize {
        unsafe { dispatch_group_wait(self, timeout) }
    }

    #[inline]
    pub fn notify_f(&self, queue: &Queue, context: *mut c_void, work: Function) {
        unsafe { dispatch_group_notify_f(self, queue, context, work) }
    }

    #[inline]
    pub fn enter(&self) {
        unsafe { dispatch_group_enter(&self) }
    }

    #[inline]
    pub fn leave(&self) {
        unsafe { dispatch_group_leave(self) }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_group_create<'a>() -> Retained<'a, Group>;
    fn dispatch_group_wait(group: &Group, timeout: Time) -> isize;

    fn dispatch_group_notify_f(group: &Group, queue: &Queue, context: *mut c_void, work: Function);
    fn dispatch_group_enter(group: &Group);
    fn dispatch_group_leave(group: &Group);

}
