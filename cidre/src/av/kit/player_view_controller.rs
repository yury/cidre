use crate::{arc, av, cg, define_obj_type, ns, objc, ui};

#[cfg(target_os = "visionos")]
use crate::blocks;

#[cfg(target_os = "tvos")]
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

    #[objc::msg_send(appliesPreferredDisplayCriteriaAutomatically)]
    #[objc::available(tvos = 11.2, visionos = 1.0)]
    pub fn applies_preferred_display_criteria_automatically(&self) -> bool;

    #[objc::msg_send(setAppliesPreferredDisplayCriteriaAutomatically:)]
    #[objc::available(tvos = 11.2, visionos = 1.0)]
    pub fn set_applies_preferred_display_criteria_automatically(&mut self, val: bool);

    #[objc::msg_send(pixelBufferAttributes)]
    #[objc::available(ios = 9.0, tvos = 11.2)]
    pub fn pixel_buf_attrs(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(delegate)]
    #[objc::available(ios = 9.0, visionos = 1.0)]
    pub fn delegate(&self) -> Option<arc::R<AnyPlayerViewControllerDelegate>>;

    #[objc::msg_send(setDelegate:)]
    #[objc::available(ios = 9.0, visionos = 1.0)]
    pub fn set_delegate<D: PlayerViewControllerDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(speeds)]
    #[objc::available(ios = 16.0, tvos = 16.0, visionos = 1.0)]
    pub fn speeds(&self) -> arc::R<ns::Array<av::PlaybackSpeed>>;

    #[objc::msg_send(setSpeeds:)]
    #[objc::available(ios = 16.0, tvos = 16.0, visionos = 1.0)]
    pub fn set_speeds(&mut self, val: &ns::Array<av::PlaybackSpeed>);

    #[objc::msg_send(selectedSpeed)]
    #[objc::available(ios = 16.0, tvos = 16.0, visionos = 1.0)]
    pub fn selected_speed(&self) -> Option<arc::R<av::PlaybackSpeed>>;

    /// Calls to selectSpeed with av::PlaybackSpeeds not contained within the speeds property array will be ignored.
    #[objc::msg_send(selectSpeed:)]
    #[objc::available(ios = 16.0, tvos = 16.0, visionos = 1.0)]
    pub fn select_speed(&mut self, val: &av::PlaybackSpeed);

    // TODO: unobscuredContentGuide

    /// An optional array of BCP 47 language codes to filter the subtitle options presented to the user.
    ///
    /// BCP47 language codes; None allows all languages
    #[objc::msg_send(allowedSubtitleOptionLanguages)]
    #[objc::available(tvos = 9.0)]
    pub fn allowed_subtitle_option_langs(&self) -> Option<ns::Array<ns::String>>;

    #[objc::msg_send(setAllowedSubtitleOptionLanguages:)]
    #[objc::available(tvos = 9.0)]
    pub fn set_allowed_subtitle_option_langs(&mut self, val: Option<ns::Array<ns::String>>);

    #[objc::msg_send(requiresFullSubtitles)]
    #[objc::available(tvos = 9.0)]
    pub fn requires_full_subtitles(&self) -> bool;

    #[objc::msg_send(setRequiresFullSubtitles:)]
    #[objc::available(tvos = 9.0)]
    pub fn set_requires_full_subtitles(&mut self, val: bool);

    #[objc::msg_send(skippingBehavior)]
    #[objc::available(tvos = 10.0)]
    pub fn skipping_behavior(&self) -> av::PlayerViewControllerSkippingBehavior;

    #[objc::msg_send(setSkippingBehavior:)]
    #[objc::available(tvos = 10.0)]
    pub fn set_skipping_behavior(&mut self, val: av::PlayerViewControllerSkippingBehavior);

    #[objc::msg_send(isSkipForwardEnabled)]
    #[objc::available(tvos = 10.0)]
    pub fn is_skip_forward_enabled(&self) -> bool;

    #[objc::msg_send(setSkipForwardEnabled:)]
    #[objc::available(tvos = 10.0)]
    pub fn set_skip_forward_enabled(&mut self, val: bool);

    #[objc::msg_send(isSkipBackwardEnabled)]
    #[objc::available(tvos = 10.0)]
    pub fn is_skip_backward_enabled(&self) -> bool;

    #[objc::msg_send(setSkipBackwardEnabled:)]
    #[objc::available(tvos = 10.0)]
    pub fn set_skip_backward_enabled(&mut self, val: bool);

    #[objc::msg_send(playbackControlsIncludeTransportBar)]
    #[objc::available(tvos = 11.0)]
    pub fn playback_controls_include_transport_bar(&self) -> bool;

    #[objc::msg_send(setPlaybackControlsIncludeTransportBar:)]
    #[objc::available(tvos = 11.0)]
    pub fn set_playback_controls_include_transport_bar(&mut self, val: bool);

    #[objc::msg_send(playbackControlsIncludeInfoViews)]
    #[objc::available(tvos = 11.0)]
    pub fn playback_controls_include_info_views(&self) -> bool;

    #[objc::msg_send(setPlaybackControlsIncludeInfoViews:)]
    #[objc::available(tvos = 11.0)]
    pub fn set_playback_controls_include_info_views(&mut self, val: bool);

    #[objc::msg_send(transportBarIncludesTitleView)]
    #[objc::available(tvos = 15.0)]
    pub fn transport_bar_includes_title_view(&self) -> bool;

    #[objc::msg_send(setTransportBarIncludesTitleView:)]
    #[objc::available(tvos = 15.0)]
    pub fn set_transport_bar_includes_title_view(&mut self, val: bool);

    #[objc::msg_send(customOverlayViewController)]
    #[objc::available(tvos = 13.0)]
    pub fn custom_overlay_view_controller(&self) -> Option<arc::R<ui::ViewController>>;

    #[objc::msg_send(setCustomOverlayViewController:)]
    #[objc::available(tvos = 13.0)]
    pub fn set_custom_overlay_view_controller(&mut self, val: Option<&ui::ViewController>);

    // TODO: transportBarCustomMenuItems

    #[objc::msg_send(requiresMonoscopicViewingMode)]
    #[objc::available(visionos = 1.0)]
    pub fn requires_monoscopic_viewing_mode(&self) -> bool;

    /// Whether or not monoscopic is the only permitted viewing mode. Default is false
    #[objc::msg_send(setRequiresMonoscopicViewingMode:)]
    #[objc::available(visionos = 1.0)]
    pub fn set_requires_monoscopic_viewing_mode(&mut self, val: bool);

    // TODO: contextualActions
    // TODO: contextualActionsInfoView
    // TODO: contextualActionsPreviewImage

    #[objc::msg_send(customInfoViewControllers)]
    #[objc::available(tvos = 15.0, visionos = 1.0)]
    pub fn custom_info_view_controllers(&self) -> arc::R<ns::Array<ui::ViewController>>;

    #[objc::msg_send(setCustomInfoViewControllers:)]
    #[objc::available(tvos = 15.0, visionos = 1.0)]
    pub fn set_custom_info_view_controllers(&mut self, val: &ns::Array<ui::ViewController>);

    #[objc::msg_send(canBeginTrimming)]
    #[objc::available(visionos = 1.0)]
    pub fn can_begin_trimming(&self) -> bool;

    #[objc::msg_send(beginTrimmingWithCompletionHandler:)]
    #[objc::available(visionos = 1.0)]
    pub fn begin_trimming_with_ch(&self, block: Option<&mut blocks::EscBlock<fn(success: bool)>>);
}

#[objc::protocol(AVPlayerViewControllerDelegate)]
pub trait PlayerViewControllerDelegate: objc::Obj {
    // #[objc::optional]
    // #[objc::msg_send(playerViewController:willBeginFullScreenPresentationWithAnimationCoordinator:)]
    // pub fn pv_controller()
}

define_obj_type!(
    pub AnyPlayerViewControllerDelegate(ns::Id)
);

#[link(name = "av_kit", kind = "static")]
unsafe extern "C" {
    static AV_PLAYER_VIEW_CONTROLLER: &'static objc::Class<PlayerViewController>;
}
