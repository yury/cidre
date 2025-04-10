#[cfg(feature = "blocks")]
use crate::blocks;

use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    pub UserActivityPersistentId(ns::String)
);

impl UserActivityPersistentId {
    #[inline]
    pub fn with_string(str: &ns::String) -> &Self {
        unsafe { std::mem::transmute(str) }
    }

    #[inline]
    pub fn with_raw<S: AsRef<ns::String>>(raw: &S) -> arc::R<Self> {
        Self::with_string(raw.as_ref()).retained()
    }
}

impl AsRef<UserActivityPersistentId> for ns::String {
    #[inline]
    fn as_ref(&self) -> &UserActivityPersistentId {
        UserActivityPersistentId::with_string(self)
    }
}

define_obj_type!(
    #[doc(alias = "NSUserActivity")]
    pub UserActivity(ns::Id)
);

impl arc::A<UserActivity> {
    #[objc::msg_send(initWithActivityType:)]
    pub fn init_with_activity_type(self, activity_type: &ns::String) -> arc::R<UserActivity>;
}

impl UserActivity {
    define_cls!(NS_USER_ACTIVITY);

    pub fn with_activity_type(activity_type: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_activity_type(activity_type)
    }

    #[objc::msg_send(activityType)]
    pub fn activity_type(&self) -> arc::R<ns::String>;

    /// An optional, user-visible title for this activity, such as a document name or web page title.
    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(setUserInfo:)]
    pub fn set_user_info(&mut self, val: Option<&ns::Dictionary<ns::String, ns::Id>>);

    #[objc::msg_send(addUserInfoEntriesFromDictionary:)]
    pub fn add_user_info_entries_from_dictionary(
        &mut self,
        val: &ns::Dictionary<ns::String, ns::Id>,
    );

    #[objc::msg_send(requiredUserInfoKeys)]
    pub fn required_user_info_keys(&self) -> Option<arc::R<ns::Set<ns::String>>>;

    #[objc::msg_send(setRequiredUserInfoKeys:)]
    pub fn set_required_user_info_keys(&mut self, val: Option<&ns::Set<ns::String>>);

    #[objc::msg_send(needsSave)]
    pub fn needs_save(&self) -> bool;

    #[objc::msg_send(setNeedsSave:)]
    pub fn set_needs_save(&mut self, val: bool);

    #[objc::msg_send(webpageURL)]
    pub fn webpage_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(setWebpageURL:)]
    pub fn set_webpage_url(&mut self, val: Option<&ns::Url>);

    #[objc::msg_send(referrerURL)]
    pub fn referrer_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(setReferrerURL:)]
    pub fn set_referrer_url(&mut self, val: Option<&ns::Url>);

    #[objc::msg_send(expirationDate)]
    pub fn expiration_date(&self) -> Option<arc::R<ns::Date>>;

    #[objc::msg_send(setExpirationDate:)]
    pub fn set_expiration_date(&mut self, val: Option<&ns::Date>);

    #[objc::msg_send(keywords)]
    pub fn keywords(&self) -> arc::R<ns::Set<ns::String>>;

    #[objc::msg_send(setKeywords:)]
    pub fn set_keywords(&mut self, val: &ns::Set<ns::String>);

    #[objc::msg_send(supportsContinuationStreams)]
    pub fn supports_continuation_streams(&self) -> bool;

    #[objc::msg_send(setSupportsContinuationStreams:)]
    pub fn set_supports_continuation_streams(&mut self, val: bool);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyUserActivityDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: UserActivityDelegate>(&self, val: Option<&D>);

    #[objc::msg_send(targetContentIdentifier)]
    pub fn target_content_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTargetContentIdentifier:)]
    pub fn set_target_content_id(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(becomeCurrent)]
    pub fn become_current(&self);

    #[objc::msg_send(resignCurrent)]
    pub fn resign_current(&self);

    #[objc::msg_send(invalidate)]
    pub fn invalidate(&mut self);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(getContinuationStreamsWithCompletionHandler:)]
    pub fn continuation_streams_ch_block(
        &self,
        ch: &mut blocks::SendBlock<
            fn(
                input_stream: Option<&ns::InputStream>,
                output_stream: Option<&ns::OutputStream>,
                err: Option<&ns::Error>,
            ),
        >,
    );

    #[cfg(feature = "blocks")]
    pub fn continuation_streams_ch(
        &self,
        ch: impl FnMut(Option<&ns::InputStream>, Option<&ns::OutputStream>, Option<&ns::Error>)
        + 'static
        + Send,
    ) {
        let mut ch = blocks::SendBlock::new3(ch);
        self.continuation_streams_ch_block(&mut ch);
    }

    #[objc::msg_send(isEligibleForHandoff)]
    fn is_eligible_for_handoff(&self) -> bool;

    /// Set to true if this user activity should be eligible to be handed off to another device
    #[objc::msg_send(setEligibleForHandoff:)]
    fn set_eligible_for_handoff(&mut self, val: bool);

    #[objc::msg_send(isEligibleForSearch)]
    fn is_eligible_for_search(&self) -> bool;

    #[objc::msg_send(setEligibleForSearch:)]
    fn set_eligible_for_search(&mut self, val: bool);

    #[objc::msg_send(isEligibleForPublicIndexing)]
    fn is_eligible_for_public_indexing(&self) -> bool;

    #[objc::msg_send(setEligibleForPublicIndexing:)]
    fn set_eligible_for_public_indexing(&mut self, val: bool);

    #[objc::msg_send(isEligibleForPrediction)]
    #[objc::available(ios = 12.0, watchos = 5.0)]
    fn is_eligible_for_prediction(&self) -> bool;

    #[objc::msg_send(setEligibleForPrediction:)]
    #[objc::available(ios = 12.0, watchos = 5.0)]
    fn set_eligible_for_prediction(&mut self, val: bool);

    #[objc::msg_send(persistentIdentifier)]
    #[objc::available(macos = 10.15, ios = 12.0, watchos = 5.0)]
    pub fn persistent_id(&self) -> Option<arc::R<UserActivityPersistentId>>;

    #[objc::msg_send(setPersistentIdentifier:)]
    #[objc::available(macos = 10.15, ios = 12.0, watchos = 5.0)]
    pub fn set_persistent_id(&mut self, val: Option<UserActivityPersistentId>);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(deleteSavedUserActivitiesWithPersistentIdentifiers:completionHandler:)]
    #[objc::available(macos = 10.15, ios = 12.0, watchos = 5.0)]
    fn delete_saved_user_activities_with_ids_ch_block(
        ids: &ns::Array<UserActivityPersistentId>,
        ch: &mut blocks::CompletionBlock,
    );

    #[cfg(feature = "blocks")]
    #[objc::available(macos = 10.15, ios = 12.0, watchos = 5.0)]
    fn delete_saved_user_activities_with_ids(
        ids: &ns::Array<UserActivityPersistentId>,
        ch: impl FnMut() + 'static,
    ) {
        let mut ch = blocks::CompletionBlock::new0(ch);
        Self::delete_saved_user_activities_with_ids_ch_block(ids, &mut ch);
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(deleteAllSavedUserActivitiesWithCompletionHandler:)]
    #[objc::available(macos = 10.15, ios = 12.0, watchos = 5.0)]
    fn delete_all_saved_user_activities_ch_block(ch: &mut blocks::CompletionBlock);

    #[cfg(feature = "blocks")]
    #[objc::available(macos = 10.15, ios = 12.0, watchos = 5.0)]
    fn delete_all_saved_user_activities(ch: impl FnMut() + 'static) {
        let mut ch = blocks::CompletionBlock::new0(ch);
        Self::delete_all_saved_user_activities_ch_block(&mut ch);
    }
}

