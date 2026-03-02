pub mod vector_types;
pub use vector_types::Simd;

#[allow(non_camel_case_types)]
pub type i8x2 = Simd<i8, 2, 2>;
#[allow(non_camel_case_types)]
pub type i8x3 = Simd<i8, 4, 3>;
#[allow(non_camel_case_types)]
pub type i8x4 = Simd<i8, 4, 4>;

#[allow(non_camel_case_types)]
pub type u8x2 = Simd<u8, 2, 2>;
#[allow(non_camel_case_types)]
pub type u8x3 = Simd<u8, 4, 3>;
#[allow(non_camel_case_types)]
pub type u8x4 = Simd<u8, 4, 4>;

#[allow(non_camel_case_types)]
pub type i16x2 = Simd<i16, 2, 2>;
#[allow(non_camel_case_types)]
pub type i16x3 = Simd<i16, 4, 3>;
#[allow(non_camel_case_types)]
pub type i16x4 = Simd<i16, 4, 4>;

#[allow(non_camel_case_types)]
pub type u16x2 = Simd<u16, 2, 2>;
#[allow(non_camel_case_types)]
pub type u16x3 = Simd<u16, 4, 3>;
#[allow(non_camel_case_types)]
pub type u16x4 = Simd<u16, 4, 4>;

#[cfg(not(target_arch = "aarch64"))]
#[allow(non_camel_case_types)]
pub type f32x2 = Simd<f32, 2, 2>;

#[cfg(target_arch = "aarch64")]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct f32x2(pub std::arch::aarch64::float32x2_t);

#[cfg(target_arch = "aarch64")]
impl PartialEq for f32x2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let cmp = std::arch::aarch64::vceq_f32(self.0, other.0);
            let bits: u64 = std::mem::transmute(cmp);
            bits == u64::MAX
        }
    }
}

#[cfg(target_arch = "aarch64")]
impl PartialEq<[f32; 2]> for f32x2 {
    fn eq(&self, other: &[f32; 2]) -> bool {
        self == &Self::load(other)
    }
}

#[cfg(target_arch = "aarch64")]
impl Default for f32x2 {
    fn default() -> Self {
        Self::load(&[0.0; 2])
    }
}

#[cfg(target_arch = "aarch64")]
impl f32x2 {
    pub fn x(&self) -> f32 {
        unsafe { std::arch::aarch64::vget_lane_f32::<0>(self.0) }
    }
    pub fn y(&self) -> f32 {
        unsafe { std::arch::aarch64::vget_lane_f32::<1>(self.0) }
    }

    pub fn r(&self) -> f32 {
        unsafe { std::arch::aarch64::vget_lane_f32::<0>(self.0) }
    }
    pub fn g(&self) -> f32 {
        unsafe { std::arch::aarch64::vget_lane_f32::<1>(self.0) }
    }

    pub fn set_x(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vset_lane_f32::<0>(val, self.0) }
    }
    pub fn set_y(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vset_lane_f32::<1>(val, self.0) }
    }

    pub fn set_r(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vset_lane_f32::<0>(val, self.0) }
    }
    pub fn set_g(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vset_lane_f32::<1>(val, self.0) }
    }

    #[inline]
    pub fn with_xy(x: f32, y: f32) -> Self {
        Self::load(&[x, y])
    }

    pub fn load(vals: &[f32; 2]) -> Self {
        Self(unsafe { std::arch::aarch64::vld1_f32(vals.as_ptr()) })
    }

    pub fn splat(val: f32) -> Self {
        Self(unsafe { std::arch::aarch64::vdup_n_f32(val) })
    }

    pub fn to_bits(&self) -> u64 {
        unsafe { std::mem::transmute(*self) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Add for f32x2 {
    type Output = f32x2;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vadd_f32(self.0, rhs.0)) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Mul for f32x2 {
    type Output = f32x2;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vmul_f32(self.0, rhs.0)) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Sub for f32x2 {
    type Output = f32x2;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vsub_f32(self.0, rhs.0)) }
    }
}

#[cfg(feature = "half")]
#[allow(non_camel_case_types)]
pub type f16x2 = Simd<half::f16, 2, 2>;

#[cfg(not(target_arch = "aarch64"))]
#[allow(non_camel_case_types)]
pub type f32x3 = Simd<f32, 4, 3>;

#[cfg(target_arch = "aarch64")]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct f32x3(pub std::arch::aarch64::float32x4_t);

#[cfg(target_arch = "aarch64")]
impl PartialEq for f32x3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let cmp = std::arch::aarch64::vceqq_f32(self.0, other.0);
            let bits: u128 = std::mem::transmute(cmp);
            bits == u128::MAX
        }
    }
}

#[cfg(target_arch = "aarch64")]
impl PartialEq<[f32; 3]> for f32x3 {
    fn eq(&self, other: &[f32; 3]) -> bool {
        self == &Self::load(other)
    }
}

#[cfg(target_arch = "aarch64")]
impl Default for f32x3 {
    fn default() -> Self {
        Self::load(&[0.0; 3])
    }
}

#[cfg(target_arch = "aarch64")]
impl f32x3 {
    pub fn x(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<0>(self.0) }
    }
    pub fn y(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<1>(self.0) }
    }
    pub fn z(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<2>(self.0) }
    }

    pub fn r(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<0>(self.0) }
    }
    pub fn g(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<1>(self.0) }
    }
    pub fn b(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<2>(self.0) }
    }

    pub fn set_x(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<0>(val, self.0) }
    }
    pub fn set_y(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<1>(val, self.0) }
    }
    pub fn set_z(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<2>(val, self.0) }
    }

    pub fn set_r(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<0>(val, self.0) }
    }
    pub fn set_g(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<1>(val, self.0) }
    }
    pub fn set_b(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<2>(val, self.0) }
    }

    #[inline]
    pub fn with_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::load(&[x, y, z])
    }

    #[inline]
    pub fn with_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::load(&[r, g, b])
    }

    #[inline]
    pub fn with_xyz_f32(x: f32, y: f32, z: f32) -> Self {
        Self::load(&[x, y, z])
    }

    #[inline]
    pub fn with_rgb_f32(r: f32, g: f32, b: f32) -> Self {
        Self::load(&[r, g, b])
    }

    #[inline]
    pub fn load(vals: &[f32; 3]) -> Self {
        let vals = [vals[0], vals[1], vals[2], 0.0];
        Self(unsafe { std::arch::aarch64::vld1q_f32(vals.as_ptr()) })
    }

    #[inline]
    pub fn splat(val: f32) -> Self {
        Self::load(&[val; 3])
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        unsafe {
            let mul = std::arch::aarch64::vmulq_f32(self.0, other.0);
            std::arch::aarch64::vgetq_lane_f32::<0>(mul)
                + std::arch::aarch64::vgetq_lane_f32::<1>(mul)
                + std::arch::aarch64::vgetq_lane_f32::<2>(mul)
        }
    }

    #[inline]
    pub fn fmla(&self, n: &Self, m: &Self) -> f32x3 {
        unsafe {
            let out = std::arch::aarch64::vfmaq_f32(self.0, n.0, m.0);
            f32x3(std::arch::aarch64::vsetq_lane_f32::<3>(0.0, out))
        }
    }

    pub fn to_bits(&self) -> u128 {
        unsafe { std::mem::transmute(*self) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Add for f32x3 {
    type Output = f32x3;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vaddq_f32(self.0, rhs.0)) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Mul for f32x3 {
    type Output = f32x3;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vmulq_f32(self.0, rhs.0)) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Sub for f32x3 {
    type Output = f32x3;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vsubq_f32(self.0, rhs.0)) }
    }
}

#[cfg(feature = "half")]
#[allow(non_camel_case_types)]
pub type f16x3 = Simd<half::f16, 4, 3>;

#[cfg(not(target_arch = "aarch64"))]
#[allow(non_camel_case_types)]
pub type f32x4 = Simd<f32, 4, 4>;

#[cfg(target_arch = "aarch64")]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct f32x4(pub std::arch::aarch64::float32x4_t);

#[cfg(target_arch = "aarch64")]
impl PartialEq for f32x4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let cmp = std::arch::aarch64::vceqq_f32(self.0, other.0);
            let bits: u128 = std::mem::transmute(cmp);
            bits == u128::MAX
        }
    }
}

#[cfg(target_arch = "aarch64")]
impl PartialEq<[f32; 4]> for f32x4 {
    fn eq(&self, other: &[f32; 4]) -> bool {
        self == &Self::load(other)
    }
}

#[cfg(target_arch = "aarch64")]
impl Default for f32x4 {
    fn default() -> Self {
        Self::load(&[0.0; 4])
    }
}

#[cfg(target_arch = "aarch64")]
impl f32x4 {
    #[inline]
    pub fn xyz(&self) -> f32x3 {
        f32x3(unsafe { std::arch::aarch64::vsetq_lane_f32::<3>(0.0, self.0) })
    }

