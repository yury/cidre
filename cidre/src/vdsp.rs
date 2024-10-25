use std::ptr::NonNull;

#[doc(alias = "vDSP_Length")]
pub type Len = usize;

#[doc(alias = "vDSP_Stride")]
pub type Stride = isize;

/// Helper
#[inline]
fn with<R>(f: impl FnOnce(*mut R)) -> R {
    let mut res = std::mem::MaybeUninit::uninit();
    f(res.as_mut_ptr());
    unsafe { res.assume_init() }
}

#[doc(alias = "FFTDirection")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum FftDirection {
    Forward = 1,
    Inverse = -1,
}

#[doc(alias = "FFTRadix")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum FftRadix {
    _2 = 0,
    _3 = 1,
    _5 = 2,
}

#[doc(alias = "DSPComplex")]
#[doc(alias = "DSPDoubleComplex")]
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

#[doc(alias = "DSPSplitComplex")]
#[doc(alias = "DSPDoubleSplitComplex")]
#[derive(Debug)]
#[repr(C)]
pub struct SplitComplex<'r, 'i, T> {
    re: *mut T,
    im: *mut T,
    life_times: std::marker::PhantomData<(&'r T, &'i T)>,
}

impl<'r, 'i, T> SplitComplex<'r, 'i, T> {
    pub fn new(re: &'r [T], im: &'i [T]) -> Self {
        assert_eq!(re.len(), im.len());
        Self {
            re: re.as_ptr() as _,
            im: im.as_ptr() as _,
            life_times: std::marker::PhantomData,
        }
    }

    pub fn new_mut(re: &'r mut [T], im: &'i mut [T]) -> Self {
        assert_eq!(re.len(), im.len());
        Self {
            re: re.as_mut_ptr(),
            im: im.as_mut_ptr(),
            life_times: std::marker::PhantomData,
        }
    }
}

impl<'r, 'i> SplitComplex<'r, 'i, f32> {
    pub fn new_f32(re: &'r mut [f32], im: &'i mut [f32]) -> Self {
        assert_eq!(re.len(), im.len());
        Self {
            re: re.as_ptr() as _,
            im: im.as_ptr() as _,
            life_times: std::marker::PhantomData,
        }
    }
}

#[doc(alias = "OpaqueFFTSetup")]
#[doc(alias = "OpaqueFFTSetupD")]
#[repr(transparent)]
pub struct FftSetup<T>(std::ffi::c_void, std::marker::PhantomData<T>);

struct FftVt<T> {
    transform_io: unsafe extern "C-unwind" fn(
        __Setup: *mut FftSetup<T>,
        __C: *mut SplitComplex<T>,
        __IC: Stride,
        __Log2N: Len,
        __Direction: FftDirection,
    ),
    transform: unsafe extern "C-unwind" fn(
        __Setup: *mut FftSetup<T>,
        __C: *const SplitComplex<T>,
        __IC: Stride,
        __Buffer: *mut SplitComplex<T>,
        __Log2N: Len,
        __Direction: FftDirection,
    ),
    zrip: unsafe extern "C-unwind" fn(
        __Setup: *mut FftSetup<T>,
        __C: *const SplitComplex<T>,
        __IC: Stride,
        __Log2N: Len,
        __Direction: FftDirection,
    ),

    destroy: unsafe extern "C-unwind" fn(*mut FftSetup<T>),
}

impl FftVt<f32> {
    pub fn new_f32() -> Self {
        Self {
            transform_io: _fft_zip_f32,
            transform: _fft_zipt_f32,
            zrip: _fft_zrip_f32,
            destroy: _destroy_fftsetup_f32,
        }
    }
}

impl FftVt<f64> {
    pub fn new_f64() -> Self {
        Self {
            transform_io: _fft_zip_f64,
            transform: _fft_zipt_f64,
            zrip: _fft_zrip_f64,
            destroy: _destroy_fftsetup_f64,
        }
    }
}

pub struct Fft<T>(NonNull<FftSetup<T>>, FftVt<T>);

impl<T> Fft<T> {
    #[inline]
    pub fn zr_io(&mut self, re_io: &mut [T], im_io: &mut [T], direction: FftDirection) {
        let log2n = (re_io.len() as f64).log2().ceil();
        let mut split = SplitComplex::new_mut(re_io, im_io);
        unsafe { (self.1.zrip)(self.0.as_mut(), &mut split, 1, log2n as _, direction) }
    }

    #[inline]
    pub fn transform_io(&mut self, re_io: &mut [T], im_io: &mut [T], direction: FftDirection) {
        let log2n = (re_io.len() as f64).log2().ceil();
        let mut split = SplitComplex::new_mut(re_io, im_io);
        unsafe { (self.1.transform_io)(self.0.as_mut(), &mut split, 1, log2n as _, direction) }
    }

    #[inline]
    pub fn forward_io(&mut self, re_io: &mut [T], im_io: &mut [T]) {
        self.transform_io(re_io, im_io, FftDirection::Forward)
    }

    #[inline]
    pub fn inverse_io(&mut self, re_io: &mut [T], im_io: &mut [T]) {
        self.transform_io(re_io, im_io, FftDirection::Inverse)
    }

    #[inline]
    pub fn transform(
        &mut self,
        re: &mut [T],
        im: &mut [T],
        tmp_re: &mut [T],
        tmp_im: &mut [T],
        direction: FftDirection,
    ) {
        let n = re.len();
        let log2n = (n as f64).log2().ceil();
        let c = SplitComplex::new_mut(re, im);
        let mut buf = SplitComplex::new_mut(tmp_re, tmp_im);
        unsafe { (self.1.transform)(self.0.as_mut(), &c, 1, &mut buf, log2n as _, direction) }
    }

    #[inline]
    pub fn forward(&mut self, re: &mut [T], im: &mut [T], tmp_re: &mut [T], tmp_im: &mut [T]) {
        self.transform(re, im, tmp_re, tmp_im, FftDirection::Forward)
    }

    #[inline]
    pub fn inverse(&mut self, re: &mut [T], im: &mut [T], buf_re: &mut [T], buf_im: &mut [T]) {
        self.transform(re, im, buf_re, buf_im, FftDirection::Inverse)
    }
}

impl Fft<f32> {
    #[inline]
    pub fn new_f32(log2n: Len, radix: FftRadix) -> Option<Self> {
        let setup = unsafe { _create_fftsetup_f32(log2n, radix) };
        setup.map(|v| Self(v, FftVt::new_f32()))
    }
}

impl Fft<f64> {
    pub fn new_f64(log2n: Len, radix: FftRadix) -> Option<Self> {
        let setup = unsafe { _create_fftsetup_f64(log2n, radix) };
        setup.map(|v| Self(v, FftVt::new_f64()))
    }
}

impl<T> Drop for Fft<T> {
    fn drop(&mut self) {
        unsafe { (self.1.destroy)(self.0.as_ptr()) };
    }
}

