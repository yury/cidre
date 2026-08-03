use crate::{arc, define_obj_type, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Type {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    Sampler = 3,

    ImageblockData = 16,
    Imageblock = 17,
    VisibleFnTable = 24,
    PrimitiveAccStruct = 25,
    InstanceAccStruct = 26,
    IntersectionFnTable = 27,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Access {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}

define_obj_type!(pub BaseType(ns::Id));

impl BaseType {}

define_obj_type!(pub StructMember(ns::Id));

impl StructMember {}

define_obj_type!(pub StructType(BaseType));
define_obj_type!(pub ArrayType(BaseType));
define_obj_type!(pub PointerType(BaseType));
define_obj_type!(pub TextureRefType(BaseType));

define_obj_type!(
    #[doc(alias = "MTLArgument")]
    pub Arg(ns::Id)
);

define_obj_type!(
    #[doc(alias = "MTLBinding")]
    pub Binding(ns::Id)
);

impl Arg {
    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<ns::String>;
}
