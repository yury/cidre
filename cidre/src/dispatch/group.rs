use std::{ffi::c_void, mem::transmute};

use crate::{arc, define_obj_type, dispatch};

use super::{Queue, Time};

define_obj_type!(pub Group(dispatch::Object));

impl Group {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { dispatch_group_create() }
    }

    #[inline]
    pub fn wait(&self, timeout: Time) -> isize {
        unsafe { dispatch_group_wait(self, timeout) }
    }

    #[inline]
    pub fn notify_f<T>(&self, queue: &Queue, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_group_notify_f(self, queue, context as _, transmute(work)) }
    }

    #[inline]
    pub fn enter(&self) {
        unsafe { dispatch_group_enter(self) }
    }

    #[inline]
    pub fn leave(&self) {
        unsafe { dispatch_group_leave(self) }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_group_create() -> arc::R<Group>;
    fn dispatch_group_wait(group: &Group, timeout: Time) -> isize;

    fn dispatch_group_notify_f(
        group: &Group,
        queue: &Queue,
        context: *mut c_void,
        work: dispatch::Fn<c_void>,
    );
    fn dispatch_group_enter(group: &Group);
    fn dispatch_group_leave(group: &Group);

}
