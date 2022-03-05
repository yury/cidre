use crate::{
    cf::{self, Retained},
    define_obj_type,
    objc::Id,
};

use super::Device;

#[repr(usize)]
pub enum LanguageVersion {
    _1_0 = (1 << 16),
    _1_1 = (1 << 16) + 1,
    _1_2 = (1 << 16) + 2,
    _2_0 = (2 << 16),
    _2_1 = (2 << 16) + 1,
    _2_2 = (2 << 16) + 2,
    _2_3 = (2 << 16) + 3,
    _2_4 = (2 << 16) + 4,
}

#[repr(isize)]
pub enum Type {
    Executable = 0,
    Dynamic = 1,
}

define_obj_type!(CompileOptions(Id));

impl CompileOptions {}

define_obj_type!(Funtion(Id));

impl Funtion {
    #[inline]
    pub fn device(&self) -> &Device {
        unsafe { rsel_device(self) }
    }

    #[inline]
    pub fn label<'a>(&self) -> Option<Retained<'a, cf::String>> {
        unsafe { rsel_label(self) }
    }
}

define_obj_type!(Library(Id));

impl Library {
    #[inline]
    pub fn device(&self) -> &Device {
        unsafe { rsel_device(self) }
    }

    #[inline]
    pub fn label<'a>(&self) -> Option<Retained<'a, cf::String>> {
        unsafe { rsel_label(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_device(id: &Id) -> &Device;
    fn rsel_label<'a>(id: &Id) -> Option<Retained<'a, cf::String>>;
}
