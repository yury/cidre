use std::mem::MaybeUninit;

use crate::{
    cg, cm, cv, define_obj_type, ns, objc,
    vn::{self, ElementType},
};

define_obj_type!(Observation(ns::Id));
define_obj_type!(RecognizedText(ns::Id));

impl RecognizedText {
    #[objc::msg_send(string)]
    pub fn string(&self) -> &ns::String;
}

impl Observation {
    /// The unique identifier assigned to an observation.
    #[objc::msg_send(uuid)]
    pub fn uuid(&self) -> &ns::UUID;

    /// The level of confidence normalized to [0, 1] where 1 is most confident
    ///
    /// Confidence can always be returned as 1.0 if confidence is not supported or has no meaning
    #[objc::msg_send(confidence)]
    pub fn confidence(&self) -> vn::Confidence;

    /// The duration of the observation reporting when first detected and how long it is valid.
    ///
    /// The duration of the observation when used with a sequence of buffers.
    /// If a request does not support a timeRange or the timeRange is not known,
    /// the start time and duration will be set to 0.
    #[objc::msg_send(timeRange)]
    pub fn time_range(&self) -> cm::TimeRange;
}

define_obj_type!(DetectedObjectObservation(Observation));

impl DetectedObjectObservation {
    #[objc::msg_send(boundingBox)]
    pub fn bounding_box(&self) -> cg::Rect;

    #[objc::msg_send(globalSegmentationMask)]
    pub fn global_segmentation_mask(&self) -> Option<&vn::PixelBufferObservation>;
}

define_obj_type!(FaceObservation(DetectedObjectObservation));

impl FaceObservation {
    /// The face landmarks populated by the vn::DetectFaceLandmarksRequest.
    /// This is set to nil if only a vn::DetectFaceRectanglesRequest was performed.
    #[objc::msg_send(landmarks)]
    pub fn landmarks(&self) -> Option<&vn::FaceLandmarks2D>;

    /// The capture quality of the face as a normalized value between 0.0 and 1.0
    /// that can be used to compare the quality of the face in terms of it capture
    /// attributes (lighting, blur, position). This score can be used to compare
    /// the capture quality of a face against other captures of the same face in a given set.
    #[doc(alias = "faceCaptureQuality")]
    #[inline]
    #[objc::msg_send(faceCaptureQuality)]
    pub fn face_capture_quality(&self) -> Option<&ns::Number>;

    /// Face roll angle populated by vn::DetectFaceRectanglesRequest.
    /// The roll is reported in radians, positive angle corresponds
    /// to counterclockwise direction, range [-Pi, Pi). None value indicates
    /// that the roll angle hasn't been computed
    #[objc::msg_send(roll)]
    pub fn roll(&self) -> Option<&ns::Number>;

    /// Face yaw angle populated by vn::DetectFaceRectanglesRequest.
    /// The yaw is reported in radians, positive angle corresponds to
    /// counterclockwise direction, range [-Pi/2, Pi/2]. None value indicates
    /// that the yaw angle hasn't been computed
    #[objc::msg_send(yaw)]
    pub fn yaw(&self) -> Option<&ns::Number>;

    /// Face pitch angle populated by VNDetectFaceRectanglesRequest.
    /// The pitch is reported in radians, positive angle corresponds
    /// to nodding head down direction, range [-Pi/2, Pi/2]. None value indicates
    /// that the pitch angle hasn't been computed
    #[objc::msg_send(pitch)]
    pub fn pitch(&self) -> Option<&ns::Number>;
}

define_obj_type!(ClassificationObservation(Observation));

impl ClassificationObservation {
    #[objc::msg_send(identifier)]
    pub fn identifier(&self) -> &ns::String;

    #[objc::msg_send(hasPrecisionRecallCurve)]
    pub fn has_precision_recall_curve(&self) -> bool;

    #[objc::msg_send(hasMinimumRecall:forPrecision:)]
    pub fn has_minimum_recall_for_precision(&self, minimum_recall: f32, precision: f32) -> bool;

    #[objc::msg_send(hasMinimumPrecision:forRecall:)]
    pub fn has_minimum_precision_for_recall(&self, minimum_precistion: f32, recall: f32) -> bool;
}

define_obj_type!(RecognizedObjectObservation(DetectedObjectObservation));

