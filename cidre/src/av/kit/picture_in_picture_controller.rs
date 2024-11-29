use crate::{api, arc, av, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVPictureInPictureController")]
    pub PipController(ns::Id)
);

impl arc::A<PipController> {
    #[objc::msg_send(initWithContentSource:)]
    #[api::available(macos = 12.0, ios = 15.0, tvos = 15.0, visionos = 1.0)]
    pub fn init_with_content_src(
        self,
        content_src: &av::PipControllerContentSrc,
    ) -> arc::R<PipController>;

    #[objc::msg_send(initWithPlayerLayer:)]
    pub fn init_with_player_layer(
        self,
        player_layer: &av::PlayerLayer,
    ) -> Option<arc::R<PipController>>;
}

impl PipController {
    #[api::available(macos = 10.15, ios = 9.0, tvos = 15.0, visionos = 1.0)]
    crate::define_cls!(AV_PICTURE_IN_PICTURE_CONTROLLER);

    #[objc::msg_send(isPictureInPictureSupported)]
    pub fn is_pip_supported() -> bool;

    #[api::available(macos = 12.0, ios = 15.0, tvos = 15.0, visionos = 1.0)]
    pub fn with_content_src(content_src: &av::PipControllerContentSrc) -> arc::R<Self> {
        Self::alloc().init_with_content_src(content_src)
    }

    pub fn with_player_layer(player_layer: &av::PlayerLayer) -> Option<arc::R<Self>> {
        Self::alloc().init_with_player_layer(player_layer)
    }

    #[objc::msg_send(contentSource)]
    #[api::available(macos = 12.0, ios = 15.0, tvos = 15.0, visionos = 1.0)]
    pub fn content_src(&self) -> Option<arc::R<av::PipControllerContentSrc>>;

    #[objc::msg_send(setContentSource:)]
    #[api::available(macos = 12.0, ios = 15.0, tvos = 15.0, visionos = 1.0)]
    pub fn set_content_src(&mut self, value: Option<&av::PipControllerContentSrc>);

    #[objc::msg_send(playerLayer)]
    pub fn player_layer(&self) -> arc::R<av::PlayerLayer>;

    #[objc::msg_send(startPictureInPicture)]
    pub fn start_pip(&mut self);

    #[objc::msg_send(stopPictureInPicture)]
    pub fn stop_pip(&mut self);

    #[objc::msg_send(isPictureInPicturePossible)]
    pub fn is_pip_possible(&self) -> bool;

    #[objc::msg_send(isPictureInPictureActive)]
    pub fn is_pip_active(&self) -> bool;

    #[objc::msg_send(isPictureInPictureSuspended)]
    pub fn is_pip_suspended(&self) -> bool;

    #[objc::msg_send(canStopPictureInPicture)]
    #[api::available(tvos = 14.0)]
    pub fn can_stop_pip(&self) -> bool;

    #[objc::msg_send(requiresLinearPlayback)]
    pub fn requires_linear_playback(&self) -> bool;

    #[objc::msg_send(setRequiresLinearPlayback:)]
    pub fn set_requires_linear_playback(&mut self, val: bool);

    #[objc::msg_send(canStartPictureInPictureAutomaticallyFromInline)]
    #[api::available(ios = 14.0, visionos = 1.0)]
    pub fn can_start_pip_automatically_from_inline(&self) -> bool;

    #[objc::msg_send(setCanStartPictureInPictureAutomaticallyFromInline:)]
    #[api::available(ios = 14.0, visionos = 1.0)]
    pub fn set_can_start_pip_automatically_from_inline(&mut self, val: bool);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: PipControllerDelegate>(&mut self, val: Option<&D>);
}

define_obj_type!(pub AnyDelegate(ns::Id));

impl PipControllerDelegate for AnyDelegate {}

define_obj_type!(
    #[doc(alias = "AVPictureInPictureControllerContentSource")]
    pub PipControllerContentSrc(ns::Id)
);

impl arc::A<PipControllerContentSrc> {
    #[objc::msg_send(initWithPlayerLayer:)]
    #[api::available(macos = 10.15, ios = 9.0, tvos = 14.0, visionos = 1.0)]
    pub fn init_with_player_layer(
        self,
        player_layer: &av::PlayerLayer,
    ) -> arc::R<PipControllerContentSrc>;
}

impl PipControllerContentSrc {
    #[api::available(macos = 10.15, ios = 9.0, tvos = 14.0, visionos = 1.0)]
    crate::define_cls!(AV_PICTURE_IN_PICTURE_CONTROLLER_CONTENT_SRC);

    #[api::available(macos = 10.15, ios = 9.0, tvos = 14.0, visionos = 1.0)]
    pub fn with_player_layer(player_layer: &av::PlayerLayer) -> arc::R<Self> {
        Self::alloc().init_with_player_layer(player_layer)
    }

    #[objc::msg_send(playerLayer)]
    pub fn player_layer(&self) -> Option<arc::R<av::PlayerLayer>>;
}

#[objc::protocol(AVPictureInPictureControllerDelegate)]
pub trait PipControllerDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(pictureInPictureControllerWillStartPictureInPicture:)]
    fn pip_controller_will_start_pip(&mut self, ctrl: &av::PipController);

    #[objc::optional]
    #[objc::msg_send(pictureInPictureControllerDidStartPictureInPicture:)]
    fn pip_controller_did_start_pip(&mut self, ctrl: &av::PipController);

    #[objc::optional]
    #[objc::msg_send(pictureInPictureController:failedToStartPictureInPictureWithError:)]
    fn pip_controller_did_failed_to_start_pip(&mut self, ctrl: &av::PipController, err: &ns::Error);

    #[objc::optional]
    #[objc::msg_send(pictureInPictureControllerWillStopPictureInPicture:)]
    fn pip_controller_will_stop_pip(&mut self, ctrl: &av::PipController);

    #[objc::optional]
    #[objc::msg_send(pictureInPictureControllerDidStopPictureInPicture:)]
    fn pip_controller_did_stop_pip(&mut self, ctrl: &av::PipController);
}

#[link(name = "av_kit", kind = "static")]
extern "C" {
    static AV_PICTURE_IN_PICTURE_CONTROLLER: &'static objc::Class<PipController>;
    static AV_PICTURE_IN_PICTURE_CONTROLLER_CONTENT_SRC:
        &'static objc::Class<PipControllerContentSrc>;
}

#[cfg(test)]
mod tests {
    use crate::av;
    #[test]
    fn basics() {
        assert!(av::PipController::is_pip_supported());
    }
}
