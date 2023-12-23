use crate::{define_obj_type, ns, objc};

#[doc(alias = "MTLDataType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum DType {
    /// A placeholder that represents a GPU function parameter that doesn’t have a valid data type.
    None = 0,

    /// A structure instance.
    Struct = 1,

    /// An array instance.
    Array = 2,

    /// A 32-bit floating-point value.
    F32 = 3,

    /// A two-component vector with 32-bit floating-point values.
    F32x2 = 4,

    /// A three-component vector with 32-bit floating-point values.
    F32x3 = 5,

    /// A four-component vector with 32-bit floating-point values.
    F32x4 = 6,

    /// A 2x2 component matrix with 32-bit floating-point values.
    F32x2x2 = 7,

    /// A 2x3 component matrix with 32-bit floating-point values.
    F32x2x3 = 8,

    /// A 2x4 component matrix with 32-bit floating-point values.
    F32x2x4 = 9,

    /// A 3x2 component matrix with 32-bit floating-point values.
    F32x3x2 = 10,

    /// A 3x3 component matrix with 32-bit floating-point values.
    F32x3x3 = 11,

    /// A 3x4 component matrix with 32-bit floating-point values.
    F32x3x4 = 12,

    /// A 4x2 component matrix with 32-bit floating-point values.
    F32x4x2 = 13,

    /// A 4x3 component matrix with 32-bit floating-point values.
    F32x4x3 = 14,

    /// A 4x4 component matrix with 32-bit floating-point values.
    F32x4x4 = 15,

    /// A 16-bit floating-point value.
    F16 = 16,

    /// A two-component vector with 16-bit floating-point values.
    F16x2 = 17,
    F16x3 = 18,
    F16x4 = 19,

    F16x2x2 = 20,
    F16x2x3 = 21,
    F16x2x4 = 22,

    F16x3x2 = 23,
    F16x3x3 = 24,
    F16x3x4 = 25,

    F16x4x2 = 26,
    F16x4x3 = 27,
    F16x4x4 = 28,

    I32 = 29,
    I32x2 = 30,
    I32x3 = 31,
    I32x4 = 32,

    U32 = 33,
    U32x2 = 34,
    U32x3 = 35,
    U32x4 = 36,

    I16 = 37,
    I16x2 = 38,
    I16x3 = 39,
    I16x4 = 40,

    U16 = 41,
    U16x2 = 42,
    U16x3 = 43,
    U16x4 = 44,

    I8 = 45,
    I8x2 = 46,
    I8x3 = 47,
    I8x4 = 48,

    U8 = 49,
    U8x2 = 50,
    U8x3 = 51,
    U8x4 = 52,

    Bool = 53,
    Boolx2 = 54,
    Boolx3 = 55,
    Boolx4 = 56,

    Texture = 58,
    Sampler = 59,
    Pointer = 60,

    R8Unorm = 62,
    R8Snorm = 63,
    R16Unorm = 64,
    R16Snorm = 65,
    Rg8Unorm = 66,
    Rg8Snorm = 67,
    Rg16Unorm = 68,
    Rg16Snorm = 69,
    Rgba8Unorm = 70,
    Rgba8UnormSrgb = 71,
    Rgba8Snorm = 72,
    Rgba16Unorm = 73,
    Rgba16Snorm = 74,
    Rgb10A2Unorm = 75,
    Rg11B10Float = 76,
    Rgb9E5Float = 77,
    RenderPipeline = 78,
    ComputePipeline = 79,
    IndirectCmdBuf = 80,
    I64 = 81,
    I64x2 = 82,
    I64x3 = 83,
    I64x4 = 84,

    U64 = 85,
    U64x2 = 86,
    U64x3 = 87,
    U64x4 = 88,
    VisibleFnTable = 115,
    IntersectionFnTable = 116,
    PrimitiveAccStruct = 117,
    InstanceAccStruct = 118,
}

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

define_obj_type!(pub Arg(ns::Id));

impl Arg {
    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;
}
