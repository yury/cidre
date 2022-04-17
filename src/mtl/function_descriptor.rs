use crate::{
    cf::{self, runtime::Autoreleased},
    define_obj_type, msg_send, ns,
};

define_obj_type!(FunctionDescriptor(ns::Id));

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
    #[inline]
    pub fn name(&self) -> Option<&cf::String> {
        msg_send!("mtl", self, sel_name)
    }

    #[inline]
    pub fn set_name(&mut self, name: Option<&cf::String>) {
        msg_send!("mtl", self, sel_setName, name)
    }

    /// ```rust
    /// use cidre::{cf, mtl};
    ///
    /// let mut fd = mtl::FunctionDescriptor::default();
    ///
    /// assert!(fd.name().is_none());
    ///
    /// let name = cf::String::from_str("hello");
    ///
    /// fd.set_name(Some(&name));
    ///
    /// let actual_name = fd.name().unwrap();
    ///
    /// assert!(name.equal(&actual_name));
    /// ```
    #[inline]
    pub fn default<'a>() -> Autoreleased<'a, FunctionDescriptor> {
        unsafe { MTLFunctionDescriptor_functionDescriptor() }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLFunctionDescriptor_functionDescriptor<'a>() -> Autoreleased<'a, FunctionDescriptor>;
}
