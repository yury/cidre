use std::mem::transmute;

use crate::{cf, cg, cm, cv, define_obj_type, objc, vn};

define_obj_type!(Observation(objc::Id));

impl Observation {
    /// The unique identifier assigned to an observation.
    /// TODO: may be use ns::UUID
    pub fn uuid(&self) -> &cf::UUID {
        unsafe { rsel_uuid(self) }
    }

    /// The level of confidence normalized to [0, 1] where 1 is most confident
    ///
    /// Confidence can always be returned as 1.0 if confidence is not supported or has no meaning
    pub fn confidence(&self) -> vn::Confidence {
        unsafe { rsel_confidence(self) }
    }

    /// The duration of the observation reporting when first detected and how long it is valid.
    ///
    /// The duration of the observation when used with a sequence of buffers.
    /// If a request does not support a timeRange or the timeRange is not known,
    /// the start time and duration will be set to 0.
    pub fn time_range(&self) -> cm::TimeRange {
        unsafe { rsel_timeRange(self) }
    }
}

define_obj_type!(DetectedObjectObservation(Observation));

impl DetectedObjectObservation {
    #[inline]
    pub fn bounding_box(&self) -> cg::Rect {
        unsafe { rsel_boundingBox(self) }
    }

    #[inline]
    pub fn global_segmentation_mask(&self) -> Option<&vn::PixelBufferObservation> {
        unsafe { rsel_globalSegmentationMask(self) }
    }
}

define_obj_type!(FaceObservation(DetectedObjectObservation));

impl FaceObservation {
    /// The face landmarks populated by the vn::DetectFaceLandmarksRequest.
    /// This is set to nil if only a vn::DetectFaceRectanglesRequest was performed.
    #[inline]
    pub fn landmarks(&self) -> Option<&vn::FaceLandmarks2D> {
        unsafe { rsel_landmarks(self) }
    }

    /// The capture quality of the face as a normalized value between 0.0 and 1.0
    /// that can be used to compare the quality of the face in terms of it capture
    /// attributes (lighting, blur, position). This score can be used to compare
    /// the capture quality of a face against other captures of the same face in a given set.
    #[inline]
    pub fn face_capture_quality(&self) -> Option<&cf::Number> {
        unsafe { rsel_faceCaptureQuality(self) }
    }

    /// Face roll angle populated by vn::DetectFaceRectanglesRequest.
    /// The roll is reported in radians, positive angle corresponds
    /// to counterclockwise direction, range [-Pi, Pi). None value indicates
    /// that the roll angle hasn't been computed
    #[inline]
    pub fn roll(&self) -> Option<&cf::Number> {
        unsafe { rsel_roll(self) }
    }

    /// Face yaw angle populated by vn::DetectFaceRectanglesRequest.
    /// The yaw is reported in radians, positive angle corresponds to
    /// counterclockwise direction, range [-Pi/2, Pi/2]. None value indicates
    /// that the yaw angle hasn't been computed
    #[inline]
    pub fn yaw(&self) -> Option<&cf::Number> {
        unsafe { rsel_yaw(self) }
    }

    /// Face pitch angle populated by VNDetectFaceRectanglesRequest.
    /// The pitch is reported in radians, positive angle corresponds
    /// to nodding head down direction, range [-Pi/2, Pi/2]. None value indicates
    /// that the pitch angle hasn't been computed
    #[inline]
    pub fn pitch(&self) -> Option<&cf::Number> {
        unsafe { rsel_pitch(self) }
    }
}

define_obj_type!(ClassificationObservation(Observation));

impl ClassificationObservation {
    #[inline]
    pub fn identifier(&self) -> &cf::String {
        unsafe { rsel_identifier(self) }
    }

    #[inline]
    pub fn has_precision_recall_curve(&self) -> bool {
        unsafe { rsel_hasPrecisionRecallCurve(self) }
    }
}

define_obj_type!(RecognizedObjectObservation(DetectedObjectObservation));

impl RecognizedObjectObservation {
    pub fn labels(&self) -> &cf::ArrayOf<vn::ClassificationObservation> {
        unsafe { rsel_labels(self) }
    }
}

