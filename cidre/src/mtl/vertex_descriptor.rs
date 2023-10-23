use crate::{arc, define_obj_type, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum VertexFormat {
    Invalid = 0,

    /// Two unsigned 8-bit values.
    #[doc(alias = "MTLVertexFormatUChar2")]
    U8x2 = 1,
    /// Three unsigned 8-bit values.
    #[doc(alias = "MTLVertexFormatUChar3")]
    U8x3 = 2,
    /// Four unsigned 8-bit values.
    #[doc(alias = "MTLVertexFormatUChar4")]
    U8x4 = 3,

    /// Two signed 8-bit two's complement values.
    #[doc(alias = "MTLVertexFormatChar2")]
    I8x2 = 4,

    /// Three signed 8-bit two's complement values.
    #[doc(alias = "MTLVertexFormatChar3")]
    I8x3 = 5,

    /// Four signed 8-bit two's complement values.
    #[doc(alias = "MTLVertexFormatChar4")]
    I8x4 = 6,

    /// Two unsigned normalized 8-bit values.
    #[doc(alias = "MTLVertexFormatUChar2Normalized")]
    U8x2Normalized = 7,

    /// Three unsigned normalized 8-bit values.
    #[doc(alias = "MTLVertexFormatUChar3Normalized")]
    U8x3Normalized = 8,

    //// Four unsigned normalized 8-bit values.
    #[doc(alias = "MTLVertexFormatUChar4Normalized")]
    U8x4Normalized = 9,

    /// Two signed normalized 8-bit two's complement values.
    #[doc(alias = "MTLVertexFormatChar2Normalized")]
    I8x2Normalized = 10,

    /// Three signed normalized 8-bit two's complement values.
    #[doc(alias = "MTLVertexFormatChar3Normalized")]
    I8x3Normalized = 11,

    /// Four signed normalized 8-bit two's complement values.
    #[doc(alias = "MTLVertexFormatChar4Normalized")]
    I8x4Normalized = 12,

    /// Two unsigned 16-bit values.
    #[doc(alias = "MTLVertexFormatUShort2")]
    U16x2 = 13,

    /// Three unsigned 16-bit values.
    #[doc(alias = "MTLVertexFormatUShort3")]
    U16x3 = 14,

    /// Four unsigned 16-bit values.
    #[doc(alias = "MTLVertexFormatUShort4")]
    U16x4 = 15,

    /// Two signed 16-bit two's complement values.
    #[doc(alias = "MTLVertexFormatShort2")]
    I16x2 = 16,

    /// Three signed 16-bit two's complement values.
    #[doc(alias = "MTLVertexFormatShort3")]
    I16x3 = 17,

    /// Four signed 16-bit two's complement values.
    #[doc(alias = "MTLVertexFormatShort3")]
    I16x4 = 18,

    /// Two unsigned normalized 16-bit values.
    #[doc(alias = "MTLVertexFormatUShort2Normalized")]
    U16x2Normalized = 19,

    /// Three unsigned normalized 16-bit values.
    #[doc(alias = "MTLVertexFormatUShort3Normalized")]
    U16x3Normalized = 20,

    /// Four unsigned normalized 16-bit values.
    #[doc(alias = "MTLVertexFormatUShort4Normalized")]
    U16x4Normalized = 21,

    /// Two signed normalized 16-bit two's complement values.
    #[doc(alias = "MTLVertexFormatShort2Normalized")]
    I16x2Normalized = 22,

    /// Three signed normalized 16-bit two's complement values.
    #[doc(alias = "MTLVertexFormatShort3Normalized")]
    I16x3Normalized = 23,

    /// Four signed normalized 16-bit two's complement values.
    #[doc(alias = "MTLVertexFormatShort4Normalized")]
    I16x4Normalized = 24,

    /// Two half-precision floating-point values.
    #[doc(alias = "MTLVertexFormatHalf2")]
    F16x2 = 25,

    /// Three half-precision floating-point values.
    #[doc(alias = "MTLVertexFormatHalf3")]
    F16x3 = 26,

    /// Four half-precision floating-point values.
    #[doc(alias = "MTLVertexFormatHalf4")]
    F16x4 = 27,

    /// One single-precision floating-point value.
    #[doc(alias = "MTLVertexFormatFloat")]
    F32 = 28,

    /// Two single-precision floating-point values.
    #[doc(alias = "MTLVertexFormatFloat2")]
    F32x2 = 29,

    /// Three single-precision floating-point values.
    #[doc(alias = "MTLVertexFormatFloat3")]
    F32x3 = 30,

    /// Four single-precision floating-point values.
    #[doc(alias = "MTLVertexFormatFloat4")]
    F32x4 = 31,

    /// One signed 32-bit two's complement value.
    #[doc(alias = "MTLVertexFormatInt")]
    I32 = 32,

    /// Two signed 32-bit two's complement values.
    #[doc(alias = "MTLVertexFormatInt2")]
    I32x2 = 33,

    /// Three signed 32-bit two's complement values.
    #[doc(alias = "MTLVertexFormatInt3")]
    I32x3 = 34,

    /// Four signed 32-bit two's complement values.
    #[doc(alias = "MTLVertexFormatInt4")]
    I32x4 = 35,

    /// One unsigned 32-bit value.
    #[doc(alias = "MTLVertexFormatUInt")]
    U32 = 36,

    /// Two unsigned 32-bit values.
    #[doc(alias = "MTLVertexFormatUInt2")]
    U32x2 = 37,

    /// Three unsigned 32-bit values.
    #[doc(alias = "MTLVertexFormatUInt3")]
    U32x3 = 38,

    /// Four unsigned 32-bit values.
    #[doc(alias = "MTLVertexFormatUInt4")]
    U32x4 = 39,

    /// One packed 32-bit value with four normalized
    /// signed two's complement integer values, arranged as
    /// 10 bits, 10 bits, 10 bits, and 2 bits.
    #[doc(alias = "MTLVertexFormatInt1010102Normalized")]
    I1010102Normalized = 40,

    /// One packed 32-bit value with four normalized
    /// unsigned integer values, arranged as
    /// 10 bits, 10 bits, 10 bits, and 2 bits.
    #[doc(alias = "MTLVertexFormatUInt1010102Normalized")]
    U1010102Normalized = 41,

    /// Four unsigned normalized 8-bit values, arranged as
    /// blue, green, red, and alpha components.
    #[doc(alias = "MTLVertexFormatUChar4Normalized_BGRA")]
    U8x4NormalizedBGRA = 42,

    /// One unsigned 8-bit value.
    #[doc(alias = "MTLVertexFormatUChar")]
    U8 = 45,

    /// One signed 8-bit two's complement value.
    #[doc(alias = "MTLVertexFormatChar")]
    I8 = 46,

    /// One unsigned normalized 8-bit value.
    #[doc(alias = "MTLVertexFormatUCharNormalized")]
    U8Normalized = 47,

    /// One signed normalized 8-bit two's complement value.
    #[doc(alias = "MTLVertexFormatCharNormalized")]
    I8Normalized = 48,

    /// One unsigned 16-bit value.
    #[doc(alias = "MTLVertexFormatUShort")]
    U16 = 49,

    /// One signed 16-bit two's complement value.
    #[doc(alias = "MTLVertexFormatShort")]
    I16 = 50,

    /// One unsigned normalized 16-bit value.
    #[doc(alias = "MTLVertexFormatUShortNormalized")]
    U16Normalized = 51,

    /// One signed normalized 16-bit two's complement value.
    #[doc(alias = "MTLVertexFormatShortNormalized")]
    I16Normalized = 52,

    /// One half-precision floating-point value.
    #[doc(alias = "MTLVertexFormatHalf")]
    F16 = 53,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum VertexStepFn {
    /// The vertex function fetches attribute data once
    /// and uses that data for every vertex.
    #[doc(alias = "MTLVertexStepFunctionConstant")]
    Constant = 0,

    /// The vertex function fetches and uses new attribute
    /// data for every vertex.
    #[doc(alias = "MTLVertexStepFunctionPerVertex")]
    PerVertex = 1,

    /// The vertex function regularly fetches new attribute data for a number
    /// of instances that is determined by step_rate.
    #[doc(alias = "MTLVertexStepFunctionPerInstance")]
    PerInstance = 2,

    /// The post-tessellation vertex function fetches data based
    /// on the patch index of the patch.
    #[doc(alias = "MTLVertexStepFunctionPerPatch")]
    PerPatch = 3,

    /// The post-tessellation vertex function fetches data based on
    /// the control-point indices associated with the patch.
    #[doc(alias = "MTLVertexStepFunctionPerPatchControlPoint")]
    PerPatchControlPoint = 4,
}

define_obj_type!(
    VertexBufLayoutDesc(ns::Id),
    MTL_VERTEX_BUFFER_LAYOUT_DESCRIPTOR
);
impl VertexBufLayoutDesc {
    /// The distance, in bytes, between the attribute data
    /// of two vertices in the buffer.
    ///
    /// The stride must be a multiple of 4 bytes. The default value is 0.
    #[objc::msg_send(stride)]
    pub fn stride(&self) -> usize;

    #[objc::msg_send(setStride:)]
    pub fn set_stride(&mut self, value: usize);

    /// The circumstances under which the vertex and its attributes
    /// are presented to the vertex function.
    #[objc::msg_send(stepFunction)]
    pub fn step_fn(&self) -> VertexStepFn;

    #[objc::msg_send(setStepFunction:)]
    pub fn set_step_fn(&mut self, value: VertexStepFn);

    /// The interval at which the vertex and its attributes are presented
    /// to the vertex function.
    #[objc::msg_send(stepRate)]
    pub fn step_rate(&self) -> usize;

    #[objc::msg_send(setStepRate:)]
    pub fn set_step_rate(&mut self, value: usize);
}

define_obj_type!(VertexBufLayoutDescArray(ns::Id));
impl VertexBufLayoutDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn object_at(&self, index: usize) -> &VertexBufLayoutDesc;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn object_at_mut(&mut self, index: usize) -> &mut VertexBufLayoutDesc;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set_object_at(&mut self, buffer_desc: Option<&VertexBufLayoutDesc>, index: usize);
}

impl std::ops::Index<usize> for VertexBufLayoutDescArray {
    type Output = VertexBufLayoutDesc;

    fn index(&self, index: usize) -> &Self::Output {
        self.object_at(index)
    }
}

impl std::ops::IndexMut<usize> for VertexBufLayoutDescArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.object_at_mut(index)
    }
}