impl RecognizedObjectObservation {
    #[objc::msg_send(lables)]
    pub fn labels(&self) -> &ns::Array<vn::ClassificationObservation>;
}

define_obj_type!(CoreMLFeatureValueObservation(Observation));

impl CoreMLFeatureValueObservation {
    #[objc::msg_send(featureName)]
    pub fn feature_name(&self) -> &ns::String;
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
    #[objc::msg_send(characterBoxes)]
    pub fn character_boxes(&self) -> Option<&ns::Array<RectangleObservation>>;
}

define_obj_type!(RecognizedTextObservation(RectangleObservation));

impl RecognizedTextObservation {
    #[objc::msg_send(topCandidates:)]
    pub fn top_candidates(&self, max: usize) -> &ns::Array<RecognizedText>;
}

define_obj_type!(PixelBufferObservation(Observation));

impl PixelBufferObservation {
    #[objc::msg_send(pixelBuffer)]
    pub fn pixel_buffer(&self) -> &cv::PixelBuffer;

    #[objc::msg_send(featureName)]
    pub fn feature_name(&self) -> Option<&ns::String>;
}

define_obj_type!(BarcodeObservation(RectangleObservation));

define_obj_type!(HorizonObservation(Observation));

impl HorizonObservation {
    #[objc::msg_send(angle)]
    pub fn angle(&self) -> cg::Float;

    #[objc::msg_send(transform)]
    pub fn transform(&self) -> cg::AffineTransform;

    #[objc::msg_send(transformForImageWidth:height:)]
    pub fn tranform_for_image(&self, width: usize, height: usize) -> cg::AffineTransform;
}

define_obj_type!(HumanObservation(DetectedObjectObservation));

impl HumanObservation {
    #[objc::msg_send(upperBodyOnly)]
    pub fn upper_body_only(&self) -> bool;
}

define_obj_type!(SaliencyImageObservation(PixelBufferObservation));

impl SaliencyImageObservation {
    #[objc::msg_send(salientObjects)]
    pub fn salient_objects(&self) -> Option<&ns::Array<RectangleObservation>>;
}

define_obj_type!(FeaturePrintObservation(Observation));

impl FeaturePrintObservation {
    #[objc::msg_send(elementType)]
    pub fn element_type(&self) -> vn::ElementType;

    #[objc::msg_send(elementCount)]
    pub fn element_count(&self) -> usize;

    #[objc::msg_send(data)]
    pub fn data(&self) -> &ns::Data;

    pub fn vec_of<T: Sized>(&self) -> Vec<T> {
        let count = self.element_count();
        let mut vec: Vec<MaybeUninit<T>> = Vec::with_capacity(count);
        unsafe {
            let ptr = vec.as_mut_ptr() as *mut u8;
            self.data().get_bytes(ptr, count * std::mem::size_of::<T>());
            vec.set_len(count);
            std::mem::transmute(vec)
        }
    }

    #[inline]
    pub fn vec_f32(&self) -> Vec<f32> {
        debug_assert!(self.element_type() == ElementType::F32);
        self.vec_of::<f32>()
    }

    #[inline]
    pub fn vec_f64(&self) -> Vec<f64> {
        debug_assert!(self.element_type() == ElementType::F64);
        self.vec_of::<f64>()
    }

    /// # Safety
    /// use `compute_distance`
    #[doc(alias = "computeDistance:toFeaturePrintObservation:error:")]
    #[objc::msg_send(computeDistance:toFeaturePrintObservation:error:)]
    pub unsafe fn compute_distance_err<'ar>(
        &self,
        distance: &mut f32,
        to: &FeaturePrintObservation,
        error: &mut Option<&'ar ns::Error>,
    ) -> bool;

    /// Computes the distance between two feature print observations.
    ///
    /// Shorter distances indicate greater similarity between feature prints.
    #[doc(alias = "computeDistance:toFeaturePrintObservation:error:")]
    #[inline]
    pub fn compute_distance<'ar>(
        &self,
        to: &FeaturePrintObservation,
    ) -> Result<f32, &'ar ns::Error> {
        let mut distance = 0f32;
        let mut error = None;
        unsafe {
            let res = self.compute_distance_err(&mut distance, to, &mut error);
            if res {
                Ok(distance)
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }
}
