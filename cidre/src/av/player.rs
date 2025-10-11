use crate::{arc, define_cls, define_obj_type, ns, objc};

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

    pub fn with_player_item<'ear>(
        item: Option<&PlayerItem>,
    ) -> Result<arc::R<Self>, &'ear ns::Exception> {
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

define_obj_type!(
    #[doc(alias = "AVQueuePlayer")]
    pub QueuePlayer(Player)
);

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_PLAYER: &'static objc::Class<Player>;
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
