use crate::{arc, av, cg, cm, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "AVPlayerItemStatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    Unknown = 0,
    ReadyToPlay = 1,
    Failed = 2,
}

define_obj_type!(
    #[doc(alias = "AVPlayerItem")]
    pub Item(ns::Id)
);

impl Item {
    define_cls!(AV_PLAYER_ITEM);

    #[objc::msg_send(playerItemWithURL:)]
    pub fn with_url(val: &ns::Url) -> arc::R<Self>;

    #[objc::msg_send(playerItemWithAsset:)]
    pub fn with_asset(val: &av::Asset) -> arc::R<Self>;

    #[objc::msg_send(playerItemWithAsset:automaticallyLoadedAssetKeys:)]
    pub fn with_asset_auto_loaded_keys(
        val: &av::Asset,
        asset_keys: Option<&ns::Array<ns::String>>,
    ) -> arc::R<Self>;

    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::R<Self>;

    #[objc::msg_send(status)]
    pub fn status(&self) -> Status;

    /// If the receiver's status is AVPlayerItemStatusFailed, this describes the error that caused the failure.
    ///
    /// The value of this property is an NSError that describes what caused the receiver to no longer be able to be played.
    /// If the receiver's status is not AVPlayerItemStatusFailed, the value of this property is nil.
    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<arc::R<ns::Error>>;
}

/// AVPlayerItemInspection
impl Item {
    /// Accessor for underlying av::Asset.
    #[objc::msg_send(asset)]
    pub fn asset(&self) -> arc::R<av::Asset>;

    #[objc::msg_send(tracks)]
    pub fn tracks(&self) -> arc::R<ns::Array<av::PlayerItemTrack>>;

    #[objc::msg_send(duration)]
    pub fn duration(&self) -> cm::Time;

    #[objc::msg_send(presentationSize)]
    pub fn presentation_size(&self) -> cg::Size;

    #[objc::msg_send(automaticallyLoadedAssetKeys)]
    pub fn auto_loaded_asset_keys(&self) -> arc::R<ns::Array<ns::String>>;
}

/// AVPlayerItemRateAndSteppingSupport
impl Item {
    #[objc::msg_send(canPlayFastForward)]
    pub fn can_play_fast_forward(&self) -> bool;

    #[objc::msg_send(canPlaySlowForward)]
    pub fn can_play_slow_forward(&self) -> bool;

    #[objc::msg_send(canPlayReverse)]
    pub fn can_play_reverse(&self) -> bool;

    #[objc::msg_send(canPlaySlowReverse)]
    pub fn can_play_slow_reverse(&self) -> bool;

    #[objc::msg_send(canPlayFastReverse)]
    pub fn can_play_fast_reverse(&self) -> bool;

    #[objc::msg_send(canStepForward)]
    pub fn can_step_forward(&self) -> bool;

    #[objc::msg_send(canStepBackward)]
    pub fn can_step_backward(&self) -> bool;

    #[objc::msg_send(configuredTimeOffsetFromLive)]
    pub fn configured_time_offset_from_live(&self) -> bool;

    #[objc::msg_send(setConfiguredTimeOffsetFromLive:)]
    pub fn set_configured_time_offset_from_live(&mut self, val: cm::Time);

    #[objc::msg_send(recommendedTimeOffsetFromLive)]
    pub fn recommended_time_offset_from_live(&self) -> cm::Time;

    #[objc::msg_send(automaticallyPreservesTimeOffsetFromLive)]
    pub fn automatically_preserves_time_offset_from_live(&self) -> bool;

    #[objc::msg_send(setAutomaticallyPreservesTimeOffsetFromLive:)]
    pub fn set_automatically_preserves_time_offset_from_live(&mut self, val: bool);
}

/// AVPlayerItemTimeControl
impl Item {
    #[objc::msg_send(currentTime)]
    pub fn current_time(&self) -> cm::Time;

    #[objc::msg_send(forwardPlaybackEndTime)]
    pub fn forward_playback_end_time(&self) -> cm::Time;

    #[objc::msg_send(setForwardPlaybackEndTime:)]
    pub fn set_forward_playback_end_time(&mut self, val: cm::Time);

    #[objc::msg_send(reversePlaybackEndTime)]
    pub fn reverse_playback_end_time(&self) -> cm::Time;

    #[objc::msg_send(setReversePlaybackEndTime:)]
    pub fn set_reverse_playback_end_time(&mut self, val: cm::Time);

