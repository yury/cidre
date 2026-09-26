use crate::{api, arc, av, cm, define_cls, define_obj_type, define_opts, ns, objc};

#[cfg(feature = "blocks")]
use crate::{blocks, dispatch};

pub mod item;
pub use item::Item as PlayerItem;
pub use item::Status as ItemStatus;

pub mod item_track;
pub use item_track::ItemTrack as PlayerItemTrack;

pub mod item_output;
pub use item_output::ItemOutput;
pub use item_output::ItemOutputPullDelegate;
pub use item_output::ItemOutputPullDelegateImpl;
pub use item_output::ItemVideoOutput;

pub mod item_sample_buffer_output;

mod looper;
pub use looper::Looper as PlayerLooper;
pub use looper::LooperItemOrdering as PlayerLooperItemOrdering;
pub use looper::LooperStatus as PlayerLooperStatus;

#[doc(alias = "AVPlayerStatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    Unknown = 0,
    ReadyToPlay = 1,
    Failed = 2,
}

#[doc(alias = "AVPlayerTimeControlStatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum TimeControlStatus {
    Paused = 0,
    WaitingToPlayAtSpecifiedRate = 1,
    Playing = 2,
}

#[doc(alias = "AVPlayerActionAtItemEnd")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum ActionAtItemEnd {
    Advance = 0,
    Pause = 1,
    None = 2,
}

define_opts!(
    #[doc(alias = "AVPlayerHDRMode")]
    pub HdrMode(isize)
);

impl HdrMode {
    pub const HLG: Self = Self(0x1);
    pub const HDR10: Self = Self(0x2);
    pub const DOLBY_VISION: Self = Self(0x4);
}

#[doc(alias = "AVPlayerAudiovisualBackgroundPlaybackPolicy")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum AudiovisualBackgroundPlaybackPolicy {
    Automatic = 1,
    Pauses = 2,
    ContinuesIfPossible = 3,
}

define_obj_type!(
    #[doc(alias = "AVPlayer")]
    pub Player(ns::Id)
);

impl Player {
    #[objc::init(initWithURL:)]
    pub fn init_with_url(self, url: &ns::Url) -> arc::R<Player>;

    #[objc::init(initWithPlayerItem:)]
    pub unsafe fn init_with_player_item_throws(self, item: Option<&PlayerItem>) -> arc::R<Player>;

    define_cls!(AV_PLAYER);

    pub fn with_url(url: &ns::Url) -> arc::R<Self> {
        Self::alloc().init_with_url(url)
    }

    pub unsafe fn with_player_item_throws(item: Option<&PlayerItem>) -> arc::R<Self> {
        unsafe { Self::alloc().init_with_player_item_throws(item) }
    }

    pub fn with_player_item<'ear>(item: Option<&PlayerItem>) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe { Self::with_player_item_throws(item) })
    }

    #[objc::msg_send(status)]
    pub fn status(&self) -> Status;

    /// If the receiver's status is Status::Failed, this describes the error that caused the failure.
    ///
    /// The value of this property is an ns::Error that describes what caused the receiver
    /// to no longer be able to play items.
    /// If the receiver's status is not Status::Failed, the value of this property is nil.
    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<arc::R<ns::Error>>;
}

/// AVPlayerPlaybackControl
impl Player {
    #[objc::msg_send(rate)]
    pub fn rate(&self) -> f32;

    #[objc::msg_send(setRate:)]
    pub fn set_rate(&mut self, val: f32);

    #[objc::msg_send(defaultRate)]
    pub fn default_rate(&self) -> f32;

    #[objc::msg_send(setDefaultRate:)]
    pub fn set_default_rate(&mut self, val: f32);

    #[objc::msg_send(play)]
    pub fn play(&mut self);

    /// Pauses playback.
    ///
    /// Equivalent to setting the value of rate to 0.0.
    #[objc::msg_send(pause)]
    pub fn pause(&mut self);

    #[objc::msg_send(timeControlStatus)]
    pub fn time_control_status(&self) -> TimeControlStatus;

