use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(pub Transaction(ns::Id), CA_TRANSACTION);
impl Transaction {
    #[objc::msg_send2(begin)]
    pub fn begin();

    #[objc::msg_send2(commit)]
    pub fn commit();

    #[objc::msg_send2(flush)]
    pub fn flush();

    #[objc::msg_send2(lock)]
    pub fn lock();

    #[objc::msg_send2(unlock)]
    pub fn unlock();

    #[objc::msg_send2(disableActions)]
    pub fn disable_actions() -> bool;

    #[objc::msg_send2(setDisableActions:)]
    pub fn set_disable_actions(val: bool);

    #[inline]
    pub fn perform<R, F: FnMut() -> R>(mut f: F) -> R {
        Self::begin();
        let r = f();
        Self::commit();
        r
    }
    #[inline]
    pub fn perform_with_disabled_actions<R, F: FnMut() -> R>(mut f: F) -> R {
        Self::perform(|| {
            Self::set_disable_actions(true);
            f()
        })
    }
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static CA_TRANSACTION: &'static objc::Class<Transaction>;
}