    #[objc::msg_send(seekableTimeRanges)]
    pub fn seekable_time_ranges(&self) -> arc::R<ns::Array<ns::Value>>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(seekToTime:completionHandler:)]
    pub unsafe fn seek_to_time_ch_throws(
        &mut self,
        val: cm::Time,
        block: Option<&mut blocks::SendBlock<fn(finished: bool)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn seek_to_time_ch<'ear>(
        &mut self,
        val: cm::Time,
        block: Option<&mut blocks::SendBlock<fn(finished: bool)>>,
    ) -> ns::ExResult<'ear> {
        unsafe { ns::try_catch(|| self.seek_to_time_ch_throws(val, block)) }
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(seekToTime:toleranceBefore:toleranceAfter:completionHandler:)]
    pub unsafe fn seek_to_time_with_tolerance_ch_throws(
        &mut self,
        val: cm::Time,
        tolerance_befor: cm::Time,
        tolerance_after: cm::Time,
        block: Option<&mut blocks::SendBlock<fn(finished: bool)>>,
    );

    #[cfg(feature = "blocks")]
    pub fn seek_to_time_with_tolerance_ch<'ear>(
        &mut self,
        val: cm::Time,
        tolerance_befor: cm::Time,
        tolerance_after: cm::Time,
        block: Option<&mut blocks::SendBlock<fn(finished: bool)>>,
    ) -> ns::ExResult<'ear> {
        unsafe {
            ns::try_catch(|| {
                self.seek_to_time_with_tolerance_ch_throws(
                    val,
                    tolerance_befor,
                    tolerance_after,
                    block,
                )
            })
        }
    }

    #[objc::msg_send(cancelPendingSeeks)]
    pub fn cancel_pending_seeks(&mut self);

    #[objc::msg_send(currentDate)]
    pub fn current_date(&self) -> Option<arc::R<ns::Date>>;

    #[objc::msg_send(stepByCount:)]
    pub fn step_by_count(&mut self, steps_count: ns::Integer);

    #[objc::msg_send(timebase)]
    pub fn timebase(&self) -> &cm::Timebase;

    #[objc::msg_send(timebase)]
    pub fn timebase_mut(&mut self) -> &mut cm::Timebase;
}

/// AVPlayerItemVisualPresentation
impl Item {}

/// AVPlayerItemAudioProcessing
impl Item {
    #[objc::msg_send(audioTimePitchAlgorithm)]
    pub fn audio_time_pitch_algorithm(&self) -> Option<arc::R<av::AudioTimePitchAlgorithm>>;

    #[objc::msg_send(setAudioTimePitchAlgorithm:)]
    pub fn set_audio_time_pitch_algorithm(&mut self, val: Option<&av::AudioTimePitchAlgorithm>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::msg_send(allowedAudioSpatializationFormats)]
    pub fn allowed_audio_spatialization_formats(&self) -> av::AudioSpatializationFormats;

    #[cfg(not(target_os = "watchos"))]
    #[objc::msg_send(setAllowedAudioSpatializationFormats:)]
    pub fn set_allowed_audio_spatialization_formats(&mut self, val: av::AudioSpatializationFormats);

    #[objc::msg_send(audioMix)]
    pub fn audio_mix(&self) -> Option<arc::R<av::AudioMix>>;

    #[objc::msg_send(setAudioMix:)]
    pub fn set_audio_mix(&mut self, val: Option<&av::AudioMix>);
}

/// AVPlayerItemPlayability
impl Item {
    #[objc::msg_send(loadedTimeRanges)]
    pub fn loaded_time_ranges(&self) -> arc::R<ns::Array<ns::Value>>;

    #[objc::msg_send(isPlaybackLikelyToKeepUp)]
    pub fn is_playback_likely_to_keep_up(&self) -> bool;

    #[objc::msg_send(isPlaybackBufferFull)]
    pub fn is_playback_buffer_full(&self) -> bool;

    #[objc::msg_send(isPlaybackBufferEmpty)]
    pub fn is_playback_buffer_empty(&self) -> bool;

    #[objc::msg_send(canUseNetworkResourcesForLiveStreamingWhilePaused)]
    pub fn can_use_network_resources_for_live_streaming_while_paused(&self) -> bool;

    #[objc::msg_send(setCanUseNetworkResourcesForLiveStreamingWhilePaused:)]
    pub fn set_can_use_network_resources_for_live_streaming_while_paused(&mut self, val: bool);

    #[objc::msg_send(preferredForwardBufferDuration)]
    pub fn preferred_forward_buf_duration(&self) -> ns::TimeInterval;

    #[objc::msg_send(setPreferredForwardBufferDuration:)]
    pub fn set_preferred_forward_buf_duration(&mut self, val: ns::TimeInterval);
}

/// AVPlayerItemOutputs
impl Item {
    /// Adds the specified instance of AVPlayerItemOutput to the receiver's collection of outputs.
    #[objc::msg_send(addOutput:)]
    pub fn add_output(&mut self, output: &av::PlayerItemOutput);

    /// Removes the specified instance of AVPlayerItemOutput from the receiver's collection of outputs.
    #[objc::msg_send(removeOutput:)]
    pub fn remove_output(&mut self, output: &av::PlayerItemOutput);

