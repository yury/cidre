use crate::{define_options, ns};

define_options!(KernelOptions(usize));

/// Options used when creating mps::Kernel objects
impl KernelOptions {
    /// Use default options
    #[doc(alias = "MPSKernelOptionsNone")]
    pub const NONE: Self = Self(0usize);

    /// Most MPS functions will sanity check their arguments. This has a small but
    /// non-zero CPU cost. Setting the MPSKernelOptionsSkipAPIValidation will skip these checks.
    /// MPSKernelOptionsSkipAPIValidation does not skip checks for memory allocation failure.
    /// Caution:  turning on MPSKernelOptionsSkipAPIValidation can result in undefined behavior
    /// if the requested operation can not be completed for some reason. Most error states
    /// will be passed through to Metal which may do nothing or abort the program if Metal
    /// API validation is turned on.
    #[doc(alias = "MPSKernelOptionsSkipAPIValidation")]
    pub const SKIP_APIVALIDATION: Self = Self(1usize << 0);

    /// When possible, MPSKernels use a higher precision data representation internally than
    /// the destination storage format to avoid excessive accumulation of computational
    /// rounding error in the result. MPSKernelOptionsAllowReducedPrecision advises the
    /// MPSKernel that the destination storage format already has too much precision for
    /// what is ultimately required downstream, and the MPSKernel may use reduced precision
    /// internally when it feels that a less precise result would yield better performance.
    /// The expected performance win is often small, perhaps 0-20%. When enabled, the
    /// precision of the result may vary by hardware and operating syste
    #[doc(alias = "MPSKernelOptionsAllowReducedPrecision")]
    pub const ALLOW_REDUCED_PRECISION: Self = Self(1usize << 1);

    /// Some MPSKernels may automatically split up the work internally into multiple tiles.
    /// This improves performance on larger textures and reduces the amount of memory needed by
    /// MPS for temporary storage. However, if you are using your own tiling scheme to achieve
    /// similar results, your tile sizes and MPS's choice of tile sizes may interfere with
    /// one another causing MPS to subdivide your tiles for its own use inefficiently. Pass
    /// MPSKernelOptionsDisableInternalTiling to force MPS to process your data tile as a
    /// single chunk.
    #[doc(alias = "MPSKernelOptionsDisableInternalTiling")]
    pub const DISABLE_INTERNAL_TILING: Self = Self(1usize << 2);

    /// Enabling this bit will cause various -encode... methods to call MTLCommandEncoder
    /// push/popDebugGroup.  The debug string will be drawn from MPSKernel.label, if any
    /// or the name of the class otherwise.
    #[doc(alias = "MPSKernelOptionsInsertDebugGroups")]
    pub const INSERT_DEBUG_GROUPS: Self = Self(1usize << 3);

    /// Some parts of MPS can provide debug commentary and tuning advice when run.
    /// Setting this bit to 1 will cause the commentary to be emitted to stderr. Otherwise,
    /// the code is silent.  This is especially useful for debugging MPSNNGraph. This option
    /// is on by default when the MPS_LOG_INFO environment variable is defined.  For
    /// even more detailed output on a MPS object, you can use the po command in llvm
    /// with MPS objects:
    /// ```llvm
    /// po <mps pointer>
    /// ```
    pub const VERBOSE: Self = Self(1usize << 4);
}

/// Options used to control edge behaviour of filter when filter reads beyond boundary of src image
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum ImageEdgeMode {
    /// Out of bound pixels are (0,0,0,1) for image with pixel format without alpha channel
    /// and (0,0,0,0) for image with pixel format that has an alpha channel
    #[doc(alias = "MPSImageEdgeModeZero")]
    Zero = 0,

    /// Out of bound pixels are clamped to nearest edge pixel
    #[doc(alias = "MPSImageEdgeModeClamp")]
    Clamp = 1,

    /// Out of bound pixels are mirrored wrt. the nearest edge pixel center - ie. the edge of the image is not repeated.
    /// NOTE: The only filter that currently supports this mode is @ref MPSNNPad - using this with other filters results in undefined behavior.
    #[doc(alias = "MPSImageEdgeModeMirror")]
    Mirror,

    /// Out of bound pixels are mirrored wrt. the nearest edge pixel nearest border - ie. the edge of the image is repeated.
    /// NOTE: The only filter that currently supports this mode is @ref MPSNNPad - using this with other filters results in undefined behavior.
    #[doc(alias = "MPSImageEdgeModeMirrorWithEdge")]
    MirrorWithEdge,

    /// Out of bound pixels are filled with a constant value defined by the filter.
    /// NOTE: The only filter that currently supports this mode is @ref MPSNNPad - using this with other filters results in undefined behavior.
    #[doc(alias = "MPSImageEdgeModeConstant")]
    Constant,
}

