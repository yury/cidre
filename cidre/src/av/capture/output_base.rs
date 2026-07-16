use crate::{arc, av, define_obj_type, ns, objc};

#[cfg(feature = "cg")]
use crate::cg;

define_obj_type!(
    #[doc(alias = "AVCaptureOutput")]
    pub Output(ns::Id)
);

impl Output {
    #[cfg(not(target_os = "visionos"))]
    #[objc::msg_send(connections)]
    pub fn connections(&self) -> arc::R<ns::Array<av::CaptureConnection>>;

    #[objc::msg_send(connectionWithMediaType:)]
    pub fn connection_with_media_type(
        &self,
        media_type: av::MediaType,
    ) -> Option<arc::R<av::CaptureConnection>>;

    #[cfg(not(target_os = "visionos"))]
    #[objc::msg_send(transformedMetadataObjectForMetadataObject:connection:)]
    pub fn transformed_metadata_obj_for_metadata_obj(
        &self,
        metadata_object: &av::MetadataObj,
        connection: &av::CaptureConnection,
    ) -> Option<arc::R<av::MetadataObj>>;

    #[cfg(feature = "cg")]
    #[objc::msg_send(metadataOutputRectOfInterestForRect:)]
    pub fn metadata_output_rect_of_interest_for_rect(
        &self,
        rect_in_output_coordinates: cg::Rect,
    ) -> cg::Rect;

    #[cfg(all(feature = "cg", not(target_os = "visionos")))]
    #[objc::msg_send(metadataOutputRectOfInterestForRect:)]
    pub fn rect_for_metadata_output_rect_of_interest(
        &self,
        rect_in_metadata_output_coordinates: cg::Rect,
    ) -> cg::Rect;

    #[objc::msg_send(isDeferredStartSupported)]
    #[objc::available(macos = 26.0, maccatalyst = 26.0, ios = 26.0, tvos = 26.0)]
    pub fn is_deferred_start_supported(&self) -> bool;

    #[objc::msg_send(isDeferredStartEnabled)]
    #[objc::available(macos = 26.0, maccatalyst = 26.0, ios = 26.0, tvos = 26.0)]
    pub fn is_deferred_start_enabled(&self) -> bool;

    #[objc::msg_send(setDeferredStartEnabled:)]
    #[objc::available(macos = 26.0, maccatalyst = 26.0, ios = 26.0, tvos = 26.0)]
    pub fn set_deferred_start_enabled(&mut self, val: bool);
}

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum DataDroppedReason {
    None = 0,
    LateData = 1,
    OutOfBuffers = 2,
    Discontinuity = 3,
}
