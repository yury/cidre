use crate::{arc, av, ca, cg, define_cls, define_obj_type, objc};

define_obj_type!(
    #[doc(alias = "AVCaptureVideoPreviewLayer")]
    pub VideoPreviewLayer(ca::Layer)
);

impl arc::A<VideoPreviewLayer> {
    #[objc::msg_send(initWithSession:)]
    pub fn init_with_session(self, session: &av::CaptureSession) -> arc::R<VideoPreviewLayer>;

    #[objc::msg_send(initWithSessionWithNoConnection:)]
    pub fn init_with_session_no_connection(
        self,
        session: &av::CaptureSession,
    ) -> arc::R<VideoPreviewLayer>;
}

impl VideoPreviewLayer {
    define_cls!(AV_CAPTURE_VIDEO_PREVIEW_LAYER);

    pub fn with_session(session: &av::CaptureSession) -> arc::R<Self> {
        Self::alloc().init_with_session(session)
    }

    pub fn with_session_no_connection(session: &av::CaptureSession) -> arc::R<Self> {
        Self::alloc().init_with_session_no_connection(session)
    }

    #[objc::msg_send(session)]
    pub fn session(&self) -> Option<&av::CaptureSession>;

    #[objc::msg_send(setSession:)]
    pub fn set_session(&mut self, val: Option<&av::CaptureSession>);

    #[objc::msg_send(setSessionWithNoConnection:)]
    pub fn set_session_no_connection(&mut self, val: &av::CaptureSession);

    #[objc::msg_send(connection)]
    pub fn connection(&self) -> Option<&av::CaptureConnection>;

    #[objc::msg_send(videoGravity)]
    pub fn video_gravity(&self) -> &av::LayerVideoGravity;

    #[objc::msg_send(setVideoGravity:)]
    pub fn set_video_gravity(&mut self, val: &av::LayerVideoGravity);

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isPreviewing)]
    pub fn is_previewing(&self) -> bool;

    #[objc::msg_send(captureDevicePointOfInterestForPoint:)]
    pub fn capture_device_poi_for_point(&self, point: cg::Point) -> cg::Point;

    #[objc::msg_send(pointForCaptureDevicePointOfInterest:)]
    pub fn point_for_capture_device_poi(&self, point: cg::Point) -> cg::Point;

    #[objc::msg_send(metadataOutputRectOfInterestForRect:)]
    pub fn metadata_output_roi_for_rect(&self, rect: cg::Rect) -> cg::Rect;

    #[objc::msg_send(rectForMetadataOutputRectOfInterest:)]
    pub fn rect_for_metadata_output_roi(&self, rect: cg::Rect) -> cg::Rect;

    #[objc::msg_send(transformedMetadataObjectForMetadataObject:)]
    pub fn transformed_metadata_obj_for_metadata_obj(
        &self,
        obj: &av::MetadataObj,
    ) -> Option<arc::R<av::MetadataObj>>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_VIDEO_PREVIEW_LAYER: &'static objc::Class<VideoPreviewLayer>;
}
