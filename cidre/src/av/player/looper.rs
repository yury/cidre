use crate::{arc, av, cm, define_cls, define_obj_type, ns, objc};

#[doc(alias = "AVPlayerLooperStatus")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(isize)]
pub enum LooperStatus {
    Unknown = 0,
    Ready = 1,
    Failed = 2,
    Cancelled = 3,
}

#[doc(alias = "AVPlayerLooperItemOrdering")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(isize)]
pub enum LooperItemOrdering {
    LoopingItemsPrecedeExistingItems = 0,
    LoopingItemsFollowExistingItems = 1,
}

define_obj_type!(
    #[doc(alias = "AVPlayerLooper")]
    pub Looper(ns::Id)
);

unsafe impl Send for Looper {}
unsafe impl Sync for Looper {}

impl Looper {
    define_cls!(AV_PLAYER_LOOPER);

    #[objc::msg_send(playerLooperWithPlayer:templateItem:)]
    pub unsafe fn with_player_throws(
        player: &av::QueuePlayer,
        template_item: &av::PlayerItem,
    ) -> arc::R<Self>;

    pub fn with_player<'ear>(
        player: &av::QueuePlayer,
        template_item: &av::PlayerItem,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        unsafe { ns::try_catch(|| Self::with_player_throws(player, template_item)) }
    }

    #[objc::msg_send(playerLooperWithPlayer:templateItem:timeRange:)]
    pub unsafe fn with_player_time_range_throws(
        player: &av::QueuePlayer,
        template_item: &av::PlayerItem,
        loop_range: cm::TimeRange,
    ) -> arc::R<Self>;

    pub fn with_player_time_range<'ear>(
        player: &av::QueuePlayer,
        template_item: &av::PlayerItem,
        loop_range: cm::TimeRange,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        unsafe {
            ns::try_catch(|| Self::with_player_time_range_throws(player, template_item, loop_range))
        }
    }

    #[objc::msg_send(status)]
    pub fn status(&self) -> LooperStatus;

    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<arc::R<ns::Error>>;

    #[objc::msg_send(disableLooping)]
    pub fn disable_looping(&mut self);

    #[objc::msg_send(loopCount)]
    pub fn loop_count(&self) -> isize;

    #[objc::msg_send(loopingPlayerItems)]
    pub fn looping_player_items(&self) -> arc::R<ns::Array<av::PlayerItem>>;
}

unsafe extern "C" {
    static AV_PLAYER_LOOPER: &'static objc::Class<Looper>;
}