#[objc::protocol(NSUserActivityDelegate)]
pub trait UserActivityDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(userActivityWillSave:)]
    fn user_activity_will_save(&mut self, user_activity: &mut ns::UserActivity);

    #[objc::optional]
    #[objc::msg_send(userActivityWasContinued:)]
    fn user_activity_was_continued(&mut self, user_activity: &mut ns::UserActivity);

    #[objc::optional]
    #[objc::msg_send(userActivity:didReceiveInputStream:outputStream:)]
    fn user_activity_did_recieve_streams(
        &mut self,
        user_activity: &mut ns::UserActivity,
        input_stream: &mut ns::InputStream,
        output_stream: &mut ns::OutputStream,
    );
}

define_obj_type!(
    pub AnyUserActivityDelegate(ns::Id)
);

impl UserActivityDelegate for AnyUserActivityDelegate {}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_USER_ACTIVITY: &'static objc::Class<UserActivity>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let activity_type = ns::str!(c"coding");

        let mut activity = ns::UserActivity::with_activity_type(activity_type);

        assert_eq!(activity.activity_type().as_ref(), activity_type);

        let keywords = activity.keywords();
        assert!(keywords.is_empty());

        activity.become_current();
        activity.resign_current();
        activity.invalidate();
        activity.become_current();

        assert!(activity.persistent_id().is_none());
    }
}