    pub fn x(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<0>(self.0) }
    }
    pub fn y(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<1>(self.0) }
    }
    pub fn z(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<2>(self.0) }
    }
    pub fn w(&self) -> f32 {
        unsafe { std::arch::aarch64::vgetq_lane_f32::<3>(self.0) }
    }

    pub fn set_x(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<0>(val, self.0) }
    }
    pub fn set_y(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<1>(val, self.0) }
    }
    pub fn set_z(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<2>(val, self.0) }
    }
    pub fn set_w(&mut self, val: f32) {
        self.0 = unsafe { std::arch::aarch64::vsetq_lane_f32::<3>(val, self.0) }
    }

    #[inline]
    pub fn with_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::load(&[x, y, z, w])
    }

    #[inline]
    pub fn with_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::load(&[r, g, b, a])
    }

    #[inline]
    pub fn load(vals: &[f32; 4]) -> Self {
        Self(unsafe { std::arch::aarch64::vld1q_f32(vals.as_ptr()) })
    }

    #[inline]
    pub fn splat(val: f32) -> Self {
        Self(unsafe { std::arch::aarch64::vdupq_n_f32(val) })
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        unsafe {
            let mul = std::arch::aarch64::vmulq_f32(self.0, other.0);
            std::arch::aarch64::vaddvq_f32(mul)
        }
    }

    #[inline]
    pub fn fmla(&self, n: &Self, m: &Self) -> f32x4 {
        unsafe { f32x4(std::arch::aarch64::vfmaq_f32(self.0, n.0, m.0)) }
    }

    #[inline]
    pub fn mul_f32(&self, val: f32) -> f32x4 {
        unsafe { f32x4(std::arch::aarch64::vmulq_n_f32(self.0, val)) }
    }

    #[inline]
    pub fn div_f32(&self, val: f32) -> f32x4 {
        unsafe {
            f32x4(std::arch::aarch64::vdivq_f32(
                self.0,
                std::arch::aarch64::vdupq_n_f32(val),
            ))
        }
    }

    pub fn to_bits(&self) -> u128 {
        unsafe { std::mem::transmute(*self) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Add for f32x4 {
    type Output = f32x4;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vaddq_f32(self.0, rhs.0)) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Mul for f32x4 {
    type Output = f32x4;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vmulq_f32(self.0, rhs.0)) }
    }
}

#[cfg(target_arch = "aarch64")]
impl std::ops::Sub for f32x4 {
    type Output = f32x4;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self(std::arch::aarch64::vsubq_f32(self.0, rhs.0)) }
    }
}

#[allow(non_camel_case_types)]
pub type f64x4 = Simd<f64, 4, 4>;

#[cfg(feature = "half")]
#[allow(non_camel_case_types)]
pub type f16x4 = Simd<half::f16, 4, 4>;

#[cfg(feature = "half")]
#[allow(non_camel_case_types)]
pub struct f16x2x2(pub [f16x2; 2]);

#[cfg(feature = "half")]
#[allow(non_camel_case_types)]
pub struct f16x3x2(pub [f16x2; 3]);

#[cfg(feature = "half")]
#[allow(non_camel_case_types)]
pub struct f16x4x2(pub [f16x2; 4]);

#[cfg(feature = "half")]
#[allow(non_camel_case_types)]
pub struct f16x2x3(pub [f16x3; 2]);

#[cfg(feature = "half")]
#[allow(non_camel_case_types)]
pub struct f16x3x3(pub [f16x3; 3]);

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x2x2(pub [f32x2; 2]);

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x3x2(pub [f32x2; 3]);

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x4x2(pub [f32x2; 4]);

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x2x3(pub [f32x3; 2]);

#[cfg(not(target_arch = "aarch64"))]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x3x3(pub [f32x3; 3]);

#[cfg(target_arch = "aarch64")]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x3x3(pub std::arch::aarch64::float32x4x3_t);

#[cfg(not(target_arch = "aarch64"))]
impl f32x3x3 {
    pub fn diagonal(v: f32x3) -> Self {
        Self([
            f32x3::with_xyz_f32(v.x(), 0.0, 0.0),
            f32x3::with_xyz_f32(0.0, v.y(), 0.0),
            f32x3::with_xyz_f32(0.0, 0.0, v.z()),
        ])
    }

    pub const fn identity() -> Self {
        Self([
            f32x3::with_xyz_f32(1.0, 0.0, 0.0),
            f32x3::with_xyz_f32(0.0, 1.0, 0.0),
            f32x3::with_xyz_f32(0.0, 0.0, 1.0),
        ])
    }

    pub const fn translate(tx: f32, ty: f32) -> Self {
        Self([
            f32x3::with_xyz_f32(1.0, 0.0, tx),
            f32x3::with_xyz_f32(0.0, 1.0, ty),
            f32x3::with_xyz_f32(0.0, 0.0, 1.0),
        ])
    }
}

#[cfg(target_arch = "aarch64")]
impl f32x3x3 {
    pub fn diagonal(v: f32x3) -> Self {
        Self(std::arch::aarch64::float32x4x3_t(
            f32x3::with_xyz_f32(v.x(), 0.0, 0.0).0,
            f32x3::with_xyz_f32(0.0, v.y(), 0.0).0,
            f32x3::with_xyz_f32(0.0, 0.0, v.z()).0,
        ))
    }

    pub fn identity() -> Self {
        Self(std::arch::aarch64::float32x4x3_t(
            f32x3::with_xyz_f32(1.0, 0.0, 0.0).0,
            f32x3::with_xyz_f32(0.0, 1.0, 0.0).0,
            f32x3::with_xyz_f32(0.0, 0.0, 1.0).0,
        ))
    }

    pub fn translate(tx: f32, ty: f32) -> Self {
        Self(std::arch::aarch64::float32x4x3_t(
            f32x3::with_xyz_f32(1.0, 0.0, tx).0,
            f32x3::with_xyz_f32(0.0, 1.0, ty).0,
            f32x3::with_xyz_f32(0.0, 0.0, 1.0).0,
        ))
    }
}

impl std::ops::Index<usize> for f32x3x3 {
    type Output = f32x3;

    #[cfg(not(target_arch = "aarch64"))]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }

    #[cfg(target_arch = "aarch64")]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => unsafe { std::mem::transmute(&self.0.0) },
            1 => unsafe { std::mem::transmute(&self.0.1) },
            2 => unsafe { std::mem::transmute(&self.0.2) },
            _ => panic!(),
        }
    }
}

impl std::ops::IndexMut<usize> for f32x3x3 {
    #[cfg(not(target_arch = "aarch64"))]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }

    #[cfg(target_arch = "aarch64")]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => unsafe { std::mem::transmute(&mut self.0.0) },
            1 => unsafe { std::mem::transmute(&mut self.0.1) },
            2 => unsafe { std::mem::transmute(&mut self.0.2) },
            _ => panic!(),
        }
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x4x4(pub [f32x4; 4]);

#[cfg(target_arch = "aarch64")]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x4x4(pub std::arch::aarch64::float32x4x4_t);

#[cfg(target_arch = "aarch64")]
impl PartialEq for f32x4x4 {
    fn eq(&self, other: &Self) -> bool {
        // self[0] == other[0] && self[1] == other[1] && self[2] == other[2] && self[3] == other[3]
        unsafe {
            let cmp0 = std::arch::aarch64::vceqq_f32(self.0.0, other.0.0);
            let cmp1 = std::arch::aarch64::vceqq_f32(self.0.1, other.0.1);
            let cmp2 = std::arch::aarch64::vceqq_f32(self.0.2, other.0.2);
            let cmp3 = std::arch::aarch64::vceqq_f32(self.0.3, other.0.3);

            let cmp0: u128 = std::mem::transmute(cmp0);
            let cmp1: u128 = std::mem::transmute(cmp1);
            let cmp2: u128 = std::mem::transmute(cmp2);
            let cmp3: u128 = std::mem::transmute(cmp3);

            cmp0 == u128::MAX && cmp1 == u128::MAX && cmp2 == u128::MAX && cmp3 == u128::MAX
        }
    }
}

impl f32x4x4 {
    #[cfg(not(target_arch = "aarch64"))]
    pub fn diagonal(v: f32x4) -> Self {
        Self([
            f32x4::with_xyzw(v.x(), 0.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, v.y(), 0.0, 0.0),
            f32x4::with_xyzw(0.0, 0.0, v.z(), 0.0),
            f32x4::with_xyzw(0.0, 0.0, 0.0, v.w()),
        ])
    }

