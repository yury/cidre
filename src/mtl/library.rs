use std::{fmt::Debug, intrinsics::transmute};

use crate::{
    cf::{self, Retained},
    define_mtl, define_obj_type, msg_send, mtl, ns,
    objc::Id,
};

#[derive(Debug, PartialEq, Eq)]
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
    _3_0 = (3 << 16),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum Type {
    Executable = 0,
    Dynamic = 1,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum OptimizationLevel {
    Default = 0,
    Size = 1,
}

define_obj_type!(CompileOptions(Id));

impl CompileOptions {
    /// ```
    /// use cidre::mtl;
    ///
    /// let mut options = mtl::CompileOptions::new();
    ///
    /// assert_eq!(true, options.fast_math_enabled());
    /// options.set_fast_math_enabled(false);
    /// assert_eq!(false, options.fast_math_enabled());
    ///
    /// assert_ne!(options.language_version(), mtl::LanguageVersion::_2_0);
    ///
    /// options.set_language_version(mtl::LanguageVersion::_2_4);
    /// assert_eq!(options.language_version(), mtl::LanguageVersion::_2_4);
    ///
    /// ```
    pub fn new() -> Retained<CompileOptions> {
        unsafe { MTLCompileOptions_new() }
    }

    pub fn fast_math_enabled(&self) -> bool {
        unsafe { rsel_fastMathEnabled(self) }
    }

    pub fn set_fast_math_enabled(&mut self, value: bool) {
        unsafe { wsel_setFastMathEnabled(self, value) }
    }

    pub fn language_version(&self) -> LanguageVersion {
        unsafe { rsel_languageVersion(self) }
    }

    pub fn set_language_version(&mut self, value: LanguageVersion) {
        unsafe { wsel_setLanguageVersion(self, value) }
    }
}

define_obj_type!(Function(Id));

impl Function {
    define_mtl!(device, label, set_label);

    #[inline]
    pub fn name(&self) -> &cf::String {
        msg_send!("common", self, sel_name)
    }

    #[inline]
    pub fn new_argument_encoder_with_buffer_index(
        &self,
        index: ns::UInteger,
    ) -> Retained<mtl::ArgumentEncoder> {
        unsafe { rsel_newArgumentEncoderWithBufferIndex(self, index) }
    }
}

define_obj_type!(Library(Id));

impl Library {
    define_mtl!(device, label, set_label);

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let source = cf::String::from_str("kernel void function_a() {}; void function_b() {}");
    /// let lib = device.library_with_source(&source, None).unwrap();
    /// let names = lib.function_names();
    /// assert_eq!(1, names.len());
    /// let n = &names[0];
    ///
    /// let expected_name = cf::String::from_str("function_a");
    ///
    /// assert!(n.equal(&expected_name));
    /// ```
    #[inline]
    pub fn function_names(&self) -> &cf::ArrayOf<cf::String> {
        unsafe { rsel_functionNames(self) }
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let source = cf::String::from_str("kernel void function_a() {}");
    /// let lib = device.library_with_source(&source, None).unwrap();
    ///
    /// let func_name = cf::String::from_str_no_copy("function_a");
    /// let func = lib.new_function_with_name(&func_name).unwrap();
    /// let name = func.name();
    /// assert!(func_name.equal(&name));
    ///
    /// ```
    pub fn new_function_with_name(&self, name: &cf::String) -> Option<Retained<Function>> {
        unsafe { rsel_newFunctionWithName(self, name) }
    }

    /// # Safety
    /// Use new_function_with_name_constant_values
    #[inline]
    pub unsafe fn new_function_with_name_constant_values_error<'ar>(
        &self,
        name: &cf::String,
        constant_values: &mtl::FunctionConstantValues,
        error: &mut Option<&'ar cf::Error>,
    ) -> Option<Retained<Function>> {
        rsel_newFunctionWithName_constantValues_error(self, name, constant_values, error)
    }

    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let source = cf::String::from_str("kernel void function_a() {}");
    /// let lib = device.library_with_source(&source, None).unwrap();
    ///
    /// let func_name = cf::String::from_str_no_copy("function_a");
    /// let constant_values = mtl::FunctionConstantValues::new();
    /// let func = lib.new_function_with_name_constant_values(&func_name, &constant_values).unwrap();
    /// let name = func.name();
    /// assert!(func_name.equal(&name));
    ///
    /// ```
    pub fn new_function_with_name_constant_values<'ar>(
        &self,
        name: &cf::String,
        constant_values: &mtl::FunctionConstantValues,
    ) -> Result<Retained<Function>, &'ar cf::Error> {
        let mut error = None;

        let res = unsafe {
            Self::new_function_with_name_constant_values_error(
                self,
                name,
                constant_values,
                &mut error,
            )
        };

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }
}

// impl Debug for Library {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut st = f.debug_struct("Library");
//         match self.label() {
//             Some(label) => st.field("label", &Cow::from(label)),
//             None => st.field("label", &"<none>"),
//         }
//         .finish()
//     }
// }

pub type ErrorDomain = cf::ErrorDomain;

impl ErrorDomain {
    /// ```
    /// use cidre::{cf, mtl};
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let source = cf::String::from_str("vid function_a() {}");
    /// let err = device.library_with_source(&source, None).unwrap_err();
    ///
    /// assert_eq!(mtl::LibraryError::CompileFailure, err.get_code());
    ///
    /// ```
    pub fn library() -> &'static ErrorDomain {
        unsafe { MTLLibraryErrorDomain }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum FunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
    Visible = 5,
    Intersection = 6,
    Mesh = 7,
    Object = 8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum Error {
    Unsupported = 1,
    Internal = 2,
    CompileFailure = 3,
    CompileWarning = 4,
    FunctionNotFound = 5,
    FileNotFound = 6,
}

impl PartialEq<isize> for Error {
    fn eq(&self, other: &isize) -> bool {
        *self as isize == *other
    }
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    static MTLLibraryErrorDomain: &'static ErrorDomain;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLCompileOptions_new() -> Retained<CompileOptions>;
    fn rsel_fastMathEnabled(id: &Id) -> bool;
    fn wsel_setFastMathEnabled(id: &mut Id, value: bool);

    fn rsel_languageVersion(id: &Id) -> LanguageVersion;
    fn wsel_setLanguageVersion(id: &mut Id, value: LanguageVersion);

    fn rsel_functionNames(id: &Id) -> &cf::ArrayOf<cf::String>;

    fn rsel_newFunctionWithName(id: &Library, name: &cf::String) -> Option<Retained<Function>>;
    fn rsel_newFunctionWithName_constantValues_error<'ar>(
        id: &Library,
        name: &cf::String,
        constant_values: &mtl::FunctionConstantValues,
        error: &mut Option<&'ar cf::Error>,
    ) -> Option<Retained<Function>>;

    fn rsel_newArgumentEncoderWithBufferIndex(
        id: &Function,
        index: ns::UInteger,
    ) -> Retained<mtl::ArgumentEncoder>;
}

#[cfg(test)]
mod tests {

    use crate::{cf, mtl};

    #[test]
    fn foo() {
        let device = mtl::Device::default().unwrap();
        let source = cf::String::from_str("kernel void function_a() {}");

        device.library_with_source_options_completion(&source, None, move |lib, error| {
            println!("nice!!! {:?} {:?}", lib, error);
        });
    }
}
