use std::mem::transmute;

use crate::{cf, cg, cm, cv, define_obj_type, msg_send, ns, vn};

define_obj_type!(Observation(ns::Id));
define_obj_type!(RecognizedText(ns::Id));

impl RecognizedText {
    pub fn string(&self) -> &cf::String {
        unsafe { rsel_string(self) }
    }
}

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

    #[inline]
    pub fn has_minimum_recall_for_precision(&self, minimum_recall: f32, precision: f32) -> bool {
        unsafe { rsel_hasMinimumRecall_forPrecision(self, minimum_recall, precision) }
    }

    #[inline]
    pub fn has_minimum_precision_for_recall(&self, minimum_precistion: f32, recall: f32) -> bool {
        unsafe { rsel_hasMinimumPrecision_forRecall(self, minimum_precistion, recall) }
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

impl RecognizedTextObservation {
    pub fn top_candidates(&self, max: usize) -> &cf::ArrayOf<RecognizedText> {
        unsafe { rsel_topCandidates(self, max) }
    }
}

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

define_obj_type!(SaliencyImageObservation(PixelBufferObservation));

impl SaliencyImageObservation {
    #[inline]
    pub fn salient_objects(&self) -> Option<&cf::ArrayOf<RectangleObservation>> {
        unsafe { rsel_salientObjects(self) }
    }
}

define_obj_type!(FeaturePrintObservation(Observation));

impl FeaturePrintObservation {
    pub fn element_type(&self) -> vn::ElementType {
        unsafe { vn_rsel_elementType(self) }
    }

    pub fn element_count(&self) -> usize {
        unsafe { vn_rsel_elementCount(self) }
    }

    pub fn data(&self) -> Option<&cf::Data> {
        unsafe { vn_rsel_data(self) }
    }

    /// # Safety
    /// use `compute_distance`
    #[doc(alias = "computeDistance:toFeaturePrintObservation:error:")]
    #[inline]
    pub unsafe fn compute_distance_error<'ar>(
        &self,
        distance: &mut f32,
        to: &FeaturePrintObservation,
        error: &mut Option<&'ar cf::Error>,
    ) -> bool {
        msg_send!(
            "vn",
            self,
            sel_computeDistance_toFeaturePrintObservation_error,
            distance,
            to,
            error
        )
    }

    /// Computes the distance between two feature print observations.
    ///
    /// Shorter distances indicate greater similarity between feature prints.
    #[doc(alias = "computeDistance:toFeaturePrintObservation:error:")]
    #[inline]
    pub fn compute_distance<'ar>(
        &self,
        to: &FeaturePrintObservation,
    ) -> Result<f32, &'ar cf::Error> {
        unsafe {
            let mut distance = 0f32;
            let mut error = None;
            let res = self.compute_distance_error(&mut distance, to, &mut error);
            if res {
                Ok(distance)
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_uuid(id: &ns::Id) -> &cf::UUID;
    fn rsel_confidence(id: &ns::Id) -> vn::Confidence;
    fn rsel_timeRange(id: &ns::Id) -> cm::TimeRange;

    fn rsel_boundingBox(id: &ns::Id) -> cg::Rect;
    fn rsel_globalSegmentationMask(id: &ns::Id) -> Option<&vn::PixelBufferObservation>;

    fn rsel_landmarks(id: &ns::Id) -> Option<&vn::FaceLandmarks2D>;
    fn rsel_faceCaptureQuality(id: &ns::Id) -> Option<&cf::Number>;
    fn rsel_roll(id: &ns::Id) -> Option<&cf::Number>;
    fn rsel_yaw(id: &ns::Id) -> Option<&cf::Number>;
    fn rsel_pitch(id: &ns::Id) -> Option<&cf::Number>;

    fn rsel_identifier(id: &ns::Id) -> &cf::String;
    fn rsel_hasPrecisionRecallCurve(id: &ns::Id) -> bool;

    fn rsel_hasMinimumRecall_forPrecision(id: &ns::Id, minimum_recall: f32, precision: f32)
        -> bool;

    fn rsel_hasMinimumPrecision_forRecall(
        id: &ns::Id,
        minimum_precistion: f32,
        recall: f32,
    ) -> bool;

    fn rsel_labels(id: &ns::Id) -> &cf::ArrayOf<vn::ClassificationObservation>;

    fn rsel_pixelBuffer(id: &ns::Id) -> &cv::PixelBuffer;
    fn rsel_featureName(id: &ns::Id) -> Option<&cf::String>;

    fn rsel_characterBoxes(id: &ns::Id) -> Option<&cf::ArrayOf<vn::RectangleObservation>>;

    fn rsel_angle(id: &ns::Id) -> cg::Float;
    fn rsel_transform(id: &ns::Id) -> cg::AffineTransform;
    fn rsel_transformForImageWidth_height(
        id: &ns::Id,
        width: usize,
        height: usize,
    ) -> cg::AffineTransform;

    fn rsel_upperBodyOnly(id: &ns::Id) -> bool;

    fn rsel_salientObjects(id: &ns::Id) -> Option<&cf::ArrayOf<RectangleObservation>>;

    fn vn_rsel_elementType(id: &FeaturePrintObservation) -> vn::ElementType;
    fn vn_rsel_elementCount(id: &FeaturePrintObservation) -> usize;
    fn vn_rsel_data(id: &FeaturePrintObservation) -> Option<&cf::Data>;

    // rsel_abc(, id, computeDistance, float *, toFeaturePrintObservation, VNFeaturePrintObservation *, error, NSError **, BOOL)

    // fn rsel_computeDistance_toFeaturePrintObservation_error<'ar>(
    //     id: &ns::Id,
    //     distance: &mut f32,
    //     to: &FeaturePrintObservation,
    //     error: &mut Option<&'ar cf::Error>,
    // ) -> bool;

    fn rsel_topCandidates(
        id: &ns::Id,
        max_candidate_count: usize,
    ) -> &cf::ArrayOf<vn::RecognizedText>;

    fn rsel_string(id: &ns::Id) -> &cf::String;
}
