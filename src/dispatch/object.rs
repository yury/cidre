use crate::{define_obj_type, objc::Id};

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
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_activate(object: &Object);
    fn dispatch_suspend(object: &Object);
    fn dispatch_resume(object: &Object);
}
