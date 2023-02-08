use std::{fmt::Debug, intrinsics::transmute};

use crate::{
    arc, define_cls, define_mtl, define_obj_type, mtl, ns,
    objc::{self, Class},
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

define_obj_type!(CompileOptions(ns::Id));

impl arc::A<CompileOptions> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<CompileOptions>;
}

impl CompileOptions {
    define_cls!(MTL_COMPILE_OPTIONS);
    /// ```no_run
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
    pub fn new() -> arc::R<CompileOptions> {
        Self::alloc().init()
    }

    #[objc::msg_send(fastMathEnabled)]
    pub fn fast_math_enabled(&self) -> bool;

    #[objc::msg_send(setFastMathEnabled:)]
    pub fn set_fast_math_enabled(&mut self, value: bool);

    #[objc::msg_send(languageVersion)]
    pub fn language_version(&self) -> LanguageVersion;

    #[objc::msg_send(setLanguageVersion:)]
    pub fn set_language_version(&mut self, value: LanguageVersion);
}

define_obj_type!(Function(ns::Id));

impl Function {
    define_mtl!(device, label, set_label);

    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(newArgumentEncoderWithBufferIndex:)]
    pub fn new_argument_encoder_with_buf_index_ar(
        &self,
        index: ns::UInteger,
    ) -> arc::Rar<mtl::ArgumentEncoder>;

    #[objc::rar_retain()]
    pub fn new_argument_encoder_with_buf_index(
        &self,
        index: ns::UInteger,
    ) -> arc::R<mtl::ArgumentEncoder>;
}

define_obj_type!(Library(ns::Id));

impl Library {
    define_mtl!(device, label, set_label);

    #[objc::msg_send(functionNames)]
    pub fn fn_names(&self) -> &ns::Array<ns::String>;

    #[objc::msg_send(newFunctionWithName:)]
    pub fn new_fn_ar(&self, name: &ns::String) -> Option<arc::Rar<Function>>;

    #[objc::rar_retain()]
    pub fn new_fn(&self, name: &ns::String) -> Option<arc::R<Function>>;

    /// # Safety
    /// Use new_function_with_name_constant_values
    #[objc::msg_send(newFunctionWithName:constantValues:error:)]
    pub unsafe fn new_fn_const_values_err_ar<'ar>(
        &self,
        name: &ns::String,
        constant_values: &mtl::FunctionConstantValues,
        error: &mut Option<&'ar ns::Error>,
    ) -> Option<arc::Rar<Function>>;

    #[objc::rar_retain()]
    pub unsafe fn new_fn_const_values_err<'ar>(
        &self,
        name: &ns::String,
        constant_values: &mtl::FunctionConstantValues,
        error: &mut Option<&'ar ns::Error>,
    ) -> Option<arc::R<Function>>;

    pub fn new_fn_const_values<'ar>(
        &self,
        name: &ns::String,
        constant_values: &mtl::FunctionConstantValues,
    ) -> Result<arc::R<Function>, &'ar ns::Error> {
        let mut error = None;

        let res = unsafe { Self::new_fn_const_values_err(self, name, constant_values, &mut error) };

        if let Some(err) = error {
            return Err(err);
        }

        unsafe { Ok(transmute(res)) }
    }
}

pub type ErrorDomain = ns::ErrorDomain;

impl ErrorDomain {
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
    static MTL_COMPILE_OPTIONS: &'static Class<CompileOptions>;
}

#[cfg(test)]
mod tests {

    use crate::{blocks, mtl, ns};

    #[test]
    fn foo() {
        let device = mtl::Device::default().unwrap();
        let source = ns::String::with_str("kernel void function_a() {}");

        let handler = blocks::once2(move |lib, error| {
            println!("nice!!! {:?} {:?}", lib, error);
        });
        device.new_lib_with_src_options_completion(&source, None, handler.escape());
    }

    #[test]
    fn function_names() {
        let device = mtl::Device::default().unwrap();

        let source = ns::String::with_str("kernel void function_a() {}; void function_b() {}");
        let lib = device.new_lib_with_src(&source, None).unwrap();
        let names = lib.fn_names();
        assert_eq!(1, names.len());
        let n = &names[0];

        let expected_name = ns::String::with_str("function_a");

        assert!(n.eq(&expected_name));
    }

    #[test]
    fn error_basics() {
        let device = mtl::Device::default().unwrap();

        let source = ns::String::with_str("vid function_a() {}");
        let err = device.new_lib_with_src(&source, None).unwrap_err();

        assert_eq!(mtl::LibraryError::CompileFailure, err.code());
    }

    #[test]
    fn new_function_with_name() {
        let device = mtl::Device::default().unwrap();

        let source = ns::String::with_str("kernel void function_a() {}");
        let lib = device.new_lib_with_src(&source, None).unwrap();

        let func_name = ns::String::with_str_no_copy("function_a");
        let func = lib.new_fn(&func_name).unwrap();
        let name = func.name();
        assert!(func_name.is_equal(&name));
    }

    #[test]
    fn new_function_with_name_constant_values() {
        let device = mtl::Device::default().unwrap();

        let source = ns::String::with_str("kernel void function_a() {}");
        let lib = device.new_lib_with_src(&source, None).unwrap();

        let func_name = ns::String::with_str_no_copy("function_a");
        let constant_values = mtl::FunctionConstantValues::new();
        let func = lib
            .new_fn_const_values(&func_name, &constant_values)
            .unwrap();
        let name = func.name();
        assert!(func_name.is_equal(name));
    }

    #[test]
    fn compile_options() {
        let mut options = mtl::CompileOptions::new();

        assert_eq!(true, options.fast_math_enabled());
        options.set_fast_math_enabled(false);
        assert_eq!(false, options.fast_math_enabled());

        assert_ne!(options.language_version(), mtl::LanguageVersion::_2_0);

        options.set_language_version(mtl::LanguageVersion::_2_4);
        assert_eq!(options.language_version(), mtl::LanguageVersion::_2_4);
    }
}
