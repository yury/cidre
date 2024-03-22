use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "MCPeerID")]
    pub PeerId(ns::Id)
);

impl arc::A<PeerId> {
    #[objc::msg_send(initWithDisplayName:)]
    pub unsafe fn init_with_display_name_throws(self, display_name: &ns::String) -> arc::R<PeerId>;
}

impl PeerId {
    define_cls!(MC_PEER_ID);

    #[inline]
    pub fn with_display_name<'ear>(name: &ns::String) -> Result<arc::R<Self>, &'ear ns::Exception> {
        ns::try_catch(|| unsafe { Self::alloc().init_with_display_name_throws(name) })
    }

    #[objc::msg_send(displayName)]
    pub fn display_name(&self) -> &ns::String;
}

#[link(name = "mc", kind = "static")]
extern "C" {
    static MC_PEER_ID: &'static objc::Class<PeerId>;
}

#[cfg(test)]
mod tests {
    use crate::{mc, ns};

    #[test]
    fn basics() {
        let name = ns::str!(c"test");
        let peer = mc::PeerId::with_display_name(name).unwrap();
        assert_eq!(peer.display_name(), name);

        let name = ns::str!(c"");
        let _err = mc::PeerId::with_display_name(name).expect_err("should be err");
    }
}
