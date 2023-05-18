use crate::{define_obj_type, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum DataType {
    None = 0,

    Struct = 1,
    Array = 2,

    F32 = 3,
    F32x2 = 4,
    F32x3 = 5,
    F32x4 = 6,

    F32x2x2 = 7,
    F32x2x3 = 8,
    F32x2x4 = 9,

    F32x3x2 = 10,
    F32x3x3 = 11,
    F32x3x4 = 12,

    F32x4x2 = 13,
    F32x4x3 = 14,
    F32x4x4 = 15,

    F16 = 16,
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
    RG8Unorm = 66,
    RG8Snorm = 67,
    RG16Unorm = 68,
    RG16Snorm = 69,
    RGBA8Unorm = 70,
    RGBA8UnormSRGB = 71,
    RGBA8Snorm = 72,
    RGBA16Unorm = 73,
    RGBA16Snorm = 74,
    RGB10A2Unorm = 75,
    RG11B10Float = 76,
    RGB9E5Float = 77,
    RenderPipeline = 78,
    ComputePipeline = 79,
    IndirectCommandBuffer = 80,
    I64 = 81,
    I64x2 = 82,
    I64x3 = 83,
    I64x4 = 84,

    U64 = 85,
    U64x2 = 86,
    U64x3 = 87,
    U64x4 = 88,
    VisibleFunctionTable = 115,
    IntersectionFunctionTable = 116,
    PrimitiveAccelerationStructure = 117,
    InstanceAccelerationStructure = 118,
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
    VisibleFunctionTable = 24,
    PrimitiveAccelerationStructure = 25,
    InstanceAccelerationStructure = 26,
    IntersectionFunctionTable = 27,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Access {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}

define_obj_type!(BaseType(ns::Id));

impl BaseType {}

define_obj_type!(StructMember(ns::Id));

impl StructMember {}

define_obj_type!(StructType(BaseType));
define_obj_type!(ArrayType(BaseType));
define_obj_type!(PointerType(BaseType));
define_obj_type!(TextureReferenceType(BaseType));

define_obj_type!(Argument(ns::Id));

impl Argument {
    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;
}
