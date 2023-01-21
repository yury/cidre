use std::ffi::c_void;

use crate::{arc, cf, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(FunctionConstantValues(ns::Id), MTL_FUNCTION_CONSTANT_VALUES);

impl FunctionConstantValues {
    define_mtl!(reset);

    #[objc::msg_send(setConstantValue:type:atIndex:)]
    pub fn set_value_at(
        &mut self,
        value: *const c_void,
        type_: mtl::DataType,
        at_index: ns::UInteger,
    );

    #[objc::msg_send(setConstantValues:type:withRange:)]
    pub fn set_values(&mut self, value: *const c_void, type_: mtl::DataType, with_range: ns::Range);

    #[objc::msg_send(setConstantValue:type:withName:)]
    pub fn set_value_with_name(
        &mut self,
        value: *const c_void,
        type_: mtl::DataType,
        name: &cf::String,
    );
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_FUNCTION_CONSTANT_VALUES: &'static objc::Class<FunctionConstantValues>;
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
