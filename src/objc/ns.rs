use std::{
    intrinsics::transmute,
    ops::{Deref, DerefMut},
};

use crate::cf::{
    runtime::{Release, Retain},
    Retained,
};

use super::{Class, Id, Sel};

#[repr(transparent)]
pub struct Object(Id);

impl Deref for Object {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Retain for Object {
    #[inline]
    fn retained<'a>(&self) -> Retained<'a, Self> {
        unsafe { Id::retain(self) }
    }
}

impl Release for Object {
    #[inline]
    unsafe fn release(&mut self) {
        Id::release(self)
    }
}

#[link_section = "__DATA,__objc_classrefs,regular,no_dead_strip"]
static OBJECT_CLASS: &Class = unsafe { &OBJC_CLASS_NSObject };


impl Object {
    /// ```
    /// use cidre::objc::ns;
    ///
    /// let obj = ns::Object::new();
    ///  
    /// obj.as_type_ref().show(); 
    /// ```
    pub fn new<'a>() -> Retained<'a, Object> {
        unsafe { transmute(objc_opt_new(OBJECT_CLASS)) }
    }

    /// ```
    /// use cidre::objc::ns;
    ///
    /// let obj = ns::Object::alloc_init();
    ///  
    /// obj.as_type_ref().show(); 
    /// ```
    pub fn alloc_init<'a>() -> Retained<'a, Object> {
        unsafe { transmute(objc_alloc_init(OBJECT_CLASS)) }
    }

    /// ```
    /// use cidre::objc::ns;
    ///
    /// let obj = ns::Object::alloc();
    ///  
    /// obj.as_type_ref().show(); 
    /// ```
    pub fn alloc<'a>() -> Retained<'a, Object> {
        unsafe { transmute(objc_alloc(OBJECT_CLASS)) }
    }
}

extern "C" {
    #[link_name = "OBJC_CLASS_$_NSObject"]
    static OBJC_CLASS_NSObject: Class;

    fn objc_alloc<'a>(cls: &Class) -> Option<Retained<'a, Id>>;
    fn objc_alloc_init<'a>(cls: &Class) -> Option<Retained<'a, Id>>;
    fn objc_opt_new<'a>(cls: &Class) -> Option<Retained<'a, Id>>;

}