    #[cfg(target_arch = "aarch64")]
    pub fn diagonal(v: f32x4) -> Self {
        Self(std::arch::aarch64::float32x4x4_t(
            f32x4::with_xyzw(v.x(), 0.0, 0.0, 0.0).0,
            f32x4::with_xyzw(0.0, v.y(), 0.0, 0.0).0,
            f32x4::with_xyzw(0.0, 0.0, v.z(), 0.0).0,
            f32x4::with_xyzw(0.0, 0.0, 0.0, v.w()).0,
        ))
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn identity() -> Self {
        Self([
            f32x4::with_xyzw(1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 1.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 0.0, 1.0, 0.0),
            f32x4::with_xyzw(0.0, 0.0, 0.0, 1.0),
        ])
    }
    #[cfg(target_arch = "aarch64")]
    pub fn identity() -> Self {
        Self(std::arch::aarch64::float32x4x4_t(
            f32x4::with_xyzw(1.0, 0.0, 0.0, 0.0).0,
            f32x4::with_xyzw(0.0, 1.0, 0.0, 0.0).0,
            f32x4::with_xyzw(0.0, 0.0, 1.0, 0.0).0,
            f32x4::with_xyzw(0.0, 0.0, 0.0, 1.0).0,
        ))
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn translate(tx: f32, ty: f32, tz: f32) -> Self {
        Self([
            f32x4::with_xyzw(1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 1.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 0.0, 1.0, 0.0),
            f32x4::with_xyzw(tx, ty, tz, 1.0),
        ])
    }

    #[cfg(target_arch = "aarch64")]
    pub fn translate(tx: f32, ty: f32, tz: f32) -> Self {
        Self(std::arch::aarch64::float32x4x4_t(
            f32x4::with_xyzw(1.0, 0.0, 0.0, 0.0).0,
            f32x4::with_xyzw(0.0, 1.0, 0.0, 0.0).0,
            f32x4::with_xyzw(0.0, 0.0, 1.0, 0.0).0,
            f32x4::with_xyzw(tx, ty, tz, 1.0).0,
        ))
    }

    #[inline]
    pub fn tx(&self) -> f32 {
        self[3].x()
    }

    #[inline]
    pub fn set_tx(&mut self, value: f32) {
        self[3].set_x(value)
    }

    #[inline]
    pub fn ty(&self) -> f32 {
        self[3].y()
    }

    #[inline]
    pub fn set_ty(&mut self, value: f32) {
        self[3].set_y(value)
    }

    #[inline]
    pub fn tz(&self) -> f32 {
        self[3].z()
    }

    #[inline]
    pub fn set_tz(&mut self, value: f32) {
        self[3].set_z(value)
    }

    #[inline]
    pub fn sx(&self) -> f32 {
        self[0].x()
    }

    #[inline]
    pub fn set_sx(&mut self, value: f32) {
        self[0].set_x(value)
    }

    #[inline]
    pub fn sy(&self) -> f32 {
        self[1].y()
    }

    #[inline]
    pub fn set_sy(&mut self, value: f32) {
        self[1].set_y(value)
    }

    #[inline]
    pub fn sz(&self) -> f32 {
        self[2].z()
    }

    #[inline]
    pub fn set_sz(&mut self, value: f32) {
        self[2].set_z(value)
    }
}

impl std::ops::Index<usize> for f32x4x4 {
    type Output = f32x4;

    #[cfg(not(target_arch = "aarch64"))]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }

    #[cfg(target_arch = "aarch64")]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => unsafe { std::mem::transmute(&self.0.0) },
            1 => unsafe { std::mem::transmute(&self.0.1) },
            2 => unsafe { std::mem::transmute(&self.0.2) },
            3 => unsafe { std::mem::transmute(&self.0.3) },
            _ => panic!(),
        }
    }
}

impl std::ops::IndexMut<usize> for f32x4x4 {
    #[cfg(not(target_arch = "aarch64"))]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }

    #[cfg(target_arch = "aarch64")]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => unsafe { std::mem::transmute(&mut self.0.0) },
            1 => unsafe { std::mem::transmute(&mut self.0.1) },
            2 => unsafe { std::mem::transmute(&mut self.0.2) },
            3 => unsafe { std::mem::transmute(&mut self.0.3) },
            _ => panic!(),
        }
    }
}

pub trait SimdMul<Rhs = Self> {
    type Output;

    fn simd_mul(self, rhs: Rhs) -> Self::Output;
}

#[inline]
pub fn mul<L, R>(lhs: L, rhs: R) -> <L as SimdMul<R>>::Output
where
    L: SimdMul<R>,
{
    lhs.simd_mul(rhs)
}

pub trait SimdMix<Rhs = Self, T = Self> {
    type Output;

    fn simd_mix(self, rhs: Rhs, t: T) -> Self::Output;
}

#[inline]
pub fn mix<L, R, T>(lhs: L, rhs: R, t: T) -> <L as SimdMix<R, T>>::Output
where
    L: SimdMix<R, T>,
{
    lhs.simd_mix(rhs, t)
}

pub trait SimdInverse {
    type Output;

    fn simd_inverse(self) -> Self::Output;
}

#[inline]
pub fn inverse<T>(val: T) -> <T as SimdInverse>::Output
where
    T: SimdInverse,
{
    val.simd_inverse()
}

pub trait SimdLenSq {
    type Output;

    fn simd_len_sq(self) -> Self::Output;
}

#[inline]
pub fn len_sq<T>(val: T) -> <T as SimdLenSq>::Output
where
    T: SimdLenSq,
{
    val.simd_len_sq()
}

pub trait SimdNormalized {
    type Output;

    fn simd_normalized(self) -> Self::Output;
}

#[inline]
pub fn normalized<T>(val: T) -> <T as SimdNormalized>::Output
where
    T: SimdNormalized,
{
    val.simd_normalized()
}

#[inline]
#[cfg(target_arch = "aarch64")]
fn f32x4_dot_cols(c0: f32x4, c1: f32x4, c2: f32x4, c3: f32x4, v: f32x4) -> f32x4 {
    let out = unsafe {
        let acc = std::arch::aarch64::vmulq_laneq_f32::<0>(c0.0, v.0);
        let acc = std::arch::aarch64::vmlaq_laneq_f32::<1>(acc, c1.0, v.0);
        let acc = std::arch::aarch64::vmlaq_laneq_f32::<2>(acc, c2.0, v.0);
        std::arch::aarch64::vmlaq_laneq_f32::<3>(acc, c3.0, v.0)
    };
    f32x4(out)
}

#[inline]
#[cfg(not(target_arch = "aarch64"))]
fn f32x4_dot_cols(c0: f32x4, c1: f32x4, c2: f32x4, c3: f32x4, v: f32x4) -> f32x4 {
    let vx = v.x();
    let vy = v.y();
    let vz = v.z();
    let vw = v.w();

    f32x4::with_xyzw(
        c0.x() * vx + c1.x() * vy + c2.x() * vz + c3.x() * vw,
        c0.y() * vx + c1.y() * vy + c2.y() * vz + c3.y() * vw,
        c0.z() * vx + c1.z() * vy + c2.z() * vz + c3.z() * vw,
        c0.w() * vx + c1.w() * vy + c2.w() * vz + c3.w() * vw,
    )
}

#[inline]
#[cfg(not(target_arch = "aarch64"))]
fn f32x3x3_with_cols(c0: f32x3, c1: f32x3, c2: f32x3) -> f32x3x3 {
    f32x3x3([c0, c1, c2])
}

#[inline]
#[cfg(target_arch = "aarch64")]
fn f32x3x3_with_cols(c0: f32x3, c1: f32x3, c2: f32x3) -> f32x3x3 {
    f32x3x3(std::arch::aarch64::float32x4x3_t(c0.0, c1.0, c2.0))
}

#[inline]
#[cfg(not(target_arch = "aarch64"))]
fn f32x3_cross(a: f32x3, b: f32x3) -> f32x3 {
    f32x3::with_xyz(
        a.y() * b.z() - a.z() * b.y(),
        a.z() * b.x() - a.x() * b.z(),
        a.x() * b.y() - a.y() * b.x(),
    )
}

#[inline]
#[cfg(not(target_arch = "aarch64"))]
fn f32x4x4_with_cols(c0: f32x4, c1: f32x4, c2: f32x4, c3: f32x4) -> f32x4x4 {
    f32x4x4([c0, c1, c2, c3])
}

#[inline]
#[cfg(target_arch = "aarch64")]
fn f32x4x4_with_cols(c0: f32x4, c1: f32x4, c2: f32x4, c3: f32x4) -> f32x4x4 {
    f32x4x4(std::arch::aarch64::float32x4x4_t(c0.0, c1.0, c2.0, c3.0))
}

impl SimdMul<f32x4> for f32x4x4 {
    type Output = f32x4;

    fn simd_mul(self, rhs: f32x4) -> Self::Output {
        f32x4_dot_cols(self[0], self[1], self[2], self[3], rhs)
    }
}

impl std::ops::Mul<f32x4> for f32x4x4 {
    type Output = f32x4;

    fn mul(self, rhs: f32x4) -> Self::Output {
        mul(self, rhs)
    }
}

impl SimdMul<f32x4x4> for f32x4x4 {
    type Output = f32x4x4;

    fn simd_mul(self, rhs: f32x4x4) -> Self::Output {
        let c0 = mul(self, rhs[0]);
        let c1 = mul(self, rhs[1]);
        let c2 = mul(self, rhs[2]);
        let c3 = mul(self, rhs[3]);
        f32x4x4_with_cols(c0, c1, c2, c3)
    }
}

impl std::ops::Mul<f32x4x4> for f32x4x4 {
    type Output = f32x4x4;

    fn mul(self, rhs: f32x4x4) -> Self::Output {
        mul(self, rhs)
    }
}

impl SimdMix<f32x4, f32x4> for f32x4 {
    type Output = f32x4;

