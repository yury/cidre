use std::ffi::c_void;

use crate::io;
use crate::ns::Id;
use crate::{
    cf::{self, Retained},
    define_obj_type, mtl, ns,
};

define_obj_type!(FunctionConstantValues(Id));

impl FunctionConstantValues {
    /// ```
    /// use cidre::mtl;
    ///
    /// let mut fcv = mtl::FunctionConstantValues::new();
    /// let v = false;
    /// fcv.set_value_at(&v as *const bool as _, mtl::DataType::Bool, 0);
    /// fcv.reset();
    /// ```
    pub fn new<'a>() -> Retained<'a, FunctionConstantValues> {
        unsafe { MTLFunctionConstantValues_new() }
    }

    pub fn set_value_at(
        &mut self,
        value: *const c_void,
        type_: mtl::DataType,
        at_index: ns::UInteger,
    ) {
        unsafe { wsel_setConstantValue_type_atIndex(self, value, type_, at_index) }
    }

    pub fn set_values(
        &mut self,
        value: *const c_void,
        type_: mtl::DataType,
        with_range: ns::Range,
    ) {
        unsafe { wsel_setConstantValues_type_withRange(self, value, type_, with_range) }
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let mut fcv = mtl::FunctionConstantValues::new();
    /// let v = false;
    /// let name = cf::String::from_str("name");
    /// fcv.set_value_with_name(&v as *const bool as _, mtl::DataType::Bool, &name);
    /// fcv.reset();
    /// ```
    pub fn set_value_with_name(
        &mut self,
        value: *const c_void,
        type_: mtl::DataType,
        name: &cf::String,
    ) {
        unsafe { wsel_setConstantValue_type_withName(self, value, type_, name) }
    }

    pub fn reset(&mut self) {
        unsafe { wsel_reset(self) }
    }
}

extern "C" {
    fn wsel_reset(id: &Id);
    fn MTLFunctionConstantValues_new<'a>() -> Retained<'a, FunctionConstantValues>;

    fn wsel_setConstantValue_type_atIndex(
        id: &FunctionConstantValues,
        value: *const c_void,
        type_: mtl::DataType,
        at_index: ns::UInteger,
    );
    fn wsel_setConstantValues_type_withRange(
        id: &FunctionConstantValues,
        values: *const c_void,
        type_: mtl::DataType,
        with_range: ns::Range,
    );
    fn wsel_setConstantValue_type_withName(
        id: &FunctionConstantValues,
        value: *const c_void,
        type_: mtl::DataType,
        with_name: &cf::String,
    );
}
