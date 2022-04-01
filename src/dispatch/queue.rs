use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::ptr::NonNull;

use crate::cf::Retained;
use crate::define_obj_type;
use crate::dispatch::Object;

use super::Function;

define_obj_type!(Queue(Object));
define_obj_type!(Global(Queue));
define_obj_type!(Serial(Queue));
define_obj_type!(Main(Serial));
define_obj_type!(Concurent(Queue));

define_obj_type!(Attr(Object));

#[repr(usize)]
pub enum AutoreleaseFrequency {
    Inherit = 0,
    WorkItem = 1,
    Never = 2,
}

/// ```
/// use cidre::dispatch;
///
/// let q = dispatch::Queue::main();
///
/// q.as_type_ref().show();
/// ```
impl Queue {
    #[inline]
    pub fn new<'a>() -> Retained<'a, Queue> {
        Self::with_label_and_attrs(None, None)
    }

    #[inline]
    pub fn with_label_and_attrs<'a>(
        label: Option<&CStr>,
        attr: Option<&Attr>,
    ) -> Retained<'a, Queue> {
        unsafe {
            let label = label.map(|f| NonNull::new_unchecked(f.as_ptr() as *mut _));
            dispatch_queue_create(label, attr)
        }
    }

    #[inline]
    pub fn main<'a>() -> &'a Queue {
        Main::default()
    }

    #[inline]
    pub fn async_f(&self, context: *mut c_void, work: Function) {
        unsafe { dispatch_async_f(self, context, work) }
    }

    #[inline]
    pub fn sync_f(&self, context: *mut c_void, work: Function) {
        unsafe { dispatch_sync_f(self, context, work) }
    }
}

impl Main {
    #[inline]
    fn default<'a>() -> &'a Main {
        unsafe { &_dispatch_main_q }
    }
}

extern "C" {
    static _dispatch_main_q: Main;
    static _dispatch_queue_attr_concurrent: &'static Object;

    fn dispatch_async_f(queue: &Queue, context: *mut c_void, work: Function);
    fn dispatch_sync_f(queue: &Queue, context: *mut c_void, work: Function);
    fn dispatch_queue_create<'a>(
        label: Option<NonNull<c_char>>,
        attr: Option<&Attr>,
    ) -> Retained<'a, Queue>;

    fn dispatch_queue_attr_make_with_autorelease_frequency<'a>(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> Retained<'a, Attr>;
}

#[cfg(test)]
mod tests {

    use std::ffi::c_void;

    use crate::dispatch;

    extern "C" fn foo(_ctx: *mut c_void) {
        println!("nice");
    }

    #[test]
    fn test_queue() {
        let q = dispatch::Queue::new();

        q.as_type_ref().show();

        q.sync_f(std::ptr::null_mut(), foo);
    }
}
