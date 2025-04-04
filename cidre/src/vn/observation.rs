use crate::{arc, cg, cm, cv, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNObservation")]
    pub Observation(ns::Id)
);

define_obj_type!(
    #[doc(alias = "VNRecognizedText")]
    pub RecognizedText(ns::Id)
);

impl RecognizedText {
    /// The top candidate for recognized text.
    #[objc::msg_send(string)]
    pub fn string(&self) -> arc::R<ns::String>;

    /// A normalized confidence score for the text recognition result.
    #[objc::msg_send(confidence)]
    pub fn confidence(&self) -> vn::Confidence;

    #[objc::msg_send(boundingBoxForRange:error:)]
    pub unsafe fn bounding_box_for_range_err<'ear>(
        &self,
        range: ns::Range,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<vn::RectangleObservation>>;

    pub fn bounding_box_for_range<'ear>(
        &self,
        range: ns::Range,
    ) -> ns::Result<'ear, arc::R<vn::RectangleObservation>> {
        ns::if_none(|err| unsafe { self.bounding_box_for_range_err(range, err) })
    }
}

impl Observation {
    /// The unique identifier assigned to an observation.
    #[objc::msg_send(uuid)]
    pub fn uuid(&self) -> arc::R<ns::Uuid>;

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

define_obj_type!(pub DetectedObjectObservation(Observation));

impl DetectedObjectObservation {
    #[objc::msg_send(boundingBox)]
    pub fn bounding_box(&self) -> cg::Rect;

    #[objc::msg_send(globalSegmentationMask)]
    pub fn global_segmentation_mask(&self) -> Option<arc::R<vn::PixelBufObservation>>;
}

define_obj_type!(pub FaceObservation(DetectedObjectObservation));

impl FaceObservation {
    /// The face landmarks populated by the vn::DetectFaceLandmarksRequest.
    /// This is set to nil if only a vn::DetectFaceRectanglesRequest was performed.
    #[objc::msg_send(landmarks)]
    pub fn landmarks(&self) -> Option<arc::R<vn::FaceLandmarks2d>>;

    /// The capture quality of the face as a normalized value between 0.0 and 1.0
    /// that can be used to compare the quality of the face in terms of it capture
    /// attributes (lighting, blur, position). This score can be used to compare
    /// the capture quality of a face against other captures of the same face in a given set.
    #[doc(alias = "faceCaptureQuality")]
    #[inline]
    #[objc::msg_send(faceCaptureQuality)]
    pub fn face_capture_quality(&self) -> Option<arc::R<ns::Number>>;

    /// Face roll angle populated by vn::DetectFaceRectanglesRequest.
    /// The roll is reported in radians, positive angle corresponds
    /// to counterclockwise direction, range [-Pi, Pi). None value indicates
    /// that the roll angle hasn't been computed
    #[objc::msg_send(roll)]
    pub fn roll(&self) -> Option<arc::R<ns::Number>>;

    /// Face yaw angle populated by vn::DetectFaceRectanglesRequest.
    /// The yaw is reported in radians, positive angle corresponds to
    /// counterclockwise direction, range [-Pi/2, Pi/2]. None value indicates
    /// that the yaw angle hasn't been computed
    #[objc::msg_send(yaw)]
    pub fn yaw(&self) -> Option<arc::R<ns::Number>>;

    /// Face pitch angle populated by VNDetectFaceRectanglesRequest.
    /// The pitch is reported in radians, positive angle corresponds
    /// to nodding head down direction, range [-Pi/2, Pi/2]. None value indicates
    /// that the pitch angle hasn't been computed
    #[objc::msg_send(pitch)]
    pub fn pitch(&self) -> Option<arc::R<ns::Number>>;
}

define_obj_type!(pub ClassificationObservation(Observation));

impl ClassificationObservation {
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(hasPrecisionRecallCurve)]
    pub fn has_precision_recall_curve(&self) -> bool;

    #[objc::msg_send(hasMinimumRecall:forPrecision:)]
    pub fn has_minimum_recall_for_precision(&self, minimum_recall: f32, precision: f32) -> bool;

    #[objc::msg_send(hasMinimumPrecision:forRecall:)]
    pub fn has_minimum_precision_for_recall(&self, minimum_precistion: f32, recall: f32) -> bool;
}

define_obj_type!(pub RecognizedObjectObservation(DetectedObjectObservation));

impl RecognizedObjectObservation {
    #[objc::msg_send(lables)]
    pub fn labels(&self) -> arc::R<ns::Array<vn::ClassificationObservation>>;
}

