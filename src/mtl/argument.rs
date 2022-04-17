use crate::{cf, define_obj_type, msg_send, objc::Id};

#[repr(usize)]
pub enum DataType {
    None = 0,

    Struct = 1,
    Array = 2,

    Float = 3,
    Float2 = 4,
    Float3 = 5,
    Float4 = 6,

    Float2x2 = 7,
    Float2x3 = 8,
    Float2x4 = 9,

    Float3x2 = 10,
    Float3x3 = 11,
    Float3x4 = 12,

    Float4x2 = 13,
    Float4x3 = 14,
    Float4x4 = 15,

    Half = 16,
    Half2 = 17,
    Half3 = 18,
    Half4 = 19,

    Half2x2 = 20,
    Half2x3 = 21,
    Half2x4 = 22,

    Half3x2 = 23,
    Half3x3 = 24,
    Half3x4 = 25,

    Half4x2 = 26,
    Half4x3 = 27,
    Half4x4 = 28,

    Int = 29,
    Int2 = 30,
    Int3 = 31,
    Int4 = 32,

    UInt = 33,
    UInt2 = 34,
    UInt3 = 35,
    UInt4 = 36,

    Short = 37,
    Short2 = 38,
    Short3 = 39,
    Short4 = 40,

    UShort = 41,
    UShort2 = 42,
    UShort3 = 43,
    UShort4 = 44,

    Char = 45,
    Char2 = 46,
    Char3 = 47,
    Char4 = 48,

    UChar = 49,
    UChar2 = 50,
    UChar3 = 51,
    UChar4 = 52,

    Bool = 53,
    Bool2 = 54,
    Bool3 = 55,
    Bool4 = 56,

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
    Long = 81,
    Long2 = 82,
    Long3 = 83,
    Long4 = 84,

    ULong = 85,
    ULong2 = 86,
    ULong3 = 87,
    ULong4 = 88,
    VisibleFunctionTable = 115,
    IntersectionFunctionTable = 116,
    PrimitiveAccelerationStructure = 117,
    InstanceAccelerationStructure = 118,
}

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

#[repr(usize)]
pub enum Access {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}

define_obj_type!(BaseType(Id));

impl BaseType {}

define_obj_type!(StructMember(Id));

impl StructMember {}

define_obj_type!(StructType(BaseType));
define_obj_type!(ArrayType(BaseType));
define_obj_type!(PointerType(BaseType));
define_obj_type!(TextureReferenceType(BaseType));

define_obj_type!(Argument(Id));

impl Argument {
    pub fn name(&self) -> &cf::String {
        msg_send!("mtl", self, sel_name)
    }
}
