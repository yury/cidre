use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(
    /// A specification of how to create a visible function table.
    #[doc(alias = "MTLVisibleFunctionTableDescriptor")]
    pub Desc(ns::Id),
    MTL_VISIBLE_FUNCTION_TABLE_DESCRIPTOR
);

impl Desc {
    /// The number of functions in the table.
    #[objc::msg_send(functionCount)]
    pub fn fn_count(&self) -> usize;

    #[objc::msg_send(setFunctionCount:)]
    pub fn set_fn_count(&mut self, value: usize);
}

define_obj_type!(
    /// A table of shader functions visible to your app that you can pass into
    /// compute commands to customize the behavior of a shader.
    #[doc(alias = "MTLVisibleFunctionTable")]
    pub VisibleFnTable(mtl::Res)
);

impl VisibleFnTable {
    define_mtl!(gpu_res_id);

    /// Sets a table entry to point to a callable function.
    #[objc::msg_send(setFunction:atIndex:)]
    pub fn set_fn_at<O: objc::Obj, H: mtl::FnHandle<O>>(
        &mut self,
        function: Option<&H>,
        index: usize,
    );

    /// Sets a range of table entries to point to an array of callable functions.
    ///
    /// # Arguments
    /// * `functions` - An array of function handles for the functions to be called.
    /// * `range` - A range of indices to change in the table.
    #[objc::msg_send(setFunctions:withRange:)]
    pub fn set_fns_with_range<O: objc::Obj, H: mtl::FnHandle<O>>(
        &mut self,
        functions: *const Option<&H>,
        range: ns::Range,
    );
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_VISIBLE_FUNCTION_TABLE_DESCRIPTOR: &'static objc::Class<Desc>;
}
