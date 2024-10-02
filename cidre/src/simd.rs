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

#[allow(non_camel_case_types)]
pub type f32x2 = Simd<f32, 2, 2>;
#[allow(non_camel_case_types)]
pub type f32x3 = Simd<f32, 4, 3>;
#[allow(non_camel_case_types)]
pub type f32x4 = Simd<f32, 4, 4>;

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
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x4x4(pub [f32x4; 4]);

impl f32x4x4 {
    pub fn diagonal(v: f32x4) -> Self {
        Self([
            f32x4::with_xyzw_f32(v.x(), 0.0, 0.0, 0.0),
            f32x4::with_xyzw_f32(0.0, v.y(), 0.0, 0.0),
            f32x4::with_xyzw_f32(0.0, 0.0, v.z(), 0.0),
            f32x4::with_xyzw_f32(0.0, 0.0, 0.0, v.w()),
        ])
    }

    pub const fn identity() -> Self {
        Self([
            f32x4::with_xyzw_f32(1.0, 0.0, 0.0, 0.0),
            f32x4::with_xyzw_f32(0.0, 1.0, 0.0, 0.0),
            f32x4::with_xyzw_f32(0.0, 0.0, 1.0, 0.0),
            f32x4::with_xyzw_f32(0.0, 0.0, 0.0, 1.0),
        ])
    }

    pub const fn translate(tx: f32, ty: f32, tz: f32) -> Self {
        Self([
            f32x4::with_xyzw_f32(1.0, 0.0, 0.0, tx),
            f32x4::with_xyzw_f32(0.0, 1.0, 0.0, ty),
            f32x4::with_xyzw_f32(0.0, 0.0, 1.0, tz),
            f32x4::with_xyzw_f32(0.0, 0.0, 0.0, 1.0),
        ])
    }

    #[inline]
    pub fn tx(&self) -> f32 {
        self[0].w()
    }

    #[inline]
    pub fn set_tx(&mut self, value: f32) {
        self[0].set_w(value)
    }

    #[inline]
    pub fn ty(&self) -> f32 {
        self[1].w()
    }

    #[inline]
    pub fn set_ty(&mut self, value: f32) {
        self[1].set_w(value)
    }

    #[inline]
    pub fn tz(&self) -> f32 {
        self[2].w()
    }

    #[inline]
    pub fn set_tz(&mut self, value: f32) {
        self[2].set_w(value)
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
        self[0].y()
    }

    #[inline]
    pub fn set_sy(&mut self, value: f32) {
        self[0].set_y(value)
    }

    #[inline]
    pub fn sz(&self) -> f32 {
        self[0].z()
    }

    #[inline]
    pub fn set_sz(&mut self, value: f32) {
        self[0].set_z(value)
    }
}

impl std::ops::Index<usize> for f32x4x4 {
    type Output = f32x4;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for f32x4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
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
}
