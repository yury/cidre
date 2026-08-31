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

    /// # Safety
    ///
    /// `context` must remain valid for every installed handler and finalizer
    /// that accesses it, including calls already in flight.
    #[inline]
    pub unsafe fn set_context(&mut self, context: *mut c_void) {
        unsafe { dispatch_set_context(self, context) }
    }

    #[inline]
    pub fn context(&self) -> *mut c_void {
        unsafe { dispatch_get_context(self) }
    }

    /// # Safety
    ///
    /// The current context must be null or point to a valid `T` until the
    /// finalizer runs. The finalizer must release that context at most once.
    #[inline]
    pub unsafe fn set_finalizer_f<T>(&mut self, finalizer: Option<dispatch::Fn<T>>) {
        unsafe {
            dispatch_set_finalizer_f(
                self,
                transmute::<Option<dispatch::Fn<T>>, Option<dispatch::Fn<c_void>>>(finalizer),
            )
        }
    }

    /// Will panic if queue is initialized already
    #[inline]
    pub unsafe fn set_qos_class_floor(&mut self, qos_class: QosClass, relative_priority: i32) {
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
unsafe extern "C-unwind" {
    fn dispatch_activate(object: &Object);
    fn dispatch_suspend(object: &Object);
    fn dispatch_resume(object: &Object);
    fn dispatch_set_context(object: &mut Object, context: *mut c_void);
    fn dispatch_get_context(object: &Object) -> *mut c_void;
    fn dispatch_set_finalizer_f(object: &mut Object, finalizer: Option<dispatch::Fn<c_void>>);
    fn dispatch_set_qos_class_floor(
        object: &mut Object,
        qos_class: QosClass,
        relative_priority: i32,
    );

    fn dispatch_set_target_queue(object: &mut Object, queue: Option<&dispatch::Queue>);
}

#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc::{self, Sender},
        time::Duration,
    };

    use crate::dispatch;

    struct FinalizerCtx(Sender<()>);

    extern "C-unwind" fn finalizer(ctx: *mut FinalizerCtx) {
        let ctx = unsafe { Box::from_raw(ctx) };
        let _ = ctx.0.send(());
    }

    #[test]
    fn finalizer_receives_context() {
        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(FinalizerCtx(sender)));

        {
            let mut queue = dispatch::Queue::new();
            unsafe {
                queue.set_context(context.cast());
                queue.set_finalizer_f(Some(finalizer));
            }
        }

        receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("dispatch finalizer did not run");
    }
}
