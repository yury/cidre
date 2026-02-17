use crate::{cv, define_obj_type, ns, objc};

#[doc(alias = "ARConfidenceLevel")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum ConfidenceLevel {
    /// Low confidence for the associated depth value.
    Low,
    /// Medium confidence for the associated depth value.
    Medium,
    /// High confidence for the associated depth value.
    High,
}

define_obj_type!(
    #[doc(alias = "ARDepthData")]
    /// A container for depth data and associated confidence.
    pub DepthData(ns::Id)
);

impl DepthData {
    /// Per-pixel depth data in meters.
    #[objc::msg_send(depthMap)]
    #[objc::available(ios = 14.0)]
    pub fn depth_map(&self) -> &cv::PixelBuf;

    /// Confidence level per depth pixel.
    #[objc::msg_send(confidenceMap)]
    #[objc::available(ios = 14.0)]
    pub fn confidence_map(&self) -> Option<&cv::PixelBuf>;
}
