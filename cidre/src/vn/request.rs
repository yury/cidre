use crate::{arc, blocks, cg, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNRequest")]
    pub Request(ns::Id)
);

/// A block that is executed at the completion of a request.
#[doc(alias = "VNRequestCompletionHandler")]
pub type RequestCh<R = Request> = blocks::EscBlock<fn(request: &mut R, error: Option<&ns::Error>)>;

impl Request {
    /// The specific algorithm or implementation revision that is to be used to perform the request.
    #[objc::msg_send(revision)]
    pub fn revision(&self) -> usize;

    #[objc::msg_send(setRevision:)]
    pub fn set_revision(&mut self, value: usize);

    #[objc::msg_send(usesCPUOnly)]
    pub fn uses_cpu_only(&self) -> bool;

    #[objc::msg_send(setUsesCPUOnly:)]
    pub fn set_uses_cpu_only(&mut self, value: bool);

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::Observation>>>;

    #[objc::msg_send(completionHandler)]
    pub fn completion_handler(&self) -> Option<arc::R<RequestCh>>;

    #[objc::msg_send(preferBackgroundProcessing)]
    pub fn prefer_background_processing(&self) -> bool;

    #[objc::msg_send(setPreferBackgroundProcessing:)]
    pub fn set_prefer_background_processing(&mut self, val: bool);

    #[objc::msg_send(cancel)]
    pub fn cancel(&mut self);

    pub const REVISION_UNSPECIFIED: usize = 0;
}

define_obj_type!(
    #[doc(alias = "VNImageBasedRequest")]
    pub ImageBasedRequest(Request)
);

impl ImageBasedRequest {
    /// The region of the image in which the request will be performed.
    /// The rectangle is normalized to the dimensions of the image being
    /// processed and has its origin specified relative to the image's lower-left corner.
    ///
    /// The default value for this property is { { 0, 0 }, { 1, 1 } }.  Setting this
    /// property to a rectangle that is outside of the normalized coordinate space will
    /// be accepted but result in the request failing to be performed.
    #[objc::msg_send(regionOfInterest)]
    pub fn region_of_interest(&self) -> cg::Rect;

    #[objc::msg_send(setRegionOfInterest:)]
    pub fn set_region_of_interest(&mut self, value: cg::Rect);
}

define_obj_type!(
    #[doc(alias = "VNDetectHorizonRequest")]
    pub DetectHorizonRequest(ImageBasedRequest),
    VN_DETECT_HORIZON_REQUEST
);

impl DetectHorizonRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::HorizonObservation>>>;
}

#[doc(alias = "VNRequestProgressHandler")]
pub type RequestProgressHandler =
    blocks::EscBlock<fn(request: &Request, fraction_completed: f64, error: Option<&ns::Error>)>;

#[objc::protocol(VNRequestProgressProviding)]
pub trait RequestProgressProviding: objc::Obj {
    #[objc::msg_send(progressHandler)]
    fn progress_handler(&self) -> Option<arc::R<RequestProgressHandler>>;

    #[objc::msg_send(setProgressHandler:)]
    fn set_progress_handler(&mut self, handler: &mut RequestProgressHandler);

    #[objc::msg_send(indeterminate)]
    fn indeterminate(&self) -> bool;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_DETECT_HORIZON_REQUEST: &'static objc::Class<DetectHorizonRequest>;
}
