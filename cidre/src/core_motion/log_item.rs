use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CMLogItem")]
    pub LogItem(ns::Id)
);

impl LogItem {
    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> ns::TimeInterval;
}
