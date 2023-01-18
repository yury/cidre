use crate::cg;

/// ```
/// use cidre::ca;
///
/// let t = ca::Transform3D::identity();
///
/// assert!(t.is_identity());
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Transform3D {
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m14: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m24: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
    pub m34: f64,
    pub m41: f64,
    pub m42: f64,
    pub m43: f64,
    pub m44: f64,
}

impl Transform3D {
    /// The identity transform: [1 0 0 0; 0 1 0 0; 0 0 1 0; 0 0 0 1].
    #[inline]
    pub fn identity() -> Self {
        unsafe { CATransform3DIdentity }
    }

    /// Returns a transform that translates by '(tx, ty, tz)':
    /// self =  [1 0 0 0; 0 1 0 0; 0 0 1 0; tx ty tz 1].
    #[inline]
    pub fn new_translation(tx: f64, ty: f64, tz: f64) -> Self {
        unsafe { CATransform3DMakeTranslation(tx, ty, tz) }
    }

    /// Returns a transform that scales by `(sx, sy, sz)':
    /// self = [sx 0 0 0; 0 sy 0 0; 0 0 sz 0; 0 0 0 1].
    #[inline]
    pub fn new_scale(sx: f64, sy: f64, sz: f64) -> Self {
        unsafe { CATransform3DMakeScale(sx, sy, sz) }
    }

    /// Returns a transform that rotates by 'angle' radians about the vector
    /// '(x, y, z)'. If the vector has length zero the identity transform is
    /// returned.
    #[inline]
    pub fn new_rotation(angle: f64, x: f64, y: f64, z: f64) -> Self {
        unsafe { CATransform3DMakeRotation(angle, x, y, z) }
    }

    #[inline]
    pub fn from_cg_affine_transform(m: cg::AffineTransform) -> Self {
        unsafe { CATransform3DMakeAffineTransform(m) }
    }

    /// Returns true if 'self' is the identity transform.
    #[inline]
    pub fn is_identity(&self) -> bool {
        unsafe { CATransform3DIsIdentity(*self) }
    }

    #[inline]
    pub fn translate(&self, tx: f64, ty: f64, tz: f64) -> Self {
        unsafe { CATransform3DTranslate(*self, tx, ty, tz) }
    }

    #[inline]
    pub fn scale(&self, sx: f64, sy: f64, sz: f64) -> Self {
        unsafe { CATransform3DScale(*self, sx, sy, sz) }
    }

    #[inline]
    pub fn rotate(&self, angle: f64, x: f64, y: f64, z: f64) -> Self {
        unsafe { CATransform3DRotate(*self, angle, x, y, z) }
    }

    #[inline]
    pub fn concat(&self, other: &Transform3D) -> Self {
        unsafe { CATransform3DConcat(*self, *other) }
    }

    #[inline]
    pub fn is_affine(&self) -> bool {
        unsafe { CATransform3DIsAffine(*self) }
    }

    #[inline]
    pub fn to_affine_transform(&self) -> cg::AffineTransform {
        unsafe { CATransform3DGetAffineTransform(*self) }
    }
}

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {
    static CATransform3DIdentity: Transform3D;
    fn CATransform3DIsIdentity(t: Transform3D) -> bool;
    fn CATransform3DMakeTranslation(tx: f64, ty: f64, tz: f64) -> Transform3D;
    fn CATransform3DMakeScale(sx: f64, sy: f64, sz: f64) -> Transform3D;
    fn CATransform3DMakeRotation(angle: f64, x: f64, y: f64, z: f64) -> Transform3D;
    fn CATransform3DTranslate(t: Transform3D, tx: f64, ty: f64, tz: f64) -> Transform3D;
    fn CATransform3DScale(t: Transform3D, sx: f64, sy: f64, sz: f64) -> Transform3D;
    fn CATransform3DRotate(t: Transform3D, angle: f64, x: f64, y: f64, z: f64) -> Transform3D;
    fn CATransform3DConcat(a: Transform3D, b: Transform3D) -> Transform3D;
    fn CATransform3DMakeAffineTransform(m: cg::AffineTransform) -> Transform3D;
    fn CATransform3DIsAffine(t: Transform3D) -> bool;
    fn CATransform3DGetAffineTransform(t: Transform3D) -> cg::AffineTransform;
}
