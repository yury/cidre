use std::hash::{Hash, Hasher};

macro_rules! accessors {
    (x) => {
        #[inline]
        pub fn x(&self) -> T {
            self.0[0]
        }

        #[inline]
        pub unsafe fn x_unchecked(&self) -> &T {
            unsafe { self.0.get_unchecked(0) }
        }

        #[inline]
        pub fn set_x(&mut self, val: T) {
            self.0[0] = val;
        }

        #[inline]
        pub fn r(&self) -> T {
            self.0[0]
        }

        #[inline]
        pub unsafe fn r_unchecked(&self) -> &T {
            unsafe { self.0.get_unchecked(0) }
        }

        #[inline]
        pub fn set_r(&mut self, val: T) {
            self.0[0] = val;
        }
    };
    (y) => {
        #[inline]
        pub fn y(&self) -> T {
            self.0[1]
        }

        #[inline]
        pub unsafe fn y_unchecked(&self) -> &T {
            unsafe { self.0.get_unchecked(1) }
        }

        #[inline]
        pub fn set_y(&mut self, val: T) {
            self.0[1] = val;
        }

        #[inline]
        pub fn g(&self) -> T {
            self.0[1]
        }

        #[inline]
        pub unsafe fn g_unchecked(&self) -> &T {
            unsafe { self.0.get_unchecked(1) }
        }

        #[inline]
        pub fn set_g(&mut self, val: T) {
            self.0[1] = val;
        }
    };
    (z) => {
        #[inline]
        pub fn z(&self) -> T {
            self.0[2]
        }

        #[inline]
        pub unsafe fn z_unchecked(&self) -> &T {
            unsafe { self.0.get_unchecked(2) }
        }

        #[inline]
        pub fn set_z(&mut self, val: T) {
            self.0[2] = val;
        }

        #[inline]
        pub fn b(&self) -> T {
            self.0[2]
        }

        #[inline]
        pub unsafe fn b_unchecked(&self) -> &T {
            unsafe { self.0.get_unchecked(2) }
        }

        #[inline]
        pub fn set_b(&mut self, val: T) {
            self.0[2] = val;
        }
    };
    (w) => {
        #[inline]
        pub fn w(&self) -> T {
            self.0[3]
        }

        #[inline]
        pub unsafe fn w_unchecked(&self) -> &T {
            unsafe { self.0.get_unchecked(3) }
        }

        #[inline]
        pub fn set_w(&mut self, val: T) {
            self.0[3] = val;
        }

        #[inline]
        pub fn a(&self) -> T {
            self.0[3]
        }

        #[inline]
        pub unsafe fn a_unchecked(&self) -> &T {
            unsafe { self.0.get_unchecked(3) }
        }

        #[inline]
        pub fn set_a(&mut self, val: T) {
            self.0[3] = val;
        }
    };
    (x, y) => {
        accessors!(x);
        accessors!(y);

        #[inline]
        pub fn xy(&self) -> Simd<T, 2, 2> {
            Simd::with_xy(self.x(), self.y())
        }
    };
    (x, y, z) => {
        accessors!(x, y);
        accessors!(z);

        #[inline]
        pub fn xyz(&self) -> Simd<T, 3, 3> {
            Simd([self.x(), self.y(), self.z()])
        }
    };
    (x, y, z, w) => {
        accessors!(x, y, z);
        accessors!(w);
    };
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Simd<T, const LANES: usize, const N: usize>([T; LANES]);

impl<T: Default + Copy, const LANES: usize, const N: usize> Default for Simd<T, LANES, N> {
    fn default() -> Self {
        Self([T::default(); LANES])
    }
}

impl<T: PartialEq, const LANES: usize, const N: usize> PartialEq for Simd<T, LANES, N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.0[i] != other.0[i] {
                // TODO: use abs
                return false;
            }
        }
        true
    }
}

impl<T, const LANES: usize, const N: usize> std::ops::Index<usize> for Simd<T, LANES, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const LANES: usize, const N: usize> std::ops::IndexMut<usize> for Simd<T, LANES, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const LANES: usize, const N: usize> Eq for Simd<f32, LANES, N> {}

impl<const LANES: usize, const N: usize> Hash for Simd<f32, LANES, N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = format!("{self:?}");
        s.hash(state);
    }
}

impl<T: Copy> Simd<T, 1, 1> {
    #[inline]
    pub fn with_x(x: T) -> Self {
        Self([x])
    }

    accessors!(x);
}

impl<T: Copy> Simd<T, 2, 2> {
    #[inline]
    pub const fn with_xy(x: T, y: T) -> Self {
        Self([x, y])
    }

    #[inline]
    pub fn with_rg(r: T, g: T) -> Self {
        Self([r, g])
    }

    accessors!(x, y);
}

impl<T: Default + Copy> Simd<T, 4, 3> {
    #[inline]
    pub fn with_xyz(x: T, y: T, z: T) -> Self {
        Self([x, y, z, Default::default()])
    }

    #[inline]
    pub fn with_rgb(r: T, g: T, b: T) -> Self {
        Self([r, g, b, Default::default()])
    }

    accessors!(x, y, z);
}

impl Simd<f32, 4, 3> {
    #[inline]
    pub const fn with_xyz_f32(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z, 0.0])
    }

    #[inline]
    pub const fn with_rgb_f32(r: f32, g: f32, b: f32) -> Self {
        Self([r, g, b, 0.0])
    }
}

impl<T: Copy> Simd<T, 4, 4> {
    #[inline]
    pub const fn with_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }

    #[inline]
    pub const fn with_rgba(r: T, g: T, b: T, a: T) -> Self {
        Self([r, g, b, a])
    }

    accessors!(x, y, z, w);
}

impl Simd<f32, 4, 4> {
    #[inline]
    pub const fn with_xyzw_f32(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }

    #[inline]
    pub const fn with_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }
}

#[cfg(test)]
pub mod tests {
    use crate::simd::{f32x2, f32x4x4};

    #[test]
    fn basics() {
        let mut f = f32x2::with_xy(0., 0.);
        f.set_x(10.0);
        assert_eq!(f.r(), 10.0);

        let mut f = f32x4x4::identity();
        assert_eq!(f[0][0], 1.0);

        f[0][1] = 10.0;
        assert_eq!(f[0][1], 10.0);
    }
}
