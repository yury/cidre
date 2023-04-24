#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum IndexType {
    U16 = 0,
    U32 = 1,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum AttributeFormat {
    /// An invalid format.
    #[doc(alias = "MTLAttributeFormatInvalid")]
    Invalid = 0,

    /// Two unsigned 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar2")]
    UChar2 = 1,

    /// Three unsigned 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar3")]
    UChar3 = 2,

    /// Four unsigned 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar4")]
    UChar4 = 3,

    /// Two signed 8-bit two's complement values.
    #[doc(alias = "MTLAttributeFormatChar2")]
    Char2 = 4,

    /// Three signed 8-bit two's complement values.
    #[doc(alias = "MTLAttributeFormatChar3")]
    Char3 = 5,

    /// Four signed 8-bit two's complement values.
    #[doc(alias = "MTLAttributeFormatChar4")]
    Char4 = 6,

    /// Two unsigned normalized 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar2Normalized")]
    UChar2Normalized = 7,

    /// Three unsigned normalized 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar3Normalized")]
    UChar3Normalized = 8,

    /// Four unsigned normalized 8-bit values.
    #[doc(alias = "MTLAttributeFormatUChar4Normalized")]
    UChar4Normalized = 9,

    Char2Normalized = 10,
    Char3Normalized = 11,
    Char4Normalized = 12,

    UShort2 = 13,
    UShort3 = 14,
    UShort4 = 15,

    Short2 = 16,
    Short3 = 17,
    Short4 = 18,

    UShort2Normalized = 19,
    UShort3Normalized = 20,
    UShort4Normalized = 21,

    Short2Normalized = 22,
    Short3Normalized = 23,
    Short4Normalized = 24,

    Half2 = 25,
    Half3 = 26,
    Half4 = 27,

    Float = 28,
    Float2 = 29,
    Float3 = 30,
    Float4 = 31,

    Int = 32,
    Int2 = 33,
    Int3 = 34,
    Int4 = 35,

    UInt = 36,
    UInt2 = 37,
    UInt3 = 38,
    UInt4 = 39,

    Int1010102Normalized = 40,
    UInt1010102Normalized = 41,

    UChar4NormalizedBGRA = 42,

    UChar = 45,
    Char = 46,
    UCharNormalized = 47,
    CharNormalized = 48,

    UShort = 49,
    Short = 50,
    UShortNormalized = 51,
    ShortNormalized = 52,

    Half = 53,
}
