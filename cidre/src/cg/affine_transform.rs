use crate::cg;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct AffineTransform {
    pub a: cg::Float,
    pub b: cg::Float,
    pub c: cg::Float,
    pub d: cg::Float,
    pub tx: cg::Float,
    pub ty: cg::Float,
}

/*                      |--------------------------- Components ------------------------|
 *
 *      | a  b  0 |     | sx  0  0 |   |  1  0  0 |   | cos(t)  sin(t)  0 |   | 1  0  0 |
 *      | c  d  0 |  =  |  0 sy  0 | * | sh  1  0 | * |-sin(t)  cos(t)  0 | * | 0  1  0 |
 *      | tx ty 1 |     |  0  0  1 |   |  0  0  1 |   |   0       0     1 |   | tx ty 1 |
 *    AffineTransform      scale           shear            rotation          translation
 */

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Components {
    /// Initial scaling in X and Y dimensions. {sx,sy}
    /// Negative values indicate the image has been flipped in this dimension.
    pub scale: cg::Size,

    /// shear distortion (sh). Turns rectangles to parallelograms. 0 for no shear. Typically 0.
    pub horizontal_shear: cg::Float,

    /// Rotation angle in radians about the origin. (t) Sign convention for clockwise rotation
    /// may differ between various Apple frameworks based on origin placement. Please see discussion.
    pub rotation: cg::Float,

    /// Displacement from the origin (ty, ty)
    pub translation: cg::Vector,
}

impl AffineTransform {
    #[inline]
    pub fn with_components(components: Components) -> Self {
        unsafe { CGAffineTransformMakeWithComponents(components) }
    }

    #[inline]
    pub fn identity() -> Self {
        unsafe { CGAffineTransformIdentity }
    }

    #[inline]
    pub fn new_translation(tx: f64, ty: f64) -> Self {
        unsafe { CGAffineTransformMakeTranslation(tx, ty) }
    }

    #[inline]
    pub fn new_scale(sx: f64, sy: f64) -> Self {
        unsafe { CGAffineTransformMakeScale(sx, sy) }
    }

    #[inline]
    pub fn new_rotation(angle: f64) -> Self {
        unsafe { CGAffineTransformMakeRotation(angle) }
    }

    #[inline]
    pub fn is_identity(&self) -> bool {
        unsafe { CGAffineTransformIsIdentity(*self) }
    }

    #[inline]
    pub fn translate(&self, tx: f64, ty: f64) -> Self {
        unsafe { CGAffineTransformTranslate(*self, tx, ty) }
    }

    #[inline]
    pub fn scale(&self, sx: f64, sy: f64) -> Self {
        unsafe { CGAffineTransformScale(*self, sx, sy) }
    }

    #[inline]
    pub fn rotate(&self, angle: f64) -> Self {
        unsafe { CGAffineTransformRotate(*self, angle) }
    }

    #[inline]
    pub fn invert(&self) -> Self {
        unsafe { CGAffineTransformInvert(*self) }
    }

    #[inline]
    pub fn concat(&self, other: &Self) -> Self {
        unsafe { CGAffineTransformConcat(*self, *other) }
    }

    #[inline]
    pub fn equal_to(&self, other: &Self) -> bool {
        unsafe { CGAffineTransformEqualToTransform(*self, *other) }
    }

    #[inline]
    pub fn decompose(&self) -> Components {
        unsafe { CGAffineTransformDecompose(*self) }
    }
}

impl PartialEq for AffineTransform {
    /// ```
    /// use cidre::cg::AffineTransform;
    ///
    /// let a = AffineTransform::identity();
    /// assert!(a.is_identity());
    /// assert_eq!(a, AffineTransform::identity());
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal_to(other)
    }
}

impl cg::Point {
    #[inline]
    pub fn apply_affine_transform(&self, t: &AffineTransform) -> Self {
        cg::Point {
            x: t.a * self.x + t.c * self.y + t.tx,
            y: t.b * self.x + t.d * self.y + t.ty,
        }
    }
}

impl cg::Size {
    #[inline]
    pub fn apply_affine_transform(&self, t: &AffineTransform) -> Self {
        cg::Size {
            width: t.a * self.width + t.c * self.height,
            height: t.b * self.width + t.d * self.height,
        }
    }
}

impl cg::Rect {
    #[inline]
    pub fn apply_affine_transform(&self, t: &AffineTransform) -> Self {
        unsafe { CGRectApplyAffineTransform(*self, *t) }
    }
}

unsafe extern "C-unwind" {
    static CGAffineTransformIdentity: AffineTransform;

    fn CGAffineTransformMakeTranslation(tx: f64, ty: f64) -> AffineTransform;
    fn CGAffineTransformMakeScale(sx: f64, sy: f64) -> AffineTransform;
    fn CGAffineTransformMakeRotation(angle: f64) -> AffineTransform;
    fn CGAffineTransformIsIdentity(t: AffineTransform) -> bool;
    fn CGAffineTransformTranslate(t: AffineTransform, tx: f64, ty: f64) -> AffineTransform;
    fn CGAffineTransformScale(t: AffineTransform, sx: f64, sy: f64) -> AffineTransform;
    fn CGAffineTransformRotate(t: AffineTransform, angle: f64) -> AffineTransform;
    fn CGAffineTransformInvert(t: AffineTransform) -> AffineTransform;
    fn CGAffineTransformConcat(t: AffineTransform, other: AffineTransform) -> AffineTransform;
    fn CGAffineTransformEqualToTransform(t: AffineTransform, other: AffineTransform) -> bool;
    fn CGRectApplyAffineTransform(rect: cg::Rect, t: AffineTransform) -> cg::Rect;
    fn CGAffineTransformDecompose(t: AffineTransform) -> Components;
    fn CGAffineTransformMakeWithComponents(components: Components) -> AffineTransform;

}
