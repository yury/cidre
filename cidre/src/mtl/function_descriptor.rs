use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(FunctionDescriptor(ns::Id), MTL_FUNCTION_DESCRIPTOR);

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum FunctionOptions {
    None = 0,
    CompileToBinary = 1 << 0,
}

impl Default for FunctionOptions {
    fn default() -> Self {
        FunctionOptions::None
    }
}

impl FunctionDescriptor {
    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<&ns::String>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, name: Option<&ns::String>);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_FUNCTION_DESCRIPTOR: &'static objc::Class<FunctionDescriptor>;
}

#[cfg(test)]
mod tests {

    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let mut fd = mtl::FunctionDescriptor::new();

        assert!(fd.name().is_none());

        let name = ns::String::with_str("hello");

        fd.set_name(Some(&name));

        let actual_name = fd.name().unwrap();

        assert!(name.is_equal(&actual_name));
    }
}
