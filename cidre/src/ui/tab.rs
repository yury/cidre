use crate::{arc, define_obj_type, ns, objc, ui};

// UITabPlacement
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum TabPlacement {
    /// Resolves to `.default` for root-level tabs, and `.optional` for all others.
    Automatic = 0,

    /// The tab can be added or removed from the tab bar, and appears by default.
    Default = 1,

    /// The tab can be added or removed from the tab bar, but does NOT appear by default.
    Optional = 2,

    /// The tab cannot be removed from the tab bar, but can be moved within.
    Movable = 3,

    /// The tab is always available and visible in the tab bar.
    ///
    /// Pinned items are placed at the trailing side of the bar.
    Pinned = 4,

    /// The tab cannot be moved or removed from the tab bar, and is displayed before
    /// all customizable tabs.
    Fixed = 5,

    /// The tab cannot be added to the tab bar.
    SideBarOnly = 6,
}

define_obj_type!(
    #[doc(alias = "UITab")]
    pub Tab(ns::Id)
);

impl Tab {
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setIdentifier:)]
    pub fn set_id_string(&mut self, val: &ns::String);

    #[inline]
    pub fn set_id<S: AsRef<ns::String>>(&mut self, val: &S) {
        self.set_id_string(val.as_ref());
    }

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title_string(&mut self, val: &ns::String);

    #[inline]
    pub fn set_title<S: AsRef<ns::String>>(&mut self, val: &S) {
        self.set_title_string(val.as_ref());
    }

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ui::Image>);

    #[objc::msg_send(subtitle)]
    pub fn subtitle(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setSubtitle:)]
    pub fn set_subtitle_string(&mut self, val: Option<&ns::String>);

    #[inline]
    pub fn set_subtitle<S: AsRef<ns::String>>(&mut self, val: Option<&S>) {
        self.set_subtitle_string(val.map(|s| s.as_ref()));
    }

    #[objc::msg_send(badgeValue)]
    pub fn badge_value(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setBadgeValue:)]
    pub fn set_badge_value_string(&mut self, val: Option<&ns::String>);

    #[inline]
    pub fn set_badge_value<S: AsRef<ns::String>>(&mut self, val: Option<&S>) {
        self.set_badge_value_string(val.map(|s| s.as_ref()));
    }

    #[objc::msg_send(preferredPlacement)]
    pub fn preferred_placement(&self) -> TabPlacement;

    #[objc::msg_send(setPreferredPlacement:)]
    pub fn set_preferred_placement(&mut self, val: TabPlacement);

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setUserInfo:)]
    pub fn set_user_info(&mut self, val: Option<&ns::Id>);
}
