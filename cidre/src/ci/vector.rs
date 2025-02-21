use crate::{arc, cg, ci, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CIVector")]
    pub Vec(ns::Id)
);

impl arc::A<ci::Vec> {
    #[objc::msg_send(initWithValues:count:)]
    pub fn init_with_vals(self, values: *const cg::Float, count: usize) -> arc::R<ci::Vec>;

    #[objc::msg_send(initWithString:)]
    pub fn init_ns_string(self, val: &ns::String) -> arc::R<ci::Vec>;
}

impl ci::Vec {
    define_cls!(CI_VECTOR);

    #[inline]
    pub fn with_vals(vals: &[cg::Float]) -> arc::R<Self> {
        Self::alloc().init_with_vals(vals.as_ptr(), vals.len())
    }

    #[inline]
    pub fn with_x(x: cg::Float) -> arc::R<Self> {
        Self::with_vals(&[x])
    }

    #[inline]
    pub fn with_xy(x: cg::Float, y: cg::Float) -> arc::R<Self> {
        Self::with_vals(&[x, y])
    }

    #[inline]
    pub fn with_xyz(x: cg::Float, y: cg::Float, z: cg::Float) -> arc::R<Self> {
        Self::with_vals(&[x, y, z])
    }

    #[inline]
    pub fn with_xyzw(x: cg::Float, y: cg::Float, z: cg::Float, w: cg::Float) -> arc::R<Self> {
        Self::with_vals(&[x, y, z, w])
    }

    #[inline]
    pub fn with_cg_point(val: cg::Point) -> arc::R<Self> {
        Self::with_xy(val.x, val.y)
    }

    #[inline]
    pub fn with_cg_rect(val: cg::Rect) -> arc::R<Self> {
        Self::with_xyzw(val.origin.x, val.origin.y, val.size.width, val.size.height)
    }

    #[inline]
    pub fn with_cg_affine_transform(val: cg::AffineTransform) -> arc::R<Self> {
        Self::with_vals(&[val.a, val.b, val.c, val.d, val.tx, val.ty])
    }

    #[inline]
    pub fn with_ns_string(val: &ns::String) -> arc::R<Self> {
        Self::alloc().init_ns_string(val)
    }

    /// Creates a cg::Point structure from the first two values (X and Y) in the vector.
    #[objc::msg_send(CGPointValue)]
    pub fn cg_point(&self) -> cg::Point;

    /// Creates a cg::Rect structure whose origin is the X and Y values in the vector and
    /// whose size is the Z and W values in the vector.
    #[objc::msg_send(CGRectValue)]
    pub fn cg_rect(&self) -> cg::Rect;

    #[objc::msg_send(CGAffineTransformValue)]
    pub fn cg_affine_transform(&self) -> cg::AffineTransform;

    #[objc::msg_send(valueAtIndex:)]
    pub fn get(&self, index: usize) -> cg::Float;

    /// Return the number of values stored in the vector.
    #[objc::msg_send(count)]
    pub fn len(&self) -> usize;

    #[objc::msg_send(stringRepresentation)]
    pub fn ns_string(&self) -> arc::R<ns::String>;

    #[objc::msg_send(X)]
    pub fn x(&self) -> cg::Float;

    #[objc::msg_send(Y)]
    pub fn y(&self) -> cg::Float;

    #[objc::msg_send(Z)]
    pub fn z(&self) -> cg::Float;

    #[objc::msg_send(W)]
    pub fn w(&self) -> cg::Float;
}

#[link(name = "ci", kind = "static")]
unsafe extern "C" {
    static CI_VECTOR: &'static objc::Class<Vec>;
}

#[cfg(test)]
mod tests {
    use crate::{cg, ci, ns};

    #[test]
    fn basics() {
        let vec = ci::Vec::with_xy(1.0, 2.0);
        let point = vec.cg_point();
        assert_eq!(vec.len(), 2);

        assert_eq!(point, cg::Point { x: 1.0, y: 2.0 });

        let str = vec.ns_string();
        assert_eq!(ns::str!(c"[1 2]"), &str);

        let rect = vec.cg_rect();
        assert_eq!(
            rect,
            cg::Rect {
                origin: point,
                size: cg::Size::zero()
            }
        );

        let _t = vec.cg_affine_transform();

        assert_eq!(0.0f64, vec.get(1000));

        let vec = ci::Vec::with_ns_string(ns::str!(c"hello"));
        assert_eq!(vec.x(), 0.0f64);
    }
}
