use crate::{arc, define_obj_type, define_opts, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLFunctionDescriptor")]
    pub FnDesc(ns::Id), MTL_FUNCTION_DESCRIPTOR
);

define_opts!(
    #[doc(alias = "MTLFunctionOptions")]
    pub FnOpts(usize)
);

impl FnOpts {
    /// Default usage
    #[doc(alias = "MTLFunctionOptionNone")]
    pub const NONE: Self = Self(0);

    /// Compiles the found function. This enables dynamic linking of this `mtl::Fn`.
    /// Only supported for `visible` functions.
    #[doc(alias = "MTLFunctionOptionCompileToBinary")]
    pub const COMPILE_TO_BINARY: Self = Self(1 << 0);

    /// Stores and tracks this function in a Metal Pipelines Script
    ///
    /// This flag is optional and only supported in the context of binary archives.
    /// This flag is required for inspecting and consuming binary archives with specialized
    /// mtl::Fns via the metal-source tool. It is not required for recompilation, nor for storing
    /// functions in binary archives. Set this flag only if you intend to use metal-source on
    /// a serialized binary archive.
    #[doc(alias = "MTLFunctionOptionStoreFunctionInMetalPipelinesScript")]
    pub const STORE_FN_IN_METAL_PIPELINES_SCRIPT: Self = Self(1 << 1);

    /// Function creation fails (i.e nil is returned) if:
    ///
    /// - A lookup binary archive has been specified
    /// - The function has not been found in the archive
    #[doc(alias = "MTLFunctionOptionFailOnBinaryArchiveMiss")]
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1 << 1);
}

impl FnDesc {
    /// The name of the `visible` function to find.
    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    /// The name of the `visible` function to find.
    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, name: Option<&ns::String>);

    /// An optional new name for a `visible` function to allow reuse with different specializations.
    #[objc::msg_send(specializedName)]
    pub fn specialized_name(&self) -> Option<arc::R<ns::String>>;

    /// An optional new name for a `visible` function to allow reuse with different specializations.
    #[objc::msg_send(setSpecializedName:)]
    pub fn set_specialized_name(&mut self, name: Option<&ns::String>);

    /// The set of constant values assigned to the function constants. Compilation fails if you do not provide
    /// valid constant values for all required function constants.
    #[objc::msg_send(constantValues)]
    pub fn const_values(&self) -> Option<arc::R<mtl::FnConstValues>>;

    /// The set of constant values assigned to the function constants. Compilation fails if you do not provide
    /// valid constant values for all required function constants.
    #[objc::msg_send(setConstantValues:)]
    pub fn set_const_values(&mut self, val: Option<&mtl::FnConstValues>);

    /// The options to use for this new `mtl::Fn`.
    #[objc::msg_send(options)]
    pub fn opts(&self) -> mtl::FnOpts;

    /// The options to use for this new `mtl::Fn`.
    #[objc::msg_send(setOptions:)]
    pub fn set_opts(&mut self, val: mtl::FnOpts);
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL_FUNCTION_DESCRIPTOR: &'static objc::Class<FnDesc>;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let mut fd = mtl::FnDesc::new();

        assert!(fd.name().is_none());

        let name = ns::str!(c"hello");

        fd.set_name(Some(name));

        let actual_name = fd.name().unwrap();

        assert_eq!(name, &actual_name);
    }
}