    #[objc::msg_send(reasonForWaitingToPlay)]
    pub fn reason_for_waiting_to_play(&self) -> Option<arc::R<WaitingReason>>;

    #[objc::msg_send(playImmediatelyAtRate:)]
    pub fn play_immediately_at_rate(&mut self, rate: f32);
}

define_obj_type!(
    #[doc(alias = "AVPlayerWaitingReason")]
    pub WaitingReason(ns::String)
);

impl WaitingReason {
    #[doc(alias = "AVPlayerWaitingToMinimizeStallsReason")]
    pub fn minimize_stalls() -> &'static Self {
        unsafe { AVPlayerWaitingToMinimizeStallsReason }
    }

    #[doc(alias = "AVPlayerWaitingWhileEvaluatingBufferingRateReason")]
    pub fn eval_buffering_rate() -> &'static Self {
        unsafe { AVPlayerWaitingWhileEvaluatingBufferingRateReason }
    }

    #[doc(alias = "AVPlayerWaitingWithNoItemToPlayReason")]
    pub fn no_item_to_play() -> &'static Self {
        unsafe { AVPlayerWaitingWithNoItemToPlayReason }
    }

    #[doc(alias = "AVPlayerWaitingForCoordinatedPlaybackReason")]
    pub fn coordinated_playback() -> &'static Self {
        unsafe { AVPlayerWaitingForCoordinatedPlaybackReason }
    }
}

/// AVPlayerItemControl
impl Player {
    #[objc::msg_send(currentItem)]
    pub fn current_item(&self) -> Option<arc::R<av::PlayerItem>>;

    #[objc::msg_send(replaceCurrentItemWithPlayerItem:)]
    pub unsafe fn replace_current_item_throws(&mut self, item: Option<&av::PlayerItem>);

    pub fn replace_current_item<'ear>(
        &mut self,
        item: Option<&av::PlayerItem>,
    ) -> ns::ExResult<'ear> {
        unsafe { ns::try_catch(|| self.replace_current_item_throws(item)) }
    }

    #[objc::msg_send(actionAtItemEnd)]
    pub fn action_at_item_end(&self) -> ActionAtItemEnd;

    #[objc::msg_send(setActionAtItemEnd:)]
    pub unsafe fn set_action_at_item_end_throws(&mut self, val: ActionAtItemEnd);

    pub fn set_action_at_item_end<'ear>(&mut self, val: ActionAtItemEnd) -> ns::ExResult<'ear> {
        unsafe { ns::try_catch(|| self.set_action_at_item_end_throws(val)) }
    }
}

/// AVPlayerTimeControl
impl Player {
    #[objc::msg_send(currentTime)]
    pub fn current_time(&self) -> cm::Time;

    #[objc::msg_send(seekToDate:)]
    pub fn seek_to_date(&mut self, date: &ns::Date);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(seekToDate:completionHandler:)]
    pub fn seek_to_date_ch(
        &mut self,
        date: &ns::Date,
        block: &mut blocks::SendBlock<fn(finished: bool)>,
    );

    #[objc::msg_send(seekToTime:)]
    pub fn seek_to_time(&mut self, val: cm::Time);

    #[objc::msg_send(seekToTime:toleranceBefore:toleranceAfter:)]
    pub fn seek_to_time_with_tolerance(
        &mut self,
        val: cm::Time,
        tolerance_before: cm::Time,
        tolerance_after: cm::Time,
    );

    #[cfg(feature = "blocks")]
    #[objc::msg_send(seekToTime:completionHandler:)]
    pub fn seek_to_time_ch(
        &mut self,
        time: cm::Time,
        block: &mut blocks::SendBlock<fn(finished: bool)>,
    );

    #[cfg(feature = "blocks")]
    #[objc::msg_send(seekToTime:toleranceBefore:toleranceAfter:completionHandler:)]
    pub fn seek_to_time_with_tolerance_ch(
        &mut self,
        val: cm::Time,
        tolerance_before: cm::Time,
        tolerance_after: cm::Time,
        block: &mut blocks::SendBlock<fn(finished: bool)>,
    );
}

