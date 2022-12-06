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
pub struct f32x3x2([f32x2; 3]);

#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x4x2([f32x2; 4]);

#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct f32x2x3([f32x3; 2]);

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
