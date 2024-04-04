#[doc(alias = "vDSP_Length")]
pub type Len = usize;

#[doc(alias = "vDSP_Stride")]
pub type Stride = isize;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Complex<T> {
    pub real: T,
    pub imag: T,
}

#[doc(alias = "DSPComplex")]
pub type ComplexF32 = Complex<f32>;

#[doc(alias = "DSPDoubleComplex")]
pub type ComplexF64 = Complex<f64>;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SplitComplex<T> {
    pub real: *mut T,
    pub imag: *mut T,
}

#[doc(alias = "DSPSplitComplex")]
pub type SplitComplexF32 = SplitComplex<f32>;

#[doc(alias = "DSPDoubleSplitComplex")]
pub type SplitComplexF64 = SplitComplex<f64>;

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

/// Euclidean distance, squared
#[doc(alias = "vDSP_distancesq")]
pub fn distance_sq_f32(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len();
    assert_eq!(n, b.len());
    let mut res = 0.0;
    unsafe { _distance_sq_f32(a.as_ptr(), 1, b.as_ptr(), 1, &mut res, n) };
    res
}

/// Euclidean distance, squared
#[doc(alias = "vDSP_distancesqD")]
pub fn distance_sq_f64(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len();
    assert_eq!(n, b.len());
    let mut res = 0.0;
    unsafe { _distance_sq_f64(a.as_ptr(), 1, b.as_ptr(), 1, &mut res, n) };
    res
}

/// Dot product
#[doc(alias = "vDSP_dotpr")]
#[inline]
pub fn dotpr_f32(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len();
    assert_eq!(n, b.len());
    let mut res = 0.0;
    unsafe { _dotpr_f32(a.as_ptr(), 1, b.as_ptr(), 1, &mut res, n) };
    res
}

