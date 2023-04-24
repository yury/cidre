use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(Descriptor(ns::Id), MTL_VISIBLE_FUNCTION_TABLE_DESCRIPTOR);

impl Descriptor {
    /// The number of functions in the table.
    #[objc::msg_send(functionCount)]
    pub fn fn_count(&self) -> usize;

    #[objc::msg_send(setFunctionCount:)]
    pub fn set_fn_count(&mut self, value: usize);

    // pub fn set_fn_at(&mut self, function: Option<&mtl::FnHandle>, index: usize);
    //- (void)setFunction:(nullable id <MTLFunctionHandle>)function atIndex:(NSUInteger)index;
}

define_obj_type!(VisibleFnTable(mtl::Resource));

impl VisibleFnTable {
    define_mtl!(gpu_resource_id);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_VISIBLE_FUNCTION_TABLE_DESCRIPTOR: &'static objc::Class<Descriptor>;
}
