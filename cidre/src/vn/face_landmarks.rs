use std::ptr::slice_from_raw_parts;

use crate::{arc, cg, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNFaceLandmarkRegion")]
    pub Region(ns::Id)
);

impl Region {
    /// The amount of points in a given region. This can be zero if no points for a region could be found.
    #[objc::msg_send(pointCount)]
    pub fn point_count(&self) -> usize;
}

define_obj_type!(
    #[doc(alias = "VNFaceLandmarkRegion2D")]
    pub Region2d(Region)
);

impl Region2d {
    #[objc::msg_send(normalizedPoints)]
    pub fn normalized_points_(&self) -> *const cg::Point;

    pub fn normalized_points(&self) -> &[cg::Point] {
        unsafe { &*slice_from_raw_parts(self.normalized_points_(), self.point_count()) }
    }

    #[objc::msg_send(precisionEstimatesPerPoint)]
    #[objc::available(macos = 10.15, ios = 13.0, tvos = 13.0)]
    pub fn precision_estimates_per_point(&self) -> Option<arc::R<ns::Array<ns::Number>>>;

    /// Describes how to interpret the points provided by the region.
    #[objc::msg_send(pointsClassification)]
    #[objc::available(macos = 13.0, ios = 16.0, tvos = 16.0)]
    pub fn points_classification(&self) -> vn::PointsClassification;
}

define_obj_type!(
    #[doc(alias = "VNFaceLandmarks")]
    pub FaceLandmarks(ns::Id)
);

define_obj_type!(
    #[doc(alias = "VNFaceLandmarks2D")]
    pub FaceLandmarks2d(FaceLandmarks)
);

impl FaceLandmarks {
    #[objc::msg_send(confidence)]
    pub fn confidence(&self) -> vn::Confidence;
}

impl FaceLandmarks2d {
    /// The region containing all face landmark points.
    #[objc::msg_send(allPoints)]
    pub fn all_points(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points that describe the face contour from cheek over chin to cheek.
    #[objc::msg_send(faceContour)]
    pub fn face_contour(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the outline of the left eye.
    #[objc::msg_send(leftEye)]
    pub fn left_eye(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the outline of the right eye.
    #[objc::msg_send(rightEye)]
    pub fn right_eye(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the trace of the left eyebrow.
    #[objc::msg_send(leftEyebrow)]
    pub fn left_eyebrow(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the trace of the right eyebrow.
    #[objc::msg_send(rightEyebrow)]
    pub fn right_eyebrow(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the outline of the nose.
    #[objc::msg_send(nose)]
    pub fn nose(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the trace of the center crest of the nose.
    #[objc::msg_send(noseCrest)]
    pub fn nose_crest(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the trace of the center line of the face.
    #[objc::msg_send(medianLine)]
    pub fn median_line(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the outline of the outside of the lips.
    #[objc::msg_send(outerLips)]
    pub fn outer_lips(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the points describing the outline of the space between the of the lips.
    #[objc::msg_send(innerLips)]
    pub fn inner_lips(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the point where the left pupil is located.  This value may be inaccurate if
    /// the face is blinking.
    #[objc::msg_send(leftPupil)]
    pub fn left_pupil(&self) -> Option<arc::R<Region2d>>;

    /// The region containing the point where the right pupil is located.  This value may be inaccurate if
    /// the face is blinking.
    #[objc::msg_send(rightPupil)]
    pub fn right_pupil(&self) -> Option<arc::R<Region2d>>;
}
