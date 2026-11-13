use crate::{arc, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSTrackingAreaOptions")]
    pub TrackingAreaOpts(usize)
);

impl TrackingAreaOpts {
    pub const MOUSE_ENTERED_AND_EXITED: Self = Self(0x01);
    pub const MOUSE_MOVED: Self = Self(0x02);
    pub const CURSOR_UPDATE: Self = Self(0x04);
    pub const ACTIVE_WHEN_FIRST_RESPONDER: Self = Self(0x10);
    pub const ACTIVE_IN_KEY_WINDOW: Self = Self(0x20);
    pub const ACTIVE_IN_ACTIVE_APP: Self = Self(0x40);
    pub const ACTIVE_ALWAYS: Self = Self(0x80);
    pub const ASSUME_INSIDE: Self = Self(0x100);
    pub const IN_VISIBLE_RECT: Self = Self(0x200);
    pub const ENABLED_DURING_MOUSE_DRAG: Self = Self(0x400);
}

define_obj_type!(
    #[doc(alias = "NSTrackingArea")]
    pub TrackingArea(ns::Id),
    sym NSTrackingArea
);

impl TrackingArea {
    #[objc::init(initWithRect:options:owner:userInfo:)]
    pub fn init_with_rect(
        self,
        rect: ns::Rect,
        options: TrackingAreaOpts,
        owner: Option<&ns::Id>,
        user_info: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> arc::R<TrackingArea>;

    pub fn with_rect(
        rect: ns::Rect,
        options: TrackingAreaOpts,
        owner: Option<&ns::Id>,
        user_info: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_rect(rect, options, owner, user_info)
    }

    #[objc::msg_send(rect)]
    pub fn rect(&self) -> ns::Rect;

    #[objc::msg_send(options)]
    pub fn options(&self) -> TrackingAreaOpts;

    #[objc::msg_send(owner)]
    pub fn owner(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> Option<arc::R<ns::Dictionary<ns::Id, ns::Id>>>;
}
