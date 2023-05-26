macro_rules! accessors {
    (x) => {
        #[inline]
        pub fn x(&self) -> T {
            self.0[0]
        }

        #[inline]
        pub fn set_x(&mut self, value: T) {
            self.0[0] = value;
        }

        #[inline]
        pub fn r(&self) -> T {
            self.0[0]
        }

        #[inline]
        pub fn set_r(&mut self, value: T) {
            self.0[0] = value;
        }
    };
    (y) => {
        #[inline]
        pub fn y(&self) -> T {
            self.0[1]
        }

        #[inline]
        pub fn set_y(&mut self, value: T) {
            self.0[1] = value;
        }

        #[inline]
        pub fn g(&self) -> T {
            self.0[1]
        }

        #[inline]
        pub fn set_g(&mut self, value: T) {
            self.0[1] = value;
        }
    };
    (z) => {
        #[inline]
        pub fn z(&self) -> T {
            self.0[2]
        }

        #[inline]
        pub fn set_z(&mut self, value: T) {
            self.0[2] = value;
        }

        #[inline]
        pub fn b(&self) -> T {
            self.0[2]
        }

        #[inline]
        pub fn set_b(&mut self, value: T) {
            self.0[2] = value;
        }
    };
    (w) => {
        #[inline]
        pub fn w(&self) -> T {
            self.0[3]
        }

        #[inline]
        pub fn set_w(&mut self, value: T) {
            self.0[3] = value;
        }

        #[inline]
        pub fn a(&self) -> T {
            self.0[3]
        }

        #[inline]
        pub fn set_a(&mut self, value: T) {
            self.0[3] = value;
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
#[repr(transparent)]
pub struct Simd<T, const LANES: usize, const N: usize>([T; LANES]);

impl<T: Copy> Simd<T, 1, 1> {
    #[inline]
    pub fn with_x(x: T) -> Self {
        Self([x])
    }

    accessors!(x);
}

impl<T: Copy> Simd<T, 2, 2> {
    #[inline]
    pub fn with_xy(x: T, y: T) -> Self {
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
    pub fn with_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }

    #[inline]
    pub fn with_rgba(r: T, g: T, b: T, a: T) -> Self {
        Self([r, g, b, a])
    }

    accessors!(x, y, z, w);
}

#[cfg(test)]
pub mod tests {
    use crate::simd::f32x2;

    #[test]
    fn basics() {
        let mut f = f32x2::with_xy(0., 0.);
        f.set_x(10.0);
        assert_eq!(f.r(), 10.0);
    }
}
