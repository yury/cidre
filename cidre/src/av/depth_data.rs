use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "AVDepthData")]
    /// Disparity/depth map and metadata captured by AVFoundation.
    pub DepthData(ns::Id)
);