/// Encodes the representation of a single channel within a MPSImage.
///
/// A MPSImage pixel may have many channels in it, sometimes many more than 4, the
/// limit of what MTLPixelFormats encode. The storage format for a single channel
/// within a pixel can be given by the MPSImageFeatureChannelFormat. The number
/// of channels is given by the featureChannels parameter of appropriate MPSImage
/// APIs. The size of the pixel is size of the channel format multiplied by the
/// number of feature channels. No padding is allowed, except to round out to a full
/// byte.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum ImageFeatureChannelFormat {
    /// No format. This can mean  according to context invalid format or any format.  In the
    /// latter case, it is an invitation to MPS to pick a format.
    #[doc(alias = "MPSImageFeatureChannelFormatNone")]
    None = 0,

    /// u8 with value [0,255] encoding [0,1.0]
    #[doc(alias = "MPSImageFeatureChannelFormatUnorm8")]
    Unorm8 = 1,

    /// u16 with value [0,65535] encoding [0,1.0]
    #[doc(alias = "MPSImageFeatureChannelFormatUnorm16")]
    Unorm16 = 2,

    /// IEEE-754 16-bit floating-point value. "half precision" Representable normal range is +-[2**-14, 65504], 0, Infinity, NaN. 11 bits of precision + exponent
    #[doc(alias = "MPSImageFeatureChannelFormatFloat16")]
    Float16 = 3,

    /// IEEE-754 32-bit floating-point value.  "single precision" (standard float type in C) 24 bits of precision + exponent
    #[doc(alias = "MPSImageFeatureChannelFormatFloat32")]
    F32 = 4,

    /// Reserved for later expansion
    #[doc(alias = "MPSImageFeatureChannelFormat_reserved0")]
    Reserved0 = 5,

    /// Always last
    #[doc(alias = "MPSImageFeatureChannelFormatCount")]
    Count,
}

/// A value to specify a type of data.
#[doc(alias = "MPSDataType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum DataType {
    Invalid,

    /// 32-bit floating point (single-precision).
    F32 = Self::FLOAT_BIT | 32u32,

    /// 16-bit floating point (half-precision).  (IEEE-754-2008 float16 exchange format)
    F16 = Self::FLOAT_BIT | 16u32,

    ///Complex number composed of two 32-bit floating point numbers (single-precision).
    ComplexF32 = Self::FLOAT_BIT | Self::COMPLEX_BIT | 64u32,

    /// Complex number composed of two 16-bit floating point numbers (half-precision).  (IEEE-754-2008 float16 exchange format)
    ComplexF16 = Self::FLOAT_BIT | Self::COMPLEX_BIT | 32u32,

    /// Signed 8-bit integer.
    I8 = Self::SIGNED_BIT | 8u32,

    ///  Signed 16-bit integer.
    I16 = Self::SIGNED_BIT | 16u32,

    ///
    I32 = Self::SIGNED_BIT | 32u32,
    I64 = Self::SIGNED_BIT | 64u32,

    /// Unsigned 8-bit integer. Not normalized
    U8 = 8,

    /// Unsigned 16-bit integer. Not normalized
    U16 = 16,

    /// Unsigned 32-bit integer. Not normalized
    U32 = 32,
    U64 = 64,
    Bool = Self::ALTERNATE_ENCODING_BIT | 8u32,

    /// Unsigned 1-bit normalized value.
    Unorm1 = Self::NORMALIZED_BIT | 1u32,

    /// Unsigned 8-bit normalized value.
    Unorm8 = Self::NORMALIZED_BIT | 8u32,
}

impl DataType {
    /// A common bit for all floating point data types. Zero for integer types
    pub const FLOAT_BIT: u32 = 0x10000000;

    /// A common bit for all complex point data types. Zero for integer types
    pub const COMPLEX_BIT: u32 = 0x01000000;

    pub const SIGNED_BIT: u32 = 0x20000000;

