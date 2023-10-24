#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum IndexType {
    U16 = 0,
    U32 = 1,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum AttrFormat {
    /// An invalid format.
    #[doc(alias = "MTLAttributeFormatInvalid")]
    Invalid = 0,

    /// Two unsigned 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar2")]
    U8x2 = 1,

    /// Three unsigned 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar3")]
    U8x3 = 2,

    /// Four unsigned 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar4")]
    U8x4 = 3,

    /// Two signed 8-bit two's complement values.
    #[doc(alias = "MTLAttributeFormatChar2")]
    I8x2 = 4,

    /// Three signed 8-bit two's complement values.
    #[doc(alias = "MTLAttributeFormatChar3")]
    I8x3 = 5,

    /// Four signed 8-bit two's complement values.
    #[doc(alias = "MTLAttributeFormatChar4")]
    I8x4 = 6,

    /// Two unsigned normalized 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar2Normalized")]
    U8x2Normalized = 7,

    /// Three unsigned normalized 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar3Normalized")]
    U8x3Normalized = 8,

    /// Four unsigned normalized 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar4Normalized")]
    U8x4Normalized = 9,

    I8x2Normalized = 10,
    I8x3Normalized = 11,
    I8x4Normalized = 12,

    U16x2 = 13,
    U16x3 = 14,
    U16x4 = 15,

    I16x2 = 16,
    I16x3 = 17,
    I16x4 = 18,

    U16x2Normalized = 19,
    U16x3Normalized = 20,
    U16x4Normalized = 21,

    I16x2Normalized = 22,
    I16x3Normalized = 23,
    I16x4Normalized = 24,

    F16x2 = 25,
    F16x3 = 26,
    F16x4 = 27,

    F32 = 28,
    F32x2 = 29,
    F32x3 = 30,
    F32x4 = 31,

    I32 = 32,
    I32x2 = 33,
    I32x3 = 34,
    I32x4 = 35,

    U32 = 36,
    U32x2 = 37,
    U32x3 = 38,
    U32x4 = 39,

    I1010102Normalized = 40,
    U1010102Normalized = 41,

    UChar4NormalizedBGRA = 42,

    U8 = 45,
    I8 = 46,
    U8Normalized = 47,
    I8Normalized = 48,

    U16 = 49,
    I16 = 50,
    U16Normalized = 51,
    I16Normalized = 52,

    F16 = 53,
}
