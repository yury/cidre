use crate::{arc, define_obj_type, mtl, ns, objc};

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

#[doc(alias = "MTLBindingType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
#[repr(isize)]
pub enum BindingType {
    #[doc(alias = "MTLBindingTypeBuffer")]
    Buf = 0,
    #[doc(alias = "MTLBindingTypeThreadgroupMemory")]
    ThreadgroupMemory = 1,
    #[doc(alias = "MTLBindingTypeTexture")]
    Texture = 2,
    #[doc(alias = "MTLBindingTypeSampler")]
    Sampler = 3,
    #[doc(alias = "MTLBindingTypeImageblockData")]
    ImageblockData = 16,
    #[doc(alias = "MTLBindingTypeImageblock")]
    Imageblock = 17,
    #[doc(alias = "MTLBindingTypeVisibleFunctionTable")]
    VisibleFnTable = 24,
    #[doc(alias = "MTLBindingTypePrimitiveAccelerationStructure")]
    PrimitiveAccStruct = 25,
    #[doc(alias = "MTLBindingTypeInstanceAccelerationStructure")]
    InstanceAccStruct = 26,
    #[doc(alias = "MTLBindingTypeIntersectionFunctionTable")]
    IntersectionFnTable = 27,
    #[doc(alias = "MTLBindingTypeObjectPayload")]
    ObjPayload = 34,
    #[doc(alias = "MTLBindingTypeTensor")]
    Tensor = 37,
}

impl Binding {
    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> BindingType;

    #[objc::msg_send(access)]
    pub fn access(&self) -> Access;

    #[objc::msg_send(index)]
    pub fn index(&self) -> usize;

    #[objc::msg_send(isUsed)]
    pub fn is_used(&self) -> bool;

    #[objc::msg_send(isArgument)]
    pub fn is_arg(&self) -> bool;

    /// Returns tensor binding if type is [`BindingType::Tensor`]
    #[inline]
    pub fn as_tensor(&self) -> Option<&TensorBinding> {
        if self.type_() == BindingType::Tensor {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }
}

define_obj_type!(
    /// An object that represents a tensor bound to a graphics or compute function
    /// or a machine learning function.
    #[doc(alias = "MTLTensorBinding")]
    pub TensorBinding(Binding)
);

impl TensorBinding {
    /// The underlying data format of this tensor.
    #[objc::msg_send(tensorDataType)]
    pub fn tensor_d_type(&self) -> mtl::TensorDType;

    /// The data format you use for indexing into the tensor.
    #[objc::msg_send(indexType)]
    pub fn index_type(&self) -> mtl::DType;

    /// The array of sizes, in elements, one for each dimension of this tensor.
    ///
    /// For machine learning pipelines it is the default shape or `None`
    /// in the case of an undefined shape.
    #[objc::msg_send(dimensions)]
    pub fn dims(&self) -> Option<arc::R<mtl::TensorExtents>>;
}

impl Arg {
    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<ns::String>;
}
