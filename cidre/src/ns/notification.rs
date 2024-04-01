use crate::{arc, cf, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(pub NotificationName(ns::String));

impl NotificationName {
    pub fn with_raw(string: &ns::String) -> &Self {
        unsafe { std::mem::transmute(string) }
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

define_obj_type!(
    #[doc(alias = "NSNotificationCenter")]
    pub NotificationCenter(ns::Id), NS_NOTIFICATION_CENTER
);

unsafe impl Send for NotificationCenter {}

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
    pub fn add_observer_block_ar(
        &mut self,
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        queue: Option<&ns::OpQueue>,
        using_block: &mut blocks::SyncBlock<fn(&ns::Notification)>,
    ) -> arc::Rar<ns::Id>;

    #[cfg(feature = "blocks")]
    #[objc::rar_retain]
    pub fn add_observer_block(
        &mut self,
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        queue: Option<&ns::OpQueue>,
        using_block: &mut blocks::SyncBlock<fn(&ns::Notification)>,
    ) -> arc::R<ns::Id>;

    #[cfg(feature = "blocks")]
    pub fn add_observer(
        &mut self,
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        queue: Option<&ns::OpQueue>,
        block: impl FnMut(&ns::Notification) + 'static + std::marker::Sync,
    ) -> arc::R<ns::Id> {
        let mut block = blocks::SyncBlock::new1(block);
        self.add_observer_block(name, object, queue, &mut block)
    }

    #[cfg(feature = "blocks")]
    pub fn add_observer_guard(
        &mut self,
        name: &ns::NotificationName,
        object: Option<&ns::Id>,
        queue: Option<&ns::OpQueue>,
        block: impl FnMut(&ns::Notification) + 'static + std::marker::Sync,
    ) -> NotificationGuard {
        let mut block = blocks::SyncBlock::new1(block);
        let token = self.add_observer_block(name, object, queue, &mut block);
        NotificationGuard {
            token,
            center: self.retained(),
        }
    }
}

pub struct NotificationGuard {
    token: arc::R<ns::Id>,
    center: arc::R<ns::NotificationCenter>,
}

impl Drop for NotificationGuard {
    fn drop(&mut self) {
        self.center.remove_observer(&self.token);
    }
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
        let mut block = blocks::SyncBlock::new1(move |note: &ns::Notification| {
            println!("{note:?}");
            let expected_name = ns::String::with_str("test");
            assert!(expected_name.is_equal(note.name()));
            let mut guard = block_counter.lock().unwrap();
            *guard += 1;
        });
        let name = ns::NotificationName::with_str("test");
        let token = nc.add_observer_block(&name, None, None, &mut block);
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

    #[test]
    fn guard() {
        let nc = ns::NotificationCenter::default();
        let counter = Arc::new(Mutex::new(0));
        let block_counter = counter.clone();
        let name = ns::NotificationName::with_raw(ns::str!(c"test"));
        {
            let _g = nc.add_observer_guard(name, None, None, move |_note| {
                let mut guard = block_counter.lock().unwrap();
                *guard += 1;
            });
            nc.post_with_name_object(name, None);
            {
                let guard = counter.lock().unwrap();
                assert_eq!(1, *guard);
            }
        }
        nc.post_with_name_object(name, None);
        {
            let guard = counter.lock().unwrap();
            assert_eq!(1, *guard);
        }
    }
}