    /// The collection of associated outputs.
    #[objc::msg_send(outputs)]
    pub fn outputs(&self) -> arc::R<ns::Array<av::PlayerItemOutput>>;
}

impl ns::NotificationName {
    /// A notification the system posts when a player item’s time changes discontinuously.
    ///
    /// The notification’s object is the player item.
    ///
    /// The system may post this notification on a thread other than the one you use to register the observer.
    #[doc(alias = "AVPlayerItemTimeJumpedNotification")]
    #[inline]
    pub fn av_player_item_time_jumped() -> &'static Self {
        unsafe { AVPlayerItemTimeJumpedNotification }
    }

    /// A notification the system posts when a player item plays to its end time.
    ///
    /// The notification’s object is the item that finished playing.
    ///
    /// The system may post this notification on a thread other than the one you use to register the observer.
    #[doc(alias = "AVPlayerItemDidPlayToEndTimeNotification")]
    #[inline]
    pub fn av_player_item_did_play_to_end_time() -> &'static Self {
        unsafe { AVPlayerItemDidPlayToEndTimeNotification }
    }

    /// A notification that the system posts when a player item fails to play to its end time.
    ///
    /// The notification’s object is the player item that finished playing.
    ///
    /// The system may post this notification on a thread other than the one you use to register the observer.
    #[doc(alias = "AVPlayerItemFailedToPlayToEndTimeNotification")]
    #[inline]
    pub fn av_player_item_failed_play_to_end_time() -> &'static Self {
        unsafe { AVPlayerItemFailedToPlayToEndTimeNotification }
    }

    /// A notification the system posts when a player item media doesn’t arrive in time to continue playback.
    ///
    /// The notification’s object is the player item whose playback is unable to continue due to network delays.
    /// Streaming-media playback continues after the player item retrieves a sufficient amount of data.
    /// File-based playback doesn’t continue.
    ///
    /// # Important
    /// The system may post this notification on a thread other than the one you use to register the observer.
    #[doc(alias = "AVPlayerItemPlaybackStalledNotification")]
    #[inline]
    pub fn av_player_item_playback_stalled() -> &'static Self {
        unsafe { AVPlayerItemPlaybackStalledNotification }
    }

    /// A notification the system posts when a player item adds a new entry to its access log.
    ///
    /// The notification’s object is the player item.
    ///
    /// # Important
    /// The system may post this notification on a thread other than the one you use to register the observer.
    #[doc(alias = "AVPlayerItemNewAccessLogEntryNotification")]
    #[inline]
    pub fn av_player_item_new_access_log_entry() -> &'static Self {
        unsafe { AVPlayerItemNewAccessLogEntryNotification }
    }

    /// A notification the system posts when a player item adds a new entry to its error log.
    ///
    /// The notification’s object is the player item
    ///
    /// The system may post this notification on a thread other than the one you use to register the observer.
    #[doc(alias = "AVPlayerItemNewErrorLogEntryNotification")]
    #[inline]
    pub fn av_player_item_error_access_log_entry() -> &'static Self {
        unsafe { AVPlayerItemNewErrorLogEntryNotification }
    }

    /// A notification the player item posts when its offset from the live time changes.
    ///
    /// Register to observe notifications of this type to observe changes to the value of the recommended_time_offset_from_live property
    #[doc(alias = "AVPlayerItemRecommendedTimeOffsetFromLiveDidChangeNotification")]
    #[inline]
    pub fn av_player_item_recommended_time_offset_from_live_did_change() -> &'static Self {
        unsafe { AVPlayerItemRecommendedTimeOffsetFromLiveDidChangeNotification }
    }

    /// A notification the player item posts when its media selection changes.
    #[doc(alias = "AVPlayerItemMediaSelectionDidChangeNotification")]
    #[inline]
    pub fn av_player_item_media_selection_did_change() -> &'static Self {
        unsafe { AVPlayerItemMediaSelectionDidChangeNotification }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {
    static AVPlayerItemTimeJumpedNotification: &'static ns::NotificationName;
    static AVPlayerItemDidPlayToEndTimeNotification: &'static ns::NotificationName;
    static AVPlayerItemFailedToPlayToEndTimeNotification: &'static ns::NotificationName;
    static AVPlayerItemPlaybackStalledNotification: &'static ns::NotificationName;
    static AVPlayerItemNewAccessLogEntryNotification: &'static ns::NotificationName;
    static AVPlayerItemNewErrorLogEntryNotification: &'static ns::NotificationName;
    static AVPlayerItemRecommendedTimeOffsetFromLiveDidChangeNotification:
        &'static ns::NotificationName;
    static AVPlayerItemMediaSelectionDidChangeNotification: &'static ns::NotificationName;
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_PLAYER_ITEM: &'static objc::Class<Item>;
}