    #[cfg(target_arch = "aarch64")]
    fn simd_mix(self, rhs: f32x4, t: f32x4) -> Self::Output {
        unsafe {
            let delta = std::arch::aarch64::vsubq_f32(rhs.0, self.0);
            let scaled = std::arch::aarch64::vmulq_f32(delta, t.0);
            f32x4(std::arch::aarch64::vaddq_f32(self.0, scaled))
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn simd_mix(self, rhs: f32x4, t: f32x4) -> Self::Output {
        f32x4::with_xyzw(
            self.x() + (rhs.x() - self.x()) * t.x(),
            self.y() + (rhs.y() - self.y()) * t.y(),
            self.z() + (rhs.z() - self.z()) * t.z(),
            self.w() + (rhs.w() - self.w()) * t.w(),
        )
    }
}

impl SimdMix<f32x4, f32> for f32x4 {
    type Output = f32x4;

    #[cfg(target_arch = "aarch64")]
    fn simd_mix(self, rhs: f32x4, t: f32) -> Self::Output {
        unsafe {
            let delta = std::arch::aarch64::vsubq_f32(rhs.0, self.0);
            let scaled = std::arch::aarch64::vmulq_f32(delta, std::arch::aarch64::vdupq_n_f32(t));
            f32x4(std::arch::aarch64::vaddq_f32(self.0, scaled))
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn simd_mix(self, rhs: f32x4, t: f32) -> Self::Output {
        f32x4::with_xyzw(
            self.x() + (rhs.x() - self.x()) * t,
            self.y() + (rhs.y() - self.y()) * t,
            self.z() + (rhs.z() - self.z()) * t,
            self.w() + (rhs.w() - self.w()) * t,
        )
    }
}

impl SimdLenSq for f32x4 {
    type Output = f32;

    #[inline]
    fn simd_len_sq(self) -> Self::Output {
        self.dot(&self)
    }
}

impl SimdNormalized for f32x4 {
    type Output = f32x4;

    #[cfg(target_arch = "aarch64")]
    fn simd_normalized(self) -> Self::Output {
        let len_sq = self.dot(&self);
        if len_sq == 0.0 {
            return self;
        }

        let inv_len = len_sq.sqrt().recip();
        unsafe {
            f32x4(std::arch::aarch64::vmulq_f32(
                self.0,
                std::arch::aarch64::vdupq_n_f32(inv_len),
            ))
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn simd_normalized(self) -> Self::Output {
        let len_sq = self.dot(&self);
        if len_sq == 0.0 {
            return self;
        }

        let inv_len = len_sq.sqrt().recip();
        f32x4::with_xyzw(
            self.x() * inv_len,
            self.y() * inv_len,
            self.z() * inv_len,
            self.w() * inv_len,
        )
    }
}

impl SimdNormalized for f32x3 {
    type Output = f32x3;

    #[cfg(target_arch = "aarch64")]
    fn simd_normalized(self) -> Self::Output {
        let len_sq = self.dot(&self);
        if len_sq == 0.0 {
            return self;
        }

        let inv_len = len_sq.sqrt().recip();
        unsafe {
            let scaled =
                std::arch::aarch64::vmulq_f32(self.0, std::arch::aarch64::vdupq_n_f32(inv_len));
            f32x3(std::arch::aarch64::vsetq_lane_f32::<3>(0.0, scaled))
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn simd_normalized(self) -> Self::Output {
        let len_sq = self.dot(&self);
        if len_sq == 0.0 {
            return self;
        }

        let inv_len = len_sq.sqrt().recip();
        f32x3::with_xyz_f32(self.x() * inv_len, self.y() * inv_len, self.z() * inv_len)
    }
}

impl SimdMix<f32x3, f32x3> for f32x3 {
    type Output = f32x3;

    #[cfg(target_arch = "aarch64")]
    fn simd_mix(self, rhs: f32x3, t: f32x3) -> Self::Output {
        unsafe {
            let delta = std::arch::aarch64::vsubq_f32(rhs.0, self.0);
            let scaled = std::arch::aarch64::vmulq_f32(delta, t.0);
            f32x3(std::arch::aarch64::vaddq_f32(self.0, scaled))
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn simd_mix(self, rhs: f32x3, t: f32x3) -> Self::Output {
        f32x3::with_xyz_f32(
            self.x() + (rhs.x() - self.x()) * t.x(),
            self.y() + (rhs.y() - self.y()) * t.y(),
            self.z() + (rhs.z() - self.z()) * t.z(),
        )
    }
}

impl SimdMix<f32x3, f32> for f32x3 {
    type Output = f32x3;

    #[cfg(target_arch = "aarch64")]
    fn simd_mix(self, rhs: f32x3, t: f32) -> Self::Output {
        unsafe {
            let delta = std::arch::aarch64::vsubq_f32(rhs.0, self.0);
            let scaled = std::arch::aarch64::vmulq_f32(delta, std::arch::aarch64::vdupq_n_f32(t));
            f32x3(std::arch::aarch64::vaddq_f32(self.0, scaled))
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn simd_mix(self, rhs: f32x3, t: f32) -> Self::Output {
        f32x3::with_xyz_f32(
            self.x() + (rhs.x() - self.x()) * t,
            self.y() + (rhs.y() - self.y()) * t,
            self.z() + (rhs.z() - self.z()) * t,
        )
    }
}

impl SimdMix<f32x4x4, f32x4x4> for f32x4x4 {
    type Output = f32x4x4;

    fn simd_mix(self, rhs: f32x4x4, t: f32x4x4) -> Self::Output {
        let c0 = mix(self[0], rhs[0], t[0]);
        let c1 = mix(self[1], rhs[1], t[1]);
        let c2 = mix(self[2], rhs[2], t[2]);
        let c3 = mix(self[3], rhs[3], t[3]);
        f32x4x4_with_cols(c0, c1, c2, c3)
    }
}

impl SimdMix<f32x4x4, f32> for f32x4x4 {
    type Output = f32x4x4;

    fn simd_mix(self, rhs: f32x4x4, t: f32) -> Self::Output {
        let c0 = mix(self[0], rhs[0], t);
        let c1 = mix(self[1], rhs[1], t);
        let c2 = mix(self[2], rhs[2], t);
        let c3 = mix(self[3], rhs[3], t);
        f32x4x4_with_cols(c0, c1, c2, c3)
    }
}

impl SimdMix<f32x3x3, f32x3x3> for f32x3x3 {
    type Output = f32x3x3;

    fn simd_mix(self, rhs: f32x3x3, t: f32x3x3) -> Self::Output {
        let c0 = mix(self[0], rhs[0], t[0]);
        let c1 = mix(self[1], rhs[1], t[1]);
        let c2 = mix(self[2], rhs[2], t[2]);
        f32x3x3_with_cols(c0, c1, c2)
    }
}

impl SimdMix<f32x3x3, f32> for f32x3x3 {
    type Output = f32x3x3;

    fn simd_mix(self, rhs: f32x3x3, t: f32) -> Self::Output {
        let c0 = mix(self[0], rhs[0], t);
        let c1 = mix(self[1], rhs[1], t);
        let c2 = mix(self[2], rhs[2], t);
        f32x3x3_with_cols(c0, c1, c2)
    }
}

impl SimdInverse for f32x3x3 {
    type Output = f32x3x3;

    fn simd_inverse(self) -> Self::Output {
        #[cfg(target_arch = "aarch64")]
        {
            let q0: std::arch::aarch64::float32x4_t;
            let q1: std::arch::aarch64::float32x4_t;
            let q2: std::arch::aarch64::float32x4_t;

            unsafe {
                core::arch::asm!(
                    "bl ___invert_f3",
                    inlateout("q0") self.0.0 => q0,
                    inlateout("q1") self.0.1 => q1,
                    inlateout("q2") self.0.2 => q2,
                    clobber_abi("C"),
                );
            }

            return f32x3x3(std::arch::aarch64::float32x4x3_t(q0, q1, q2));
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
            let c0 = self[0];
            let c1 = self[1];
            let c2 = self[2];

            let cof0 = f32x3_cross(c1, c2);
            let cof1 = f32x3_cross(c2, c0);
            let cof2 = f32x3_cross(c0, c1);

            let det = c0.x() * cof0.x() + c0.y() * cof0.y() + c0.z() * cof0.z();
            let inv_det = 1.0 / det;

            f32x3x3_with_cols(
                f32x3::with_xyz(cof0.x() * inv_det, cof1.x() * inv_det, cof2.x() * inv_det),
                f32x3::with_xyz(cof0.y() * inv_det, cof1.y() * inv_det, cof2.y() * inv_det),
                f32x3::with_xyz(cof0.z() * inv_det, cof1.z() * inv_det, cof2.z() * inv_det),
            )
        }
    }
}

impl SimdInverse for f32x4x4 {
    type Output = f32x4x4;

    fn simd_inverse(self) -> Self::Output {
        #[cfg(target_arch = "aarch64")]
        {
            let q0: std::arch::aarch64::float32x4_t;
            let q1: std::arch::aarch64::float32x4_t;
            let q2: std::arch::aarch64::float32x4_t;
            let q3: std::arch::aarch64::float32x4_t;

            unsafe {
                core::arch::asm!(
                    "bl ___invert_f4",
                    inlateout("q0") self.0.0 => q0,
                    inlateout("q1") self.0.1 => q1,
                    inlateout("q2") self.0.2 => q2,
                    inlateout("q3") self.0.3 => q3,
                    clobber_abi("C"),
                );
            }

            return f32x4x4(std::arch::aarch64::float32x4x4_t(q0, q1, q2, q3));
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
            let m00 = self[0].x();
            let m01 = self[1].x();
            let m02 = self[2].x();
            let m03 = self[3].x();
            let m10 = self[0].y();
            let m11 = self[1].y();
            let m12 = self[2].y();
            let m13 = self[3].y();
            let m20 = self[0].z();
            let m21 = self[1].z();
            let m22 = self[2].z();
            let m23 = self[3].z();
            let m30 = self[0].w();
            let m31 = self[1].w();
            let m32 = self[2].w();
            let m33 = self[3].w();

            let mut inv = [0.0f32; 16];

            inv[0] = m11 * m22 * m33 - m11 * m23 * m32 - m21 * m12 * m33
                + m21 * m13 * m32
                + m31 * m12 * m23
                - m31 * m13 * m22;
            inv[4] = -m10 * m22 * m33 + m10 * m23 * m32 + m20 * m12 * m33
                - m20 * m13 * m32
                - m30 * m12 * m23
                + m30 * m13 * m22;
            inv[8] = m10 * m21 * m33 - m10 * m23 * m31 - m20 * m11 * m33
                + m20 * m13 * m31
                + m30 * m11 * m23
                - m30 * m13 * m21;
            inv[12] = -m10 * m21 * m32 + m10 * m22 * m31 + m20 * m11 * m32
                - m20 * m12 * m31
                - m30 * m11 * m22
                + m30 * m12 * m21;

            inv[1] = -m01 * m22 * m33 + m01 * m23 * m32 + m21 * m02 * m33
                - m21 * m03 * m32
                - m31 * m02 * m23
                + m31 * m03 * m22;
            inv[5] = m00 * m22 * m33 - m00 * m23 * m32 - m20 * m02 * m33
                + m20 * m03 * m32
                + m30 * m02 * m23
                - m30 * m03 * m22;
            inv[9] = -m00 * m21 * m33 + m00 * m23 * m31 + m20 * m01 * m33
                - m20 * m03 * m31
                - m30 * m01 * m23
                + m30 * m03 * m21;
            inv[13] = m00 * m21 * m32 - m00 * m22 * m31 - m20 * m01 * m32
                + m20 * m02 * m31
                + m30 * m01 * m22
                - m30 * m02 * m21;

            inv[2] = m01 * m12 * m33 - m01 * m13 * m32 - m11 * m02 * m33
                + m11 * m03 * m32
                + m31 * m02 * m13
                - m31 * m03 * m12;
            inv[6] = -m00 * m12 * m33 + m00 * m13 * m32 + m10 * m02 * m33
                - m10 * m03 * m32
                - m30 * m02 * m13
                + m30 * m03 * m12;
            inv[10] = m00 * m11 * m33 - m00 * m13 * m31 - m10 * m01 * m33
                + m10 * m03 * m31
                + m30 * m01 * m13
                - m30 * m03 * m11;
            inv[14] = -m00 * m11 * m32 + m00 * m12 * m31 + m10 * m01 * m32
                - m10 * m02 * m31
                - m30 * m01 * m12
                + m30 * m02 * m11;

            inv[3] = -m01 * m12 * m23 + m01 * m13 * m22 + m11 * m02 * m23
                - m11 * m03 * m22
                - m21 * m02 * m13
                + m21 * m03 * m12;
            inv[7] = m00 * m12 * m23 - m00 * m13 * m22 - m10 * m02 * m23
                + m10 * m03 * m22
                + m20 * m02 * m13
                - m20 * m03 * m12;
            inv[11] = -m00 * m11 * m23 + m00 * m13 * m21 + m10 * m01 * m23
                - m10 * m03 * m21
                - m20 * m01 * m13
                + m20 * m03 * m11;
            inv[15] = m00 * m11 * m22 - m00 * m12 * m21 - m10 * m01 * m22
                + m10 * m02 * m21
                + m20 * m01 * m12
                - m20 * m02 * m11;

            let det = m00 * inv[0] + m01 * inv[4] + m02 * inv[8] + m03 * inv[12];
            let inv_det = 1.0 / det;

            for v in &mut inv {
                *v *= inv_det;
            }

            f32x4x4_with_cols(
                f32x4::with_xyzw(inv[0], inv[4], inv[8], inv[12]),
                f32x4::with_xyzw(inv[1], inv[5], inv[9], inv[13]),
                f32x4::with_xyzw(inv[2], inv[6], inv[10], inv[14]),
                f32x4::with_xyzw(inv[3], inv[7], inv[11], inv[15]),
            )
        }
    }
}
#[cfg(feature = "half")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f16x4x4(pub [f16x4; 4]);

#[cfg(feature = "half")]
impl f16x4x4 {
    pub fn diagonal(v: f16x4) -> Self {
        const Z: half::f16 = half::f16::ZERO;
        Self([
            f16x4::with_xyzw_f16(v.x(), Z, Z, Z),
            f16x4::with_xyzw_f16(Z, v.y(), Z, Z),
            f16x4::with_xyzw_f16(Z, Z, v.z(), Z),
            f16x4::with_xyzw_f16(Z, Z, Z, v.w()),
        ])
    }

    pub const fn identity() -> Self {
        const Z: half::f16 = half::f16::ZERO;
        const O: half::f16 = half::f16::ONE;
        Self([
            f16x4::with_xyzw_f16(O, Z, Z, Z),
            f16x4::with_xyzw_f16(Z, O, Z, Z),
            f16x4::with_xyzw_f16(Z, Z, O, Z),
            f16x4::with_xyzw_f16(Z, Z, Z, O),
        ])
    }

    pub const fn translate(tx: half::f16, ty: half::f16, tz: half::f16) -> Self {
        const Z: half::f16 = half::f16::ZERO;
        const O: half::f16 = half::f16::ONE;
        Self([
            f16x4::with_xyzw_f16(O, Z, Z, Z),
            f16x4::with_xyzw_f16(Z, O, Z, Z),
            f16x4::with_xyzw_f16(Z, Z, O, Z),
            f16x4::with_xyzw_f16(tx, ty, tz, O),
        ])
    }

    #[inline]
    pub fn tx(&self) -> half::f16 {
        self[3].x()
    }

    #[inline]
    pub fn set_tx(&mut self, value: half::f16) {
        self[3].set_x(value)
    }

    #[inline]
    pub fn ty(&self) -> half::f16 {
        self[3].y()
    }

    #[inline]
    pub fn set_ty(&mut self, value: half::f16) {
        self[3].set_y(value)
    }

    #[inline]
    pub fn tz(&self) -> half::f16 {
        self[3].z()
    }

    #[inline]
    pub fn set_tz(&mut self, value: half::f16) {
        self[3].set_z(value)
    }

    #[inline]
    pub fn sx(&self) -> half::f16 {
        self[0].x()
    }

    #[inline]
    pub fn set_sx(&mut self, value: half::f16) {
        self[0].set_x(value)
    }

    #[inline]
    pub fn sy(&self) -> half::f16 {
        self[1].y()
    }

    #[inline]
    pub fn set_sy(&mut self, value: half::f16) {
        self[1].set_y(value)
    }

    #[inline]
    pub fn sz(&self) -> half::f16 {
        self[2].z()
    }

    #[inline]
    pub fn set_sz(&mut self, value: half::f16) {
        self[2].set_z(value)
    }
}

#[cfg(feature = "half")]
impl std::ops::Index<usize> for f16x4x4 {
    type Output = f16x4;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(feature = "half")]
impl std::ops::IndexMut<usize> for f16x4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(feature = "half")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f16quat(pub f16x4);

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32quat(pub f32x4);

impl std::ops::Deref for f32quat {
    type Target = f32x4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for f32quat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl f32quat {
    pub fn load(vals: &[f32; 4]) -> Self {
        Self(f32x4::load(vals))
    }

    pub fn from_f32x3x3(matrix: f32x3x3) -> Self {
        let m00 = matrix[0].x();
        let m01 = matrix[0].y();
        let m02 = matrix[0].z();
        let m10 = matrix[1].x();
        let m11 = matrix[1].y();
        let m12 = matrix[1].z();
        let m20 = matrix[2].x();
        let m21 = matrix[2].y();
        let m22 = matrix[2].z();

        let trace = m00 + m11 + m22;

        if trace >= 0.0 {
            let r = 2.0 * (1.0 + trace).sqrt();
            let rinv = 1.0 / r;
            return Self(f32x4::with_xyzw(
                rinv * (m12 - m21),
                rinv * (m20 - m02),
                rinv * (m01 - m10),
                r * 0.25,
            ));
        }

        if m00 >= m11 && m00 >= m22 {
            let r = 2.0 * (1.0 - m11 - m22 + m00).sqrt();
            let rinv = 1.0 / r;
            return Self(f32x4::with_xyzw(
                r * 0.25,
                rinv * (m01 + m10),
                rinv * (m02 + m20),
                rinv * (m12 - m21),
            ));
        }

        if m11 >= m22 {
            let r = 2.0 * (1.0 - m00 - m22 + m11).sqrt();
            let rinv = 1.0 / r;
            return Self(f32x4::with_xyzw(
                rinv * (m01 + m10),
                r * 0.25,
                rinv * (m12 + m21),
                rinv * (m20 - m02),
            ));
        }

        let r = 2.0 * (1.0 - m00 - m11 + m22).sqrt();
        let rinv = 1.0 / r;
        Self(f32x4::with_xyzw(
            rinv * (m02 + m20),
            rinv * (m12 + m21),
            r * 0.25,
            rinv * (m01 - m10),
        ))
    }

    pub fn from_f32x4x4(matrix: f32x4x4) -> Self {
        let m00 = matrix[0].x();
        let m01 = matrix[0].y();
        let m02 = matrix[0].z();
        let m10 = matrix[1].x();
        let m11 = matrix[1].y();
        let m12 = matrix[1].z();
        let m20 = matrix[2].x();
        let m21 = matrix[2].y();
        let m22 = matrix[2].z();

        let trace = m00 + m11 + m22;

        if trace >= 0.0 {
            let r = 2.0 * (1.0 + trace).sqrt();
            let rinv = 1.0 / r;
            return Self(f32x4::with_xyzw(
                rinv * (m12 - m21),
                rinv * (m20 - m02),
                rinv * (m01 - m10),
                r * 0.25,
            ));
        }

        if m00 >= m11 && m00 >= m22 {
            let r = 2.0 * (1.0 - m11 - m22 + m00).sqrt();
            let rinv = 1.0 / r;
            return Self(f32x4::with_xyzw(
                r * 0.25,
                rinv * (m01 + m10),
                rinv * (m02 + m20),
                rinv * (m12 - m21),
            ));
        }

        if m11 >= m22 {
            let r = 2.0 * (1.0 - m00 - m22 + m11).sqrt();
            let rinv = 1.0 / r;
            return Self(f32x4::with_xyzw(
                rinv * (m01 + m10),
                r * 0.25,
                rinv * (m12 + m21),
                rinv * (m20 - m02),
            ));
        }

        let r = 2.0 * (1.0 - m00 - m11 + m22).sqrt();
        let rinv = 1.0 / r;
        Self(f32x4::with_xyzw(
            rinv * (m02 + m20),
            rinv * (m12 + m21),
            r * 0.25,
            rinv * (m01 - m10),
        ))
    }
}

impl SimdMul<f32quat> for f32quat {
    type Output = f32quat;

    fn simd_mul(self, rhs: f32quat) -> Self::Output {
        let x1 = self.x();
        let y1 = self.y();
        let z1 = self.z();
        let w1 = self.w();

        let x2 = rhs.x();
        let y2 = rhs.y();
        let z2 = rhs.z();
        let w2 = rhs.w();

        f32quat(f32x4::with_xyzw(
            w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
            w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
            w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2,
            w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
        ))
    }
}

impl std::ops::Mul<f32quat> for f32quat {
    type Output = f32quat;

    fn mul(self, rhs: f32quat) -> Self::Output {
        mul(self, rhs)
    }
}

impl std::ops::Neg for f32quat {
    type Output = f32quat;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl SimdLenSq for f32quat {
    type Output = f32;

    #[inline]
    fn simd_len_sq(self) -> Self::Output {
        self.dot(&self)
    }
}

impl SimdNormalized for f32quat {
    type Output = f32quat;

    #[inline]
    fn simd_normalized(self) -> Self::Output {
        f32quat(normalized(self.0))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f64quat(pub f64x4);

#[cfg(feature = "half")]
impl f16quat {
    #[inline]
    pub fn with_angle(angle: half::f16, axis: f16x3) -> Self {
        let half_angle = angle.to_f32() * 0.5;
        let sin = half_angle.sin();
        let x = sin * axis.x().to_f32();
        let y = sin * axis.y().to_f32();
        let z = sin * axis.z().to_f32();
        let w = half_angle.cos();
        Self(Simd([
            half::f16::from_f32(x),
            half::f16::from_f32(y),
            half::f16::from_f32(z),
            half::f16::from_f32(w),
        ]))
    }
}

impl f32quat {
    #[inline]
    pub fn identity() -> Self {
        Self(f32x4::with_xyzw(0.0, 0.0, 0.0, 1.0))
    }

    #[inline]
    #[doc(alias = "simd_dot")]
    pub fn dot(&self, other: &Self) -> f32 {
        self.0.dot(&other.0)
    }

    #[inline]
    #[doc(alias = "simd_negate")]
    pub fn negate(self) -> Self {
        #[cfg(target_arch = "aarch64")]
        {
            Self(f32x4(unsafe { std::arch::aarch64::vnegq_f32(self.0.0) }))
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
            Self(f32x4::with_xyzw(-self.x(), -self.y(), -self.z(), -self.w()))
        }
    }

    #[inline]
    pub fn with_angle(angle: f32, axis: f32x3) -> Self {
        let half_angle = angle * 0.5;
        let sin = half_angle.sin();
        Self(f32x4::load(&[
            sin * axis.x(),
            sin * axis.y(),
            sin * axis.z(),
            half_angle.cos(),
        ]))
    }

    #[inline]
    #[doc(alias = "simd_conjugate")]
    pub fn conjugate(self) -> Self {
        #[cfg(target_arch = "aarch64")]
        {
            let sign = f32x4::with_xyzw(-1.0, -1.0, -1.0, 1.0);
            Self(self.0 * sign)
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
            Self(f32x4::with_xyzw(-self.x(), -self.y(), -self.z(), self.w()))
        }
    }
}

pub mod packed {
    use super::Simd;
    #[allow(non_camel_case_types)]
    pub type i8x2 = Simd<i8, 2, 2>;
    #[allow(non_camel_case_types)]
    pub type i8x4 = Simd<i8, 4, 4>;

    #[allow(non_camel_case_types)]
    pub type u8x2 = Simd<u8, 2, 2>;
    #[allow(non_camel_case_types)]
    pub type u8x4 = Simd<u8, 4, 4>;

    #[allow(non_camel_case_types)]
    pub type i16x2 = Simd<i16, 2, 2>;
    #[allow(non_camel_case_types)]
    pub type i16x4 = Simd<i16, 4, 4>;

    #[allow(non_camel_case_types)]
    pub type u16x2 = Simd<u16, 2, 2>;
    #[allow(non_camel_case_types)]
    pub type u16x4 = Simd<u16, 4, 4>;

    /// A vector of two 32-bit signed (twos-complement) integers with
    /// relaxed alignment.
    /// In C++ and Metal, this type is also available as
    /// simd::packed::int2. The alignment of this type is that of the underlying
    /// scalar element type, so you can use it to load or store from an array of
    /// that type.                                                                */
    #[allow(non_camel_case_types)]
    pub type i32x2 = Simd<i32, 2, 2>;

    #[allow(non_camel_case_types)]
    pub type i32x4 = Simd<i32, 4, 4>;

    #[allow(non_camel_case_types)]
    pub type u32x2 = Simd<u32, 2, 2>;

    #[allow(non_camel_case_types)]
    pub type u32x4 = Simd<u32, 4, 4>;

    #[allow(non_camel_case_types)]
    pub type i64x2 = Simd<i64, 2, 2>;

    #[allow(non_camel_case_types)]
    pub type i64x4 = Simd<i64, 4, 4>;

    #[allow(non_camel_case_types)]
    pub type u64x2 = Simd<u64, 2, 2>;

    #[allow(non_camel_case_types)]
    pub type u64x4 = Simd<u64, 4, 4>;

    #[allow(non_camel_case_types)]
    pub type f32x2 = Simd<f32, 2, 2>;

    #[allow(non_camel_case_types)]
    pub type f32x4 = Simd<f32, 4, 4>;
}

#[cfg(test)]
mod tests {
    use super::f32quat;
    use super::f32x2;
    use super::f32x2x2;
    use super::f32x3;
    use super::f32x3x3;
    use super::f32x4;
    use super::f32x4x4;
    use crate::simd;

    fn assert_f32_close(a: f32, b: f32) {
        let delta = (a - b).abs();
        assert!(delta < 1e-6, "{a} != {b} (delta={delta})");
    }

    fn assert_f32x4_close(a: f32x4, b: f32x4) {
        assert_f32_close(a.x(), b.x());
        assert_f32_close(a.y(), b.y());
        assert_f32_close(a.z(), b.z());
        assert_f32_close(a.w(), b.w());
    }

    fn assert_f32x3_close(a: f32x3, b: f32x3) {
        assert_f32_close(a.x(), b.x());
        assert_f32_close(a.y(), b.y());
        assert_f32_close(a.z(), b.z());
    }

    fn assert_f32x4x4_close(a: f32x4x4, b: f32x4x4) {
        assert_f32x4_close(a[0], b[0]);
        assert_f32x4_close(a[1], b[1]);
        assert_f32x4_close(a[2], b[2]);
        assert_f32x4_close(a[3], b[3]);
    }

    fn assert_f32x3x3_close(a: f32x3x3, b: f32x3x3) {
        assert_f32x3_close(a[0], b[0]);
        assert_f32x3_close(a[1], b[1]);
        assert_f32x3_close(a[2], b[2]);
    }

    fn assert_f32quat_close(a: f32quat, b: f32quat) {
        assert_f32x4_close(a.0, b.0);
    }

    fn assert_f32quat_equiv(a: f32quat, b: f32quat) {
        let same = (a.x() - b.x()).abs() < 1e-6
            && (a.y() - b.y()).abs() < 1e-6
            && (a.z() - b.z()).abs() < 1e-6
            && (a.w() - b.w()).abs() < 1e-6;
        if same {
            return;
        }

        let neg_b = f32quat(f32x4::with_xyzw(-b.x(), -b.y(), -b.z(), -b.w()));
        assert_f32quat_close(a, neg_b);
    }

    fn rot_matrix(c0: f32x4, c1: f32x4, c2: f32x4) -> f32x4x4 {
        super::f32x4x4_with_cols(c0, c1, c2, f32x4::with_xyzw(0.0, 0.0, 0.0, 1.0))
    }

    #[test]
    fn mul() {
        let _x = f32x2x2([f32x2::with_xy(1.0, 0.0), f32x2::with_xy(1.0, 0.0)]);
        let _y = f32x2x2([f32x2::with_xy(1.0, 0.0), f32x2::with_xy(1.0, 0.0)]);
    }

    #[test]
    fn f32x4x4_mul_identity() {
        let a = f32x4x4::translate(2.0, 3.0, 4.0);
        let i = f32x4x4::identity();

        assert_eq!(a * i, a);
        assert_eq!(i * a, a);
        assert_eq!(simd::mul(a, i), a);
        assert_eq!(simd::mul(i, a), a);
    }

    #[test]
    fn f32x4x4_mul_translate_compose() {
        let a = f32x4x4::translate(2.0, 3.0, 4.0);
        let b = f32x4x4::translate(5.0, 7.0, 11.0);
        let c = a * b;
        let d = simd::mul(a, b);

        assert_eq!(c.tx(), 7.0);
        assert_eq!(c.ty(), 10.0);
        assert_eq!(c.tz(), 15.0);
        assert_eq!(c, d);
    }

    #[test]
    fn f32x4x4_mul_f32x4() {
        let m = f32x4x4::translate(2.0, 3.0, 4.0);
        let v = f32x4::with_xyzw(1.0, 2.0, 3.0, 1.0);
        let o1 = m * v;
        let o2 = simd::mul(m, v);

        assert_eq!(o1, f32x4::with_xyzw(3.0, 5.0, 7.0, 1.0));
        assert_eq!(o1, o2);
    }

    #[test]
    fn f32x4_dot() {
        let a = f32x4::with_xyzw(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::with_xyzw(5.0, 6.0, 7.0, 8.0);
        assert_eq!(a.dot(&b), 70.0);
    }

    #[test]
    fn f32x4_fmla() {
        let a = f32x4::with_xyzw(1.0, 2.0, 3.0, 4.0);
        let n = f32x4::with_xyzw(10.0, 20.0, 30.0, 40.0);
        let m = f32x4::with_xyzw(0.1, -0.5, 2.0, 0.25);
        assert_eq!(a.fmla(&n, &m), f32x4::with_xyzw(2.0, -8.0, 63.0, 14.0));
    }

    #[test]
    fn f32x4_mul_f32() {
        let v = f32x4::with_xyzw(1.0, -2.0, 3.5, 0.25);
        assert_eq!(v.mul_f32(2.0), f32x4::with_xyzw(2.0, -4.0, 7.0, 0.5));
    }

    #[test]
    fn f32x4_div_f32() {
        let v = f32x4::with_xyzw(2.0, -4.0, 7.0, 0.5);
        assert_eq!(v.div_f32(2.0), f32x4::with_xyzw(1.0, -2.0, 3.5, 0.25));
    }

    #[test]
    fn f32x4_xyz() {
        let v = f32x4::with_xyzw(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.xyz(), f32x3::with_xyz(1.0, 2.0, 3.0));
    }

    #[test]
    fn f32x4_len_sq() {
        let a = f32x4::with_xyzw(1.0, 2.0, 3.0, 4.0);
        assert_eq!(simd::len_sq(a), 30.0);
    }

    #[test]
    fn f32x4_normalized() {
        let a = f32x4::with_xyzw(3.0, 4.0, 0.0, 0.0);
        let n = simd::normalized(a);
        assert_f32_close(n.x(), 0.6);
        assert_f32_close(n.y(), 0.8);
        assert_f32_close(n.z(), 0.0);
        assert_f32_close(n.w(), 0.0);
        assert_f32_close(simd::len_sq(n), 1.0);
    }

    #[test]
    fn f32x4_mix() {
        let a = f32x4::with_xyzw(0.0, 10.0, 20.0, 30.0);
        let b = f32x4::with_xyzw(10.0, 20.0, 30.0, 40.0);

        let v = simd::mix(a, b, f32x4::with_xyzw(0.0, 0.25, 0.5, 1.0));
        assert_eq!(v, f32x4::with_xyzw(0.0, 12.5, 25.0, 40.0));

        let s = simd::mix(a, b, 0.5);
        assert_eq!(s, f32x4::with_xyzw(5.0, 15.0, 25.0, 35.0));
    }

    #[test]
    fn f32x3_dot() {
        let a = f32x3::with_xyz(1.0, 2.0, 3.0);
        let b = f32x3::with_xyz(5.0, 6.0, 7.0);
        assert_eq!(a.dot(&b), 38.0);
    }

    #[test]
    fn f32x3_fmla() {
        let a = f32x3::with_xyz(1.0, 2.0, 3.0);
        let n = f32x3::with_xyz(10.0, 20.0, 30.0);
        let m = f32x3::with_xyz(0.1, -0.5, 2.0);
        assert_eq!(a.fmla(&n, &m), f32x3::with_xyz(2.0, -8.0, 63.0));
    }

    #[test]
    fn f32x3_normalized() {
        let a = f32x3::with_xyz(3.0, 4.0, 0.0);
        let n = simd::normalized(a);
        assert_f32_close(n.x(), 0.6);
        assert_f32_close(n.y(), 0.8);
        assert_f32_close(n.z(), 0.0);
        assert_f32_close(n.dot(&n), 1.0);
    }

    #[test]
    fn f32x3_mix() {
        let a = f32x3::with_xyz(0.0, 10.0, 20.0);
        let b = f32x3::with_xyz(10.0, 20.0, 30.0);

        let v = simd::mix(a, b, f32x3::with_xyz(0.0, 0.25, 1.0));
        assert_eq!(v, f32x3::with_xyz(0.0, 12.5, 30.0));

        let s = simd::mix(a, b, 0.5);
        assert_eq!(s, f32x3::with_xyz(5.0, 15.0, 25.0));
    }

    #[test]
    fn f32x4x4_mix() {
        let a = super::f32x4x4_with_cols(
            f32x4::with_xyzw(0.0, 10.0, 20.0, 30.0),
            f32x4::with_xyzw(1.0, 11.0, 21.0, 31.0),
            f32x4::with_xyzw(2.0, 12.0, 22.0, 32.0),
            f32x4::with_xyzw(3.0, 13.0, 23.0, 33.0),
        );
        let b = super::f32x4x4_with_cols(
            f32x4::with_xyzw(10.0, 20.0, 30.0, 40.0),
            f32x4::with_xyzw(11.0, 21.0, 31.0, 41.0),
            f32x4::with_xyzw(12.0, 22.0, 32.0, 42.0),
            f32x4::with_xyzw(13.0, 23.0, 33.0, 43.0),
        );

        let t = super::f32x4x4_with_cols(
            f32x4::with_xyzw(0.0, 0.25, 0.5, 1.0),
            f32x4::with_xyzw(1.0, 0.5, 0.25, 0.0),
            f32x4::with_xyzw(0.5, 0.5, 0.5, 0.5),
            f32x4::with_xyzw(0.0, 1.0, 0.0, 1.0),
        );

        let v = simd::mix(a, b, t);
        let expected_v = super::f32x4x4_with_cols(
            f32x4::with_xyzw(0.0, 12.5, 25.0, 40.0),
            f32x4::with_xyzw(11.0, 16.0, 23.5, 31.0),
            f32x4::with_xyzw(7.0, 17.0, 27.0, 37.0),
            f32x4::with_xyzw(3.0, 23.0, 23.0, 43.0),
        );
        assert_eq!(v, expected_v);

        let s = simd::mix(a, b, 0.5);
        let expected_s = super::f32x4x4_with_cols(
            f32x4::with_xyzw(5.0, 15.0, 25.0, 35.0),
            f32x4::with_xyzw(6.0, 16.0, 26.0, 36.0),
            f32x4::with_xyzw(7.0, 17.0, 27.0, 37.0),
            f32x4::with_xyzw(8.0, 18.0, 28.0, 38.0),
        );
        assert_eq!(s, expected_s);
    }

    #[test]
    fn f32x3x3_mix() {
        let a = super::f32x3x3_with_cols(
            f32x3::with_xyz(0.0, 10.0, 20.0),
            f32x3::with_xyz(1.0, 11.0, 21.0),
            f32x3::with_xyz(2.0, 12.0, 22.0),
        );
        let b = super::f32x3x3_with_cols(
            f32x3::with_xyz(10.0, 20.0, 30.0),
            f32x3::with_xyz(11.0, 21.0, 31.0),
            f32x3::with_xyz(12.0, 22.0, 32.0),
        );

        let t = super::f32x3x3_with_cols(
            f32x3::with_xyz(0.0, 0.25, 0.5),
            f32x3::with_xyz(1.0, 0.5, 0.0),
            f32x3::with_xyz(0.5, 1.0, 0.5),
        );

        let v = simd::mix(a, b, t);
        let expected_v = super::f32x3x3_with_cols(
            f32x3::with_xyz(0.0, 12.5, 25.0),
            f32x3::with_xyz(11.0, 16.0, 21.0),
            f32x3::with_xyz(7.0, 22.0, 27.0),
        );
        assert_f32x3x3_close(v, expected_v);

        let s = simd::mix(a, b, 0.5);
        let expected_s = super::f32x3x3_with_cols(
            f32x3::with_xyz(5.0, 15.0, 25.0),
            f32x3::with_xyz(6.0, 16.0, 26.0),
            f32x3::with_xyz(7.0, 17.0, 27.0),
        );
        assert_f32x3x3_close(s, expected_s);
    }

    #[test]
    fn f32x3x3_inverse() {
        let d = f32x3x3::diagonal(f32x3::with_xyz(2.0, 4.0, 5.0));
        let d_inv = simd::inverse(d);
        let d_expected = f32x3x3::diagonal(f32x3::with_xyz(0.5, 0.25, 0.2));
        assert_f32x3x3_close(d_inv, d_expected);

        let t = super::f32x3x3_with_cols(
            f32x3::with_xyz(1.0, 0.0, 0.0),
            f32x3::with_xyz(2.0, 1.0, 0.0),
            f32x3::with_xyz(3.0, 4.0, 1.0),
        );
        let t_inv = simd::inverse(t);
        let t_expected = super::f32x3x3_with_cols(
            f32x3::with_xyz(1.0, 0.0, 0.0),
            f32x3::with_xyz(-2.0, 1.0, 0.0),
            f32x3::with_xyz(5.0, -4.0, 1.0),
        );
        assert_f32x3x3_close(t_inv, t_expected);
    }

    #[test]
    fn f32x4x4_inverse() {
        let d = f32x4x4::diagonal(f32x4::with_xyzw(2.0, 4.0, 5.0, 10.0));
        let d_inv = simd::inverse(d);
        let d_expected = f32x4x4::diagonal(f32x4::with_xyzw(0.5, 0.25, 0.2, 0.1));
        assert_f32x4x4_close(d_inv, d_expected);

        let t = super::f32x4x4_with_cols(
            f32x4::with_xyzw(1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw(2.0, 1.0, 0.0, 0.0),
            f32x4::with_xyzw(3.0, 5.0, 1.0, 0.0),
            f32x4::with_xyzw(4.0, 6.0, 7.0, 1.0),
        );
        let t_inv = simd::inverse(t);
        let t_expected = super::f32x4x4_with_cols(
            f32x4::with_xyzw(1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw(-2.0, 1.0, 0.0, 0.0),
            f32x4::with_xyzw(7.0, -5.0, 1.0, 0.0),
            f32x4::with_xyzw(-41.0, 29.0, -7.0, 1.0),
        );
        assert_f32x4x4_close(t_inv, t_expected);

        let id = f32x4x4::identity();
        assert_f32x4x4_close(simd::mul(t, t_inv), id);
        assert_f32x4x4_close(simd::mul(t_inv, t), id);
    }

    #[cfg(feature = "half")]
    #[test]
    fn f16quat() {
        use super::f16quat;
        use super::f16x3;

        let angle = half::f16::FRAC_PI_2;
        let z = half::f16::ZERO;
        let o = half::f16::ONE;
        let axis = f16x3::with_xyz(z, z, o);
        let quat = f16quat::with_angle(angle, axis);

        let i = quat.0.x().to_f32();
        let j = quat.0.y().to_f32();
        let k = quat.0.z().to_f32();
        let w = quat.0.w().to_f32();

        assert_eq!([i, j, k, w], [0.0, 0.0, 0.70703125, 0.70751953]);
    }

    #[test]
    fn f32quat_with_angle() {
        let quat = f32quat::with_angle(std::f32::consts::FRAC_PI_2, f32x3::with_xyz(0.0, 0.0, 1.0));
        assert_eq!(quat.0, [0.0, 0.0, 0.70710677, 0.70710677]);
    }

    #[test]
    fn f32quat_mul_identity_and_function_parity() {
        let identity = f32quat::identity();
        let q = f32quat::with_angle(std::f32::consts::FRAC_PI_2, f32x3::with_xyz(0.0, 0.0, 1.0));

        assert_f32quat_close(identity * q, q);
        assert_f32quat_close(q * identity, q);
        assert_f32quat_close(simd::mul(identity, q), q);
        assert_f32quat_close(simd::mul(q, identity), q);
    }

    #[test]
    fn f32quat_dot() {
        let a = f32quat(f32x4::with_xyzw(1.0, 2.0, 3.0, 4.0));
        let b = f32quat(f32x4::with_xyzw(5.0, 6.0, 7.0, 8.0));
        assert_eq!(a.dot(&b), 70.0);
    }

    #[test]
    fn f32quat_len_sq() {
        let q = f32quat(f32x4::with_xyzw(1.0, -2.0, 3.0, -4.0));
        assert_eq!(simd::len_sq(q), 30.0);
    }

    #[test]
    fn f32quat_normalized() {
        let q = f32quat(f32x4::with_xyzw(0.0, 0.0, 2.0, 2.0));
        let n = simd::normalized(q);
        let expected = f32quat(f32x4::with_xyzw(
            0.0,
            0.0,
            std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::FRAC_1_SQRT_2,
        ));
        assert_f32quat_close(n, expected);
        assert_f32_close(simd::len_sq(n), 1.0);
    }

    #[test]
    fn f32quat_negate() {
        let q = f32quat(f32x4::with_xyzw(1.0, -2.0, 3.5, -4.0));
        let expected = f32quat(f32x4::with_xyzw(-1.0, 2.0, -3.5, 4.0));
        assert_eq!(q.negate(), expected);
        assert_eq!(-q, expected);
    }

    #[test]
    fn f32quat_mul_known_result() {
        let qx = f32quat::with_angle(std::f32::consts::FRAC_PI_2, f32x3::with_xyz(1.0, 0.0, 0.0));
        let qy = f32quat::with_angle(std::f32::consts::FRAC_PI_2, f32x3::with_xyz(0.0, 1.0, 0.0));
        let q = qx * qy;
        let expected = f32quat(f32x4::with_xyzw(0.5, 0.5, 0.5, 0.5));
        assert_f32quat_close(q, expected);
    }

    #[test]
    fn f32quat_mul_associativity_spot_check() {
        let qa = f32quat::with_angle(0.3, f32x3::with_xyz(1.0, 0.0, 0.0));
        let qb = f32quat::with_angle(0.7, f32x3::with_xyz(0.0, 1.0, 0.0));
        let qc = f32quat::with_angle(1.1, f32x3::with_xyz(0.0, 0.0, 1.0));

        let lhs = (qa * qb) * qc;
        let rhs = qa * (qb * qc);
        assert_f32quat_close(lhs, rhs);
    }

    #[test]
    fn f32quat_conjugate_matches_component_sign_flip() {
        let q = f32quat(f32x4::with_xyzw(1.0, -2.0, 3.5, -4.0));
        let c = q.conjugate();

        let expected = f32quat(f32x4::with_xyzw(-1.0, 2.0, -3.5, -4.0));
        assert_eq!(c, expected);
    }

    #[test]
    fn f32quat_conjugate_is_inverse_for_unit_quat() {
        let q = f32quat::with_angle(0.7, f32x3::with_xyz(0.0, 0.0, 1.0));
        let qc = q.conjugate();
        let identity = f32quat::identity();

        assert_f32quat_close(q * qc, identity);
        assert_f32quat_close(qc * q, identity);
    }

    #[test]
    fn f32quat_from_matrix_identity() {
        let q = f32quat::from_f32x4x4(f32x4x4::identity());
        let expected = f32quat::identity();
        assert_f32quat_equiv(q, expected);
    }

    #[test]
    fn f32quat_from_matrix_rot_z_90() {
        let m = rot_matrix(
            f32x4::with_xyzw(0.0, 1.0, 0.0, 0.0),
            f32x4::with_xyzw(-1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 0.0, 1.0, 0.0),
        );
        let q = f32quat::from_f32x4x4(m);
        let expected =
            f32quat::with_angle(std::f32::consts::FRAC_PI_2, f32x3::with_xyz(0.0, 0.0, 1.0));
        assert_f32quat_equiv(q, expected);
    }

    #[test]
    fn f32quat_from_matrix_rot_x_180_branch_x() {
        let m = rot_matrix(
            f32x4::with_xyzw(1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, -1.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 0.0, -1.0, 0.0),
        );
        let q = f32quat::from_f32x4x4(m);
        let expected = f32quat(f32x4::with_xyzw(1.0, 0.0, 0.0, 0.0));
        assert_f32quat_equiv(q, expected);
    }

    #[test]
    fn f32quat_from_matrix_rot_y_180_branch_y() {
        let m = rot_matrix(
            f32x4::with_xyzw(-1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 1.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 0.0, -1.0, 0.0),
        );
        let q = f32quat::from_f32x4x4(m);
        let expected = f32quat(f32x4::with_xyzw(0.0, 1.0, 0.0, 0.0));
        assert_f32quat_equiv(q, expected);
    }

    #[test]
    fn f32quat_from_matrix_rot_z_180_branch_z() {
        let m = rot_matrix(
            f32x4::with_xyzw(-1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, -1.0, 0.0, 0.0),
            f32x4::with_xyzw(0.0, 0.0, 1.0, 0.0),
        );
        let q = f32quat::from_f32x4x4(m);
        let expected = f32quat(f32x4::with_xyzw(0.0, 0.0, 1.0, 0.0));
        assert_f32quat_equiv(q, expected);
    }
}