define_obj_type!(VertexAttributeDesc(ns::Id), MTL_VERTEX_ATTRIBUTE_DESCRIPTOR);

impl VertexAttributeDesc {
    #[objc::msg_send(format)]
    pub fn format(&self) -> VertexFormat;

    #[objc::msg_send(setFormat:)]
    pub fn set_format(&mut self, value: VertexFormat);

    #[objc::msg_send(offset)]
    pub fn offset(&self) -> usize;

    #[objc::msg_send(setOffset:)]
    pub fn set_offset(&mut self, value: usize);

    #[objc::msg_send(bufferIndex)]
    pub fn buf_index(&self) -> usize;

    #[objc::msg_send(setBufferIndex:)]
    pub fn set_buf_index(&self, value: usize);
}

define_obj_type!(VertexAttributeDescArray(ns::Id));
impl VertexAttributeDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn object_at(&self, index: usize) -> &VertexAttributeDesc;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn object_at_mut(&mut self, index: usize) -> &mut VertexAttributeDesc;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set_object_at(&mut self, buffer_desc: Option<&VertexAttributeDesc>, index: usize);
}

impl std::ops::Index<usize> for VertexAttributeDescArray {
    type Output = VertexAttributeDesc;

    fn index(&self, index: usize) -> &Self::Output {
        self.object_at(index)
    }
}

