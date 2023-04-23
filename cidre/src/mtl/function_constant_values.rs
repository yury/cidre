use std::ffi::c_void;

use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(FnConstValues(ns::Id), MTL_FUNCTION_CONSTANT_VALUES);

impl FnConstValues {
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
        name: &ns::String,
    );
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_FUNCTION_CONSTANT_VALUES: &'static objc::Class<FnConstValues>;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let mut fcv = mtl::FnConstValues::new();
        let v = false;
        fcv.set_value_at(&v as *const bool as _, mtl::DataType::Bool, 0);
        fcv.reset();
        let name = ns::String::with_str("name");
        fcv.set_value_with_name(&v as *const bool as _, mtl::DataType::Bool, &name);
    }
}
