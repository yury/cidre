#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum PixelFormat {
    /// The default value of the pixel format for the mtl::RenderPipelineState.
    /// You cannot create a texture with this value.
    #[doc(alias = "MTLPixelFormatInvalid")]
    Invalid = 0,

    /* Normal 8 bit formats */
    /// Ordinary format with one 8-bit normalized unsigned integer component
    #[doc(alias = "MTLPixelFormatA8Unorm")]
    A8UNorm = 1,

    /// Ordinary format with one 8-bit normalized unsigned integer component
    #[doc(alias = "MTLPixelFormatR8Unorm")]
    R8UNorm = 10,

    /// Ordinary format with one 8-bit normalized unsigned integer component with
    /// conversion between sRGB and linear space.
    #[doc(alias = "MTLPixelFormatR8Unorm_sRGB")]
    R8UNormSrgb = 11,

    /// Ordinary format with one 8-bit normalized signed integer component.
    #[doc(alias = "MTLPixelFormatR8Snorm")]
    R8SNorm = 12,

    /// Ordinary format with one 8-bit unsigned integer component.
    #[doc(alias = "MTLPixelFormatR8Uint")]
    R8UInt = 13,

    /// Ordinary format with one 8-bit signed integer component.
    #[doc(alias = "MTLPixelFormatR8Sint")]
    R8SInt = 14,

    /* Normal 16 bit formats */
    /// Ordinary format with one 16-bit normalized unsigned integer component.
    #[doc(alias = "MTLPixelFormatR16Unorm")]
    R16UNorm = 20,

    /// Ordinary format with one 16-bit normalized signed integer component.
    #[doc(alias = "MTLPixelFormatR16Snorm")]
    R16SNorm = 22,

    /// Ordinary format with one 16-bit unsigned integer component.
    #[doc(alias = "MTLPixelFormatR16Uint")]
    R16UInt = 23,

    /// Ordinary format with one 16-bit signed integer component.
    #[doc(alias = "MTLPixelFormatR16Sint")]
    R16SInt = 24,

    /// Ordinary format with one 16-bit floating-point component.
    #[doc(alias = "MTLPixelFormatR16Float")]
    R16Float = 25,

    /// Ordinary format with two 8-bit normalized unsigned integer components.
    #[doc(alias = "MTLPixelFormatRG8Unorm")]
    Rg8UNorm = 30,

    /// Ordinary format with two 8-bit normalized unsigned integer components
    /// with conversion between sRGB and linear space.
    #[doc(alias = "MTLPixelFormatRG8Unorm_sRGB")]
    Rg8UNormSrgb = 31,

    /// Ordinary format with two 8-bit normalized signed integer components.
    #[doc(alias = "MTLPixelFormatRG8Snorm")]
    Rg8SNorm = 32,

    /// Ordinary format with two 8-bit unsigned integer components.
    #[doc(alias = "MTLPixelFormatRG8Uint")]
    Rg8UInt = 33,

    /// Ordinary format with two 8-bit signed integer components.
    #[doc(alias = "MTLPixelFormatRG8Sint")]
    Rg8SInt = 34,

    /* Packed 16 bit formats */
    /// Packed 16-bit format with normalized unsigned integer color components:
    /// 5 bits for blue, 6 bits for green, 5 bits for red, packed into 16 bits.
    #[doc(alias = "MTLPixelFormatB5G6R5Unorm")]
    B5G6R5UNorm = 40,

    /// Packed 16-bit format with normalized unsigned integer color components:
    /// 5 bits each for BGR and 1 for alpha, packed into 16 bits.
    #[doc(alias = "MTLPixelFormatA1BGR5Unorm")]
    A1Bgr5UNorm = 41,

    /// Packed 16-bit format with normalized unsigned integer color components:
    /// 4 bits each for ABGR, packed into 16 bits.
    #[doc(alias = "MTLPixelFormatABGR4Unorm")]
    Abgr4UNorm = 42,

    /// Packed 16-bit format with normalized unsigned integer color components:
    /// 5 bits each for BGR and 1 for alpha, packed into 16 bits.
    #[doc(alias = "MTLPixelFormatBGR5A1Unorm")]
    Bgr5A1UNorm = 43,

    /* Normal 32 bit formats */
    /// Ordinary format with one 32-bit unsigned integer component.
    #[doc(alias = "MTLPixelFormatR32Uint")]
    R32UInt = 53,

    /// Ordinary format with one 32-bit signed integer component.
    #[doc(alias = "MTLPixelFormatR32Sint")]
    R32SInt = 54,

    /// Ordinary format with one 32-bit floating-point component.
    #[doc(alias = "MTLPixelFormatR32Float")]
    R32Float = 55,

    /// Ordinary format with two 16-bit normalized unsigned integer components.
    #[doc(alias = "MTLPixelFormatRG16Unorm")]
    Rg16UNorm = 60,

    /// Ordinary format with two 16-bit normalized signed integer components.
    #[doc(alias = "MTLPixelFormatRG16Snorm")]
    Rg16SNorm = 62,

    /// Ordinary format with two 16-bit unsigned integer components.
    #[doc(alias = "MTLPixelFormatRG16Uint")]
    Rg16UInt = 63,

    /// Ordinary format with two 16-bit signed integer components.
    #[doc(alias = "MTLPixelFormatRG16Sint")]
    Rg16SInt = 64,

    /// Ordinary format with two 16-bit floating-point components.
    #[doc(alias = "MTLPixelFormatRG16Float")]
    Rg16Float = 65,

    /// Ordinary format with four 8-bit normalized unsigned integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA8Unorm")]
    Rgba8UNorm = 70,

    /// Ordinary format with four 8-bit normalized unsigned integer components in RGBA order with conversion between sRGB and linear space.
    #[doc(alias = "MTLPixelFormatRGBA8Unorm_sRGB")]
    Rgba8UNormSrgb = 71,

    /// Ordinary format with four 8-bit normalized signed integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA8Snorm")]
    Rgba8SNorm = 72,

    /// Ordinary format with four 8-bit unsigned integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA8Sint")]
    Rgba8UInt = 73,

    /// Ordinary format with four 8-bit signed integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA8Sint")]
    Rgba8SInt = 74,

    /// Ordinary format with four 8-bit normalized unsigned integer components in BGRA order.
    #[doc(alias = "MTLPixelFormatBGRA8Unorm")]
    Bgra8UNorm = 80,

    /// Ordinary format with four 8-bit normalized unsigned integer components in BGRA order
    /// with conversion between sRGB and linear space.
    #[doc(alias = "MTLPixelFormatBGRA8Unorm_sRGB")]
    Bgra8UNormSrgb = 81,

    /* Packed 32 bit formats */
    /// A 32-bit packed pixel format with four normalized unsigned integer components:
    /// 10-bit red, 10-bit green, 10-bit blue, and 2-bit alpha.
    #[doc(alias = "MTLPixelFormatRGB10A2Unorm")]
    Rgb10A2UNorm = 90,

    /// A 32-bit packed pixel format with four unsigned integer components: 10-bit red, 10-bit green, 10-bit blue, and 2-bit alpha.
    ///
    /// Pixel data is stored in red, green, blue, and alpha order, from least significant bit to most significant bit.
    #[doc(alias = "MTLPixelFormatRGB10A2Uint")]
    Rgb10A2UInt = 91,

    /// 32-bit format with floating-point color components, 11 bits each for red and green and 10 bits for blue.
    #[doc(alias = "MTLPixelFormatRG11B10Float")]
    Rg11B10Float = 92,

    /// Packed 32-bit format with floating-point color components: 9 bits each for RGB and 5 bits for an exponent
    /// shared by RGB, packed into 32 bits.
    #[doc(alias = "MTLPixelFormatRGB9E5Float")]
    Rgb9E5Float = 93,

    /// A 32-bit packed pixel format with four normalized unsigned integer components:
    /// 10-bit blue, 10-bit green, 10-bit red, and 2-bit alpha.
    #[doc(alias = "MTLPixelFormatBGR10A2Unorm")]
    Bgr10A2UNorm = 94,

    /// A 32-bit extended range pixel format with three fixed-point components:
    /// 10-bit blue, 10-bit green, and 10-bit red.
    ///
    /// Pixel data is stored in blue, green, and red order, from least significant bit
    /// to most significant bit. Bits 30 and 31 are used as padding, and their value is 0.
    ///
    /// The blue, green, and red components are linearly encoded, and their values range
    /// from -0.752941 to 1.25098.
    ///
    /// To display wide color values on devices with wide color displays, you can set this
    /// pixel format on the colorPixelFormat property of an 'mtk::View' or the pixelFormat
    /// property of a 'ca::MetalLayer'.
    #[doc(alias = "MTLPixelFormatBGR10_XR")]
    Bgr10Xr = 554,

    /// A 32-bit extended range pixel format with sRGB conversion and three fixed-point components:
    /// 10-bit blue, 10-bit green, and 10-bit red.
    ///
    /// Pixel data is stored in blue, green, and red order, from least significant bit to most
    /// significant bit. Bits 30 and 31 are used as padding, and their value is 0.
    ///
    /// The blue, green, and red components are gamma encoded, and their values range
    /// from -0.5271 to 1.66894, before gamma expansion.
    ///
    /// To display wide color values on devices with wide color displays, you can set this pixel
    /// format on the colorPixelFormat property of an 'mtk::View' or the pixelFormat property of
    /// a [`ca::MetalLayer`]. You must also use specify an extended sRGB color space.
    #[doc(alias = "MTLPixelFormatBGR10_XR_sRGB")]
    Bgr10XrSrgb = 555,

    /* Normal 64 bit formats */
    /// Ordinary format with two 32-bit unsigned integer components.
    #[doc(alias = "MTLPixelFormatRG32Uint")]
    Rg32UInt = 103,

    /// Ordinary format with two 32-bit signed integer components.
    #[doc(alias = "MTLPixelFormatRG32Sint")]
    Rg32SInt = 104,

    /// Ordinary format with two 32-bit floating-point components.
    #[doc(alias = "MTLPixelFormatRG32Float")]
    Rg32Float = 105,

    /// Ordinary format with four 16-bit normalized unsigned integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA16Unorm")]
    Rgba16UNorm = 110,

    /// Ordinary format with four 16-bit normalized signed integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA16Snorm")]
    Rgba16SNorm = 112,

    /// Ordinary format with four 16-bit unsigned integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA16Uint")]
    Rgba16UInt = 113,

    /// Ordinary format with four 16-bit signed integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA16Sint")]
    Rgba16SInt = 114,

    /// Ordinary format with four 16-bit floating-point components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA16Float")]
    Rgba16Float = 115,

    /// A 64-bit extended range pixel format with four fixed-point components:
    /// 10-bit blue, 10-bit green, 10-bit red, and 10-bit alpha.
    ///
    /// Pixel data is stored in blue, green, red, and alpha order, from least significant
    /// bit to most significant bit. Each component is stored in a 16-bit chunk that's arranged
    /// as follows:
    ///
    /// - The 10 left-most bits store the component's data.
    /// - The 6 right-most bits are used as padding, and their value is 0.
    ///
    /// The blue, green, and red components are linearly encoded, and their values range
    /// from -0.752941 to 1.25098. The alpha component is always clamped to a \[0.0, 1.0\] range
    /// in sampling, rendering, and writing operations, despite supporting values outside this range.
    ///
    /// To display wide color values on devices with wide color displays, you can set this pixel
    /// format on the colorPixelFormat property of an 'mtk::View' or the pixelFormat property of
    /// a [`ca::MetalLayer`].
    #[doc(alias = "MTLPixelFormatBGRA10_XR")]
    Bgra10Xr = 552,

    /// A 64-bit extended range pixel format with sRGB conversion and four fixed-point components:
    /// 10-bit blue, 10-bit green, 10-bit red, and 10-bit alpha.
    ///
    /// Pixel data is stored in blue, green, red, and alpha order, from least significant bit to most
    /// significant bit. Each component is stored in a 16-bit chunk that's arranged as follows:
    ///
    /// - The 10 left-most bits store the component's data.
    /// - The 6 right-most bits are used as padding, and their value is 0.
    ///
    /// The blue, green, and red components are gamma encoded, and their values range from
    /// -0.5271 to 1.66894, before gamma expansion. The alpha component is always clamped
    /// to a \[0.0, 1.0\] range in sampling, rendering, and writing operations, despite supporting
    /// values outside this range.
    ///
    /// To display wide color values on devices with wide color displays, you can set this
    /// pixel format on the colorPixelFormat property of an 'mtk::View' or the pixelFormat property
    /// of a [`ca::MetalLayer`]. You must also use specify an extended sRGB color space.
    #[doc(alias = "MTLPixelFormatBGRA10_XR_sRGB")]
    Bgra10XrSrgb = 553,

    /* Normal 128 bit formats */
    /// Ordinary format with four 32-bit unsigned integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA32Uint")]
    Rgba32UInt = 123,

    /// Ordinary format with four 32-bit signed integer components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA32Sint")]
    Rgba32SInt = 124,

    /// Ordinary format with four 32-bit floating-point components in RGBA order.
    #[doc(alias = "MTLPixelFormatRGBA32Float")]
    Rgba32Float = 125,

    /* Compressed formats. */
    /// Compressed format with two 16-bit color components and one 32-bit descriptor component.
    #[doc(alias = "MTLPixelFormatBC1_RGBA")]
    Bc1Rgba = 130,

    /// Compressed format with two 16-bit color components and one 32-bit descriptor component,
    /// with conversion between sRGB and linear space.
    #[doc(alias = "MTLPixelFormatBC1_RGBA_sRGB")]
    Bc1RgbaSrgb = 131,

    /// Compressed format with two 64-bit chunks.
    ///
    /// The first chunk contains two 8-bit alpha components and one 48-bit descriptor component.
    /// The second chunk contains two 16-bit color components and one 32-bit descriptor component.
    #[doc(alias = "MTLPixelFormatBC2_RGBA")]
    Bc2Rgba = 132,

    /// Compressed format with two 64-bit chunks, with conversion between sRGB and linear space.
    ///
    /// The first chunk contains two 8-bit alpha components and one 48-bit descriptor component.
    /// The second chunk contains two 16-bit color components and one 32-bit descriptor component.
    #[doc(alias = "MTLPixelFormatBC2_RGBA_sRGB")]
    Bc2RgbaSrgb = 133,

    /// Compressed format with two 64-bit chunks.
    ///
    /// The first chunk contains two 8-bit alpha components and one 48-bit descriptor component.
    /// The second chunk contains two 16-bit color components and one 32-bit descriptor component.
    #[doc(alias = "MTLPixelFormatBC3_RGBA")]
    Bc3Rgba = 134,

    /// Compressed format with two 64-bit chunks, with conversion between sRGB and linear space.
    ///
    /// The first chunk contains two 8-bit alpha components and one 48-bit descriptor component.
    /// The second chunk contains two 16-bit color components and one 32-bit descriptor component.
    #[doc(alias = "MTLPixelFormatBC3_RGBA_sRGB")]
    Bc3RgbaSrgb = 135,

    /// Compressed format with four floating-point components.
    #[doc(alias = "MTLPixelFormatBC6H_RGBFloat")]
    Bc6HRgbFloat = 150,

    /// Compressed format with four unsigned floating-point components.
    #[doc(alias = "MTLPixelFormatBC6H_RGBUfloat")]
    Bc6HRgbUFloat = 151,

    /// Compressed format with four normalized unsigned integer components.
    #[doc(alias = "MTLPixelFormatBC7_RGBAUnorm")]
    Bc7RgbaUNorm = 152,

    /// Compressed format with four normalized unsigned integer components,
    /// with conversion between sRGB and linear space.
    #[doc(alias = "MTLPixelFormatBC7_RGBAUnorm_sRGB")]
    Bc7RgbaUNormSrgb = 153,

    /// ASTC-compressed format with low-dynamic-range content,
    /// conversion between sRGB and linear space, a block width of 4, and a block height of 4.
    #[doc(alias = "MTLPixelFormatASTC_4x4_sRGB")]
    Astc4x4Srgb = 186,

    /// ASTC-compressed format with low-dynamic-range content,
    /// conversion between sRGB and linear space, a block width of 5, and a block height of 4.
    #[doc(alias = "MTLPixelFormatASTC_5x4_sRGB")]
    Astc5x4Srgb = 187,

    /// ASTC-compressed format with low-dynamic-range content,
    /// conversion between sRGB and linear space, a block width of 5, and a block height of 5.
    #[doc(alias = "MTLPixelFormatASTC_5x5_sRGB")]
    Astc5x5Srgb = 188,

    /// ASTC-compressed format with low-dynamic-range content,
    /// conversion between sRGB and linear space, a block width of 6, and a block height of 5.
    #[doc(alias = "MTLPixelFormatASTC_6x5_sRGB")]
    Astc6x5Srgb = 189,

    /// ASTC-compressed format with low-dynamic-range content,
    /// conversion between sRGB and linear space, a block width of 6, and a block height of 6.
    #[doc(alias = "MTLPixelFormatASTC_6x6_sRGB")]
    Astc6x6Srgb = 190,

    /// ASTC-compressed format with low-dynamic-range content,
    /// conversion between sRGB and linear space, a block width of 8, and a block height of 5.
    #[doc(alias = "MTLPixelFormatASTC_8x5_sRGB")]
    Astc8x5Srgb = 192,

    /// ASTC-compressed format with low-dynamic-range content,
    /// conversion between sRGB and linear space, a block width of 8, and a block height of 6.
    #[doc(alias = "MTLPixelFormatASTC_8x6_sRGB")]
    Astc8x6Srgb = 193,

    /// ASTC-compressed format with low-dynamic-range content,
    /// conversion between sRGB and linear space, a block width of 8, and a block height of 8.
    #[doc(alias = "MTLPixelFormatASTC_8x8_sRGB")]
    Astc8x8Srgb = 194,
    Astc10x5Srgb = 195,
    Astc10x6Srgb = 196,
    Astc10x8Srgb = 197,
    Astc10x10Srgb = 198,
    Astc12x10Srgb = 199,
    Astc12x12Srgb = 200,

    Astc4x4Ldr = 204,
    Astc5x4Ldr = 205,
    Astc5x5Ldr = 206,
    Astc6x5Ldr = 207,
    Astc6x6Ldr = 208,
    Astc8x5Ldr = 210,
    Astc8x6Ldr = 211,
    Astc8x8Ldr = 212,
    Astc10x5Ldr = 213,
    Astc10x6Ldr = 214,
    Astc10x8Ldr = 215,
    Astc10x10Ldr = 216,
    Astc12x10Ldr = 217,
    Astc12x12Ldr = 218,

    Astc4x4Hdr = 222,
    Astc5x4Hdr = 223,
    Astc5x5Hdr = 224,
    Astc6x5Hdr = 225,
    Astc6x6Hdr = 226,
    Astc8x5Hdr = 228,
    Astc8x6Hdr = 229,
    Astc8x8Hdr = 230,
    Astc10x5Hdr = 231,
    Astc10x6Hdr = 232,
    Astc10x8Hdr = 233,
    Astc10x10Hdr = 234,
    Astc12x10Hdr = 235,
    Asrc12x12Hdr = 236,

    /// A pixel format where the red and green channels are subsampled horizontally.
    /// Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    #[doc(alias = "MTLPixelFormatGBGR422")]
    Gbgr422 = 240,

    /// A pixel format where the red and green channels are subsampled horizontally.
    /// Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    #[doc(alias = "MTLPixelFormatBGRG422")]
    Bgrg422 = 241,

    /// A pixel format with a 16-bit normalized unsigned integer component, used for a depth render target.
    #[doc(alias = "MTLPixelFormatDepth16Unorm")]
    Depth16Unorm = 250,

    /// A pixel format with one 32-bit floating-point component, used for a depth render target.
    #[doc(alias = "MTLPixelFormatDepth32Float")]
    Depth32Float = 252,

    /// A pixel format with an 8-bit unsigned integer component, used for a stencil render target.
    #[doc(alias = "MTLPixelFormatStencil8")]
    Stencil8 = 253,

    /// A 32-bit combined depth and stencil pixel format with a 24-bit normalized unsigned integer for depth and an 8-bit unsigned integer for stencil.
    #[doc(alias = "MTLPixelFormatDepth24Unorm_Stencil8")]
    Depth24UnormStencil8 = 255,

    /// A 40-bit combined depth and stencil pixel format with a 32-bit floating-point value for depth and an 8-bit unsigned integer for stencil.
    ///
    /// When using this format, some Metal device objects allocate 64-bits per pixel.
    #[doc(alias = "MTLPixelFormatDepth32Float_Stencil8")]
    Depth32FloatStencil8 = 260,

    /// A stencil pixel format used to read the stencil value from a texture with a combined 32-bit
    /// depth and 8-bit stencil value.
    ///
    /// You can't directly read the stencil value of a texture with the `mtl::PixelFormat::Depth32FloatStencil8`
    /// format. To read stencil values from a texture with the `mtl::PixelFormat::Depth32FloatStencil8` format,
    /// create a texture view of that texture using the `mtl::PixelFormat::X32Stencil8` format, and sample the
    /// texture view instead.
    #[doc(alias = "MTLPixelFormatX32_Stencil8")]
    X32Stencil8 = 261,

    /// A stencil pixel format used to read the stencil value from a texture with a combined 24-bit
    /// depth and 8-bit stencil value.
    ///
    /// You can't directly read the stencil value of a texture with the `mtl::PixelFormat::Depth24UnormStencil8`
    /// format. To read stencil values from a texture with the `mtl::PixelFormat::Depth24UnormStencil8` format,
    /// create a texture view of that texture using the `mtl::PixelFormat::X24Stencil8` format,
    /// and sample the texture view instead.
    #[doc(alias = "MTLPixelFormatX24_Stencil8")]
    X24Stencil8 = 262,
}