    pub const ALTERNATE_ENCODING_BIT: u32 = 0x80000000;

    /// If set, the value of the shall be interpreted as value / UNORM_TYPE_MAX
    /// Normalized values have range [0, 1.0] if unsigned and [-1,1] if signed.
    /// SNORM_TYPE_MIN is interpreted as SNORM_TYPE_MIN+1 per standard Metal rules.
    pub const NORMALIZED_BIT: u32 = 0x40000000;

    /// A utility function to get the size of an MPSDataType.  This implementation
    /// relies upon the specific bit pattern used to encode the type.
    #[inline]
    pub fn size_of(&self) -> usize {
        ((*self as u32 & 0xffffu32) >> 3) as usize
    }
}

define_options!(AliasingStrategy(usize));

/// A description of whether slices,transposes and other views should alias their underlying content or be copied into new storage.
impl AliasingStrategy {
    /// View should alias if possible, otherwise make a copy. In the latter case if the allocation fails, nil will be returned.
    pub const DEFAULT: Self = Self(0);

    pub const DONT_CARE: Self = Self::DEFAULT;

    /// View must alias. If it can not, nil is returned.
    pub const SHALL_ALIAS: Self = Self(1usize << 0);

    /// Always make a copy. If there isn't enough memory available the allocation may fail and nil will be returned.
    pub const SHALL_NOT_ALIAS: Self = Self(1usize << 1);

    pub const ALIASING_RESERVED: Self = Self(Self::SHALL_ALIAS.0 | Self::SHALL_NOT_ALIAS.0);

    /// If the view doesn't alias the old object, use temporary for it. If they do alias, the same type of memory must be used.
    pub const PREFER_TEMPORARY_MEMORY: Self = Self(1usize << 2);

    /// If the view doesn't alias the old object, use non-temporary for it. If they do alias, the same type of memory must be used.
    pub const PREFER_NON_TEMPORARY_MEMORY: Self = Self(1usize << 3);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(C)]
#[doc(alias = "MPSOffset")]
pub struct Offset {
    /// The horizontal component of the offset. Units: pixels
    pub x: isize,

    /// The vertical component of the offset. Units: pixels
    pub y: isize,

    /// The depth component of the offset. Units: pixels
    pub z: isize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
#[doc(alias = "MPSOrigin")]
pub struct Origin {
    /// The x coordinate of the position
    pub x: f64,

    /// The y coordinate of the position
    pub y: f64,

    /// The z coordinate of the position
    pub z: f64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
#[doc(alias = "MPSSize")]
pub struct Size {
    /// The width of the region
    pub width: f64,

    /// The height of the region
    pub height: f64,

    /// The depth of the region
    pub depth: f64,
}

/// Describes a sub-region of an array dimension
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(C)]
#[doc(alias = "MPSDimensionSlice")]
pub struct DimensionSlice {
    /// The position of the first element in the slice
    pub start: usize,

    /// The number of elements in the slice
    pub length: usize,
}

/// A region of an image
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
#[doc(alias = "MPSRegion")]
pub struct Region {
    /// The top left corner of the region.  Units: pixel
    pub origin: Origin,

    /// The size of the region. Units: pixel
    pub size: Size,
}

/// Transform matrix for explict control over resampling in MPSImageScale.
#[doc(alias = "MPSScaleTransform")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct ScaleTransform {
    /// horizontal scaling factor
    pub scale_x: f64,

    /// vertical scaling factor
    pub scale_y: f64,

    /// horizontal translation
    pub translate_x: f64,

    /// vertical translation
    pub translate_y: f64,
}

/// A unsigned coordinate with x, y and channel components
#[doc(alias = "MPSImageCoordinate")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct ImageCoordinate {
    /// The horizontal component of the coordinate. Units: pixels
    pub x: usize,

    /// The vertical component of the coordinate. Units: pixels
    pub y: usize,

    /// The index of the channel or feature channel within the pixel
    pub channel: usize,
}

/// A unsigned coordinate with x, y and channel components
#[doc(alias = "MPSImageRegion")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct ImageRegion {
    /// The position of the top left corner of the subregio
    pub offset: ImageCoordinate,

    /// The size {pixels, pixels, channels} of the subregion
    pub size: ImageCoordinate,
}

pub type Shape = ns::Array<ns::Number>;