/// Vector add
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = A[n] + B[n];
/// ```
/// Note:
/// In order to use strides use unsafe [`_add_f32`]
#[doc(alias = "vDSP_vadd")]
#[inline]
pub fn add_f32(a: &[f32], b: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _add_f32(a.as_ptr(), 1, b.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector add
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = A[n] + B[n];
/// ```
/// Note:
/// In order to use strides use unsafe [`_add_f64`]
#[doc(alias = "vDSP_vaddD")]
#[inline]
pub fn add_f64(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _add_f64(a.as_ptr(), 1, b.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector add
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = A[n] + B[n];
/// ```
/// Note:
/// In order to use strides use unsafe [`_add_i32`]
#[doc(alias = "vDSP_vaddi")]
#[inline]
pub fn add_i32(a: &[i32], b: &[i32], c: &mut [i32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _add_i32(a.as_ptr(), 1, b.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector subtract
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = A[n] - B[n];
/// ```
/// Note:
/// In order to use strides use unsafe [`_sub_f32`]
#[doc(alias = "vDSP_vsub")]
#[inline]
pub fn sub_f32(a: &[f32], b: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _sub_f32(b.as_ptr(), 1, a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector subtract
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = A[n] - B[n];
/// ```
/// Note:
/// In order to use strides use unsafe [`_sub_f64`]
#[doc(alias = "vDSP_vsubD")]
#[inline]
pub fn sub_f64(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _sub_f64(b.as_ptr(), 1, a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector subtract
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = A[n] - B[n];
/// ```
/// Note:
/// In order to use strides use unsafe [`_sub_i32`]
#[doc(alias = "vDSP_vsubi")]
#[inline]
pub fn sub_i32(a: &[i32], b: &[i32], c: &mut [i32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _sub_i32(b.as_ptr(), 1, a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector multiply
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = A[n] * B[n];
/// ```
/// Note:
/// In order to use strides use unsafe [`_mul_f32`]
#[doc(alias = "vDSP_vmul")]
#[inline]
pub fn mul_f32(a: &[f32], b: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _mul_f32(a.as_ptr(), 1, b.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector multiply
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = A[n] * B[n];
/// ```
/// Note:
/// In order to use strides use unsafe [`_mul_f64`]
#[doc(alias = "vDSP_vmulD")]
#[inline]
pub fn mul_f64(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _mul_f64(a.as_ptr(), 1, b.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector divide
#[doc(alias = "vDSP_vsub")]
#[inline]
pub fn div_f32(a: &[f32], b: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _div_f32(b.as_ptr(), 1, a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector divide
#[doc(alias = "vDSP_vsubD")]
#[inline]
pub fn div_f64(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _div_f64(b.as_ptr(), 1, a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector divide
#[doc(alias = "vDSP_vsubi")]
#[inline]
pub fn div_i32(a: &[i32], b: &[i32], c: &mut [i32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _div_i32(b.as_ptr(), 1, a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector-scalar multiply
#[doc(alias = "vDSP_vsmul")]
#[inline]
pub fn smul_f32(a: &[f32], b: &f32, c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _smul_f32(a.as_ptr(), 1, b, c.as_mut_ptr(), 1, n) }
}

/// Vector-scalar multiply
#[doc(alias = "vDSP_vsmulD")]
#[inline]
pub fn smul_f64(a: &[f64], b: &f64, c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _smul_f64(a.as_ptr(), 1, b, c.as_mut_ptr(), 1, n) }
}

/// Vector square
#[doc(alias = "vDSP_vsq")]
#[inline]
pub fn sq_f32(a: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _sq_f32(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector square
#[doc(alias = "vDSP_vsqD")]
#[inline]
pub fn sq_f64(a: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _sq_f64(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Inplace vector square
#[doc(alias = "vDSP_vsq")]
#[inline]
pub fn sq_io_f32(io: &mut [f32]) {
    let n = io.len();
    unsafe { _sq_f32(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) }
}

/// Inplace vector square
#[doc(alias = "vDSP_vsqD")]
#[inline]
pub fn sq_io_f64(io: &mut [f64]) {
    let n = io.len();
    unsafe { _sq_f64(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) }
}

/// Vector signed square
#[doc(alias = "vDSP_vssq")]
#[inline]
pub fn ssq_f32(a: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _ssq_f32(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector signed square
#[doc(alias = "vDSP_vssqD")]
#[inline]
pub fn ssq_f64(a: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _ssq_f64(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Inplace vector signed square
#[doc(alias = "vDSP_vssq")]
#[inline]
pub fn ssq_io_f32(io: &mut [f32]) {
    let n = io.len();
    unsafe { _ssq_f32(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) }
}

/// Inplace vector signed square
#[doc(alias = "vDSP_vssqD")]
#[inline]
pub fn ssq_io_f64(io: &mut [f64]) {
    let n = io.len();
    unsafe { _ssq_f64(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) }
}

/// Mean of vector
///
/// ```
/// use cidre::vdsp;
///
/// let m = vdsp::mean_f32(&[]);
/// assert!(m.is_nan());
/// let m = vdsp::mean_f32(&[1.0, 2.0, 1.0, 1.0]);
/// assert_eq!(m, 1.25);
/// ```
#[doc(alias = "vDSP_meanv")]
#[inline]
pub fn mean_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _mean_f32(a.as_ptr(), 1, r, a.len()) })
}

#[doc(alias = "vDSP_meanv")]
#[inline]
pub fn mean_stride_f32(a: &[f32], stride: usize) -> f32 {
    let mut n = a.len();
    if stride > 1 {
        n /= stride;
    }
    with(|r| unsafe { _mean_f32(a.as_ptr(), stride as _, r, n) })
}

/// Mean of vector
#[doc(alias = "vDSP_meanvD")]
#[inline]
pub fn mean_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _mean_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Mean square of vector
#[doc(alias = "vDSP_measqv")]
#[inline]
pub fn mean_sq_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _meansq_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Mean square of vector
#[doc(alias = "vDSP_measqvD")]
#[inline]
pub fn mean_sq_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _meansq_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Euclidean distance, squared
#[doc(alias = "vDSP_distancesq")]
pub fn distance_sq_f32(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len();
    assert_eq!(n, b.len());
    with(|r| unsafe { _distance_sq_f32(a.as_ptr(), 1, b.as_ptr(), 1, r, n) })
}

/// Euclidean distance, squared
#[doc(alias = "vDSP_distancesqD")]
pub fn distance_sq_f64(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len();
    assert_eq!(n, b.len());
    with(|r| unsafe { _distance_sq_f64(a.as_ptr(), 1, b.as_ptr(), 1, r, n) })
}

/// Dot product
#[doc(alias = "vDSP_dotpr")]
#[inline]
pub fn dotpr_f32(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len();
    assert_eq!(n, b.len());
    with(|r| unsafe { _dotpr_f32(a.as_ptr(), 1, b.as_ptr(), 1, r, n) })
}

/// Dot product
#[doc(alias = "vDSP_dotprD")]
#[inline]
pub fn dotpr_f64(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len();
    assert_eq!(n, b.len());
    with(|r| unsafe { _dotpr_f64(a.as_ptr(), 1, b.as_ptr(), 1, r, n) })
}

/// Vector add and multiply
#[doc(alias = "vDSP_vam")]
#[inline]
pub fn am_f32(a: &[f32], b: &[f32], c: &[f32], d: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    assert_eq!(n, d.len());
    unsafe {
        _am_f32(
            a.as_ptr(),
            1,
            b.as_ptr(),
            1,
            c.as_ptr(),
            1,
            d.as_mut_ptr(),
            1,
            n,
        )
    }
}

/// Vector add and multiply
#[doc(alias = "vDSP_vamD")]
#[inline]
pub fn am_f64(a: &[f64], b: &[f64], c: &[f64], d: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    assert_eq!(n, d.len());
    unsafe {
        _am_f64(
            a.as_ptr(),
            1,
            b.as_ptr(),
            1,
            c.as_ptr(),
            1,
            d.as_mut_ptr(),
            1,
            n,
        )
    }
}

/// Vector multiply and add
#[doc(alias = "vDSP_vma")]
#[inline]
pub fn ma_f32(a: &[f32], b: &[f32], c: &[f32], d: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    assert_eq!(n, d.len());
    unsafe {
        _ma_f32(
            a.as_ptr(),
            1,
            b.as_ptr(),
            1,
            c.as_ptr(),
            1,
            d.as_mut_ptr(),
            1,
            n,
        )
    }
}

/// Vector multiply and add
#[doc(alias = "vDSP_vmaD")]
#[inline]
pub fn ma_f64(a: &[f64], b: &[f64], c: &[f64], d: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    assert_eq!(n, d.len());
    unsafe {
        _ma_f64(
            a.as_ptr(),
            1,
            b.as_ptr(),
            1,
            c.as_ptr(),
            1,
            d.as_mut_ptr(),
            1,
            n,
        )
    }
}

/// Vector convert between double precision and single precision
#[doc(alias = "vDSP_vdpsp")]
#[inline]
pub fn f64_f32(a: &[f64], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _f64_f32(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector convert between single precision and double precision
#[doc(alias = "vDSP_vspdp")]
#[inline]
pub fn f32_f64(a: &[f32], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _f32_f64(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Vector absolute value
#[doc(alias = "vDSP_vabs")]
#[inline]
pub fn abs_f32(a: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _abs_f32(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Inplace vector absolute value
#[doc(alias = "vDSP_vabs")]
#[inline]
pub fn abs_io_f32(io: &mut [f32]) {
    let n = io.len();
    unsafe { _abs_f32(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) }
}

/// Vector absolute value
#[doc(alias = "vDSP_vabsD")]
#[inline]
pub fn abs_f64(a: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _abs_f64(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Inplace vector absolute value
#[doc(alias = "vDSP_vabsD")]
#[inline]
pub fn abs_io_f64(io: &mut [f64]) {
    let n = io.len();
    unsafe { _abs_f64(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) }
}

/// Vector absolute value
#[doc(alias = "vDSP_vabsi")]
#[inline]
pub fn abs_i32(a: &[i32], c: &mut [i32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _abs_i32(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

/// Inplace vector absolute value
#[doc(alias = "vDSP_vabsi")]
#[inline]
pub fn abs_io_i32(io: &mut [i32]) {
    let n = io.len();
    unsafe { _abs_i32(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) }
}

/// Vector bit-wise equivalence, NOT (A XOR B)
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = ~(A[n] ^ B[n]);
/// ```
#[doc(alias = "vDSP_veqvi")]
#[inline]
pub fn eqv_i32(a: &[i32], b: &[i32], c: &mut [i32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _eqv_i32(a.as_ptr(), 1, b.as_ptr(), 1, c.as_mut_ptr(), 1, n) }
}

#[doc(alias = "vDSP_vfill")]
#[inline]
pub fn fill_f32(a: &f32, c: &mut [f32]) {
    let n = c.len();
    unsafe { _fill_f32(a, c.as_mut_ptr(), 1, n) }
}

#[doc(alias = "vDSP_vfillD")]
#[inline]
pub fn fill_f64(a: &f64, c: &mut [f64]) {
    let n = c.len();
    unsafe { _fill_f64(a, c.as_mut_ptr(), 1, n) }
}

#[doc(alias = "vDSP_vfilli")]
#[inline]
pub fn fill_i32(a: &i32, c: &mut [i32]) {
    let n = c.len();
    unsafe { _fill_i32(a, c.as_mut_ptr(), 1, n) }
}

/// Vector clear
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = 0;
/// ```
/// Note:
/// In order to use strides use unsafe [`_clr_f32`]
#[doc(alias = "vDSP_vclr")]
#[inline]
pub fn clr_f32(c: &mut [f32]) {
    unsafe { _clr_f32(c.as_mut_ptr(), 1, c.len()) }
}

/// Vector clear
///
/// ```pseudo C
/// for (n = 0; n < N; ++n)
///     C[n] = 0;
/// ```
/// Note:
/// In order to use strides use unsafe [`_clr_f64`]
#[doc(alias = "vDSP_vclrD")]
#[inline]
pub fn clr_f64(c: &mut [f64]) {
    unsafe { _clr_f64(c.as_mut_ptr(), 1, c.len()) }
}

/// Vector-scalar add
#[doc(alias = "vDSP_vsadd")]
#[inline]
pub fn sadd_f32(a: &[f32], b: &f32, c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _sadd_f32(a.as_ptr(), 1, b, c.as_mut_ptr(), 1, n) }
}

/// Vector-scalar add
#[doc(alias = "vDSP_vsaddD")]
#[inline]
pub fn sadd_f64(a: &[f64], b: &f64, c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _sadd_f64(a.as_ptr(), 1, b, c.as_mut_ptr(), 1, n) }
}

/// Vector-scalar add
#[doc(alias = "vDSP_vsaddD")]
#[inline]
pub fn sadd_i32(a: &[i32], b: &i32, c: &mut [i32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _sadd_i32(a.as_ptr(), 1, b, c.as_mut_ptr(), 1, n) }
}

/// Sum of vector elements
#[doc(alias = "vDSP_sve")]
#[inline]
pub fn se_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _se_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Sum of vector elements with stride
#[doc(alias = "vDSP_sve")]
#[inline]
pub fn se_stride_f32(a: &[f32], stride: usize) -> f32 {
    let mut n = a.len();
    if stride > 1 {
        n /= stride;
    }
    with(|r| unsafe { _se_f32(a.as_ptr(), stride as _, r, n) })
}

/// Sum of vector elements
#[doc(alias = "vDSP_sveD")]
#[inline]
pub fn se_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _se_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Sum of vector elements with stride
#[doc(alias = "vDSP_sveD")]
#[inline]
pub fn se_stride_f64(a: &[f64], stride: usize) -> f64 {
    let mut n = a.len();
    if stride > 1 {
        n /= stride;
    }
    with(|r| unsafe { _se_f64(a.as_ptr(), stride as _, r, n) })
}

/// Sum of vector elements magnitudes
#[doc(alias = "vDSP_svemg")]
#[inline]
pub fn semg_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _semg_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Sum of vector elements magnitudes
#[doc(alias = "vDSP_svemgD")]
#[inline]
pub fn semg_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _semg_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Sum of vector elements' squares
#[doc(alias = "vDSP_svesq")]
#[inline]
pub fn sesq_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _sesq_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Sum of vector elements' squares
#[doc(alias = "vDSP_svesqD")]
#[inline]
pub fn sesq_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _sesq_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Sum of vector elements' signed squares
#[doc(alias = "vDSP_svs")]
#[inline]
pub fn svs_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _svs_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Sum of vector elements' signed squares
#[doc(alias = "vDSP_svsD")]
#[inline]
pub fn svs_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _svs_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Maximum magnitude of vector
#[doc(alias = "vDSP_maxmgv")]
#[inline]
pub fn maxmg_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _maxmg_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Maximum magnitude of vector
#[doc(alias = "vDSP_maxmgvD")]
#[inline]
pub fn maxmg_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _maxmg_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Minimum magnitude of vector
#[doc(alias = "vDSP_minmgv")]
#[inline]
pub fn minmg_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _minmg_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Minimum magnitude of vector
#[doc(alias = "vDSP_minmgvD")]
#[inline]
pub fn minmg_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _minmg_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Minimum value of vector
#[doc(alias = "vDSP_minv")]
#[inline]
pub fn min_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _min_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Minimum value of vector
///
/// ```
/// use cidre::vdsp;
///
/// let a = [10.0f32, -5.0, 0.0, 5.0];
/// let min = vdsp::min_stride_f32(&[], 0);
/// let min0 = vdsp::min_stride_f32(&a, 0);
/// let min1 = vdsp::min_stride_f32(&a, 1);
/// let min2 = vdsp::min_stride_f32(&a, 2);
/// let min3 = vdsp::min_stride_f32(&a, 3);
/// let min4 = vdsp::min_stride_f32(&a, 4);
/// assert_eq!(min, f32::INFINITY);
/// assert_eq!(min0, 10.0f32);
/// assert_eq!(min1, -5.0f32);
/// assert_eq!(min2, 0.0f32);
/// assert_eq!(min3, 10.0f32);
/// assert_eq!(min4, 10.0f32);
/// ```
#[doc(alias = "vDSP_minv")]
#[inline]
pub fn min_stride_f32(a: &[f32], stride: usize) -> f32 {
    let mut n = a.len();
    if stride > 1 {
        n /= stride;
    }
    with(|r| unsafe { _min_f32(a.as_ptr(), stride as isize, r, n) })
}

/// Minimum value of vector
#[doc(alias = "vDSP_minvD")]
#[inline]
pub fn min_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _min_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Maximum value of vector
#[doc(alias = "vDSP_maxv")]
#[inline]
pub fn max_f32(a: &[f32]) -> f32 {
    with(|r| unsafe { _max_f32(a.as_ptr(), 1, r, a.len()) })
}

/// Maximum value of vector
#[doc(alias = "vDSP_maxv")]
#[inline]
pub fn max_stride_f32(a: &[f32], stride: usize) -> f32 {
    let mut n = a.len();
    if stride > 1 {
        n /= stride;
    }
    with(|r| unsafe { _max_f32(a.as_ptr(), stride as isize, r, n) })
}

/// Maximum value of vector
#[doc(alias = "vDSP_maxvD")]
#[inline]
pub fn max_f64(a: &[f64]) -> f64 {
    with(|r| unsafe { _max_f64(a.as_ptr(), 1, r, a.len()) })
}

/// Vector generate tapered ramp
#[doc(alias = "vDSP_vgen")]
#[inline]
pub fn gen_f32(a: &f32, b: &f32, c: &mut [f32]) {
    unsafe { _gen_f32(a, b, c.as_mut_ptr(), 1, c.len()) }
}

/// Vector generate tapered ramp
#[doc(alias = "vDSP_vgenD")]
#[inline]
pub fn gen_f64(a: &f64, b: &f64, c: &mut [f64]) {
    unsafe { _gen_f64(a, b, c.as_mut_ptr(), 1, c.len()) }
}

/// Vector build ramp
#[doc(alias = "vDSP_vramp")]
#[inline]
pub fn ramp_f32(a: &f32, b: &f32, c: &mut [f32]) {
    unsafe { _ramp_f32(a, b, c.as_mut_ptr(), 1, c.len()) }
}

/// Vector build ramp
#[doc(alias = "vDSP_vrampD")]
#[inline]
pub fn ramp_f64(a: &f64, b: &f64, c: &mut [f64]) {
    unsafe { _ramp_f64(a, b, c.as_mut_ptr(), 1, c.len()) }
}

/// Vector single-precision vramp and multiply.
#[doc(alias = "vDSP_vrampmul")]
#[inline]
pub fn rampmul_f32(i: &[f32], start: &mut f32, step: &f32, o: &mut [f32]) {
    let n = i.len();
    assert_eq!(n, o.len());
    unsafe { _rampmul_f32(i.as_ptr(), 1, start, step, o.as_mut_ptr(), 1, n) };
}

/// Vector double-precision vramp and multiply.
#[doc(alias = "vDSP_vrampmulD")]
#[inline]
pub fn rampmul_f64(i: &[f64], start: &mut f64, step: &f64, o: &mut [f64]) {
    let n = i.len();
    assert_eq!(n, o.len());
    unsafe { _rampmul_f64(i.as_ptr(), 1, start, step, o.as_mut_ptr(), 1, n) };
}

/// Inplace vector single-precision vramp and multiply.
#[doc(alias = "vDSP_vrampmul")]
#[inline]
pub fn rampmul_io_f32(io: &mut [f32], start: &mut f32, step: &f32) {
    unsafe { _rampmul_f32(io.as_ptr(), 1, start, step, io.as_mut_ptr(), 1, io.len()) };
}

/// Inplace vector double-precision vramp and multiply.
#[doc(alias = "vDSP_vrampmulD")]
#[inline]
pub fn rampmul_io_f64(io: &mut [f64], start: &mut f64, step: &f64) {
    unsafe { _rampmul_f64(io.as_ptr(), 1, start, step, io.as_mut_ptr(), 1, io.len()) };
}

/// Stereo vector single-precision vramp and multiply.
#[doc(alias = "vDSP_vrampmul2")]
#[inline]
pub fn rampmul2_f32(
    i0: &[f32],
    i1: &[f32],
    start: &mut f32,
    step: &f32,
    o0: &mut [f32],
    o1: &mut [f32],
) {
    let n = i0.len();
    assert_eq!(n, i1.len());
    assert_eq!(n, o0.len());
    assert_eq!(n, o1.len());
    unsafe {
        _rampmul2_f32(
            i0.as_ptr(),
            i1.as_ptr(),
            1,
            start,
            step,
            o0.as_mut_ptr(),
            o1.as_mut_ptr(),
            1,
            n,
        )
    }
}

/// Inplace stereo vector single-precision vramp and multiply.
#[doc(alias = "vDSP_vrampmul2")]
#[inline]
pub fn rampmul2_io_f32(io0: &mut [f32], io1: &mut [f32], start: &mut f32, step: &f32) {
    let n = io0.len();
    assert_eq!(n, io1.len());
    unsafe {
        _rampmul2_f32(
            io0.as_ptr(),
            io1.as_ptr(),
            1,
            start,
            step,
            io0.as_mut_ptr(),
            io1.as_mut_ptr(),
            1,
            n,
        )
    }
}

/// Stereo vector double-precision vramp and multiply.
#[doc(alias = "vDSP_vrampmul2D")]
#[inline]
pub fn rampmul2_f64(
    i0: &[f64],
    i1: &[f64],
    start: &mut f64,
    step: &f64,
    o0: &mut [f64],
    o1: &mut [f64],
) {
    let n = i0.len();
    assert_eq!(n, i1.len());
    assert_eq!(n, o0.len());
    assert_eq!(n, o1.len());
    unsafe {
        _rampmul2_f64(
            i0.as_ptr(),
            i1.as_ptr(),
            1,
            start,
            step,
            o0.as_mut_ptr(),
            o1.as_mut_ptr(),
            1,
            n,
        )
    }
}

/// Inplace stereo vector double-precision vramp and multiply.
#[doc(alias = "vDSP_vrampmul2D")]
#[inline]
pub fn rampmul2_io_f64(io0: &mut [f64], io1: &mut [f64], start: &mut f64, step: &f64) {
    let n = io0.len();
    assert_eq!(n, io1.len());
    unsafe {
        _rampmul2_f64(
            io0.as_ptr(),
            io1.as_ptr(),
            1,
            start,
            step,
            io0.as_mut_ptr(),
            io1.as_mut_ptr(),
            1,
            n,
        )
    }
}

#[doc(alias = "vDSP_vneg")]
#[inline]
pub fn neg_f32(a: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _neg_f32(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) };
}

#[doc(alias = "vDSP_vneg")]
#[inline]
pub fn neg_io_f32(io: &mut [f32]) {
    let n = io.len();
    unsafe { _neg_f32(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) };
}

#[doc(alias = "vDSP_vnegD")]
#[inline]
pub fn neg_f64(a: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _neg_f64(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) };
}

#[doc(alias = "vDSP_vnegD")]
#[inline]
pub fn neg_io_f64(io: &mut [f64]) {
    let n = io.len();
    unsafe { _neg_f64(io.as_ptr(), 1, io.as_mut_ptr(), 1, n) };
}

/// Vector tapered merge.
#[doc(alias = "vDSP_vtmerg")]
#[inline]
pub fn tmerg_f32(a: &[f32], b: &[f32], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _tmerg_f32(a.as_ptr(), 1, b.as_ptr(), 1, c.as_mut_ptr(), 1, n) };
}

/// Vector tapered merge.
#[doc(alias = "vDSP_vtmerg")]
#[inline]
pub fn tmerg_f64(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert_eq!(n, c.len());
    unsafe { _tmerg_f64(a.as_ptr(), 1, b.as_ptr(), 1, c.as_mut_ptr(), 1, n) };
}

/// Complex-split accumulating autospectrum
#[doc(alias = "vDSP_zaspec")]
#[inline]
pub fn zaspec_f32(a: &SplitComplex<f32>, c: &mut [f32]) {
    unsafe { _zaspec_f32(a, c.as_mut_ptr(), c.len()) }
}

/// Complex-split accumulating autospectrum
#[doc(alias = "vDSP_zaspecD")]
#[inline]
pub fn zaspec_f64(a: &SplitComplex<f64>, c: &mut [f64]) {
    unsafe { _zaspec_f64(a, c.as_mut_ptr(), c.len()) }
}

/// Convert a complex array to a complex-split array
#[doc(alias = "vDSP_ctoz")]
#[inline]
pub fn ctoz_f32(c: &[Complex<f32>], z_re: &mut [f32], z_im: &mut [f32]) {
    let n = c.len();
    assert_eq!(n, z_re.len());
    assert_eq!(n, z_im.len());
    unsafe {
        let split = SplitComplex::new_mut(z_re, z_im);
        _ctoz_f32(c.as_ptr(), 2, &split, 1, n)
    }
}

#[doc(alias = "vDSP_ctoz")]
#[inline]
pub fn actoz_f32(a: &[f32], z_re: &mut [f32], z_im: &mut [f32]) {
    let n = a.len() / 2;
    assert_eq!(n, z_re.len());
    assert_eq!(n, z_im.len());
    let c = a.as_ptr() as *const Complex<f32>;
    let split = SplitComplex::new_mut(z_re, z_im);
    unsafe { _ctoz_f32(c, 2, &split, 1, n) }
}

#[doc(alias = "vDSP_ctozD")]
#[inline]
pub fn ctoz_f64(c: &[Complex<f64>], z_re: &mut [f64], z_im: &mut [f64]) {
    let n = c.len();
    assert_eq!(n, z_re.len());
    assert_eq!(n, z_im.len());
    unsafe {
        let split = SplitComplex::new_mut(z_re, z_im);
        _ctoz_f64(c.as_ptr(), 2, &split, 1, n)
    }
}

/// Convert a complex-split array to a complex array
#[doc(alias = "vDSP_ztoc")]
#[inline]
pub fn ztoc_f32(re: &[f32], im: &[f32], c: &mut [Complex<f32>]) {
    let n = re.len();
    assert_eq!(n, im.len());
    assert_eq!(n, c.len());
    let split = SplitComplex::new(re, im);
    unsafe { _ztoc_f32(&split, 1, c.as_mut_ptr(), 2, n) };
}

/// Convert a complex-split array to a complex array
#[doc(alias = "vDSP_ztocD")]
#[inline]
pub fn ztoc_f64(re: &[f64], im: &[f64], c: &mut [Complex<f64>]) {
    let n = re.len();
    assert_eq!(n, im.len());
    assert_eq!(n, c.len());
    let split = SplitComplex::new(re, im);
    unsafe { _ztoc_f64(&split, 1, c.as_mut_ptr(), 2, n) };
}

#[doc(alias = "vDSP_vflt16")]
#[inline]
pub fn i16_f32(a: &[i16], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _i16_f32(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) };
}

#[doc(alias = "vDSP_vfltu16")]
#[inline]
pub fn u16_f32(a: &[u16], c: &mut [f32]) {
    let n = a.len();
    assert_eq!(n, c.len());
    unsafe { _u16_f32(a.as_ptr(), 1, c.as_mut_ptr(), 1, n) };
}

#[link(name = "Accelerate", kind = "framework")]
extern "C-unwind" {
    #[link_name = "vDSP_vadd"]
    pub fn _add_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: *mut f32,
        __IC: Stride,
        __N: Len,
    );

    #[link_name = "vDSP_vaddD"]
    pub fn _add_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: *mut f64,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vaddi")]
    #[link_name = "vDSP_vaddi"]
    pub fn _add_i32(
        __A: *const i32,
        __IA: Stride,
        __B: *const i32,
        __IB: Stride,
        __C: *mut i32,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vsub")]
    #[link_name = "vDSP_vsub"]
    pub fn _sub_f32(
        __B: *const f32,
        __IB: Stride,
        __A: *const f32,
        __IA: Stride,
        __C: *mut f32,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vsubD")]
    #[link_name = "vDSP_vsubD"]
    pub fn _sub_f64(
        __B: *const f64,
        __IB: Stride,
        __A: *const f64,
        __IA: Stride,
        __C: *mut f64,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vsubi")]
    #[link_name = "vDSP_vsubi"]
    pub fn _sub_i32(
        __B: *const i32,
        __IB: Stride,
        __A: *const i32,
        __IA: Stride,
        __C: *mut i32,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vmul")]
    #[link_name = "vDSP_vmul"]
    pub fn _mul_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: *mut f32,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vmulD")]
    #[link_name = "vDSP_vmulD"]
    pub fn _mul_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: *mut f64,
        __IC: Stride,
        __N: Len,
    );
    #[doc(alias = "vDSP_vdiv")]
    #[link_name = "vDSP_vdiv"]
    pub fn _div_f32(
        __B: *const f32,
        __IB: Stride,
        __A: *const f32,
        __IA: Stride,
        __C: *mut f32,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vdivD")]
    #[link_name = "vDSP_vdivD"]
    pub fn _div_f64(
        __B: *const f64,
        __IB: Stride,
        __A: *const f64,
        __IA: Stride,
        __C: *mut f64,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vdivi")]
    #[link_name = "vDSP_vdivi"]
    pub fn _div_i32(
        __B: *const i32,
        __IB: Stride,
        __A: *const i32,
        __IA: Stride,
        __C: *mut i32,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vsmul")]
    #[link_name = "vDSP_vsmul"]
    pub fn _smul_f32(
        __A: *const f32,
        __IA: Stride,
        __B: &f32,
        __C: *mut f32,
        __IC: Stride,
        __N: Len,
    );
    #[doc(alias = "vDSP_vsmulD")]
    #[link_name = "vDSP_vsmulD"]
    pub fn _smul_f64(
        __A: *const f64,
        __IA: Stride,
        __B: &f64,
        __C: *mut f64,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vsq")]
    #[link_name = "vDSP_vsq"]
    pub fn _sq_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vsqD")]
    #[link_name = "vDSP_vsqD"]
    pub fn _sq_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vssq")]
    #[link_name = "vDSP_vssq"]
    pub fn _ssq_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vssqD")]
    #[link_name = "vDSP_vssqD"]
    pub fn _ssq_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __IC: Stride, __N: Len);

    /// Mean of vector
    #[doc(alias = "vDSP_meanv")]
    #[link_name = "vDSP_meanv"]
    pub fn _mean_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Mean of vector
    #[doc(alias = "vDSP_meanvD")]
    #[link_name = "vDSP_meanvD"]
    pub fn _mean_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Mean square of vector
    #[doc(alias = "vDSP_measqv")]
    #[link_name = "vDSP_measqv"]
    pub fn _meansq_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Mean square of vector
    #[doc(alias = "vDSP_measqvD")]
    #[link_name = "vDSP_measqvD"]
    pub fn _meansq_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    #[doc(alias = "vDSP_distancesq")]
    #[link_name = "vDSP_distancesq"]
    pub fn _distance_sq_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: *mut f32,
        __N: Len,
    );

    #[doc(alias = "vDSP_distancesqD")]
    #[link_name = "vDSP_distancesqD"]
    pub fn _distance_sq_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: *mut f64,
        __N: Len,
    );

    #[doc(alias = "vDSP_dotpr")]
    #[link_name = "vDSP_dotpr"]
    pub fn _dotpr_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: *mut f32,
        __N: Len,
    );

    #[doc(alias = "vDSP_dotprD")]
    #[link_name = "vDSP_dotprD"]
    pub fn _dotpr_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: *mut f64,
        __N: Len,
    );

    /// Vector add and multiply.
    #[doc(alias = "vDSP_vam")]
    #[link_name = "vDSP_vam"]
    pub fn _am_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: *const f32,
        __IC: Stride,
        __D: *mut f32,
        __ID: Stride,
        __N: Len,
    );

    /// Vector add and multiply.
    #[doc(alias = "vDSP_vamD")]
    #[link_name = "vDSP_vamD"]
    pub fn _am_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: *const f64,
        __IC: Stride,
        __D: *mut f64,
        __ID: Stride,
        __N: Len,
    );

    /// Vector multiply and add.
    #[doc(alias = "vDSP_vma")]
    #[link_name = "vDSP_vma"]
    pub fn _ma_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: *const f32,
        __IC: Stride,
        __D: *mut f32,
        __ID: Stride,
        __N: Len,
    );

    /// Vector multiply and add.
    #[doc(alias = "vDSP_vmaD")]
    #[link_name = "vDSP_vmaD"]
    pub fn _ma_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: *const f64,
        __IC: Stride,
        __D: *mut f64,
        __ID: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vdpsp")]
    #[link_name = "vDSP_vdpsp"]
    pub fn _f64_f32(__A: *const f64, __IA: Stride, __C: *mut f32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vspdp")]
    #[link_name = "vDSP_vspdp"]
    pub fn _f32_f64(__A: *const f32, __IA: Stride, __C: *mut f64, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vabs")]
    #[link_name = "vDSP_vabs"]
    pub fn _abs_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vabsD")]
    #[link_name = "vDSP_vabsD"]
    pub fn _abs_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vabsi")]
    #[link_name = "vDSP_vabsi"]
    pub fn _abs_i32(__A: *const i32, __IA: Stride, __C: *mut i32, __IC: Stride, __N: Len);

    /// Vector bit-wise equivalence, NOT (A XOR B)
    #[doc(alias = "vDSP_veqvi")]
    #[link_name = "vDSP_veqvi"]
    pub fn _eqv_i32(
        __A: *const i32,
        __IA: Stride,
        __B: *const i32,
        __IB: Stride,
        __C: *mut i32,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vfill")]
    #[link_name = "vDSP_vfill"]
    pub fn _fill_f32(__A: &f32, __C: *mut f32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vfillD")]
    #[link_name = "vDSP_vfillD"]
    pub fn _fill_f64(__A: &f64, __C: *mut f64, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vfilli")]
    #[link_name = "vDSP_vfilli"]
    pub fn _fill_i32(__A: &i32, __C: *mut i32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vclr")]
    #[link_name = "vDSP_vclr"]
    pub fn _clr_f32(__C: *mut f32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vclrD")]
    #[link_name = "vDSP_vclrD"]
    pub fn _clr_f64(__C: *mut f64, __IC: Stride, __N: Len);

    /// Vector-scalar add
    #[doc(alias = "vDSP_vsadd")]
    #[link_name = "vDSP_vsadd"]
    pub fn _sadd_f32(
        __A: *const f32,
        __IA: Stride,
        __B: &f32,
        __C: *mut f32,
        __IC: Stride,
        __N: Len,
    );

    /// Vector-scalar add
    #[doc(alias = "vDSP_vsaddD")]
    #[link_name = "vDSP_vsaddD"]
    pub fn _sadd_f64(
        __A: *const f64,
        __IA: Stride,
        __B: &f64,
        __C: *mut f64,
        __IC: Stride,
        __N: Len,
    );

    /// Vector-scalar add
    #[doc(alias = "vDSP_vsaddi")]
    #[link_name = "vDSP_vsaddi"]
    pub fn _sadd_i32(
        __A: *const i32,
        __IA: Stride,
        __B: &i32,
        __C: *mut i32,
        __IC: Stride,
        __N: Len,
    );

    /// Sum of vector elements
    #[doc(alias = "vDSP_sve")]
    #[link_name = "vDSP_sve"]
    pub fn _se_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Sum of vector elements
    #[doc(alias = "vDSP_sveD")]
    #[link_name = "vDSP_sveD"]
    pub fn _se_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Sum of vector elements magnitudes
    #[doc(alias = "vDSP_svemg")]
    #[link_name = "vDSP_svemg"]
    pub fn _semg_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Sum of vector elements magnitudes
    #[doc(alias = "vDSP_svemgD")]
    #[link_name = "vDSP_svemgD"]
    pub fn _semg_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Sum of vector elements' squares
    #[doc(alias = "vDSP_svesq")]
    #[link_name = "vDSP_svesq"]
    pub fn _sesq_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Sum of vector elements' squares
    #[doc(alias = "vDSP_svesqD")]
    #[link_name = "vDSP_svesqD"]
    pub fn _sesq_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Sum of vector elements' signed squares
    #[doc(alias = "vDSP_svs")]
    #[link_name = "vDSP_svs"]
    pub fn _svs_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Sum of vector elements' signed squares
    #[doc(alias = "vDSP_svsD")]
    #[link_name = "vDSP_svsD"]
    pub fn _svs_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Maximum magnitude of vector
    #[doc(alias = "vDSP_maxmgv")]
    #[link_name = "vDSP_maxmgv"]
    pub fn _maxmg_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Maximum magnitude of vector
    #[doc(alias = "vDSP_maxmgvD")]
    #[link_name = "vDSP_maxmgvD"]
    pub fn _maxmg_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Minimum magnitude of vector
    #[doc(alias = "vDSP_minmgv")]
    #[link_name = "vDSP_minmgv"]
    pub fn _minmg_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Mimimum magnitude of vector
    #[doc(alias = "vDSP_minmgvD")]
    #[link_name = "vDSP_minmgvD"]
    pub fn _minmg_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Maximum value of vector.
    #[doc(alias = "vDSP_maxv")]
    #[link_name = "vDSP_maxv"]
    pub fn _max_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Maximum value of vector.
    #[doc(alias = "vDSP_maxvD")]
    #[link_name = "vDSP_maxvD"]
    pub fn _max_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Minimum value of vector
    #[doc(alias = "vDSP_minv")]
    #[link_name = "vDSP_minv"]
    pub fn _min_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __N: Len);

    /// Minimum value of vector
    #[doc(alias = "vDSP_minvD")]
    #[link_name = "vDSP_minvD"]
    pub fn _min_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __N: Len);

    /// Vector generate tapered ramp
    #[doc(alias = "vDSP_vgen")]
    #[link_name = "vDSP_vgen"]
    pub fn _gen_f32(__A: &f32, __B: &f32, __C: *mut f32, __IC: Stride, __N: Len);

    /// Vector generate tapered ramp
    #[doc(alias = "vDSP_vgenD")]
    #[link_name = "vDSP_vgenD"]
    pub fn _gen_f64(__A: &f64, __B: &f64, __C: *mut f64, __IC: Stride, __N: Len);

    /// Vector build ramp
    #[doc(alias = "vDSP_vramp")]
    #[link_name = "vDSP_vramp"]
    pub fn _ramp_f32(__A: &f32, __B: &f32, __C: *mut f32, __IC: Stride, __N: Len);

    /// Vector build ramp
    #[doc(alias = "vDSP_vrampD")]
    #[link_name = "vDSP_vrampD"]
    pub fn _ramp_f64(__A: &f64, __B: &f64, __C: *mut f64, __IC: Stride, __N: Len);

    /// Vector single-precision vramp and multiply.
    #[doc(alias = "vDSP_vrampmul")]
    #[link_name = "vDSP_vrampmul"]
    pub fn _rampmul_f32(
        __I: *const f32,
        __IS: Stride,
        __Start: &mut f32,
        __Step: &f32,
        __O: *mut f32,
        __OS: Stride,
        __N: Len,
    );

    /// Vector single-precision vramp and multiply.
    #[doc(alias = "vDSP_vrampmulD")]
    #[link_name = "vDSP_vrampmulD"]
    pub fn _rampmul_f64(
        __I: *const f64,
        __IS: Stride,
        __Start: &mut f64,
        __Step: &f64,
        __O: *mut f64,
        __OS: Stride,
        __N: Len,
    );

    /// Stereo vector single-precision vramp and multiply.
    #[doc(alias = "vDSP_vrampmul2")]
    #[link_name = "vDSP_vrampmul2"]
    pub fn _rampmul2_f32(
        __I0: *const f32,
        __I1: *const f32,
        __IS: Stride,
        __Start: &mut f32,
        __Step: &f32,
        __O0: *mut f32,
        __O1: *mut f32,
        __OS: Stride,
        __N: Len,
    );

    /// Stereo vector double-precision vramp and multiply.
    #[doc(alias = "vDSP_vrampmul2D")]
    #[link_name = "vDSP_vrampmul2D"]
    pub fn _rampmul2_f64(
        __I0: *const f64,
        __I1: *const f64,
        __IS: Stride,
        __Start: &mut f64,
        __Step: &f64,
        __O0: *mut f64,
        __O1: *mut f64,
        __OS: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vneg")]
    #[link_name = "vDSP_vneg"]
    pub fn _neg_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vnegD")]
    #[link_name = "vDSP_vnegD"]
    pub fn _neg_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __IC: Stride, __N: Len);

    /// Vector tapered merge.
    #[doc(alias = "vDSP_vtmerg")]
    #[link_name = "vDSP_vtmerg"]
    pub fn _tmerg_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: *mut f32,
        __IC: Stride,
        __N: Len,
    );

    /// Vector tapered merge.
    #[doc(alias = "vDSP_vtmergD")]
    #[link_name = "vDSP_vtmergD"]
    pub fn _tmerg_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: *mut f64,
        __IC: Stride,
        __N: Len,
    );

    /// Convert a complex array to a complex-split array
    #[doc(alias = "vDSP_ctoz")]
    #[link_name = "vDSP_ctoz"]
    pub fn _ctoz_f32(
        __C: *const Complex<f32>,
        __IC: Stride,
        __Z: *const SplitComplex<f32>,
        __IZ: Stride,
        __N: Len,
    );

    /// Convert a complex array to a complex-split array
    #[doc(alias = "vDSP_ctozD")]
    #[link_name = "vDSP_ctozD"]
    pub fn _ctoz_f64(
        __C: *const Complex<f64>,
        __IC: Stride,
        __Z: *const SplitComplex<f64>,
        __IZ: Stride,
        __N: Len,
    );

    /// Convert a complex-split array to a complex array
    #[doc(alias = "vDSP_ztoc")]
    #[link_name = "vDSP_ztoc"]
    pub fn _ztoc_f32(
        __Z: *const SplitComplex<f32>,
        __IZ: Stride,
        __C: *mut Complex<f32>,
        __IC: Stride,
        __N: Len,
    );

    /// Convert a complex-split array to a complex array
    #[doc(alias = "vDSP_ztocD")]
    #[link_name = "vDSP_ztocD"]
    pub fn _ztoc_f64(
        __Z: *const SplitComplex<f64>,
        __IZ: Stride,
        __C: *mut Complex<f64>,
        __IC: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_create_fftsetup")]
    #[link_name = "vDSP_create_fftsetup"]
    pub fn _create_fftsetup_f32(__Log2n: Len, __Radix: FftRadix) -> Option<NonNull<FftSetup<f32>>>;

    #[doc(alias = "vDSP_create_fftsetupD")]
    #[link_name = "vDSP_create_fftsetupD"]
    pub fn _create_fftsetup_f64(__Log2n: Len, __Radix: FftRadix) -> Option<NonNull<FftSetup<f64>>>;

    #[doc(alias = "vDSP_destroy_fftsetup")]
    #[link_name = "vDSP_destroy_fftsetup"]
    pub fn _destroy_fftsetup_f32(setup: *mut FftSetup<f32>);

    #[doc(alias = "vDSP_destroy_fftsetupD")]
    #[link_name = "vDSP_destroy_fftsetupD"]
    pub fn _destroy_fftsetup_f64(setup: *mut FftSetup<f64>);

    #[doc(alias = "vDSP_fft_zip")]
    #[link_name = "vDSP_fft_zip"]
    pub fn _fft_zip_f32(
        __Setup: *mut FftSetup<f32>,
        __C: *mut SplitComplex<f32>,
        __IC: Stride,
        __Log2N: Len,
        __Direction: FftDirection,
    );

    #[doc(alias = "vDSP_fft_zipD")]
    #[link_name = "vDSP_fft_zipD"]
    pub fn _fft_zip_f64(
        __Setup: *mut FftSetup<f64>,
        __C: *mut SplitComplex<f64>,
        __IC: Stride,
        __Log2N: Len,
        __Direction: FftDirection,
    );
    #[doc(alias = "vDSP_fft_zipt")]
    #[link_name = "vDSP_fft_zipt"]
    pub fn _fft_zipt_f32(
        __Setup: *mut FftSetup<f32>,
        __C: *const SplitComplex<f32>,
        __IC: Stride,
        __Buffer: *mut SplitComplex<f32>,
        __Log2N: Len,
        __Direction: FftDirection,
    );

    #[doc(alias = "vDSP_fft_ziptD")]
    #[link_name = "vDSP_fft_ziptD"]
    pub fn _fft_zipt_f64(
        __Setup: *mut FftSetup<f64>,
        __C: *const SplitComplex<f64>,
        __IC: Stride,
        __Buffer: *mut SplitComplex<f64>,
        __Log2N: Len,
        __Direction: FftDirection,
    );

    #[doc(alias = "vDSP_fft_zrip")]
    #[link_name = "vDSP_fft_zrip"]
    pub fn _fft_zrip_f32(
        __Setup: *mut FftSetup<f32>,
        __C: *const SplitComplex<f32>,
        __IC: Stride,
        __Log2N: Len,
        __Direction: FftDirection,
    );

    #[doc(alias = "vDSP_fft_zripD")]
    #[link_name = "vDSP_fft_zripD"]
    pub fn _fft_zrip_f64(
        __Setup: *mut FftSetup<f64>,
        __C: *const SplitComplex<f64>,
        __IC: Stride,
        __Log2N: Len,
        __Direction: FftDirection,
    );

    /// Complex-split accumulating autospectrum
    #[doc(alias = "vDSP_zaspec")]
    #[link_name = "vDSP_zaspec"]
    pub fn _zaspec_f32(__A: *const SplitComplex<f32>, __C: *mut f32, __N: Len);

    /// Complex-split accumulating autospectrum
    #[doc(alias = "vDSP_zaspecD")]
    #[link_name = "vDSP_zaspecD"]
    pub fn _zaspec_f64(__A: *const SplitComplex<f64>, __C: *mut f64, __N: Len);

    /// Converts a vector of signed 16-bit integers to single-precision floating-point values.
    #[doc(alias = "vDSP_vflt16")]
    #[link_name = "vDSP_vflt16"]
    pub fn _i16_f32(__A: *const i16, __IA: Stride, __C: *mut f32, __CI: Stride, __N: Len);

    /// Converts an array of unsigned 16-bit integers to single-precision floating-point values.
    #[doc(alias = "vDSP_vfltu16")]
    #[link_name = "vDSP_vfltu16"]
    pub fn _u16_f32(__A: *const u16, __IA: Stride, __C: *mut f32, __CI: Stride, __N: Len);

    #[doc(alias = "vDSP_vdbcon")]
    #[link_name = "vDSP_vdbcon"]
    pub fn _dbcon_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __C: *mut f32,
        __CI: Stride,
        __N: Len,
        __F: u32,
    );

    #[doc(alias = "vDSP_vdbconD")]
    #[link_name = "vDSP_vdbconD"]
    pub fn _dbcon_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __C: *mut f64,
        __CI: Stride,
        __N: Len,
        __F: u32,
    );

    #[doc(alias = "vDSP_vclip")]
    #[link_name = "vDSP_vclip"]
    pub fn _clip_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __C: *const f32,
        __D: *mut f32,
        __ID: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vclipD")]
    #[link_name = "vDSP_vclipD"]
    pub fn _clip_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __C: *const f64,
        __D: *mut f64,
        __ID: Stride,
        __N: Len,
    );
}

#[cfg(test)]
mod tests {
    use std::f32::consts::TAU;

    use crate::vdsp::{self, SplitComplex};

    #[test]
    fn add_sub() {
        const N: usize = 1_000;

        let a = vec![1f32; N];
        let b = vec![2f32; N];
        let mut c = vec![0f32; N];
        let mut r = vec![3f32; N];

        vdsp::add_f32(&a, &b, &mut c);
        assert_eq!(c, r);

        vdsp::sub_f32(&c, &b, &mut r);
        assert_eq!(r, a);
    }

    #[test]
    fn basics() {
        const N: usize = 1_000;

        let a = vec![1f32; N];
        let b = vec![2f32; N];
        let mut c = vec![0f32; N];
        let mut d = vec![0f32; N];

        vdsp::mul_f32(&a, &b, &mut c);
        assert_eq!(b, c);

        vdsp::div_f32(&c, &b, &mut d);
        assert_eq!(d, a);

        vdsp::smul_f32(&a, &-0.5, &mut c);
        assert_eq!(vec![-0.5f32; N], c);

        vdsp::ssq_f32(&c, &mut d);
        assert_eq!(vec![-0.25f32; N], d);

        vdsp::sq_f32(&c, &mut d);
        assert_eq!(vec![0.25f32; N], d);

        let dist_sq = vdsp::distance_sq_f32(&a, &b);
        assert_eq!(1000.0, dist_sq);
        let dot_pr = vdsp::dotpr_f32(&a, &b);
        assert_eq!(2000.0, dot_pr);

        vdsp::am_f32(&a, &b, &c, &mut d);
        assert_eq!(vec![-1.5f32; N], d);

        vdsp::abs_f32(&d, &mut c);
        assert_eq!(vec![1.5f32; N], c);

        vdsp::fill_f32(&1.0, &mut c);
        assert_eq!(a, c);

        vdsp::clr_f32(&mut c);
        assert_eq!(0.0, vdsp::se_f32(&c));
        assert_eq!(0.0, vdsp::semg_f32(&c));
        assert_eq!(0.0, vdsp::sesq_f32(&c));
        assert_eq!(4.0 * N as f32, vdsp::sesq_f32(&b));

        vdsp::sadd_f32(&c, &2.0, &mut d);
        assert_eq!(b, d);

        vdsp::gen_f32(&0.0, &1.0, &mut c[..3]);
        assert_eq!(&[0.0, 0.5, 1.0], &c[..3]);
    }

    #[test]
    fn stereo() {
        let mut l = vec![10.0f32, 100.0, 10.0, 100.0, 10.0, 100.0, 10.0, 100.0];
        let mut r = vec![10.0f32, 100.0, 10.0, 100.0, 10.0, 100.0, 10.0, 100.0];

        vdsp::neg_io_f32(&mut r);

        let mut start = 0.0f32;
        let step = 1.0f32;
        vdsp::rampmul2_io_f32(&mut l, &mut r, &mut start, &step);

        assert_eq!(start, step * l.len() as f32);

        assert_eq!(vdsp::maxmg_f32(&l), 700.0);
        assert_eq!(vdsp::maxmg_f32(&r), 700.0);

        assert_eq!(vdsp::minmg_f32(&l), 0.0);
        assert_eq!(vdsp::minmg_f32(&r), 0.0);

        assert_eq!(vdsp::min_f32(&l), 0.0);
        assert_eq!(vdsp::min_f32(&r), -700.0);
    }

    fn synth_signal(freq_amp_pairs: &[(f32, f32)], len: usize) -> Vec<f32> {
        let mut res = vec![0.0f32; len];

        for i in 0..len {
            let n_index = (i as f32) / ((len - 1) as f32);
            res[i] = freq_amp_pairs.iter().fold(0.0f32, |acc, pair| {
                println!("{pair:?}");
                let r = acc + (n_index + pair.0 * TAU).sin() * pair.1;
                println!("{r:?}");
                r
            });
        }
        res
    }

    // https://developer.apple.com/library/archive/documentation/Performance/Conceptual/vDSP_Programming_Guide/UsingFourierTransforms/UsingFourierTransforms.html
    #[test]
    fn fft() {
        // let n = 2048usize;
        let n = 32usize;
        // let signal = synth_signal(&[(2.0, 0.8), (7.0, 1.2), (24.0, 0.7), (50.0, 1.0)], n);
        let signal = synth_signal(&[(2.0, 1.0)], n);
        println!("{signal:?}");
        let log2n: usize = n.ilog2() as _;
        let mut ffi = vdsp::Fft::new_f32(log2n, vdsp::FftRadix::_5).unwrap();

        let half_n: usize = n / 2;

        let mut i_re = vec![0.0f32; half_n];
        let mut i_im = vec![0.0f32; half_n];

        vdsp::actoz_f32(&signal, &mut i_re, &mut i_im);
        // println!("{i_im:?}");

        println!("{signal:?}");
        ffi.zr_io(&mut i_re, &mut i_im, vdsp::FftDirection::Forward);
        println!("{i_re:?}");
        println!("{i_im:?}");

        let mut spec = vec![0.0f32; half_n];

        let split = SplitComplex::new(&i_re, &i_im);

        vdsp::zaspec_f32(&split, &mut spec);
    }

    #[test]
    fn ztoc() {
        let re = [1.0, 2.0, 3.0];
        let im = [4.0, 5.0, 6.0];
        let mut c = vec![vdsp::Complex::<f32>::default(); 3];
        vdsp::ztoc_f32(&re, &im, &mut c);
        println!("{c:?}");

        assert_eq!(c[0].re, 1.0);
        assert_eq!(c[0].im, 4.0);

        assert_eq!(c[1].re, 2.0);
        assert_eq!(c[1].im, 5.0);

        assert_eq!(c[2].re, 3.0);
        assert_eq!(c[2].im, 6.0);
    }
}
