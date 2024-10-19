#[doc(alias = "MTLPackedFloat3")]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct PackedF32x3([f32; 3]);

impl PackedF32x3 {
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn set_x(&mut self, val: f32) {
        self.0[0] = val
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn set_y(&mut self, val: f32) {
        self.0[1] = val;
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn set_z(&mut self, val: f32) {
        self.0[2] = val
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}

#[doc(alias = "MTLPackedFloatQuaternion")]
pub struct PackedF32Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl PackedF32Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

#[doc(alias = "MTLPackedFloat4x3")]
pub struct PackedF32x4x3 {
    cols: [PackedF32x3; 4],
}

impl PackedF32x4x3 {
    pub fn new() {
        Self {
            cols: Default::default(),
        }
    }

    pub fn with(
        col0: PackedF32x3,
        col1: PackedF32x3,
        col2: PackedF32x3,
        col3: PackedF32x3,
    ) -> Self {
        Self {
            cols: [col0, col1, col2, col3],
        }
    }
}

/// An axis aligned bounding box with a min and max point
#[doc(alias = "MTLAxisAlignedBoundingBox")]
pub struct AxisAlignedBoundingBox {
    pub min: PackedF32x3,
    pub max: PackedF32x3,
}

/// A transformation represented by individual components such as translation and
/// rotation. The rotation is represented by a quaternion, allowing for correct motion
/// interpolation.
#[doc(alias = "MTLComponentTransform")]
pub struct ComponentTransform {
    /// The scale of the instance applied before rotation alongside shear and pivot
    pub scale: PackedF32x3,

    /// The shear of the instance applied before rotation alongside scale and pivot
    pub shear: PackedF32x3,

    /// Translation applied before rotation alongside scale and shear. Allows
    /// rotation to pivot around a point.
    pub pivot: PackedF32x3,

    /// The rotation of the instance as a normalized quaternion. Applied after scale,
    /// shear, and pivot and before translation
    pub rotation: PackedF32x3,

    /// The translation of the instance. Applied after rotation. Typically contains
    /// the composition of object translation and the inverse of the pivot translation.
    pub translation: PackedF32x3,
}