/// AVPlayerAdvancedRateControl
impl Player {
    #[objc::msg_send(automaticallyWaitsToMinimizeStalling)]
    pub fn automatically_waits_to_minimize_stalling(&self) -> bool;

    #[objc::msg_send(setAutomaticallyWaitsToMinimizeStalling:)]
    pub fn set_automatically_waits_to_minimize_stalling(&mut self, val: bool);

    #[objc::msg_send(setRate:time:atHostTime:)]
    pub unsafe fn set_rate_time_throws(
        &mut self,
        rate: f32,
        time: cm::Time,
        at_host_clock_time: cm::Time,
    );

    pub fn set_rate_time<'ear>(
        &mut self,
        rate: f32,
        time: cm::Time,
        at_host_clock_time: cm::Time,
    ) -> ns::ExResult<'ear> {
        unsafe { ns::try_catch(|| self.set_rate_time_throws(rate, time, at_host_clock_time)) }
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(prerollAtRate:completionHandler:)]
    pub fn preroll_at_rate_ch(
        &mut self,
        rate: f32,
        block: &mut blocks::SendBlock<fn(finished: bool)>,
    );

    #[objc::msg_send(cancelPendingPrerolls)]
    pub fn cancel_pending_prerolls(&mut self);

    #[objc::msg_send(sourceClock)]
    pub fn src_clock(&self) -> Option<&cm::Clock>;

    #[objc::msg_send(setSourceClock:)]
    pub fn set_src_clock(&mut self, val: Option<&cm::Clock>);
}

/// AVPlayerTimeObservation
impl Player {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(addPeriodicTimeObserverForInterval:queue:usingBlock:)]
    pub fn add_periodic_time_observer_for_interval(
        &mut self,
        interval: cm::Time,
        queue: Option<&dispatch::Queue>,
        block: &mut blocks::SyncBlock<fn(time: cm::Time)>,
    ) -> arc::R<ns::Id>;

    #[objc::msg_send(removeTimeObserver:)]
    pub fn remove_time_observer(&mut self, observer: &ns::Id);
}

/// AVPlayerMediaControl
impl Player {
    #[objc::msg_send(volume)]
    pub fn volume(&self) -> f32;

    #[objc::msg_send(setVolume:)]
    pub fn set_volume(&mut self, val: f32);

    #[objc::msg_send(isMuted)]
    pub fn is_muted(&self) -> bool;

    #[objc::msg_send(setMuted:)]
    pub fn set_muted(&mut self, val: bool);
}

/// AVPlayerBackgroundSupport
impl Player {
    /// What the player does with an audiovisual item when the app goes to the background
    /// (default `Automatic`).
    #[objc::msg_send(audiovisualBackgroundPlaybackPolicy)]
    pub fn audiovisual_background_playback_policy(&self) -> AudiovisualBackgroundPlaybackPolicy;

    #[objc::msg_send(setAudiovisualBackgroundPlaybackPolicy:)]
    pub fn set_audiovisual_background_playback_policy(
        &mut self,
        val: AudiovisualBackgroundPlaybackPolicy,
    );
}

/// AVPlayerAudioSessionParticipation
#[cfg(not(target_abi = "macabi"))]
impl Player {
    /// Whether the player is disconnected from system audio (default `false`).
    ///
    /// When `true`, the player doesn't interact with the audio session and doesn't play audio.
    #[objc::msg_send(disconnectedFromSystemAudio)]
    #[api::available(ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn disconnected_from_sys_audio(&self) -> bool;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setDisconnectedFromSystemAudio:completionHandler:)]
    #[api::available(ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn set_disconnected_from_sys_audio_ch_block(
        &mut self,
        val: bool,
        ch: Option<&mut blocks::CompletionBlock>,
    );

    /// Changes whether the player is disconnected from system audio.
    ///
    /// The completion handler is called on an arbitrary queue.
    #[cfg(feature = "blocks")]
    #[api::available(ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    #[allow(unused_unsafe)]
    pub fn set_disconnected_from_sys_audio_ch(&mut self, val: bool, ch: impl FnMut() + 'static) {
        let mut block = blocks::CompletionBlock::new0(ch);
        unsafe { self.set_disconnected_from_sys_audio_ch_block(val, Some(&mut block)) };
    }

    #[cfg(all(feature = "blocks", feature = "async"))]
    #[api::available(ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    #[allow(unused_unsafe)]
    pub async fn set_disconnected_from_sys_audio(&mut self, val: bool) {
        let (fut, mut block) = blocks::comp0();
        unsafe { self.set_disconnected_from_sys_audio_ch_block(val, Some(&mut block)) };
        fut.await
    }
}

