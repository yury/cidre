use crate::{api, arc, av, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "AVPictureInPictureVideoCallViewController")]
    pub PipViewCallController(ui::ViewController),
    sym AVPictureInPictureVideoCallViewController,
    #[api::available(ios = 15.0, visionos = 1.0)]
);

/// VideoCallSupport
impl av::PipControllerContentSrc {
    #[objc::init(initWithActiveVideoCallSourceView:contentViewController:)]
    #[api::available(ios = 15.0, visionos = 1.0)]
    pub fn init_with_active_video_call_src_view(
        self,
        src_view: &ui::View,
        content_vc: &av::PipViewCallController,
    ) -> arc::R<av::PipControllerContentSrc>;

    #[api::available(ios = 15.0, visionos = 1.0)]
    pub fn with_active_video_call_src_view(
        src_view: &ui::View,
        content_vc: &av::PipViewCallController,
    ) -> arc::R<Self> {
        Self::alloc().init_with_active_video_call_src_view(src_view, content_vc)
    }

    #[objc::msg_send(activeVideoCallSourceView)]
    #[api::available(ios = 15.0, visionos = 1.0)]
    pub fn active_video_call_src_view(&self) -> arc::R<ui::View>;

    #[objc::msg_send(activeVideoCallContentViewController)]
    #[api::available(ios = 15.0, visionos = 1.0)]
    pub fn active_video_call_content_vc(&self) -> arc::R<av::PipViewCallController>;
}