define_obj_type!(pub CoreMLFeatureValueObservation(Observation));

impl CoreMLFeatureValueObservation {
    #[objc::msg_send(featureName)]
    pub fn feature_name(&self) -> arc::R<ns::String>;
}

define_obj_type!(pub RectangleObservation(DetectedObjectObservation));

define_obj_type!(pub TrajectoryObservation(Observation));

define_obj_type!(pub TextObservation(RectangleObservation));

impl TextObservation {
    /// Array of individual character bounding boxes found within the observation's boundingBox.
    ///
    /// If the associated request indicated that it is interested in character boxes by setting
    /// the vn::DetectTextRectanglesRequest reportCharacterBoxes property to true,
    /// this property will be non-nil (but may still be empty, depending on the detection results)
    #[objc::msg_send(characterBoxes)]
    pub fn character_boxes(&self) -> Option<arc::R<ns::Array<RectangleObservation>>>;
}

define_obj_type!(
    #[doc(alias = "VNRecognizedTextObservation")]
    pub RecognizedTextObservation(RectangleObservation)
);

impl RecognizedTextObservation {
    #[objc::msg_send(topCandidates:)]
    pub fn top_candidates(&self, max: usize) -> arc::R<ns::Array<RecognizedText>>;
}

define_obj_type!(
    #[doc(alias = "VNPixelBufferObservation")]
    pub PixelBufObservation(Observation)
);

impl PixelBufObservation {
    #[objc::msg_send(pixelBuffer)]
    pub fn pixel_buffer(&self) -> &cv::PixelBuf;

    #[objc::msg_send(featureName)]
    pub fn feature_name(&self) -> Option<arc::R<ns::String>>;
}

define_obj_type!(
    #[doc(alias = "VNBarcodeObservation")]
    pub BarcodeObservation(RectangleObservation)
);

define_obj_type!(
    #[doc(alias = "VNHorizonObservation")]
    pub HorizonObservation(Observation)
);

impl HorizonObservation {
    #[objc::msg_send(angle)]
    pub fn angle(&self) -> cg::Float;

    #[objc::msg_send(transform)]
    pub fn transform(&self) -> cg::AffineTransform;

    #[objc::msg_send(transformForImageWidth:height:)]
    pub fn tranform_for_image(&self, width: usize, height: usize) -> cg::AffineTransform;
}

define_obj_type!(
    #[doc(alias = "VNHumanObservation")]
    pub HumanObservation(DetectedObjectObservation)
);

impl HumanObservation {
    #[objc::msg_send(upperBodyOnly)]
    pub fn upper_body_only(&self) -> bool;
}

define_obj_type!(
    #[doc(alias = "VNInstanceMaskObservation")]
    pub InstanceMaskObservation(Observation)
);

impl InstanceMaskObservation {
    /// The resulting mask represents all instances in a mask image where 0 represents
    /// the background and all other values represent the indices of the instances identified.
    #[objc::msg_send(instanceMask)]
    pub fn instance_mask(&self) -> &cv::PixelBuf;

    /// The IndexSet that encompases all instances except the background
    #[objc::msg_send(allInstances)]
    pub fn all_instances(&self) -> arc::R<ns::IndexSet>;

    #[objc::msg_send(generateMaskForInstances:error:)]
    pub unsafe fn generate_mask_for_instances_err<'ear>(
        &self,
        instances: &ns::IndexSet,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::Retained<cv::PixelBuf>>;

