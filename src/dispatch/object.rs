use std::{ffi::c_void, mem::transmute};

use crate::{
    define_obj_type,
    dispatch::{Function, QOSClass},
    objc::Id,
};

define_obj_type!(Object(Id));

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

    pub fn get_context(&self) -> *mut c_void {
        unsafe { dispatch_get_context(self) }
    }

    pub fn set_finalizer_f<T>(&mut self, finalizer: Option<&Function<T>>) {
        unsafe { dispatch_set_finalizer_f(self, transmute(finalizer)) }
    }

    pub fn set_qos_class_floor(&mut self, qos_class: QOSClass, relative_priority: i32) {
        unsafe { dispatch_set_qos_class_floor(self, qos_class, relative_priority) }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_activate(object: &Object);
    fn dispatch_suspend(object: &Object);
    fn dispatch_resume(object: &Object);
    fn dispatch_set_context(object: &mut Object, context: *mut c_void);
    fn dispatch_get_context(object: &Object) -> *mut c_void;
    fn dispatch_set_finalizer_f(object: &mut Object, finalizer: Option<&Function<c_void>>);
    fn dispatch_set_qos_class_floor(
        object: &mut Object,
        qos_class: QOSClass,
        relative_priority: i32,
    );

}