define_obj_type!(CoreMLFeatureValueObservation(Observation));

impl CoreMLFeatureValueObservation {
    pub fn feature_name(&self) -> &cf::String {
        unsafe { transmute(rsel_featureName(self)) }
    }
}

define_obj_type!(RectangleObservation(DetectedObjectObservation));

define_obj_type!(TrajectoryObservation(Observation));

define_obj_type!(TextObservation(RectangleObservation));

impl TextObservation {
    /// Array of individual character bounding boxes found within the observation's boundingBox.
    ///
    /// If the associated request indicated that it is interested in character boxes by setting
    /// the vn::DetectTextRectanglesRequest reportCharacterBoxes property to true,
    /// this property will be non-nil (but may still be empty, depending on the detection results)
    pub fn character_boxes(&self) -> Option<&cf::ArrayOf<RectangleObservation>> {
        unsafe { rsel_characterBoxes(self) }
    }
}

define_obj_type!(RecognizedTextObservation(RectangleObservation));

define_obj_type!(PixelBufferObservation(Observation));

impl PixelBufferObservation {
    #[inline]
    pub fn pixel_buffer(&self) -> &cv::PixelBuffer {
        unsafe { rsel_pixelBuffer(self) }
    }

    #[inline]
    pub fn feature_name(&self) -> Option<&cf::String> {
        unsafe { rsel_featureName(self) }
    }
}

define_obj_type!(BarcodeObservation(RectangleObservation));

define_obj_type!(HorizonObservation(Observation));

impl HorizonObservation {
    #[inline]
    pub fn angle(&self) -> cg::Float {
        unsafe { rsel_angle(self) }
    }

    #[inline]
    pub fn transform(&self) -> cg::AffineTransform {
        unsafe { rsel_transform(self) }
    }

    #[inline]
    pub fn tranform_for_image(&self, width: usize, height: usize) -> cg::AffineTransform {
        unsafe { rsel_transformForImageWidth_height(self, width, height) }
    }
}

define_obj_type!(HumanObservation(DetectedObjectObservation));

impl HumanObservation {
    #[inline]
    pub fn upper_body_only(&self) -> bool {
        unsafe { rsel_upperBodyOnly(self) }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_uuid(id: &objc::Id) -> &cf::UUID;
    fn rsel_confidence(id: &objc::Id) -> vn::Confidence;
    fn rsel_timeRange(id: &objc::Id) -> cm::TimeRange;

    fn rsel_boundingBox(id: &objc::Id) -> cg::Rect;
    fn rsel_globalSegmentationMask(id: &objc::Id) -> Option<&vn::PixelBufferObservation>;

    fn rsel_landmarks(id: &objc::Id) -> Option<&vn::FaceLandmarks2D>;
    fn rsel_faceCaptureQuality(id: &objc::Id) -> Option<&cf::Number>;
    fn rsel_roll(id: &objc::Id) -> Option<&cf::Number>;
    fn rsel_yaw(id: &objc::Id) -> Option<&cf::Number>;
    fn rsel_pitch(id: &objc::Id) -> Option<&cf::Number>;

    fn rsel_identifier(id: &objc::Id) -> &cf::String;
    fn rsel_hasPrecisionRecallCurve(id: &objc::Id) -> bool;

    fn rsel_labels(id: &objc::Id) -> &cf::ArrayOf<vn::ClassificationObservation>;

    fn rsel_pixelBuffer(id: &objc::Id) -> &cv::PixelBuffer;
    fn rsel_featureName(id: &objc::Id) -> Option<&cf::String>;

    fn rsel_characterBoxes(id: &objc::Id) -> Option<&cf::ArrayOf<vn::RectangleObservation>>;

    fn rsel_angle(id: &objc::Id) -> cg::Float;
    fn rsel_transform(id: &objc::Id) -> cg::AffineTransform;
    fn rsel_transformForImageWidth_height(
        id: &objc::Id,
        width: usize,
        height: usize,
    ) -> cg::AffineTransform;

    fn rsel_upperBodyOnly(id: &objc::Id) -> bool;
}