/// Dot product
#[doc(alias = "vDSP_dotprD")]
#[inline]
pub fn dotpr_f64(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len();
    assert_eq!(n, b.len());
    let mut res = 0.0;
    unsafe { _dotpr_f64(a.as_ptr(), 1, b.as_ptr(), 1, &mut res, n) };
    res
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
    let mut res = 0.0f32;
    unsafe { _se_f32(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Sum of vector elements
#[doc(alias = "vDSP_sveD")]
#[inline]
pub fn se_f64(a: &[f64]) -> f64 {
    let mut res = 0.0f64;
    unsafe { _se_f64(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Sum of vector elements magnitudes
#[doc(alias = "vDSP_svemg")]
#[inline]
pub fn semg_f32(a: &[f32]) -> f32 {
    let mut res = 0.0f32;
    unsafe { _semg_f32(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Sum of vector elements magnitudes
#[doc(alias = "vDSP_svemgD")]
#[inline]
pub fn semg_f64(a: &[f64]) -> f64 {
    let mut res = 0.0f64;
    unsafe { _semg_f64(a.as_ptr(), 1, &mut res, a.len()) };
    res
}
/// Sum of vector elements' squares
#[doc(alias = "vDSP_svesq")]
#[inline]
pub fn sesq_f32(a: &[f32]) -> f32 {
    let mut res = 0.0f32;
    unsafe { _sesq_f32(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Sum of vector elements' squares
#[doc(alias = "vDSP_svesqD")]
#[inline]
pub fn sesq_f64(a: &[f64]) -> f64 {
    let mut res = 0.0f64;
    unsafe { _sesq_f64(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Sum of vector elements' signed squares
#[doc(alias = "vDSP_svs")]
#[inline]
pub fn svs_f32(a: &[f32]) -> f32 {
    let mut res = 0.0f32;
    unsafe { _svs_f32(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Sum of vector elements' signed squares
#[doc(alias = "vDSP_svsD")]
#[inline]
pub fn svs_f64(a: &[f64]) -> f64 {
    let mut res = 0.0f64;
    unsafe { _svs_f64(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Maximum magnitude of vector
#[doc(alias = "vDSP_maxmgv")]
#[inline]
pub fn maxmg_f32(a: &[f32]) -> f32 {
    let mut res = 0.0f32;
    unsafe { _maxmg_f32(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Maximum magnitude of vector
#[doc(alias = "vDSP_maxmgvD")]
#[inline]
pub fn maxmg_f64(a: &[f64]) -> f64 {
    let mut res = 0.0f64;
    unsafe { _maxmg_f64(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Minimum magnitude of vector
#[doc(alias = "vDSP_minmgv")]
#[inline]
pub fn minmg_f32(a: &[f32]) -> f32 {
    let mut res = 0.0f32;
    unsafe { _minmg_f32(a.as_ptr(), 1, &mut res, a.len()) };
    res
}

/// Minimum magnitude of vector
#[doc(alias = "vDSP_minmgvD")]
#[inline]
pub fn minmg_f64(a: &[f64]) -> f64 {
    let mut res = 0.0f64;
    unsafe { _minmg_f64(a.as_ptr(), 1, &mut res, a.len()) };
    res
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

#[link(name = "Accelerate", kind = "framework")]
extern "C" {
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

    #[doc(alias = "vDSP_distancesq")]
    #[link_name = "vDSP_distancesq"]
    pub fn _distance_sq_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: &mut f32,
        __N: Len,
    );

    #[doc(alias = "vDSP_distancesqD")]
    #[link_name = "vDSP_distancesqD"]
    pub fn _distance_sq_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: &mut f64,
        __N: Len,
    );

    #[doc(alias = "vDSP_dotpr")]
    #[link_name = "vDSP_dotpr"]
    pub fn _dotpr_f32(
        __A: *const f32,
        __IA: Stride,
        __B: *const f32,
        __IB: Stride,
        __C: &mut f32,
        __N: Len,
    );

    #[doc(alias = "vDSP_dotprD")]
    #[link_name = "vDSP_dotprD"]
    pub fn _dotpr_f64(
        __A: *const f64,
        __IA: Stride,
        __B: *const f64,
        __IB: Stride,
        __C: &mut f64,
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
    pub fn _se_f32(__A: *const f32, __IA: Stride, __C: &mut f32, __N: Len);

    /// Sum of vector elements
    #[doc(alias = "vDSP_sveD")]
    #[link_name = "vDSP_sveD"]
    pub fn _se_f64(__A: *const f64, __IA: Stride, __C: &mut f64, __N: Len);

    /// Sum of vector elements magnitudes
    #[doc(alias = "vDSP_svemg")]
    #[link_name = "vDSP_svemg"]
    pub fn _semg_f32(__A: *const f32, __IA: Stride, __C: &mut f32, __N: Len);

    /// Sum of vector elements magnitudes
    #[doc(alias = "vDSP_svemgD")]
    #[link_name = "vDSP_svemgD"]
    pub fn _semg_f64(__A: *const f64, __IA: Stride, __C: &mut f64, __N: Len);

    /// Sum of vector elements' squares
    #[doc(alias = "vDSP_svesq")]
    #[link_name = "vDSP_svesq"]
    pub fn _sesq_f32(__A: *const f32, __IA: Stride, __C: &mut f32, __N: Len);

    /// Sum of vector elements' squares
    #[doc(alias = "vDSP_svesqD")]
    #[link_name = "vDSP_svesqD"]
    pub fn _sesq_f64(__A: *const f64, __IA: Stride, __C: &mut f64, __N: Len);

    /// Sum of vector elements' signed squares
    #[doc(alias = "vDSP_svs")]
    #[link_name = "vDSP_svs"]
    pub fn _svs_f32(__A: *const f32, __IA: Stride, __C: &mut f32, __N: Len);

    /// Sum of vector elements' signed squares
    #[doc(alias = "vDSP_svsD")]
    #[link_name = "vDSP_svsD"]
    pub fn _svs_f64(__A: *const f64, __IA: Stride, __C: &mut f64, __N: Len);

    /// Maximum magnitude of vector
    #[doc(alias = "vDSP_maxmgv")]
    #[link_name = "vDSP_maxmgv"]
    pub fn _maxmg_f32(__A: *const f32, __IA: Stride, __C: &mut f32, __N: Len);

    /// Maximum magnitude of vector
    #[doc(alias = "vDSP_maxmgvD")]
    #[link_name = "vDSP_maxmgvD"]
    pub fn _maxmg_f64(__A: *const f64, __IA: Stride, __C: &mut f64, __N: Len);

    /// Minimum magnitude of vector
    #[doc(alias = "vDSP_minmgv")]
    #[link_name = "vDSP_minmgv"]
    pub fn _minmg_f32(__A: *const f32, __IA: Stride, __C: &mut f32, __N: Len);

    /// Mimimum magnitude of vector
    #[doc(alias = "vDSP_minmgvD")]
    #[link_name = "vDSP_minmgvD"]
    pub fn _minmg_f64(__A: *const f64, __IA: Stride, __C: &mut f64, __N: Len);

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
        __IS: Stride,
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
        __IS: Stride,
        __N: Len,
    );

    #[doc(alias = "vDSP_vneg")]
    #[link_name = "vDSP_vneg"]
    pub fn _neg_f32(__A: *const f32, __IA: Stride, __C: *mut f32, __IC: Stride, __N: Len);

    #[doc(alias = "vDSP_vnegD")]
    #[link_name = "vDSP_vnegD"]
    pub fn _neg_f64(__A: *const f64, __IA: Stride, __C: *mut f64, __IC: Stride, __N: Len);
}

#[cfg(test)]
mod tests {
    use crate::vdsp;

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
    }
}
