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

impl YpCbCrArgbMatrix {
    #[doc(alias = "kvImage_YpCbCrToARGBMatrix_ITU_R_601_4")]
    pub fn itu_r_601_4() -> &'static Self {
        unsafe { kvImage_YpCbCrToARGBMatrix_ITU_R_601_4 }
    }

    #[doc(alias = "kvImage_YpCbCrToARGBMatrix_ITU_R_709_2")]
    pub fn itu_r_709_2() -> &'static Self {
        unsafe { kvImage_YpCbCrToARGBMatrix_ITU_R_709_2 }
    }
}

#[doc(alias = "vImage_YpCbCrToARGB")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct YpCbCrToArgb {
    opaque: [u8; 128],
}

#[doc(alias = "vImage_ARGBToYpCbCrMatrix")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct ArgbToYpCbCrMatrix {
    r_yp: f32,
    g_yp: f32,
    b_yp: f32,
    r_cb: f32,
    g_cb: f32,
    b_cb_r_cr: f32,
    g_cr: f32,
    b_cr: f32,
}

impl ArgbToYpCbCrMatrix {
    #[doc(alias = "kvImage_ARGBToYpCbCrMatrix_ITU_R_601_4")]
    pub fn itu_r_601_4() -> &'static Self {
        unsafe { kvImage_ARGBToYpCbCrMatrix_ITU_R_601_4 }
    }

    #[doc(alias = "kvImage_ARGBToYpCbCrMatrix_ITU_R_709_2")]
    pub fn itu_r_709_2() -> &'static Self {
        unsafe { kvImage_ARGBToYpCbCrMatrix_ITU_R_709_2 }
    }
}

#[doc(alias = "vImage_ARGBToYpCbCr")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct ArgbToYpCbCr {
    opaque: [u8; 128],
}

#[doc(alias = "vImage_YpCbCrPixelRange")]
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct YpCbCrPixelRange {
    pub yp_bias: i32,
    pub cb_cr_bias: i32,
    pub yp_range_max: i32,
    pub cb_cr_range_max: i32,
    pub yp_max: i32,
    pub yp_min: i32,
    pub cb_cr_max: i32,
    pub cb_cr_min: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Error(pub std::num::NonZeroIsize);

impl Error {
    pub const fn new_unchcked(v: isize) -> Self {
        Self(unsafe { std::num::NonZeroIsize::new_unchecked(v) })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Status(pub isize);

impl Status {
    #[inline]
    pub fn result(self) -> Result {
        unsafe { std::mem::transmute(self) }
    }
}

pub type Result<Ok = ()> = std::result::Result<Ok, Error>;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        Self(value.0.get())
    }
}

pub mod err {
    use crate::vimage::Error;

    #[doc(alias = "kvImageRoiLargerThanInputBuffer")]
    pub const ROI_LARGER_THAN_INPUT_BUF: Error = Error::new_unchcked(-21766);

    #[doc(alias = "kvImageInvalidKernelSize")]
    pub const INVALID_KERNEL_SIZE: Error = Error::new_unchcked(-21767);

    #[doc(alias = "kvImageInvalidEdgeStyle")]
    pub const INVALID_EDGE_STYLE: Error = Error::new_unchcked(-21768);

    #[doc(alias = "kvImageInvalidOffset_X")]
    pub const INVALID_OFFSET_X: Error = Error::new_unchcked(-21769);

    #[doc(alias = "kvImageInvalidOffset_Y")]
    pub const INVALID_OFFSET_Y: Error = Error::new_unchcked(-21770);

    #[doc(alias = "kvImageMemoryAllocationError")]
    pub const MEMORY_ALLOCATION_ERR: Error = Error::new_unchcked(-21771);

    #[doc(alias = "kvImageNullPointerArgument")]
    pub const NULL_PTR_ARG: Error = Error::new_unchcked(-21772);

    #[doc(alias = "kvImageInvalidParameter")]
    pub const INVALID_PARAM: Error = Error::new_unchcked(-21773);

    #[doc(alias = "kvImageBufferSizeMismatch")]
    pub const BUF_SIZE_MISMATCH: Error = Error::new_unchcked(-21774);

    #[doc(alias = "kvImageUnknownFlagsBit")]
    pub const UNKNOWN_FLAGS_BIT: Error = Error::new_unchcked(-21775);

    #[doc(alias = "kvImageInternalError")]
    pub const INTERNAL_ERR: Error = Error::new_unchcked(-21776);

    #[doc(alias = "kvImageInvalidRowBytes")]
    pub const INVALID_ROW_BYTES: Error = Error::new_unchcked(-21777);

    #[doc(alias = "kvImageInvalidImageFormat")]
    pub const INVALID_IMAGE_FORMAT: Error = Error::new_unchcked(-21778);

    #[doc(alias = "kvImageColorSyncIsAbsent")]
    pub const COLOR_SYNC_IS_ABSENT: Error = Error::new_unchcked(-21779);

    #[doc(alias = "kvImageOutOfPlaceOperationRequired")]
    pub const OUT_OF_PLACE_OP_REQUIRED: Error = Error::new_unchcked(-21780);

    #[doc(alias = "kvImageInvalidImageObject")]
    pub const INVALID_IMAGE_OBJ: Error = Error::new_unchcked(-21781);

    #[doc(alias = "kvImageInvalidCVImageFormat")]
    pub const INVALID_CV_IMAGE_FORMAT: Error = Error::new_unchcked(-21782);

    #[doc(alias = "kvImageUnsupportedConversion")]
    pub const UNSUPPORTED_CONVERSION: Error = Error::new_unchcked(-21783);

    #[doc(alias = "kvImageCoreVideoIsAbsent")]
    pub const CORE_VIDEO_IS_ABSENT: Error = Error::new_unchcked(-21784);
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    static kvImage_YpCbCrToARGBMatrix_ITU_R_601_4: &'static YpCbCrArgbMatrix;
    static kvImage_YpCbCrToARGBMatrix_ITU_R_709_2: &'static YpCbCrArgbMatrix;
    static kvImage_ARGBToYpCbCrMatrix_ITU_R_601_4: &'static ArgbToYpCbCrMatrix;
    static kvImage_ARGBToYpCbCrMatrix_ITU_R_709_2: &'static ArgbToYpCbCrMatrix;
}

#[cfg(test)]
mod tests {
    use crate::vimage;

    #[test]
    fn basics() {
        let _m = *vimage::YpCbCrArgbMatrix::itu_r_709_2();
        let _m = *vimage::YpCbCrArgbMatrix::itu_r_601_4();
        // println!("{m:?}");
    }
}
