use crate::define_opts;

#[doc(alias = "vImagePixelCount")]
pub type PixelCount = usize;

#[doc(alias = "vImage_Buffer")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(C)]
pub struct Buf {
    pub data: *mut std::ffi::c_void,
    pub h: PixelCount,
    pub w: PixelCount,
    pub row_bytes: usize,
}

#[doc(alias = "vImage_AffineTransform")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct AffineTransformF32 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}

#[doc(alias = "vImage_AffineTransformDouble")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct AffineTransformF64 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

pub type CGAffineTransform = AffineTransformF64;

#[doc(alias = "vImage_PerpsectiveTransform")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct PerpsectiveTransform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
    pub vx: f32,
    pub vy: f32,
    pub v: f32,
}

#[doc(alias = "vImage_WarpInterpolation")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(i32)]
pub enum WarpInterpolation {
    #[doc(alias = "kvImageInterpolationNearest")]
    Nearest = 0,
    #[doc(alias = "kvImageInterpolationLinear")]
    Linear = 1,
}

define_opts!(
    #[doc(alias = "vImage_Flags")]
    pub Flags(u32)
);

impl Flags {
    #[doc(alias = "kvImageNoFlags")]
    pub const NONE: Self = Self(0);

    /// Operate on red, green and blue channels only. Alpha is copied from source
    /// to destination. For Interleaved formats only.
    #[doc(alias = "kvImageLeaveAlphaUnchanged")]
    pub const LEAVE_ALPHA_UNCHANGED: Self = Self(1);

    /// Copy edge pixels. Convolution Only.
    #[doc(alias = "kvImageCopyInPlace")]
    pub const COPY_IN_PLACE: Self = Self(2);

    /// Use the background color for missing pixels.
    #[doc(alias = "kvImageBackgroundColorFill")]
    pub const BG_COLOR_FILL: Self = Self(4);

    /// Use the nearest pixel for missing pixels.
    #[doc(alias = "kvImageEdgeExtend")]
    pub const EDGE_EXTEND: Self = Self(8);

    /// Pass to turn off internal tiling and disable internal multithreading. Use this if
    /// you want to do your own tiling, or to use the Min/Max filters in place.
    #[doc(alias = "kvImageDoNotTile")]
    pub const DO_NOT_TILE: Self = Self(16);

    /// Use a higher quality, slower resampling filter for Geometry operations
    /// (shear, scale, rotate, affine transform, etc.)
    #[doc(alias = "kvImageHighQualityResampling")]
    pub const HQ_RESAMPLING: Self = Self(32);

    /// Use only the part of the kernel that overlaps the image. For integer kernels,
    /// real_divisor = divisor * (sum of used kernel elements) / (sum of kernel elements).
    /// This should preserve image brightness at the edges. Convolution only.
    #[doc(alias = "kvImageTruncateKernel")]
    pub const TRUNCATE_KERNEL: Self = Self(64);

    /// The function will return the number of bytes required for the temp buffer.
    /// If this value is negative, it is an error, per standard usage.
    #[doc(alias = "kvImageGetTempBufferSize")]
    pub const GET_TEMP_BUF_SIZE: Self = Self(128);

    /// Some functions such as vImageConverter_CreateWithCGImageFormat have so many possible error conditions
    /// that developers may need more help than a simple error code to diagnose problems. When this
    /// flag is set and an error is encountered, an informative error message will be logged to the Apple
    /// System Logger (ASL).  The output should be visible in Console.app.
    #[doc(alias = "kvImagePrintDiagnosticsToConsole")]
    pub const PRINT_DIAGNOSTICS_TO_CONSOLE: Self = Self(256);

    /// Pass this flag to prevent vImage from allocating additional storage.
    #[doc(alias = "kvImageNoAllocate")]
    pub const NO_ALLOCATE: Self = Self(512);

    /// Use methods that are HDR-aware, capable of providing correct results for input images with pixel values
    /// outside the otherwise limited (typically [-2,2]) range. This may be SLOWER.
    #[doc(alias = "kvImageHDRContent")]
    pub const HDR_CONTENT: Self = Self(1024);

    /// Pass to disable clamping is some conversions to floating point formats. Use this if the input data
    /// may describe values outside [0,1] which should be preserved..
    #[doc(alias = "kvImageDoNotClamp")]
    pub const DO_NOT_CLAMP: Self = Self(2048);

    /// Use a lower precision, faster floating-point arithmetic for all internal computations when available.
    /// This applies to all functions operating on 16-bit half-precision floating-point pixels, Pixel_16F.
    #[doc(alias = "kvImageUseFP16Accumulator")]
    pub const USE_FP16_ACCUMULATOR: Self = Self(4096);
}

#[doc(alias = "vImage_YpCbCrToARGBMatrix")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct YpCbCrArgbMatrix {
    pub y_p: f32,
    pub cr_r: f32,
    pub cr_g: f32,
    pub cb_g: f32,
    pub cb_b: f32,
}
