use crate::{arc, cf, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(pub NotificationName(ns::String));

impl NotificationName {
    pub fn with_raw(raw: arc::R<ns::String>) -> arc::R<Self> {
        unsafe { std::mem::transmute(raw) }
    }

    pub fn with_str(str: &str) -> arc::R<Self> {
        let raw = ns::String::with_str(str);
        unsafe { std::mem::transmute(raw) }
    }

    pub fn as_cf(&self) -> &cf::NotificationName {
        unsafe { std::mem::transmute(self) }
    }
}

define_obj_type!(pub Notification(ns::Id));

impl arc::A<Notification> {
    #[objc::msg_send(initWithName:object:userInfo:)]
    pub fn init_with_name(
        self,
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        user_info: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> arc::R<Notification>;

    #[objc::msg_send(initWithCoder:)]
    pub fn init_with_coder(self, coder: &ns::Coder) -> arc::R<Notification>;
}

impl Notification {
    define_cls!(NS_NOTIFICATION);

    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::NotificationName;

    #[objc::msg_send(object)]
    pub fn id(&self) -> Option<&ns::Id>;

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> Option<&ns::Dictionary<ns::Id, ns::Id>>;

    pub fn with_name(
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        user_info: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_name(name, object, user_info)
    }

    pub fn with_coder(coder: &ns::Coder) -> arc::R<Self> {
        Self::alloc().init_with_coder(coder)
    }
}

define_obj_type!(pub NotificationCenter(ns::Id), NS_NOTIFICATION_CENTER);
impl NotificationCenter {
    #[objc::cls_msg_send(defaultCenter)]
    pub fn default() -> &'static mut Self;

    #[objc::msg_send(postNotification:)]
    pub fn post(&self, notification: &ns::Notification);

    #[objc::msg_send(postNotificationName:object:)]
    pub fn post_with_name_object(&self, name: &ns::NotificationName, object: Option<&ns::Id>);

    #[objc::msg_send(postNotificationName:object:userInfo:)]
    pub fn post_with_name_object_info(
        &self,
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        user_info: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    );

    #[objc::msg_send(removeObserver:)]
    pub fn remove_observer(&mut self, observer: &ns::Id);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(addObserverForName:object:queue:usingBlock:)]
    pub fn add_observer_for_ar<'bar, B>(
        &mut self,
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        queue: Option<&ns::OperationQueue>,
        using_block: &'static mut blocks::Block<B>,
    ) -> arc::Rar<ns::Id>
    where
        B: FnMut(&'bar ns::Notification);

    #[cfg(feature = "blocks")]
    #[objc::rar_retain]
    pub fn add_observer_for<'bar, B>(
        &mut self,
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        queue: Option<&ns::OperationQueue>,
        using_block: &'static mut blocks::Block<B>,
    ) -> arc::R<ns::Id>
    where
        B: FnMut(&'bar ns::Notification);
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_NOTIFICATION: &'static objc::Class<Notification>;
    static NS_NOTIFICATION_CENTER: &'static objc::Class<NotificationCenter>;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::{blocks, ns};

    #[test]
    fn basics() {
        let nc = ns::NotificationCenter::default();
        let counter = Arc::new(Mutex::new(0));
        let block_counter = counter.clone();
        let mut block = blocks::mut1(move |note: &ns::Notification| {
            println!("{note:?}");
            let expected_name = ns::String::with_str("test");
            assert!(expected_name.is_equal(note.name()));
            let mut guard = block_counter.lock().unwrap();
            *guard += 1;
        });
        let name = ns::NotificationName::with_str("test");
        let token = nc.add_observer_for(&name, None, None, block.escape());
        nc.post_with_name_object(&name, None);
        nc.post_with_name_object(&name, None);

        {
            let guard = counter.lock().unwrap();
            assert_eq!(2, *guard);
        }
        nc.remove_observer(&token);
        nc.post_with_name_object(&name, None);

        {
            let guard = counter.lock().unwrap();
            assert_eq!(2, *guard);
        }
    }
}
