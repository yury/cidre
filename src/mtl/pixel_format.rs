#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum PixelFormat {
    Invalid = 0,

    /* Normal 8 bit formats */
    A8Unorm = 1,
    R8Unorm = 10,
    R8UnormSRGB = 11,
    R8Snorm = 12,
    R8Uint = 13,
    R8Sint = 14,

    /* Normal 16 bit formats */
    R16Unorm = 20,
    R16Snorm = 22,
    R16Uint = 23,
    R16Sint = 24,
    R16Float = 25,

    RG8Unorm = 30,
    RG8UnormSRGB = 31,
    RG8Snorm = 32,
    RG8Uint = 33,
    RG8Sint = 34,

    /* Packed 16 bit formats */
    B5G6R5Unorm = 40,
    A1BGR5Unorm = 41,
    ABGR4Unorm = 42,
    BGR5A1Unorm = 43,

    /* Normal 32 bit formats */
    R32Uint = 53,
    R32Sint = 54,
    R32Float = 55,

    RG16Unorm = 60,
    RG16Snorm = 62,
    RG16Uint = 63,
    RG16Sint = 64,
    RG16Float = 65,

    RGBA8Unorm = 70,
    RGBA8UnormSRGB = 71,
    RGBA8Snorm = 72,
    RGBA8Uint = 73,
    RGBA8Sint = 74,

    BGRA8Unorm = 80,
    BGRA8UnormSRGB = 81,

    /* Packed 32 bit formats */
    RGB10A2Unorm = 90,
    RGB10A2Uint = 91,

    RG11B10Float = 92,
    RGB9E5Float = 93,

    BGR10A2Unorm = 94,

    BGR10XR = 554,
    BGR10XRSRGB = 555,

    /* Normal 64 bit formats */
    RG32Uint = 103,
    RG32Sint = 104,
    RG32Float = 105,

    RGBA16Unorm = 110,
    RGBA16Snorm = 112,
    RGBA16Uint = 113,
    RGBA16Sint = 114,
    RGBA16Float = 115,

    ASTC4x4sRGB = 186,
    ASTC5x4sRGB = 187,
    ASTC5x5sRGB = 188,
    ASTC6x5sRGB = 189,
    ASTC6x6sRGB = 190,
    ASTC8x5sRGB = 192,
    ASTC8x6sRGB = 193,
    ASTC8x8sRGB = 194,
    ASTC10x5sRGB = 195,
    ASTC10x6sRGB = 196,
    ASTC10x8sRGB = 197,
    ASTC10x10sRGB = 198,
    ASTC12x10sRGB = 199,
    ASTC12x12sRGB = 200,

    ASTC4x4LDR  = 204,
    ASTC5x4LDR  = 205,
    ASTC5x5LDR  = 206,
    ASTC6x5LDR  = 207,
    ASTC6x6LDR  = 208,
    ASTC8x5LDR  = 210,
    ASTC8x6LDR  = 211,
    ASTC8x8LDR  = 212,
    ASTC10x5LDR  = 213,
    ASTC10x6LDR  = 214,
    ASTC10x8LDR  = 215,
    ASTC10x10LDR = 216,
    ASTC12x10LDR = 217,
    ASTC12x12LDR = 218,

    ASTC4x4HDR = 222,
    ASTC5x4HDR = 223,
    ASTC5x5HDR = 224,
    ASTC6x5HDR = 225,
    ASTC6x6HDR = 226,
    ASTC8x5HDR = 228,
    ASTC8x6HDR = 229,
    ASTC8x8HDR = 230,
    ASTC10x5HDR = 231,
    ASTC10x6HDR = 232,
    ASTC10x8HDR = 233,
    ASTC10x10HDR = 234,
    ASTC12x10HDR = 235,
    ASTC12x12HDR = 236,

    BGRA10XR = 552,
    BGRA10XRSRGB = 553,
}
