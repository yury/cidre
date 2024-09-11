use crate::{arc, av, cg, define_obj_type, objc, ui};

#[doc(alias = "AVPlayerViewControllerSkippingBehavior")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum PlayerViewControllerSkippingBehavior {
    Default = 0,
    SkipItem,
}

define_obj_type!(
    #[doc(alias = "AVPlayerViewController")]
    pub PlayerViewController(ui::ViewController),
    AV_PLAYER_VIEW_CONTROLLER
);

impl PlayerViewController {
    /// The player from which to source the media content for the view controller.
    #[objc::msg_send(player)]
    pub fn player(&self) -> arc::R<av::Player>;

    #[objc::msg_send(setPlayer:)]
    pub fn set_player(&mut self, val: Option<&av::Player>);

    /// Whether or not the receiver shows playback controls. Default is true.
    #[objc::msg_send(showsPlaybackControls)]
    pub fn shows_playback_controls(&self) -> bool;

    #[objc::msg_send(setShowsPlaybackControls:)]
    pub fn set_shows_playback_controls(&self, val: bool);

    #[objc::msg_send(showsTimecodes)]
    #[objc::available(ios = 13.0)]
    pub fn shows_timecodes(&self) -> bool;

    #[objc::msg_send(setShowsTimecodes:)]
    #[objc::available(ios = 13.0)]
    pub fn set_shows_timecodes(&mut self, val: bool);

    #[objc::msg_send(videoGravity)]
    pub fn video_gravity(&self) -> arc::R<av::LayerVideoGravity>;

    #[cfg(target_os = "ios")]
    #[objc::msg_send(isReadyForDisplay)]
    pub fn is_ready_for_display(&self) -> bool;

    #[objc::msg_send(videoBounds)]
    pub fn video_bounds(&self) -> cg::Rect;

    #[objc::msg_send(contentOverlayView)]
    pub fn content_overlay_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(allowsPictureInPicturePlayback)]
    #[objc::available(ios = 9.0, tvos = 14.0, visionos = 1.0)]
    pub fn allows_pip_playback(&self) -> bool;

    #[objc::msg_send(setAllowsPictureInPicturePlayback:)]
    #[objc::available(ios = 9.0, tvos = 14.0, visionos = 1.0)]
    pub fn set_allows_pip_playback(&mut self, val: bool);

    #[objc::msg_send(videoFrameAnalysisTypes)]
    #[objc::available(ios = 17.0, maccatalyst = 18.0)]
    pub fn video_frame_analysis_types(&self) -> av::VideoFrameAnalysisType;

    #[objc::msg_send(setVideoFrameAnalysisTypes:)]
    #[objc::available(ios = 17.0, maccatalyst = 18.0)]
    pub fn set_video_frame_analysis_types(&mut self, val: av::VideoFrameAnalysisType);

    #[objc::msg_send(canStartPictureInPictureAutomaticallyFromInline)]
    #[objc::available(ios = 14.2, visionos = 1.0)]
    pub fn can_start_pip_automatically_from_inline(&self) -> bool;

    #[objc::msg_send(setCanStartPictureInPictureAutomaticallyFromInline:)]
    #[objc::available(ios = 14.2, visionos = 1.0)]
    pub fn set_can_start_pip_automatically_from_inline(&mut self, val: bool);

    #[objc::msg_send(updatesNowPlayingInfoCenter)]
    #[objc::available(ios = 10.0, visionos = 1.0)]
    pub fn updates_now_playing_info_center(&self) -> bool;

    #[objc::msg_send(setUpdatesNowPlayingInfoCenter:)]
    #[objc::available(ios = 10.0, visionos = 1.0)]
    pub fn set_updates_now_playing_info_center(&mut self, val: bool);

    #[objc::msg_send(entersFullScreenWhenPlaybackBegins)]
    #[objc::available(ios = 11.0, visionos = 1.0)]
    pub fn enters_full_screen_when_playback_begins(&self) -> bool;

    #[objc::msg_send(setEntersFullScreenWhenPlaybackBegins:)]
    #[objc::available(ios = 11.0, visionos = 1.0)]
    pub fn set_enters_full_screen_when_playback_begins(&mut self, val: bool);

    #[objc::msg_send(exitsFullScreenWhenPlaybackEnds)]
    #[objc::available(ios = 11.0, visionos = 1.0)]
    pub fn exits_full_screen_when_playback_begins(&self) -> bool;

    #[objc::msg_send(setExitsFullScreenWhenPlaybackBegins:)]
    #[objc::available(ios = 11.0, visionos = 1.0)]
    pub fn set_exits_full_screen_when_playback_begins(&mut self, val: bool);

    /// Disables certain user operations, such as scanning, skipping, and scrubbing.
    #[objc::msg_send(requiresLinearPlayback)]
    #[objc::available(ios = 11.0, visionos = 1.0)]
    pub fn requires_linear_playback(&self) -> bool;

    /// Disables certain user operations, such as scanning, skipping, and scrubbing.
    #[objc::msg_send(setRequiresLinearPlayback:)]
    #[objc::available(ios = 11.0, visionos = 1.0)]
    pub fn set_requires_linear_playback(&mut self, val: bool);
}

#[link(name = "av_kit", kind = "static")]
extern "C" {
    static AV_PLAYER_VIEW_CONTROLLER: &'static objc::Class<PlayerViewController>;
}
