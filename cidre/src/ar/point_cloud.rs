use std::ptr::slice_from_raw_parts;

use crate::{define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARPointCloud")]
    /// A set of tracked 3D feature points and their stable identifiers.
    pub PointCloud(ns::Id)
);

impl PointCloud {
    /// Number of points in this point cloud.
    #[objc::msg_send(count)]
    pub fn count(&self) -> usize;

    /// Raw pointer to point data (`count` elements).
    #[objc::msg_send(points)]
    pub fn points_ptr(&self) -> *const simd::f32x3;

    /// 3D points comprising this point cloud.
    #[inline]
    pub fn points(&self) -> &[simd::f32x3] {
        unsafe { &*slice_from_raw_parts(self.points_ptr(), self.count()) }
    }

    /// Raw pointer to identifier data (`count` elements).
    #[objc::msg_send(identifiers)]
    pub fn ids_ptr(&self) -> *const u64;

    /// Stable per-point identifiers corresponding to `points`.
    #[inline]
    pub fn ids(&self) -> &[u64] {
        unsafe { &*slice_from_raw_parts(self.ids_ptr(), self.count()) }
    }
}