define_obj_type!(
    #[doc(alias = "AVQueuePlayer")]
    pub QueuePlayer(Player),
    AV_QUEUE_PLAYER
);

impl QueuePlayer {
    #[objc::init(initWithItems:)]
    pub unsafe fn init_with_items_throws(
        self,
        items: &ns::Array<av::PlayerItem>,
    ) -> arc::R<QueuePlayer>;

    pub fn with_items<'ear>(items: &ns::Array<av::PlayerItem>) -> ns::ExResult<'ear, arc::R<Self>> {
        unsafe { ns::try_catch(|| Self::alloc().init_with_items_throws(items)) }
    }

    #[objc::msg_send(items)]
    pub fn items(&self) -> arc::R<ns::Array<av::PlayerItem>>;

    #[objc::msg_send(advanceToNextItem)]
    pub fn advance_to_next_item(&mut self);

    #[objc::msg_send(canInsertItem:afterItem:)]
    pub fn can_insert_item_after(
        &self,
        item: &av::PlayerItem,
        after: Option<&av::PlayerItem>,
    ) -> bool;

    #[objc::msg_send(insertItem:afterItem:)]
    pub fn insert_item(&mut self, item: &av::PlayerItem, after: Option<&av::PlayerItem>);

    #[objc::msg_send(removeItem:)]
    pub fn remove_item(&mut self, item: &av::PlayerItem);

    #[objc::msg_send(removeAllItems)]
    pub fn remove_all_items(&mut self);
}

unsafe extern "C" {
    static AV_PLAYER: &'static objc::Class<Player>;
    static AV_QUEUE_PLAYER: &'static objc::Class<QueuePlayer>;
}

impl ns::NotificationName {
    #[doc(alias = "AVPlayerRateDidChangeNotification")]
    #[inline]
    pub fn av_player_rate_did_change() -> &'static ns::NotificationName {
        unsafe { AVPlayerRateDidChangeNotification }
    }
}

unsafe extern "C" {
    static AVPlayerRateDidChangeNotification: &'static ns::NotificationName;
    static AVPlayerWaitingToMinimizeStallsReason: &'static WaitingReason;
    static AVPlayerWaitingWhileEvaluatingBufferingRateReason: &'static WaitingReason;
    static AVPlayerWaitingWithNoItemToPlayReason: &'static WaitingReason;
    static AVPlayerWaitingForCoordinatedPlaybackReason: &'static WaitingReason;
}

#[cfg(test)]
mod tests {
    use crate::{av, ns};

    #[test]
    fn basics() {
        let url = ns::Url::with_str("file:///tmp/file.mp4").expect("Url is not valid");
        let mut player = av::Player::with_url(&url);
        assert_eq!(player.status(), av::PlayerStatus::Unknown);

        assert_eq!(
            player.audiovisual_background_playback_policy(),
            av::PlayerAudiovisualBackgroundPlaybackPolicy::Automatic
        );
        player.set_audiovisual_background_playback_policy(
            av::PlayerAudiovisualBackgroundPlaybackPolicy::ContinuesIfPossible,
        );
        assert_eq!(
            player.audiovisual_background_playback_policy(),
            av::PlayerAudiovisualBackgroundPlaybackPolicy::ContinuesIfPossible
        );
    }
}