    /// The low res mask from the selected instances in the resolution of the performed analysis
    /// which is not upscaled to the image resolution.
    ///
    /// The pixel format of kCVPixelFormatType_OneComponent32Float
    #[inline]
    pub fn generate_mask_for_instances<'ear>(
        &self,
        instances: &ns::IndexSet,
    ) -> ns::Result<'ear, arc::Retained<cv::PixelBuf>> {
        ns::if_none(|err| unsafe { self.generate_mask_for_instances_err(instances, err) })
    }

    #[objc::msg_send(generateMaskedImageOfInstances:fromRequestHandler:croppedToInstancesExtent:error:)]
    pub unsafe fn generate_masked_image_for_instances_cropped_err<'ear>(
        &self,
        instances: &ns::IndexSet,
        request_handler: &vn::ImageRequestHandler,
        crop_result: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::Retained<cv::PixelBuf>>;

    /// High res image with everything but the selected instances removed to transparent black.
    #[inline]
    pub fn generate_masked_image_for_instances_cropped<'ear>(
        &self,
        instances: &ns::IndexSet,
        request_handler: &vn::ImageRequestHandler,
        crop_result: bool,
    ) -> ns::Result<'ear, arc::Retained<cv::PixelBuf>> {
        ns::if_none(|err| unsafe {
            self.generate_masked_image_for_instances_cropped_err(
                instances,
                request_handler,
                crop_result,
                err,
            )
        })
    }

    #[objc::msg_send(generateScaledMaskForImageForInstances:fromRequestHandler:error:)]
    pub unsafe fn generate_scaled_mask_for_instances_err<'ear>(
        &self,
        instances: &ns::IndexSet,
        request_handler: &vn::ImageRequestHandler,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::Retained<cv::PixelBuf>>;

    /// High res mask with the selected instances preserved while everything else is removed to transparent black.
    #[inline]
    pub fn generate_scaled_mask_for_instances<'ear>(
        &self,
        instances: &ns::IndexSet,
        request_handler: &vn::ImageRequestHandler,
    ) -> ns::Result<'ear, arc::Retained<cv::PixelBuf>> {
        ns::if_none(|err| unsafe {
            self.generate_scaled_mask_for_instances_err(instances, request_handler, err)
        })
    }
}

define_obj_type!(
    #[doc(alias = "VNSaliencyImageObservation")]
    pub SaliencyImageObservation(PixelBufObservation)
);

impl SaliencyImageObservation {
    #[objc::msg_send(salientObjects)]
    pub fn salient_objs(&self) -> Option<arc::R<ns::Array<RectangleObservation>>>;
}

define_obj_type!(
    #[doc(alias = "VNFeaturePrintObservation")]
    pub FeaturePrintObservation(Observation)
);

impl FeaturePrintObservation {
    #[objc::msg_send(elementType)]
    pub fn element_type(&self) -> vn::ElementType;

    #[objc::msg_send(elementCount)]
    pub fn element_count(&self) -> usize;

    #[objc::msg_send(data)]
    pub fn data(&self) -> arc::R<ns::Data>;

    pub fn vec_of<T: Sized>(&self) -> Vec<T> {
        let count = self.element_count();
        let mut vec: Vec<std::mem::MaybeUninit<T>> = Vec::with_capacity(count);
        unsafe {
            let ptr = vec.as_mut_ptr() as *mut u8;
            self.data().get_bytes(ptr, count * std::mem::size_of::<T>());
            vec.set_len(count);
            std::mem::transmute(vec)
        }
    }

    #[inline]
    pub fn vec_f32(&self) -> Vec<f32> {
        debug_assert!(self.element_type() == vn::ElementType::F32);
        self.vec_of::<f32>()
    }

    #[inline]
    pub fn vec_f64(&self) -> Vec<f64> {
        debug_assert!(self.element_type() == vn::ElementType::F64);
        self.vec_of::<f64>()
    }

    /// # Safety
    /// use `compute_distance`
    #[doc(alias = "computeDistance:toFeaturePrintObservation:error:")]
    #[objc::msg_send(computeDistance:toFeaturePrintObservation:error:)]
    pub unsafe fn distance_err<'ar>(
        &self,
        distance: &mut f32,
        to: &FeaturePrintObservation,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    /// Computes the distance between two feature print observations.
    ///
    /// Shorter distances indicate greater similarity between feature prints.
    #[doc(alias = "computeDistance:toFeaturePrintObservation:error:")]
    #[inline]
    pub fn distance_to<'ear>(&self, to: &FeaturePrintObservation) -> ns::Result<'ear, f32> {
        let mut distance = 0f32;
        let mut error = None;
        unsafe {
            if self.distance_err(&mut distance, to, &mut error) {
                Ok(distance)
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }
}

define_obj_type!(
    #[doc(alias = "VNImageAestheticsScoresObservation")]
    pub ImageAestheticsScoresObservation(Observation)
);

impl ImageAestheticsScoresObservation {
    /// Represents images that are not necessarily of poor image quality but may not have memorable or exciting content.
    #[objc::msg_send(isUtility)]
    pub fn is_utility(&self) -> bool;

    /// A score which incorporates aesthetic score, failure score and utility labels. `overallScore` is within the range \[-1, 1\]
    /// where 1 is most desirable and -1 is not desirable.
    #[objc::msg_send(overallScore)]
    pub fn overall_score(&self) -> f32;
}
