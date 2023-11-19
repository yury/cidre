use crate::{define_obj_type, ns};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    Unknown = 0,
    ReadyToPlay = 1,
    Failed = 2,
}

define_obj_type!(pub Item(ns::Id));

impl ns::NotificationName {
    /// A notification the system posts when a player item’s time changes discontinuously.
    ///
    /// The notification’s object is the player item.
    ///
    /// The system may post this notification on a thread other than the one you use to register the observer.
    pub fn av_player_item_time_jumped() -> &'static Self {
        unsafe { AVPlayerItemTimeJumpedNotification }
    }

    /// A notification the system posts when a player item plays to its end time.
    ///
    /// The notification’s object is the item that finished playing.
    ///
    /// The system may post this notification on a thread other than the one you use to register the observer.
    pub fn av_player_item_did_play_to_end_time() -> &'static Self {
        unsafe { AVPlayerItemDidPlayToEndTimeNotification }
    }

    /// A notification that the system posts when a player item fails to play to its end time.
    ///
    /// The notification’s object is the player item that finished playing.
    ///
    /// The system may post this notification on a thread other than the one you use to register the observer.
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
    pub fn av_player_item_playback_stalled() -> &'static Self {
        unsafe { AVPlayerItemPlaybackStalledNotification }
    }

    /// A notification the system posts when a player item adds a new entry to its access log.
    ///
    /// The notification’s object is the player item.
    ///
    /// # Important
    /// The system may post this notification on a thread other than the one you use to register the observer.
    pub fn av_player_item_new_access_log_entry() -> &'static Self {
        unsafe { AVPlayerItemNewAccessLogEntryNotification }
    }

    /// A notification the system posts when a player item adds a new entry to its error log.
    ///
    /// The notification’s object is the player item
    ///
    /// The system may post this notification on a thread other than the one you use to register the observer.
    pub fn av_player_item_error_access_log_entry() -> &'static Self {
        unsafe { AVPlayerItemNewErrorLogEntryNotification }
    }

    /// A notification the player item posts when its offset from the live time changes.
    ///
    /// Register to observe notifications of this type to observe changes to the value of the recommended_time_offset_from_live property
    pub fn av_player_item_recommended_time_offset_from_live_did_change() -> &'static Self {
        unsafe { AVPlayerItemRecommendedTimeOffsetFromLiveDidChangeNotification }
    }

    /// A notification the player item posts when its media selection changes.
    pub fn av_player_item_media_selection_did_change() -> &'static Self {
        unsafe { AVPlayerItemMediaSelectionDidChangeNotification }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
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
