use crate::{arc, define_cls, define_obj_type, ns, objc};
use crate::{av, cm};

pub mod item;
pub use item::Item as PlayerItem;
pub use item::Status as ItemStatus;

pub mod item_track;
pub use item_track::ItemTrack as PlayerItemTrack;

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

#[doc(alias = "AVPlayerHDRMode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum HdrMode {
    Hlg = 0x1,
    Hdr10 = 0x2,
    DolbyVision = 0x4,
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

impl arc::A<Player> {
    #[objc::msg_send(initWithURL:)]
    pub fn init_with_url(self, url: &ns::Url) -> arc::R<Player>;

    #[objc::msg_send(initWithPlayerItem:)]
    pub unsafe fn init_with_player_item_throws(self, item: Option<&PlayerItem>) -> arc::R<Player>;
}

impl Player {
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
}

define_obj_type!(
    #[doc(alias = "AVQueuePlayer")]
    pub QueuePlayer(Player),
    AV_QUEUE_PLAYER
);

impl arc::A<QueuePlayer> {
    #[objc::msg_send(initWithItems:)]
    pub unsafe fn init_with_items_throws(
        self,
        items: &ns::Array<av::PlayerItem>,
    ) -> arc::R<QueuePlayer>;
}

impl QueuePlayer {
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

#[link(name = "av", kind = "static")]
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

#[link(name = "AVFoundation", kind = "framework")]
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
        let player = av::Player::with_url(&url);
        assert_eq!(player.status(), av::PlayerStatus::Unknown);
    }
}
