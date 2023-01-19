use std::ffi::c_void;

use crate::{arc, cf, define_mtl, define_obj_type, mtl, ns};

define_obj_type!(FunctionConstantValues(ns::Id));

impl FunctionConstantValues {
    define_mtl!(reset);

    pub fn new() -> arc::R<FunctionConstantValues> {
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

    pub fn set_value_with_name(
        &mut self,
        value: *const c_void,
        type_: mtl::DataType,
        name: &cf::String,
    ) {
        unsafe { wsel_setConstantValue_type_withName(self, value, type_, name) }
    }
}

extern "C" {
    fn MTLFunctionConstantValues_new() -> arc::R<FunctionConstantValues>;

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

#[cfg(test)]
mod tests {
    use crate::{cf, mtl};

    #[test]
    fn basics1() {
        let mut fcv = mtl::FunctionConstantValues::new();
        let v = false;
        fcv.set_value_at(&v as *const bool as _, mtl::DataType::Bool, 0);
        fcv.reset();
    }

    #[test]
    fn basics2() {
        let mut fcv = mtl::FunctionConstantValues::new();
        let v = false;
        let name = cf::String::from_str("name");
        fcv.set_value_with_name(&v as *const bool as _, mtl::DataType::Bool, &name);
        fcv.reset();
    }
}