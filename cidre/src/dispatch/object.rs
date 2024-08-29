use std::{ffi::c_void, mem::transmute};

use crate::{
    define_obj_type,
    dispatch::{self, QosClass},
    ns,
};

define_obj_type!(pub Object(ns::Id));

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

impl Object {
    #[inline]
    pub fn activate(&self) {
        unsafe { dispatch_activate(self) }
    }

    #[inline]
    pub fn suspend(&self) {
        unsafe { dispatch_suspend(self) }
    }

    #[inline]
    pub fn resume(&self) {
        unsafe { dispatch_resume(self) }
    }

    #[inline]
    pub fn set_context(&mut self, context: *mut c_void) {
        unsafe { dispatch_set_context(self, context) }
    }

    #[inline]
    pub fn context(&self) -> *mut c_void {
        unsafe { dispatch_get_context(self) }
    }

    #[inline]
    pub fn set_finalizer_f<T>(&mut self, finalizer: Option<&dispatch::Fn<T>>) {
        unsafe { dispatch_set_finalizer_f(self, transmute(finalizer)) }
    }

    #[inline]
    pub fn set_qos_class_floor(&mut self, qos_class: QosClass, relative_priority: i32) {
        unsafe { dispatch_set_qos_class_floor(self, qos_class, relative_priority) }
    }

    /// Sets the target queue for the given object.
    #[doc(alias = "dispatch_set_target_queue")]
    #[doc(alias = "DispatchObject.setTarget(self:queue:)")]
    #[inline]
    pub fn set_target_queue(&mut self, val: Option<&dispatch::Queue>) {
        unsafe {
            dispatch_set_target_queue(self, val);
        }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_activate(object: &Object);
    fn dispatch_suspend(object: &Object);
    fn dispatch_resume(object: &Object);
    fn dispatch_set_context(object: &mut Object, context: *mut c_void);
    fn dispatch_get_context(object: &Object) -> *mut c_void;
    fn dispatch_set_finalizer_f(object: &mut Object, finalizer: Option<&dispatch::Fn<c_void>>);
    fn dispatch_set_qos_class_floor(
        object: &mut Object,
        qos_class: QosClass,
        relative_priority: i32,
    );

    fn dispatch_set_target_queue(object: &mut Object, queue: Option<&dispatch::Queue>);
}
