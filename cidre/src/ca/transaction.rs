use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(Transaction(ns::Id), CA_TRANSACTION);
impl Transaction {
    #[objc::cls_msg_send(begin)]
    pub fn begin();

    #[objc::cls_msg_send(commit)]
    pub fn commit();

    #[objc::cls_msg_send(flush)]
    pub fn flush();

    #[objc::cls_msg_send(lock)]
    pub fn lock();

    #[objc::cls_msg_send(unlock)]
    pub fn unlock();

    #[objc::cls_msg_send(disableActions)]
    pub fn disable_action() -> bool;

    #[objc::cls_msg_send(setDisableActions:)]
    pub fn set_disable_action(value: bool);

    #[inline]
    pub fn perform<R, F: FnMut() -> R>(mut f: F) -> R {
        Self::begin();
        let r = f();
        Self::commit();
        r
    }
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static CA_TRANSACTION: &'static objc::Class<Transaction>;
}
