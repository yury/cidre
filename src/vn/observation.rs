use crate::{cf, cg, cm, define_obj_type, objc, vn};

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
}

define_obj_type!(RectangleObservation(DetectedObjectObservation));

define_obj_type!(TextObservation(RectangleObservation));

define_obj_type!(RecognizedTextObservation(RectangleObservation));

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

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_uuid(id: &objc::Id) -> &cf::UUID;
    fn rsel_confidence(id: &objc::Id) -> vn::Confidence;
    fn rsel_timeRange(id: &objc::Id) -> cm::TimeRange;

    fn rsel_boundingBox(id: &objc::Id) -> cg::Rect;

    fn rsel_angle(id: &objc::Id) -> cg::Float;
    fn rsel_transform(id: &objc::Id) -> cg::AffineTransform;
    fn rsel_transformForImageWidth_height(
        id: &objc::Id,
        width: usize,
        height: usize,
    ) -> cg::AffineTransform;
}
