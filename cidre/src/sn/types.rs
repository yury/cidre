use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "SNClassifierIdentifier")]
    pub Id(ns::Id)
);

impl Id {
    pub fn v1() -> &'static Id {
        unsafe { SNClassifierIdentifierVersion1 }
    }
}

#[link(name = "SoundAnalysis", kind = "framework")]
extern "C" {
    static SNClassifierIdentifierVersion1: &'static Id;
}
