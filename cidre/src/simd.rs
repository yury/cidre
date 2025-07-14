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

#[allow(non_camel_case_types)]
pub type f32x3 = Simd<f32, 4, 3>;

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

    pub fn load(vals: &[f32; 4]) -> Self {
        Self(unsafe { std::arch::aarch64::vld1q_f32(vals.as_ptr()) })
    }

    pub fn splat(val: f32) -> Self {
        Self(unsafe { std::arch::aarch64::vdupq_n_f32(val) })
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

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x3x3(pub [f32x3; 3]);

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
    use super::f32x2;
    use super::f32x2x2;

    #[test]
    fn mul() {
        let _x = f32x2x2([f32x2::with_xy(1.0, 0.0), f32x2::with_xy(1.0, 0.0)]);
        let _y = f32x2x2([f32x2::with_xy(1.0, 0.0), f32x2::with_xy(1.0, 0.0)]);
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
    fn f32quat() {
        use std::f32;

        use crate::simd::f32quat;
        use crate::simd::f32x3;

        let quat = f32quat::with_angle(f32::consts::FRAC_PI_2, f32x3::with_xyz(0.0, 0.0, 1.0));
        assert_eq!(quat.0, [0.0, 0.0, 0.70710677, 0.70710677]);
    }
}
