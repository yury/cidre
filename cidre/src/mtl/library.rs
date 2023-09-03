use crate::{arc, define_cls, define_mtl, define_obj_type, mtl, ns, objc};

// typedef NS_ENUM(NSUInteger, MTLPatchType) {
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(usize)]
pub enum PatchType {
    None = 0,
    Triangle = 1,
    Quad = 2,
}

define_obj_type!(VertexAttribute(ns::Id));

impl VertexAttribute {
    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(attributeIndex)]
    pub fn attribute_index(&self) -> usize;

    #[objc::msg_send(attributeType)]
    pub fn attribute_type(&self) -> mtl::DataType;

    #[objc::msg_send(isActive)]
    pub fn is_active(&self) -> bool;

    #[objc::msg_send(isPatchData)]
    pub fn is_patch_data(&self) -> bool;

    #[objc::msg_send(isPatchControlPointData)]
    pub fn is_patch_ctrl_point_data(&self) -> bool;
}

define_obj_type!(Attribute(ns::Id));

impl Attribute {
    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(attributeIndex)]
    pub fn attribute_index(&self) -> usize;

    #[objc::msg_send(attributeType)]
    pub fn attribute_type(&self) -> mtl::DataType;

    #[objc::msg_send(isActive)]
    pub fn is_active(&self) -> bool;

    #[objc::msg_send(isPatchData)]
    pub fn is_patch_data(&self) -> bool;

    #[objc::msg_send(isPatchControlPointData)]
    pub fn is_patch_ctrl_point_data(&self) -> bool;
}

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
    /// A library that can create pipeline state objects.
    Executable = 0,

    /// A library that you can dynamically link to from other libraries.
    Dynamic = 1,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum OptimizationLevel {
    Default = 0,
    Size = 1,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum CompileSymbolVisibility {
    #[default]
    Default = 0,
    Hidden = 1,
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

    #[objc::msg_send(compileSymbolVisibility)]
    pub fn compile_symbol_visibility(&self) -> CompileSymbolVisibility;

    #[objc::msg_send(setCompileSymbolVisibility:)]
    pub fn set_compile_symbol_visibility(&mut self, value: CompileSymbolVisibility);

    #[objc::msg_send(allowReferencingUndefinedSymbols)]
    pub fn allow_referencing_undefined_symbols(&self) -> bool;

    #[objc::msg_send(setAllowReferencingUndefinedSymbols:)]
    pub fn set_allow_referencing_undefined_symbols(&mut self, value: bool);

    #[objc::msg_send(maxTotalThreadsPerThreadgroup)]
    pub fn max_total_threads_per_threadgroup(&self) -> usize;

    #[objc::msg_send(setMaxTotalThreadsPerThreadgroup:)]
    pub fn set_max_total_threads_per_threadgroup(&mut self, value: usize);
}

define_obj_type!(Fn(ns::Id));

impl Fn {
    define_mtl!(device, label, set_label);

    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(newArgumentEncoderWithBufferIndex:)]
    pub fn new_argument_enc_with_buf_index(
        &self,
        index: ns::UInteger,
    ) -> arc::R<mtl::ArgumentEncoder>;

    #[objc::msg_send(functionType)]
    pub fn fn_type(&self) -> mtl::FnType;

    #[objc::msg_send(patchType)]
    pub fn patch_type(&self) -> mtl::PatchType;

    #[objc::msg_send(patchControlPointCount)]
    pub fn patch_ctrl_point(&self) -> isize;

    #[objc::msg_send(vertexAttributes)]
    pub fn vertext_attributes(&self) -> Option<&ns::Array<mtl::VertexAttribute>>;

    #[objc::msg_send(stageInputAttributes)]
    pub fn stage_input_attributes(&self) -> Option<ns::Array<mtl::Attribute>>;

    #[objc::msg_send(options)]
    pub fn options(&self) -> mtl::FnOptions;
}

define_obj_type!(Lib(ns::Id));

impl Lib {
    define_mtl!(device, label, set_label);

