use crate::ns::Id;
use crate::{cf, define_obj_type, msg_send};

define_obj_type!(FunctionDescriptor(Id));

#[repr(usize)]
pub enum FunctionOptions {
    None = 0,
    CompileToBinary = 1 << 0,
}

impl FunctionDescriptor {
    #[inline]
    pub fn name<'copy>(&self) -> Option<cf::Retained<'copy, cf::String>> {
        msg_send!(self, sel_name)
    }

    #[inline]
    pub fn set_name(&mut self, name: Option<&cf::String>) {
        msg_send!(self, sel_setName, name)
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let fd = mtl::FunctionDescriptor::default();
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
    ///
    #[inline]
    pub fn default<'autorelease>() -> &'autorelease mut FunctionDescriptor {
        unsafe { MTLFunctionDescriptor_functionDescriptor() }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLFunctionDescriptor_functionDescriptor<'autorelease>(
    ) -> &'autorelease mut FunctionDescriptor;

}
