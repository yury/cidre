use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVAssetCache")]
    pub Cache(ns::Id)
);

impl Cache {
    #[objc::msg_send(isPlayableOffline)]
    pub fn is_playable_offline(&self) -> bool;
}
