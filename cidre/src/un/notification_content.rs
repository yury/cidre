use crate::{arc, define_obj_type, ns, objc, un};

pub trait NotificationContentProviding: objc::Obj {}

#[repr(usize)]
pub enum NotificationInterruptionLevel {
    /// Added to the notification list; does not light up screen or play sound
    #[doc(alias = "UNNotificationInterruptionLevelPassive")]
    Passive,

    /// Presented immediately; Lights up screen and may play a sound
    #[doc(alias = "UNNotificationInterruptionLevelActive")]
    Active,

    /// Presented immediately; Lights up screen and may play a sound; May be presented during Do Not Disturb
    #[doc(alias = "UNNotificationInterruptionLevelTimeSensitive")]
    TimeSensitive,

    /// Presented immediately; Lights up screen and plays sound; Always presented during Do Not Disturb;
    /// Bypasses mute switch; Includes default critical alert sound if no sound provided
    #[doc(alias = "UNNotificationInterruptionLevelCritical")]
    Critical,
}

define_obj_type!(
    #[doc(alias = "UNNotificationContent")]
    pub NotificationContent(ns::Id),
    UN_NOTIFICATION_CONTENT
);

impl NotificationContent {
    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::Retained<Self>;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::Retained<NotificationContentMut>;

    #[objc::msg_send(badge)]
    pub fn badge(&self) -> Option<arc::R<ns::Number>>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(body)]
    pub fn body(&self) -> arc::R<ns::String>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(categoryIdentifier)]
    pub fn category_id(&self) -> arc::R<ns::String>;

    #[cfg(not(any(target_os = "tvos", target_os = "macos")))]
    #[objc::msg_send(launchImageName)]
    pub fn launch_image_name(&self) -> arc::R<ns::String>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(sound)]
    pub fn sound(&self) -> Option<arc::R<un::NotificationSound>>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(subtitle)]
    pub fn subtitle(&self) -> arc::R<ns::String>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(threadIdentifier)]
    pub fn thread_id(&self) -> arc::R<ns::String>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> arc::R<ns::Dictionary<ns::Id, ns::Id>>;

    #[objc::msg_send(targetContentIdentifier)]
    pub fn target_content_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(interruptionLevel)]
    pub fn interruption_level(&self) -> un::NotificationInterruptionLevel;

    #[objc::msg_send(relevanceScore)]
    pub fn relevance_score(&self) -> f64;

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(filterCriteria)]
    pub fn filter_criteria(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(contentByUpdatingWithProvider:error:)]
    pub unsafe fn with_updating_with_provider_err<'ear, P: NotificationContentProviding>(
        provider: &P,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Self>>;

    #[inline]
    pub fn with_updating_with_provider<'ear, P: NotificationContentProviding>(
        provider: &P,
    ) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::with_updating_with_provider_err(provider, err) })
    }
}

define_obj_type!(
    #[doc(alias = "UNMutableNotificationContent")]
    pub NotificationContentMut(NotificationContent),
    UN_MUTABLE_NOTIFICATION_CONTENT
);

impl NotificationContentMut {
    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::R<Self>;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::R<Self>;

    #[objc::msg_send(setBadge:)]
    pub fn set_badge(&mut self, val: Option<&ns::Number>);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setBody:)]
    pub fn set_body(&mut self, val: &ns::String);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setCategoryIdentifier:)]
    pub fn set_category_id(&mut self, val: &ns::String);

    #[cfg(not(any(target_os = "tvos", target_os = "macos")))]
    #[objc::msg_send(setLaunchImageName:)]
    pub fn set_launch_image_name(&mut self, val: &ns::String);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setSound:)]
    pub fn set_sound(&mut self, val: Option<&un::NotificationSound>);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setSubtitle:)]
    pub fn set_subtitle(&mut self, val: &ns::String);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setThreadIdentifier:)]
    pub fn set_thread_id(&mut self, val: &ns::String);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setUserInfo:)]
    pub fn set_user_info(&mut self, val: &ns::Dictionary<ns::Id, ns::Id>);

    #[objc::msg_send(setTargetContentIdentifier:)]
    pub fn set_target_content_id(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(setInterruptionLevel:)]
    pub fn set_interruption_level(&mut self, val: un::NotificationInterruptionLevel);

    #[objc::msg_send(setRelevanceScore:)]
    pub fn set_relevance_score(&mut self, val: f64);

    #[cfg(not(target_os = "tvos"))]
    #[objc::msg_send(setFilterCriteria:)]
    pub fn set_filter_criteria(&mut self, val: Option<&ns::String>);
}

#[link(name = "un", kind = "static")]
unsafe extern "C" {
    static UN_NOTIFICATION_CONTENT: &'static objc::Class<NotificationContent>;
    static UN_MUTABLE_NOTIFICATION_CONTENT: &'static objc::Class<NotificationContent>;
}

#[cfg(test)]
mod tests {
    use crate::{ns, un};

    #[test]
    fn basics() {
        let content = un::NotificationContent::new();
        let mut content = content.copy_mut();
        assert!(content.badge().is_none());

        content.set_badge(Some(&ns::Number::with_i64(444_444_444_444)));
        assert!(content.badge().is_some());

        let a = content.badge().unwrap();
        let b = content.badge().unwrap();
        eprintln!("{a:?} {b:?}");
        a.as_type_ref().show();
        b.as_type_ref().show();

        assert!(content.body().is_empty());
        assert!(content.category_id().is_empty());

        assert!(content.sound().is_none());

        content.set_sound(Some(&un::NotificationSound::default()));
        let s1 = content.sound().unwrap();
        let s2 = content.sound().unwrap();
        s1.as_type_ref().show();
        s2.as_type_ref().show();

        assert!(content.subtitle().is_empty());
        assert!(content.thread_id().is_empty());
        assert!(content.user_info().is_empty());
    }
}