impl std::ops::IndexMut<usize> for VertexAttributeDescArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.object_at_mut(index)
    }
}

define_obj_type!(Desc(ns::Id), MTL_VERTEX_DESCRIPTOR);
impl Desc {
    #[objc::msg_send(layouts)]
    pub fn layouts(&self) -> &VertexBufLayoutDescArray;

    #[objc::msg_send(layouts)]
    pub fn layouts_mut(&mut self) -> &mut VertexBufLayoutDescArray;

    #[objc::msg_send(attributes)]
    pub fn attributes(&self) -> &VertexAttributeDescArray;

    #[objc::msg_send(attributes)]
    pub fn attributes_mut(&mut self) -> &mut VertexAttributeDescArray;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_VERTEX_DESCRIPTOR: &'static objc::Class<Desc>;
    static MTL_VERTEX_BUFFER_LAYOUT_DESCRIPTOR: &'static objc::Class<VertexBufLayoutDesc>;
    static MTL_VERTEX_ATTRIBUTE_DESCRIPTOR: &'static objc::Class<VertexAttributeDesc>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let mut descriptor = mtl::VertexDesc::new();
        let attrs = descriptor.attributes_mut();
        attrs[0].set_format(mtl::VertexFormat::U8x2);
        assert_eq!(descriptor.attributes()[0].format(), mtl::VertexFormat::U8x2);
        let descriptor = mtl::VertexBufLayoutDesc::new();
        assert_eq!(descriptor.stride(), 0);
    }
}
