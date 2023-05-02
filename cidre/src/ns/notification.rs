use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(NotificationName(ns::String));

define_obj_type!(Notification(ns::Id));

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

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_NOTIFICATION: &'static objc::Class<Notification>;
}
