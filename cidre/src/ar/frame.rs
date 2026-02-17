use crate::{ar, arc, cv, define_obj_type, ns, objc};

#[doc(alias = "ARSegmentationClass")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum SegmentationClass {
    /// Pixel has not been classified.
    None = 0,
    /// Pixel has been classified as person.
    Person = 255,
}

#[doc(alias = "ARWorldMappingStatus")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum WorldMappingStatus {
    /// World mapping is not available.
    NotAvailable,
    /// Mapping is available but limited for relocalization.
    Limited,
    /// Mapping is extending with user motion.
    Extending,
    /// Visible area is adequately mapped.
    Mapped,
}

define_obj_type!(
    #[doc(alias = "ARFrame")]
    /// Snapshot of everything tracked for a given moment in time.
    pub Frame(ns::Id)
);

impl Frame {
    /// Timestamp identifying the frame.
    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> ns::TimeInterval;

    /// The frame's captured image.
    #[objc::msg_send(capturedImage)]
    pub fn captured_image(&self) -> &cv::PixelBuf;

    /// Camera used to capture this frame.
    #[objc::msg_send(camera)]
    pub fn camera(&self) -> arc::R<ar::Camera>;

    /// Anchors currently in the scene.
    #[objc::msg_send(anchors)]
    pub fn anchors(&self) -> arc::R<ns::Array<ar::Anchor>>;

    /// Light estimate for the current scene, when available.
    #[objc::msg_send(lightEstimate)]
    pub fn light_estimate(&self) -> Option<arc::R<ar::LightEstimate>>;

    /// Feature points in the scene with respect to the frame origin.
    #[objc::msg_send(rawFeaturePoints)]
    pub fn raw_feature_points(&self) -> Option<arc::R<ar::PointCloud>>;

    /// World-mapping status for the currently visible area.
    #[objc::msg_send(worldMappingStatus)]
    #[objc::available(ios = 12.0)]
    pub fn world_mapping_status(&self) -> WorldMappingStatus;

    /// A buffer containing per-pixel semantic class labels.
    #[objc::msg_send(segmentationBuffer)]
    #[objc::available(ios = 13.0)]
    pub fn segmentation_buf(&self) -> Option<&cv::PixelBuf>;

    /// Scene depth data.
    #[objc::msg_send(sceneDepth)]
    #[objc::available(ios = 14.0)]
    pub fn scene_depth(&self) -> Option<arc::R<ar::DepthData>>;

    /// Scene depth data smoothed for temporal consistency.
    #[objc::msg_send(smoothedSceneDepth)]
    #[objc::available(ios = 14.0)]
    pub fn smoothed_scene_depth(&self) -> Option<arc::R<ar::DepthData>>;
}
