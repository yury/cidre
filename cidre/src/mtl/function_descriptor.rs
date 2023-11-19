use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(pub FnDesc(ns::Id), MTL_FUNCTION_DESCRIPTOR);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum FnOpts {
    #[default]
    None = 0,
    CompileToBinary = 1 << 0,
}

impl FnDesc {
    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<&ns::String>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, name: Option<&ns::String>);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_FUNCTION_DESCRIPTOR: &'static objc::Class<FnDesc>;
}

#[cfg(test)]
mod tests {

    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let mut fd = mtl::FnDesc::new();

        assert!(fd.name().is_none());

        let name = ns::String::with_str("hello");

        fd.set_name(Some(&name));

        let actual_name = fd.name().unwrap();

        assert_eq!(name.as_ref(), actual_name);
    }
}