    /// The array contains ns::String objects, with the name of each function in library.
    #[objc::msg_send(functionNames)]
    pub fn fn_names(&self) -> &ns::Array<ns::String>;

    #[objc::msg_send(newFunctionWithName:)]
    pub fn new_fn(&self, name: &ns::String) -> Option<arc::R<Fn>>;

    pub fn new_fn_str(&self, name: &str) -> Option<arc::R<Fn>> {
        self.new_fn(ns::String::with_str_no_copy(name).as_ref())
    }

    /// # Safety
    /// Use new_fn_const_values
    #[objc::msg_send(newFunctionWithName:constantValues:error:)]
    pub unsafe fn new_fn_with_consts_err<'ar>(
        &self,
        name: &ns::String,
        constant_values: &mtl::FnConstValues,
        error: *mut Option<&'ar ns::Error>,
    ) -> Option<arc::R<Fn>>;

    #[inline]
    pub fn new_fn_with_consts<'ar>(
        &self,
        name: &ns::String,
        constant_values: &mtl::FnConstValues,
    ) -> Result<arc::R<Fn>, &'ar ns::Error> {
        let mut error = None;

        if let Some(func) =
            unsafe { Self::new_fn_with_consts_err(self, name, constant_values, &mut error) }
        {
            Ok(func)
        } else {
            Err(error.unwrap())
        }
    }

    #[objc::msg_send(newFunctionWithName:descriptor:error:)]
    pub unsafe fn new_fn_with_desc_err<'ar>(
        &self,
        name: &ns::String,
        descriptor: &mtl::FnDescriptor,
        error: &mut Option<&'ar ns::Error>,
    ) -> Option<arc::R<Fn>>;

    pub fn new_fn_with_desc<'ar>(
        &self,
        name: &ns::String,
        descriptor: &mtl::FnDescriptor,
    ) -> Result<arc::R<Fn>, &'ar ns::Error> {
        let mut error = None;

        if let Some(func) =
            unsafe { Self::new_fn_with_desc_err(self, name, descriptor, &mut error) }
        {
            Ok(func)
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }

    #[objc::msg_send(type)]
    pub fn type_(&self) -> mtl::LibType;

    /// The installName provided when this mtl::Lib was created.
    ///
    /// Always nil if the type of the library is not mtl::LibType::Dynamic.
    /// [read more](https://developer.apple.com/documentation/metal/mtllibrary/3554039-installname?language=objc)
    #[objc::msg_send(installName)]
    pub fn install_name(&self) -> Option<&ns::String>;
}

pub type ErrorDomain = ns::ErrorDomain;

impl ErrorDomain {
    pub fn lib() -> &'static ErrorDomain {
        unsafe { MTLLibraryErrorDomain }
    }
}

#[doc(alias = "MTLFunctionType")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum FnType {
    /// A vertex shader, usable for a mtl::RenderPipelineState.
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
    static MTL_COMPILE_OPTIONS: &'static objc::Class<CompileOptions>;
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
        device.new_lib_with_src_completion(&source, None, handler.escape());
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

        assert_eq!(mtl::LibError::CompileFailure, err.code());
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
        assert_eq!(func.options(), mtl::FnOptions::None);
    }

    #[test]
    fn new_function_with_name_constant_values() {
        let device = mtl::Device::default().unwrap();

        let source = ns::String::with_str("kernel void function_a() {}");
        let lib = device.new_lib_with_src(&source, None).unwrap();

        let func_name = ns::String::with_str_no_copy("function_a");
        let constant_values = mtl::FnConstValues::new();
        let func = lib
            .new_fn_with_consts(&func_name, &constant_values)
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

    #[test]
    fn install_name() {
        let device = mtl::Device::default().unwrap();
        let source = ns::String::with_str("kernel void function_a() {}");
        let lib = device.new_lib_with_src(&source, None).unwrap();

        assert!(lib.install_name().is_none());
        assert_eq!(mtl::LibType::Executable, lib.type_());
    }
}
