use crate::{arc, define_cls, define_obj_type, mc, ns, objc};

define_obj_type!(
    #[doc(alias = "MCNearbyServiceBrowser")]
    pub NearbyServiceBrowser(ns::Id)
);

impl arc::A<NearbyServiceBrowser> {
    #[objc::msg_send(initWithPeer:serviceType:)]
    pub unsafe fn init_with_peer_and_service_type_throws(
        self,
        peer: &mc::PeerId,
        service_type: &ns::String,
    ) -> arc::R<NearbyServiceBrowser>;
}

impl NearbyServiceBrowser {
    define_cls!(MC_NEARBY_SERVICE_BROWSER);

    pub fn with_peer<'ear>(
        peer: &mc::PeerId,
        service_type: &ns::String,
    ) -> Result<arc::R<Self>, &'ear ns::Exception> {
        ns::try_catch(|| unsafe {
            Self::alloc().init_with_peer_and_service_type_throws(peer, service_type)
        })
    }

    #[objc::msg_send(startBrowsingForPeers)]
    pub fn start_browsing_for_peers(&mut self);

    #[objc::msg_send(stopBrowsingForPeers)]
    pub fn stop_browsing_for_peers(&mut self);

    #[objc::msg_send(invitePeer:toSession:withContext:timeout:)]
    pub fn invite_peer(
        &mut self,
        peer: &mc::PeerId,
        to_session: &mc::Session,
        ctx: Option<&ns::Data>,
        timeout: ns::TimeInterval,
    );

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<&AnyDelegate>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(myPeerID)]
    pub fn my_peer_id(&self) -> &mc::PeerId;

    #[objc::msg_send(serviceType)]
    pub fn service_type(&self) -> &ns::String;
}

#[objc::obj_trait]
pub trait Delegate: objc::Obj {
    /// Found a nearby advertising peer.
    #[objc::msg_send(browser:foundPeer:withDiscoveryInfo:)]
    fn browser_found_peer(
        &mut self,
        browser: &NearbyServiceBrowser,
        peer: &mc::PeerId,
        discorvery_info: Option<&ns::Dictionary<ns::String, ns::String>>,
    );

    /// A nearby peer has stopped advertising.
    #[objc::msg_send(browser:lostPeer:)]
    fn browser_lost_peer(&mut self, browser: &NearbyServiceBrowser, peer: &mc::PeerId);

    #[objc::optional]
    #[objc::msg_send(browser:didNotStartBrowsingForPeers:)]
    fn browser_did_not_start_browsing(&mut self, browser: &NearbyServiceBrowser, error: &ns::Error);
}

define_obj_type!(
    pub AnyDelegate(ns::Id)
);

impl Delegate for AnyDelegate {}

#[link(name = "mc", kind = "static")]
extern "C" {
    static MC_NEARBY_SERVICE_BROWSER: &'static objc::Class<NearbyServiceBrowser>;
}

#[cfg(test)]
mod tests {
    use crate::{mc, ns};

    #[test]
    fn basics() {
        let name = ns::str!(c"test");
        let peer = mc::PeerId::with_display_name(name).unwrap();
        let service = ns::str!(c"cidre-txtchat");
        let browser = mc::NearbyServiceBrowser::with_peer(&peer, &service).unwrap();
        assert_eq!(browser.service_type(), service);
        assert_eq!(browser.my_peer_id(), &peer);

        assert!(browser.delegate().is_none());
    }
}
